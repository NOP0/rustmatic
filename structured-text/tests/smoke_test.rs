use rustmatic_structured_text::{File, Program};
use std::error::Error;

/// A helper to generate a test that parses the contents of a "*.st" file in the
/// "tests/data/" directory.
macro_rules! parse_data_file {
    ($( $( #[$attr:meta] )* $name:ident => $parses_to:ident $(=> $from_str_ty:ty)? ),* $(,)?) => {
        $(
            #[test]
            $( #[$attr] )*
            fn $name() {
                let src = include_str!(concat!("data/", stringify!($name), ".st"));

                use rustmatic_structured_text::parser::{RawParser, Rule};
                use pest::Parser as _;

                if let Err(e) = RawParser::parse(Rule::$parses_to, src) {
                    on_error(e);
                }

                $(
                    if let Err(e) = <$from_str_ty as std::str::FromStr>::from_str(src) {
                        on_error(e);
                    }
                )*
            }
        )*
    };
}

fn on_error<E: Error>(e: E) -> ! {
    println!("Error: {}", e);

    let mut source = e.source();
    while let Some(s) = source {
        println!("Caused by: {}", s);
        source = s.source();
    }

    println!();

    panic!("{:#?}", e);
}

parse_data_file! {
    #[ignore]
    first_wikipedia_sample => file,
    #[ignore]
    function_block => function_block,
    #[ignore]
    if_start_while_initialized_prime_system => conditional,
    if_chain => conditional,
    simple_program => program => Program,
    function_and_program => file => File,
}
