use log::{Level, Record};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    panic,
    panic::AssertUnwindSafe,
    ptr,
    time::{Duration, Instant},
};
use wasmer_runtime::{
    error::{CallError, Error as WasmerError},
    Array, Ctx, Instance, WasmPtr,
};

pub trait Environment {
    fn elapsed(&self) -> Result<Duration, Error>;

    fn read_input(
        &self,
        address: usize,
        buffer: &mut [u8],
    ) -> Result<(), Error>;

    fn write_output(
        &mut self,
        address: usize,
        buffer: &[u8],
    ) -> Result<(), Error>;

    fn log(&self, record: &Record<'_>) -> Result<(), Error>;

    fn get_variable(&self, name: &str) -> Result<Value, Error>;

    fn set_variable(&mut self, name: &str, value: Value) -> Result<(), Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tried to access an out-of-bounds address")]
    AddressOutOfBounds,
    #[error("Unknown variable")]
    UnknownVariable,
    #[error("Incorrect variable type")]
    BadVariableType,
    #[error("An error occurred while calling a WASM function")]
    Wasm(#[source] CallError),
    #[error("A custom error occurred")]
    Other(#[source] Box<dyn std::error::Error>),
}

impl Error {
    fn code(&self) -> i32 {
        match self {
            Error::AddressOutOfBounds => WASM_ADDRESS_OUT_OF_BOUNDS,
            Error::UnknownVariable => WASM_UNKNOWN_VARIABLE,
            Error::BadVariableType => WASM_BAD_VARIABLE_TYPE,
            _ => WASM_GENERIC_ERROR,
        }
    }
}

impl From<CallError> for Error {
    fn from(other: CallError) -> Error { Error::Wasm(other) }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Integer(i32),
    Float(f64),
}

impl Value {
    pub fn kind(&self) -> Type {
        match *self {
            Value::Bool(_) => Type::Bool,
            Value::Integer(_) => Type::Integer,
            Value::Float(_) => Type::Float,
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = Error;

    fn try_from(other: Value) -> Result<bool, Self::Error> {
        match other {
            Value::Bool(b) => Ok(b),
            _ => Err(Error::BadVariableType),
        }
    }
}

impl TryFrom<Value> for i32 {
    type Error = Error;

    fn try_from(other: Value) -> Result<i32, Self::Error> {
        match other {
            Value::Integer(i) => Ok(i),
            _ => Err(Error::BadVariableType),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = Error;

    fn try_from(other: Value) -> Result<f64, Self::Error> {
        match other {
            Value::Float(f) => Ok(f),
            _ => Err(Error::BadVariableType),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Value { Value::Bool(b) }
}

impl From<i32> for Value {
    fn from(i: i32) -> Value { Value::Integer(i) }
}
impl From<f64> for Value {
    fn from(d: f64) -> Value { Value::Float(d) }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Bool,
    Integer,
    Float,
}

/// A user-provided program loaded into memory.
pub struct Program {
    instance: wasmer_runtime::Instance,
}

impl Program {
    pub fn load(wasm: &[u8]) -> Result<Self, WasmerError> {
        let imports = wasmer_runtime::imports! {
            "env" => {
                "print" => wasmer_runtime::func!(print),
                "wasm_log" => wasmer_runtime::func!(wasm_log),
                "wasm_read_input" => wasmer_runtime::func!(wasm_read_input),
                "wasm_write_output" => wasmer_runtime::func!(wasm_write_output),
                "wasm_current_time" => wasmer_runtime::func!(wasm_current_time),
                "wasm_variable_read_boolean" => wasmer_runtime::func!(wasm_variable_read_boolean),
                "wasm_variable_read_double" => wasmer_runtime::func!(wasm_variable_read_double),
                "wasm_variable_read_int" => wasmer_runtime::func!(wasm_variable_read_int),
                "wasm_variable_write_boolean" => wasmer_runtime::func!(wasm_variable_write_boolean),
                "wasm_variable_write_double" => wasmer_runtime::func!(wasm_variable_write_double),
                "wasm_variable_write_int" => wasmer_runtime::func!(wasm_variable_write_int),
            },
        };
        let instance = wasmer_runtime::instantiate(wasm, &imports)?;

        Ok(Program { instance })
    }

    pub fn poll(&mut self, env: &mut dyn Environment) -> Result<(), Error> {
        let mut state = State { env };
        self.instance.context_mut().data = &mut state as *mut State as *mut _;

        self.with_environment_context(env, |instance| {
            instance.call("poll", &[])?;
            Ok(())
        })
    }

    fn with_environment_context<F, T>(
        &mut self,
        env: &mut dyn Environment,
        func: F,
    ) -> Result<T, Error>
    where
        F: FnOnce(&Instance) -> Result<T, Error>,
    {
        let mut state = State { env };
        let instance = &mut self.instance;

        // point the data pointer at our temporary state.
        instance.context_mut().data = &mut state as *mut State<'_> as *mut _;
        // we can't use the old state variable any more (we'd have aliased
        // pointers) so deliberately shadow it
        #[allow(unused_variables)]
        let state = ();

        // execute the callback. We need to catch panics so we can clear the
        // data pointer no matter what. Using AssertUnwindSafe is
        // correct here because we'll continue panicking once the data
        // pointer is cleared
        let got = panic::catch_unwind(AssertUnwindSafe(|| func(instance)));

        // make sure the context data pointer is cleared. We don't need to drop
        // anything because it was just a `&mut State
        instance.context_mut().data = ptr::null_mut();

        match got {
            Ok(value) => value,
            Err(e) => panic::resume_unwind(e),
        }
    }
}

/// Temporary state passed to each host function via [`Ctx::data`].
struct State<'a> {
    env: &'a mut dyn Environment,
}

/// Print the provided message to the screen.
///
/// Returns `-1` if the operation failed.
fn print(ctx: &mut Ctx, msg: WasmPtr<u8, Array>, length: u32) -> i32 {
    match msg.get_utf8_string(ctx.memory(0), length) {
        Some(msg) => {
            print!("{}", msg);
            0
        },
        None => -1,
    }
}

const WASM_SUCCESS: i32 = 0;
const WASM_GENERIC_ERROR: i32 = 1;
const WASM_ADDRESS_OUT_OF_BOUNDS: i32 = 2;
const WASM_UNKNOWN_VARIABLE: i32 = 3;
const WASM_BAD_VARIABLE_TYPE: i32 = 4;

/// Convenience macro for executing a method using the [`Environment`] pointer
/// attached to [`Ctx::data`].
///
/// # Safety
///
/// This assumes the [`Ctx`] was set up correctly using
/// [`Program::with_environment_context()`].
macro_rules! try_with_env {
    ($ctx:expr, $method:ident ( $($arg:expr),* ), $failure_msg:expr) => {{
        // the data pointer should have been set by `with_environment_context()`
        if $ctx.data.is_null() {
            return WASM_GENERIC_ERROR;
        }

        let state = &mut *($ctx.data as *mut State);

        // call the method using the provided arguments
        match state.env.$method( $( $arg ),* ) {
            // happy path
            Ok(value) => value,
            Err(e) => {
                // log the original error using the failure_msg
                log::error!(concat!($failure_msg, ": {}"), e);

                // then iterate through the causes and log those too
                let mut cause = std::error::Error::source(&e);
                while let Some(inner) = cause {
                    log::error!("Caused by: {}", inner);
                    cause = inner.source();
                }

                // the operation failed, return the corresponding error code
                return e.code();
            }
        }
    }};
}

macro_rules! wasm_deref {
    (with $ctx:expr, *$ptr:ident = for $item:ident in $collection:expr) => {
        match $ptr.deref(
            $ctx.memory(0),
            0,
            $collection.len().try_into().unwrap(),
        ) {
            Some(slice) => {
                for (src, dest) in $collection.iter().zip(slice.iter()) {
                    dest.set(*src);
                }
            },
            None => return WASM_GENERIC_ERROR,
        }
    };
    (with $ctx:expr, * $ptr:ident = $value:expr) => {
        match $ptr.deref($ctx.memory(0)) {
            Some(cell) => cell.set($value),
            None => return WASM_GENERIC_ERROR,
        }
    };
}

fn wasm_current_time(
    ctx: &mut Ctx,
    secs: WasmPtr<u64>,
    nanos: WasmPtr<u32>,
) -> i32 {
    let elapsed = unsafe {
        try_with_env!(ctx, elapsed(), "Unable to calculate the elapsed time")
    };

    wasm_deref!(with ctx, *secs = elapsed.as_secs());
    wasm_deref!(with ctx, *nanos = elapsed.subsec_nanos());

    WASM_SUCCESS
}

const LOG_ERROR: i32 = 0;
const LOG_WARN: i32 = 1;
const LOG_INFO: i32 = 2;
const LOG_DEBUG: i32 = 3;
const LOG_TRACE: i32 = 4;

fn wasm_log(
    ctx: &mut Ctx,
    level: i32,
    file: WasmPtr<u8, Array>,
    file_len: i32,
    line: i32,
    message: WasmPtr<u8, Array>,
    message_len: i32,
) -> i32 {
    // Note: We can't directly accept the Level enum here because out-of-range
    // enum variants are insta-UB
    let level = match level {
        LOG_ERROR => Level::Error,
        LOG_WARN => Level::Warn,
        LOG_INFO => Level::Info,
        LOG_DEBUG => Level::Debug,
        LOG_TRACE => Level::Trace,
        _ => Level::Debug,
    };
    let filename = file.get_utf8_string(ctx.memory(0), file_len as u32);
    let message = message
        .get_utf8_string(ctx.memory(0), message_len as u32)
        .unwrap_or_default()
        .trim_end();

    unsafe {
        try_with_env!(
            ctx,
            // unfortunately constructing a log and using it needs to be in a
            // single statement because lifetimes
            log(&Record::builder()
                .level(level)
                .file(filename.as_deref())
                .line(Some(line as u32))
                .args(format_args!("{}", message))
                .build()),
            "Logging failed"
        );
    }

    WASM_SUCCESS
}

fn wasm_read_input(
    ctx: &mut Ctx,
    address: u32,
    buffer: WasmPtr<u8, Array>,
    buffer_len: i32,
) -> i32 {
    let mut temp_buffer = vec![0; buffer_len.try_into().unwrap()];

    unsafe {
        try_with_env!(
            ctx,
            read_input(address.try_into().unwrap(), &mut temp_buffer[..]),
            "Unable to read the input"
        );
    }

    wasm_deref!(with ctx, *buffer = for byte in temp_buffer);

    WASM_SUCCESS
}

fn wasm_write_output(
    ctx: &mut Ctx,
    address: u32,
    data: WasmPtr<u8, Array>,
    data_len: i32,
) -> i32 {
    let buffer: Vec<u8> =
        match data.deref(ctx.memory(0), 0, data_len.try_into().unwrap()) {
            Some(slice) => slice.iter().map(|cell| cell.get()).collect(),
            None => return WASM_GENERIC_ERROR,
        };

    unsafe {
        try_with_env!(
            ctx,
            write_output(address.try_into().unwrap(), &buffer),
            "Unable to set outputs"
        );
    }

    WASM_SUCCESS
}

fn variable_get_and_map<F, Q, T>(
    ctx: &mut Ctx,
    name: WasmPtr<u8, Array>,
    name_len: i32,
    value: WasmPtr<Q>,
    map: F,
) -> i32
where
    F: FnOnce(T) -> Q,
    T: TryFrom<Value>,
    Q: wasmer_runtime::types::ValueType,
{
    let name = match name
        .get_utf8_string(ctx.memory(0), name_len.try_into().unwrap())
    {
        Some(n) => n,
        None => return WASM_GENERIC_ERROR,
    };

    let variable = unsafe {
        try_with_env!(
            ctx,
            get_variable(name),
            "Unable to retrieve the variable"
        )
    };

    let variable = match T::try_from(variable) {
        Ok(v) => v,
        _ => return WASM_BAD_VARIABLE_TYPE,
    };

    match value.deref(ctx.memory(0)) {
        Some(cell) => {
            cell.set(map(variable));
            WASM_SUCCESS
        },
        None => WASM_GENERIC_ERROR,
    }
}

fn wasm_variable_read_boolean(
    ctx: &mut Ctx,
    name: WasmPtr<u8, Array>,
    name_len: i32,
    value: WasmPtr<u8>,
) -> i32 {
    variable_get_and_map(
        ctx,
        name,
        name_len,
        value,
        |b: bool| if b { 1 } else { 0 },
    )
}

fn wasm_variable_read_int(
    ctx: &mut Ctx,
    name: WasmPtr<u8, Array>,
    name_len: i32,
    value: WasmPtr<i32>,
) -> i32 {
    variable_get_and_map(ctx, name, name_len, value, |i| i)
}

fn wasm_variable_read_double(
    ctx: &mut Ctx,
    name: WasmPtr<u8, Array>,
    name_len: i32,
    value: WasmPtr<f64>,
) -> i32 {
    variable_get_and_map(ctx, name, name_len, value, |d| d)
}

fn set_variable<F, Q, T>(
    ctx: &mut Ctx,
    name: WasmPtr<u8, Array>,
    name_len: i32,
    value: Q,
    map: F,
) -> i32
where
    F: FnOnce(Q) -> T,
    T: Into<Value>,
{
    let name = match name
        .get_utf8_string(ctx.memory(0), name_len.try_into().unwrap())
    {
        Some(n) => n,
        None => return WASM_GENERIC_ERROR,
    };

    let value = map(value).into();

    unsafe {
        try_with_env!(
            ctx,
            set_variable(name, value),
            "Unable to set the variable"
        )
    };

    WASM_SUCCESS
}

fn wasm_variable_write_boolean(
    ctx: &mut Ctx,
    name: WasmPtr<u8, Array>,
    name_len: i32,
    value: u8,
) -> i32 {
    set_variable(ctx, name, name_len, value, |v| v != 0)
}

fn wasm_variable_write_int(
    ctx: &mut Ctx,
    name: WasmPtr<u8, Array>,
    name_len: i32,
    value: i32,
) -> i32 {
    set_variable(ctx, name, name_len, value, |v| v)
}

fn wasm_variable_write_double(
    ctx: &mut Ctx,
    name: WasmPtr<u8, Array>,
    name_len: i32,
    value: f64,
) -> i32 {
    set_variable(ctx, name, name_len, value, |v| v)
}

pub struct InMemory {
    inputs: [u8; 128],
    outputs: [u8; 128],
    variables: HashMap<String, Value>,
    started: Instant,
}

impl Default for InMemory {
    fn default() -> InMemory {
        InMemory {
            inputs: [0; 128],
            outputs: [0; 128],
            variables: HashMap::default(),
            started: Instant::now(),
        }
    }
}

impl Environment for InMemory {
    fn elapsed(&self) -> Result<Duration, Error> { Ok(self.started.elapsed()) }

    fn read_input(
        &self,
        address: usize,
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        let inputs = self
            .inputs
            .get(address..address + buffer.len())
            .ok_or(Error::AddressOutOfBounds)?;

        buffer.copy_from_slice(inputs);
        Ok(())
    }

    fn write_output(
        &mut self,
        address: usize,
        buffer: &[u8],
    ) -> Result<(), Error> {
        let dest = self
            .outputs
            .get_mut(address..address + buffer.len())
            .ok_or(Error::AddressOutOfBounds)?;

        dest.copy_from_slice(buffer);
        Ok(())
    }

    fn log(&self, record: &Record) -> Result<(), Error> {
        log::logger().log(record);
        Ok(())
    }

    fn get_variable(&self, name: &str) -> Result<Value, Error> {
        self.variables
            .get(name)
            .copied()
            .ok_or(Error::UnknownVariable)
    }

    fn set_variable(&mut self, name: &str, value: Value) -> Result<(), Error> {
        use std::collections::hash_map::Entry;

        match self.variables.entry(name.to_string()) {
            Entry::Vacant(entry) => {
                // the variable doesn't exist, we can just insert it
                entry.insert(value);
            },
            Entry::Occupied(mut entry) => {
                // make sure we aren't trying to do something like overwriting a
                // bool with a float
                if entry.get().kind() != value.kind() {
                    return Err(Error::BadVariableType);
                }

                entry.insert(value);
            },
        }

        Ok(())
    }
}
