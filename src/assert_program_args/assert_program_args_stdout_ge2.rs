//! Assert a command (built with program and args) stdout string is greater than or equal to another.
//!
//! Pseudocode:<br>
//! (program1 + args1 ⇒ command ⇒ stdout) ≥ (program2 + args2 ⇒ command ⇒ stdout)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a_program = "bin/printf-stdout";
//! let a_args = ["%s", "alfa"];
//! let b_program = "bin/printf-stdout";
//! let b_args = ["%s%s", "a", "a"];
//! assert_program_args_stdout_ge2!(&a_program, &a_args, &b_program, &b_args);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_program_args_stdout_ge2`](macro@crate::assert_program_args_stdout_ge2)
//! * [`assert_program_args_stdout_ge2_as_result`](macro@crate::assert_program_args_stdout_ge2_as_result)
//! * [`debug_assert_program_args_stdout_ge2`](macro@crate::debug_assert_program_args_stdout_ge2)

/// Assert a command (built with program and args) stdout string is greater than or equal to another.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout) ≥ (program2 + args2 ⇒ command ⇒ stdout)
///
/// * If true, return `(a_stdout, b_stdout)`.
///
/// * If true, return Result `Err` with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_program_args_stdout_ge2`](macro@crate::assert_program_args_stdout_ge2)
/// * [`assert_program_args_stdout_ge2_as_result`](macro@crate::assert_program_args_stdout_ge2_as_result)
/// * [`debug_assert_program_args_stdout_ge2`](macro@crate::debug_assert_program_args_stdout_ge2)
///
#[macro_export]
macro_rules! assert_program_args_stdout_ge2_as_result {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => {{
        match ($a_program, $a_args, $b_program, $b_args) {
            (a_program, a_args, b_program, b_args) => {
                match (
                    assert_program_args_impl_prep!(a_program, a_args),
                    assert_program_args_impl_prep!(b_program, b_args)
                ) {
                    (Ok(a_output), Ok(b_output)) => {
                        let a = a_output.stdout;
                        let b = b_output.stdout;
                        if a.ge(&b) {
                            Ok((a, b))
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_program_args_stdout_ge2!(a_program, a_args, b_program, b_args)`\n",
                                        "https://docs.rs/assertables/9.0.0/assertables/macro.assert_program_args_stdout_ge2.html\n",
                                        " a_program label: `{}`,\n",
                                        " a_program debug: `{:?}`,\n",
                                        "    a_args label: `{}`,\n",
                                        "    a_args debug: `{:?}`,\n",
                                        " b_program label: `{}`,\n",
                                        " b_program debug: `{:?}`,\n",
                                        "    b_args label: `{}`,\n",
                                        "    b_args debug: `{:?}`,\n",
                                        "               a: `{:?}`,\n",
                                        "               b: `{:?}`"
                                    ),
                                    stringify!($a_program),
                                    a_program,
                                    stringify!($a_args),
                                    a_args,
                                    stringify!($b_program),
                                    b_program,
                                    stringify!($b_args),
                                    b_args,
                                    a,
                                    b
                                )
                            )
                        }
                    },
                    (a, b) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_program_args_stdout_ge2!(a_program, a_args, b_program, b_args)`\n",
                                    "https://docs.rs/assertables/9.0.0/assertables/macro.assert_program_args_stdout_ge2.html\n",
                                    " a_program label: `{}`,\n",
                                    " a_program debug: `{:?}`,\n",
                                    "    a_args label: `{}`,\n",
                                    "    a_args debug: `{:?}`,\n",
                                    " b_program label: `{}`,\n",
                                    " b_program debug: `{:?}`,\n",
                                    "    b_args label: `{}`,\n",
                                    "    b_args debug: `{:?}`,\n",
                                    "        a output: `{:?}`,\n",
                                    "        b output: `{:?}`"
                                ),
                                stringify!($a_program),
                                a_program,
                                stringify!($a_args),
                                a_args,
                                stringify!($b_program),
                                b_program,
                                stringify!($b_args),
                                b_args,
                                a,
                                b
                            )
                        )
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn gt() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stdout";
        let b_args = ["%s%s", "a", "a"];
        let result =
            assert_program_args_stdout_ge2_as_result!(&a_program, &a_args, &b_program, &b_args);
        assert_eq!(
            result.unwrap(),
            (vec![b'a', b'l', b'f', b'a'], vec![b'a', b'a'])
        );
    }

    #[test]
    fn eq() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stdout";
        let b_args = ["%s%s%s%s", "a", "l", "f", "a"];
        let result =
            assert_program_args_stdout_ge2_as_result!(&a_program, &a_args, &b_program, &b_args);
        assert_eq!(
            result.unwrap(),
            (vec![b'a', b'l', b'f', b'a'], vec![b'a', b'l', b'f', b'a'])
        );
    }

    #[test]
    fn lt() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stdout";
        let b_args = ["%s%s", "z", "z"];
        let result =
            assert_program_args_stdout_ge2_as_result!(&a_program, &a_args, &b_program, &b_args);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_program_args_stdout_ge2!(a_program, a_args, b_program, b_args)`\n",
            "https://docs.rs/assertables/9.0.0/assertables/macro.assert_program_args_stdout_ge2.html\n",
            " a_program label: `&a_program`,\n",
            " a_program debug: `\"bin/printf-stdout\"`,\n",
            "    a_args label: `&a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_program label: `&b_program`,\n",
            " b_program debug: `\"bin/printf-stdout\"`,\n",
            "    b_args label: `&b_args`,\n",
            "    b_args debug: `[\"%s%s\", \"z\", \"z\"]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[122, 122]`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command (built with program and args) stdout string is greater than or equal to another.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout) ≥ (program2 + args2 ⇒ command ⇒ stdout)
///
/// * If true, return `(a_stdout, b_stdout)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
///
/// # fn main() {
/// let a_program = "bin/printf-stdout";
/// let a_args = ["%s", "alfa"];
/// let b_program = "bin/printf-stdout";
/// let b_args = ["%s%s", "a", "a"];
/// assert_program_args_stdout_ge2!(&a_program, &a_args, &b_program, &b_args);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a_program = "bin/printf-stdout";
/// let a_args = ["%s", "alfa"];
/// let b_program = "bin/printf-stdout";
/// let b_args = ["%s%s", "z", "z"];
/// assert_program_args_stdout_ge2!(&a_program, &a_args, &b_program, &b_args);
/// # });
/// // assertion failed: `assert_program_args_stdout_ge2!(a_program, a_args, b_program, b_args)`
/// // https://docs.rs/assertables/9.0.0/assertables/macro.assert_program_args_stdout_ge2.html
/// //  a_program label: `&a_program`,
/// //  a_program debug: `\"bin/printf-stdout\"`,
/// //     a_args label: `&a_args`,
/// //     a_args debug: `[\"%s\", \"alfa\"]`,
/// //  b_program label: `&b_program`,
/// //  b_program debug: `\"bin/printf-stdout\"`,
/// //     b_args label: `&b_args`,
/// //     b_args debug: `[\"%s%s\", \"z\", \"z\"]`,
/// //                a: `[97, 108, 102, 97]`,
/// //                b: `[122, 122]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_program_args_stdout_ge2!(a_program, a_args, b_program, b_args)`\n",
/// #     "https://docs.rs/assertables/9.0.0/assertables/macro.assert_program_args_stdout_ge2.html\n",
/// #     " a_program label: `&a_program`,\n",
/// #     " a_program debug: `\"bin/printf-stdout\"`,\n",
/// #     "    a_args label: `&a_args`,\n",
/// #     "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
/// #     " b_program label: `&b_program`,\n",
/// #     " b_program debug: `\"bin/printf-stdout\"`,\n",
/// #     "    b_args label: `&b_args`,\n",
/// #     "    b_args debug: `[\"%s%s\", \"z\", \"z\"]`,\n",
/// #     "               a: `[97, 108, 102, 97]`,\n",
/// #     "               b: `[122, 122]`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_program_args_stdout_ge2`](macro@crate::assert_program_args_stdout_ge2)
/// * [`assert_program_args_stdout_ge2_as_result`](macro@crate::assert_program_args_stdout_ge2_as_result)
/// * [`debug_assert_program_args_stdout_ge2`](macro@crate::debug_assert_program_args_stdout_ge2)
///
#[macro_export]
macro_rules! assert_program_args_stdout_ge2 {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => {{
        match $crate::assert_program_args_stdout_ge2_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_program:expr, $a_args:expr, $b_program:expr, $($message:tt)+) => {{
        match $crate::assert_program_args_stdout_ge2_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command (built with program and args) stdout string is greater than or equal to another.
///
/// Pseudocode:<br>
/// (program1 + args1 ⇒ command ⇒ stdout) ≥ (program2 + args2 ⇒ command ⇒ stdout)
///
/// This macro provides the same statements as [`assert_program_args_stdout_ge2`](macro.assert_program_args_stdout_ge2.html),
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
/// This macro is intended to work in a similar way to
/// [`::std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_program_args_stdout_ge2`](macro@crate::assert_program_args_stdout_ge2)
/// * [`assert_program_args_stdout_ge2`](macro@crate::assert_program_args_stdout_ge2)
/// * [`debug_assert_program_args_stdout_ge2`](macro@crate::debug_assert_program_args_stdout_ge2)
///
#[macro_export]
macro_rules! debug_assert_program_args_stdout_ge2 {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_program_args_stdout_ge2!($($arg)*);
        }
    };
}