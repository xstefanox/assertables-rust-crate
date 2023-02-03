/// Assert a command (built with program and args) stderr string is greater than or equal to an expression.
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
/// * [`assert_program_args_stderr_ge_expr`]
/// * [`assert_program_args_stderr_ge_expr_as_result`]
/// * [`debug_assert_program_args_stderr_ge_expr`]
///
#[macro_export]
macro_rules! assert_program_args_stderr_ge_expr_as_result {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => ({
        let mut a_command = ::std::process::Command::new($a_program);
        a_command.args($a_args);
        let a_output = a_command.output();
        if a_output.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_program_args_stderr_ge_expr!(left_program, left_args, right_expr)`\n",
                    " left_program label: `{}`,\n",
                    " left_program debug: `{:?}`,\n",
                    "    left_args label: `{}`,\n",
                    "    left_args debug: `{:?}`,\n",
                    "   right_expr label: `{}`,\n",
                    "   right_expr debug: `{:?}`,\n",
                    "        left output: `{:?}`"
                ),
                stringify!($a_program), $a_program,
                stringify!($a_args), $a_args,
                stringify!($b_expr), $b_expr,
                a_output
            ))
        } else {
            let a_string = String::from_utf8(a_output.unwrap().stderr).unwrap();
            if a_string >= $b_expr {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_program_args_stderr_ge_expr!(left_program, left_args, right_expr)`\n",
                        " left_program label: `{}`,\n",
                        " left_program debug: `{:?}`,\n",
                        "    left_args label: `{}`,\n",
                        "    left_args debug: `{:?}`,\n",
                        "   right_expr label: `{}`,\n",
                        "   right_expr debug: `{:?}`,\n",
                        "               left: `{:?}`,\n",
                        "              right: `{:?}`"
                    ),
                    stringify!($a_program), $a_program,
                    stringify!($a_args), $a_args,
                    stringify!($b_expr), $b_expr,
                    a_string,
                    $b_expr
                ))
            }
        }
    });
}

/// Assert a command (built with program and args) stderr string is greater than or equal to an expression.
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
///
/// # fn main() {
/// // Return Ok because a > b
/// let program = "bin/printf-stderr";
/// let args = ["%s", "hello"];
/// let s = String::from("hallo");
/// assert_program_args_stderr_ge_expr!(&program, &args, s);
/// //-> ()
///
/// // Return Ok because a == b
/// let program = "bin/printf-stderr";
/// let args = ["%s", "hello"];
/// let s = String::from("hello");
/// assert_program_args_stderr_ge_expr!(&program, &args, s);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let program = "bin/printf-stderr";
/// let args = ["%s", "hello"];
/// let s = String::from("hullo");
/// assert_program_args_stderr_ge_expr!(&program, &args, s);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_program_args_stderr_ge_expr!(left_program, left_args, right_expr)`\n",
///     " left_program label: `&program`,\n",
///     " left_program debug: `\"bin/printf-stderr\"`,\n",
///     "    left_args label: `&args`,\n",
///     "    left_args debug: `[\"%s\", \"hello\"]`,\n",
///     "   right_expr label: `s`,\n",
///     "   right_expr debug: `\"hullo\"`,\n",
///     "               left: `\"hello\"`,\n",
///     "              right: `\"hullo\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_program_args_stderr_ge_expr`]
/// * [`assert_program_args_stderr_ge_expr_as_result`]
/// * [`debug_assert_program_args_stderr_ge_expr`]
///
#[macro_export]
macro_rules! assert_program_args_stderr_ge_expr {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => ({
        match assert_program_args_stderr_ge_expr_as_result!($a_program, $a_args, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_program:expr, $a_args:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_program_args_stderr_ge_expr_as_result!($a_program, $a_args, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a command (built with program and args) stderr string is greater than or equal to an expression.
///
/// This macro provides the same statements as [`assert_program_args_stderr_ge_expr`],
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
/// * [`assert_program_args_stderr_ge_expr`]
/// * [`assert_program_args_stderr_ge_expr`]
/// * [`debug_assert_program_args_stderr_ge_expr`]
///
#[macro_export]
macro_rules! debug_assert_program_args_stderr_ge_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stderr_ge_expr!($($arg)*);
        }
    };
}
