/// Assert one function output is less than or equal to another function output.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// // Return Ok
/// let a: i32 = 1;
/// let b: i32 = -2;
/// let x = assert_fn_le_other_as_result!(i32::abs, a, b);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a: i32 = -2;
/// let b: i32 = 1;
/// let x = assert_fn_le_other_as_result!(i32::abs, a, b);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_fn_le_other!(pair_function, left_input, right_input)`\n",
///     " pair_function label: `i32::abs`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `-2`,\n",
///     "   right_input label: `b`,\n",
///     "   right_input debug: `1`,\n",
///     "                left: `2`,\n",
///     "               right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
/// 
/// * [`assert_fn_le_other`]
/// * [`assert_fn_le_other_as_result`]
/// * [`debug_assert_fn_le_other`]
///
#[macro_export]
macro_rules! assert_fn_le_other_as_result {
    ($function:path, $a_input:expr, $b_input:expr $(,)?) => ({
        let a_output = $function($a_input);
        let b_output = $function($b_input);
        if a_output <= b_output {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_le_other!(pair_function, left_input, right_input)`\n",
                    " pair_function label: `{}`,\n",
                    "    left_input label: `{}`,\n",
                    "    left_input debug: `{:?}`,\n",
                    "   right_input label: `{}`,\n",
                    "   right_input debug: `{:?}`,\n",
                    "                left: `{:?}`,\n",
                    "               right: `{:?}`"
                ),
                stringify!($function),
                stringify!($a_input), $a_input,
                stringify!($b_input), $b_input,
                a_output,
                b_output
            ))
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_fn_le_other_as_result_x_success_because_lt() {
        let a: i32 = 1;
        let b: i32 = -2;
        let x = assert_fn_le_other_as_result!(i32::abs, a, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_le_other_as_result_x_success_because_eq() {
        let a: i32 = 1;
        let b: i32 = -1;
        let x = assert_fn_le_other_as_result!(i32::abs, a, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_le_other_as_result_x_failure_because_gt() {
        let a: i32 = -2;
        let b: i32 = 1;
        let x = assert_fn_le_other_as_result!(i32::abs, a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_le_other!(pair_function, left_input, right_input)`\n",
                " pair_function label: `i32::abs`,\n",
                "    left_input label: `a`,\n",
                "    left_input debug: `-2`,\n",
                "   right_input label: `b`,\n",
                "   right_input debug: `1`,\n",
                "                left: `2`,\n",
                "               right: `1`"
            )
        );
    }
}

/// Assert a function output is less than or equal to another.
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// // Return Ok
/// let a: i32 = 1;
/// let b: i32 = -2;
/// assert_fn_le_other!(i32::abs, a, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a: i32 = -2;
/// let b: i32 = 1;
/// assert_fn_le_other!(i32::abs, a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_le_other!(pair_function, left_input, right_input)`\n",
///     " pair_function label: `i32::abs`,\n",
///     "    left_input label: `a`,\n",
///     "    left_input debug: `-2`,\n",
///     "   right_input label: `b`,\n",
///     "   right_input debug: `1`,\n",
///     "                left: `2`,\n",
///     "               right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
/// 
/// * [`assert_fn_le_other`]
/// * [`assert_fn_le_other_as_result`]
/// * [`debug_assert_fn_le_other`]
///
#[macro_export]
macro_rules! assert_fn_le_other {
    ($function:path, $a_input:expr, $b_input:expr $(,)?) => ({
        match assert_fn_le_other_as_result!($function, $a_input, $b_input) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($function:path, $a_input:expr, $b_input:expr, $($message:tt)+) => ({
        match assert_fn_le_other_as_result!($function, $a_input, $b_input) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
