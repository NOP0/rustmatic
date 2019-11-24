/// A helper to generate a test that parses the contents of a "*.st" file in the
/// "tests/data/" directory.
macro_rules! parse_data_file {
    ($( $( #[$attr:meta])* $name:ident, )*) => {
        $(
            parse_data_file!($(#[$attr])* $name);
        )*
    };
    ($( #[$attr:meta])* $name:ident) => {
        #[test]
        $( #[$attr] )*
        fn $name() {
            let src = include_str!(concat!("data/", stringify!($name), ".st"));
            rustmatic_structured_text::parse(src).unwrap();
        }
    };
}

parse_data_file! {
    #[ignore]
    first_wikipedia_sample,
    #[ignore]
    function_block,
}
