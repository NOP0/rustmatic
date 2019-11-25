/// A helper to generate a test that parses the contents of a "*.st" file in the
/// "tests/data/" directory.
macro_rules! parse_data_file {
    ($( $( #[$attr:meta] )* $name:ident => $parses_to:ident, )*) => {
        $(
            #[test]
            $( #[$attr] )*
            fn $name() {
                let src = include_str!(concat!("data/", stringify!($name), ".st"));

                use rustmatic_structured_text::parser::{RawParser, Rule};
                use pest::Parser as _;

                if let Err(e) = RawParser::parse(Rule::$parses_to, src) {
                    panic!("Parse failed: {0}\n\n{0:#?}", e);
                }
            }
        )*
    };
}

parse_data_file! {
    #[ignore]
    first_wikipedia_sample => file,
    #[ignore]
    function_block => function_block,
    #[ignore]
    if_start_while_initialized_prime_system => conditional,
    if_chain => conditional,
    simple_program => program,
}
