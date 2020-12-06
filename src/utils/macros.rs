//! Helper macros
//!
//! These helper macros help reduce boilerplate, improving the readability
//! of Ellie's code. Right now, there is only one macro, `ifelse!()`, which
//! helps reduce boilerplate when writing if-else statements.


/// The ifelse!() macro. Helps reduce boilerplate when writing any if-else
/// statements.
///
/// The macro can be used like so:
/// ```
/// use crate::ifelse;
///
/// pub fn main() {
///   return ifelse!(1 + 1 == 2, true, false);
/// }
/// ```
#[macro_export]
macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

#[cfg(test)]
mod tests {
    use ifelse;

    /// This test makes sure the `ifelse!()` macro works by using
    /// the macro to determine whether or not 1 + 1 == 2. This is
    /// inherently a very simple test and it doesn't do all that much,
    /// but it does do its job by confirming whether or not the macro
    /// works.
    #[test]
    fn test_ifelse_macro() {
        assert_eq!(ifelse!(1 + 1 == 2, true, false), true);
    }
}
