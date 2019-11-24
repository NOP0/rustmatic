/// A helper to generate a test that parses the contents of a "*.st" file in the
/// "tests/data/" directory.
macro_rules! parse_data_file {
    ($( $( #[$attr:meta] )* $name:ident => $parses_to:ident, )*) => {
        $(
            #[test]
            $( #[$attr] )*
            fn $name() {
                let src = include_str!(concat!("data/", stringify!($name), ".st"));

                use rustmatic_structured_text::parser::{Parser, Rule};
                use pest::Parser as _;

                Parser::parse(Rule::$parses_to, src).unwrap();
            }
        )*
    };
}

parse_data_file! {
    #[ignore]
    first_wikipedia_sample => program,
    #[ignore]
    function_block => function_block,
    #[ignore]
    if_start_while_initialized_prime_system => conditional,
    #[ignore]
    if_chain => conditional,
}
