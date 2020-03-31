#[macro_export]
macro_rules! lpr_init{
           [ $($elements:tt)* ] => {
                $(
                 $elements
            )*
           }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
