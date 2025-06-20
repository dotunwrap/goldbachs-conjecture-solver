#[macro_export]
macro_rules! parametrized_test {
    ($name: ident, $test: expr, $($param: expr),*) => {
        #[test]
        fn $name() {
            $(
                ($test)($param);
            )*
        }
    };
}
