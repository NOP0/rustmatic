mod compile;
mod environment;
mod test_case;

pub use compile::Compiler;
pub use environment::TestEnvironment;
pub use test_case::{Pass, Recipe, TestCase};

use anyhow::{Context, Error};

pub fn run_test_case(
    compiler: &Compiler,
    test_case: &TestCase,
) -> Result<(), Error> {
    let mut wasm = compiler
        .instantiate(&test_case.name, &test_case.src)
        .context("Unable to load the WASM module")?;
    let mut env = TestEnvironment::default();

    for pass in &test_case.recipe.passes {
        env.setup(pass);

        log::debug!("Polling \"{}\" at {:?}", test_case.name, env.elapsed);
        log::trace!("Environment: {:?}", env);

        wasm.poll(&mut env)
            .map_err(|e| Error::msg(e.to_string()))
            .context("Polling failed")?;

        env.compare(pass).context("Output comparison failed")?;
    }

    Ok(())
}
