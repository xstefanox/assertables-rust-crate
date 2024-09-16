//! Assert an expression is equal to another.
//!
//! * If true, return Result `Ok(())`.
//!
//! * Otherwise, return Result `Err` with a diagnostic message.
//!
//! This macro provides the same statements as [`assert_`](macro.assert_.html),
//! except this macro returns a Result, rather than doing a panic.
//!
//! This macro is useful for runtime checks, such as checking parameters,
//! or sanitizing inputs, or handling different results in different ways.
//!
//! # Module macro
//!
//! * [`assert_eq_as_result`](macro@crate::assert_eq_as_result)
//!
//! # Rust standard macros
//!
//! * [`assert_eq`](https://doc.rust-lang.org/std/macro.assert_eq.html)
//! * [`debug_assert_eq`](https://doc.rust-lang.org/std/macro.debug_assert_eq.html)

/// Assert an expression is equal to another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_eq_as_result`](macro@crate::assert_eq_as_result)
///
/// # Rust standard macros
///
/// * [`assert_eq`](https://doc.rust-lang.org/std/macro.assert_eq.html)
/// * [`debug_assert_eq`](https://doc.rust-lang.org/std/macro.debug_assert_eq.html)
///
#[macro_export]
macro_rules! assert_eq_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a, b) => {
                if a == b {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_eq!(a, b)`\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
                        ),
                        stringify!($a),
                        a,
                        stringify!($b),
                        b
                    ))
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_eq_as_result_x_success() {
        let a: i32 = 1;
        let b: i32 = 1;
        let result = assert_eq_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_eq_as_result_x_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let result = assert_eq_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_eq!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `2`",
            )
        );
    }
}
