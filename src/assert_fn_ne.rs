/// Assert a function output is not equal to another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or santizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_fn_ne`]
/// * [`assert_fn_ne_as_result`]
/// * [`debug_assert_fn_ne`]
///
#[macro_export]
macro_rules! assert_fn_ne_as_result {

    //// Arity 0

    ($a_function:path, $b_function:path $(,)?) => ({
        let a_output = $a_function();
        let b_output = $b_function();
        if a_output != b_output {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_ne!(left_function, right_function)`\n",
                    "  left_function label: `{}`,\n",
                    " right_function label: `{}`,\n",
                    "                 left: `{:?}`,\n",
                    "                right: `{:?}`"
                ),
                stringify!($a_function),
                stringify!($b_function),
                a_output,
                b_output
            ))
        }
    });

    //// Arity 1

    ($a_function:path, $a_input:expr, $b_function:path, $b_input:expr $(,)?) => ({
        let a_output = $a_function($a_input);
        let b_output = $b_function($b_input);
        if a_output != b_output {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fn_ne!(left_function, left_input, right_function, right_input)`\n",
                    "  left_function label: `{}`,\n",
                    "     left_input label: `{}`,\n",
                    "     left_input debug: `{:?}`,\n",
                    " right_function label: `{}`,\n",
                    "    right_input label: `{}`,\n",
                    "    right_input debug: `{:?}`,\n",
                    "                 left: `{:?}`,\n",
                    "                right: `{:?}`"
                ),
                stringify!($a_function),
                stringify!($a_input), $a_input,
                stringify!($b_function),
                stringify!($b_input), $b_input,
                a_output,
                b_output
            ))
        }
    });

}

#[cfg(test)]
mod test_x_result {

    //// Arity 0

    fn one() -> i8 {
        return 1;
    }

    fn two() -> i8 {
        return 2;
    }

    #[test]
    fn test_assert_fn_ne_as_result_x_arity_0_x_success() {
        let x = assert_fn_ne_as_result!(one, two);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_ne_as_result_x_arity_0_x_failure() {
        let x = assert_fn_ne_as_result!(one, one);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_ne!(left_function, right_function)`\n",
                "  left_function label: `one`,\n",
                " right_function label: `one`,\n",
                "                 left: `1`,\n",
                "                right: `1`"
            )
        );
    }

    //// Arity 1

    #[test]
    fn test_assert_fn_ne_as_result_x_arity_1_x_success() {
        let a: i32 = -1;
        let b: i32 = 2;
        let x = assert_fn_ne_as_result!(i32::abs, a, i32::abs, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_fn_ne_as_result_x_arity_1_x_failure() {
        let a: i32 = -1;
        let b: i32 = 1;
        let x = assert_fn_ne_as_result!(i32::abs, a, i32::abs, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_fn_ne!(left_function, left_input, right_function, right_input)`\n",
                "  left_function label: `i32::abs`,\n",
                "     left_input label: `a`,\n",
                "     left_input debug: `-1`,\n",
                " right_function label: `i32::abs`,\n",
                "    right_input label: `b`,\n",
                "    right_input debug: `1`,\n",
                "                 left: `1`,\n",
                "                right: `1`"
            )
        );
    }

}

/// Assert a function output is not equal to another.
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
/// let a: i32 = -1;
/// let b: i32 = 2;
/// assert_fn_ne!(i32::abs, a, i32::abs, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a: i32 = -1;
/// let b: i32 = 1;
/// assert_fn_ne!(i32::abs, a, i32::abs, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fn_ne!(left_function, left_input, right_function, right_input)`\n",
///     "  left_function label: `i32::abs`,\n",
///     "     left_input label: `a`,\n",
///     "     left_input debug: `-1`,\n",
///     " right_function label: `i32::abs`,\n",
///     "    right_input label: `b`,\n",
///     "    right_input debug: `1`,\n",
///     "                 left: `1`,\n",
///     "                right: `1`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_fn_ne`]
/// * [`assert_fn_ne_as_result`]
/// * [`debug_assert_fn_ne`]
///
#[macro_export]
macro_rules! assert_fn_ne {

    //// Arity 0

    ($a_function:path, $b_function:path $(,)?) => ({
        match assert_fn_ne_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });

    ($a_function:path, $b_function:path, $($message:tt)+) => ({
        match assert_fn_ne_as_result!($a_function, $b_function) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
    
    //// Arity 1
    
    ($a_function:path, $a_input:expr, $b_function:path, $b_input:expr $(,)?) => ({
        match assert_fn_ne_as_result!($a_function, $a_input, $b_function, $b_input) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });

    ($a_function:path, $a_input:expr, $b_function:path, $b_input:expr, $($message:tt)+) => ({
        match assert_fn_ne_as_result!($a_function, $a_input, $b_function, $b_input) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });

}

/// Assert a function output is not equal to another.
///
/// This macro provides the same statements as [`assert_fn_ne`],
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-C debug-assertions` is passed to the compiler.
///
/// This macro is useful for checks that are too expensive to be present
/// in a release build but may be helpful during development.
///
/// The result of expanding this macro is always type checked.
///
/// An unchecked assertion allows a program in an inconsistent state to
/// keep running, which might have unexpected consequences but does not
/// introduce unsafety as long as this only happens in safe code. The
/// performance cost of assertions, however, is not measurable in general.
/// Replacing `assert*!` with `debug_assert*!` is thus only encouraged
/// after thorough profiling, and more importantly, only in safe code!
///
/// This macro is intendend to work in a similar way to
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Related
///
/// * [`assert_fn_ne`]
/// * [`assert_fn_ne`]
/// * [`debug_assert_fn_ne`]
///
#[macro_export]
macro_rules! debug_assert_fn_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_fn_ne!($($arg)*);
        }
    };
}
