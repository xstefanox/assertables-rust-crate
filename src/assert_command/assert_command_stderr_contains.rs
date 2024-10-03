//! Assert a command stderr string contains a given containee.
//!
//! Pseudocode:<br>
//! (command ⇒ stderr ⇒ string) contains (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! # fn main() {
//! let mut command = Command::new("bin/printf-stderr");
//! command.args(["%s", "hello"]);
//! let containee = "ell";
//! assert_command_stderr_contains!(command, &containee);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stderr_contains`](macro@crate::assert_command_stderr_contains)
//! * [`assert_command_stderr_contains_as_result`](macro@crate::assert_command_stderr_contains_as_result)
//! * [`debug_assert_command_stderr_contains`](macro@crate::debug_assert_command_stderr_contains)

/// Assert a command stderr string contains a given containee.
///
/// Pseudocode:<br>
/// (command ⇒ stderr ⇒ string) contains (expr into string)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_command_stderr_contains`](macro.assert_command_stderr_contains.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_command_stderr_contains`](macro@crate::assert_command_stderr_contains)
/// * [`assert_command_stderr_contains_as_result`](macro@crate::assert_command_stderr_contains_as_result)
/// * [`debug_assert_command_stderr_contains`](macro@crate::debug_assert_command_stderr_contains)
///
#[macro_export]
macro_rules! assert_command_stderr_contains_as_result {
    ($command:expr, $containee:expr $(,)?) => {{
        match (/*&$command,*/ &$containee) {
            containee => {
                let output = $command.output();
                if output.is_err() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_command_stderr_contains!(command, containee)`\n",
                            "https://docs.rs/assertables/8.13.0/assertables/macro.assert_command_stderr_contains.html\n",
                            "   command label: `{}`,\n",
                            "   command debug: `{:?}`,\n",
                            " containee label: `{}`,\n",
                            " containee debug: `{:?}`,\n",
                            "          output: `{:?}`"
                        ),
                        stringify!($command),
                        $command,
                        stringify!($containee),
                        containee,
                        output
                    ))
                } else {
                    let string = String::from_utf8(output.unwrap().stderr).unwrap();
                    if string.contains($containee) {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_command_stderr_contains!(command, containee)`\n",
                                "https://docs.rs/assertables/8.13.0/assertables/macro.assert_command_stderr_contains.html\n",
                                "   command label: `{}`,\n",
                                "   command debug: `{:?}`,\n",
                                " containee label: `{}`,\n",
                                " containee debug: `{:?}`,\n",
                                "          stderr: `{:?}`"
                            ),
                            stringify!($command),
                            $command,
                            stringify!($containee),
                            containee,
                            string
                        ))
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn test_assert_command_stderr_contains_x_success() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "hello"]);
        let b = "ell";
        let result = assert_command_stderr_contains_as_result!(a, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_assert_command_stderr_contains_x_failure() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "hello"]);
        let b = "zzz";
        let result = assert_command_stderr_contains_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stderr_contains!(command, containee)`\n",
            "https://docs.rs/assertables/8.13.0/assertables/macro.assert_command_stderr_contains.html\n",
            "   command label: `a`,\n",
            "   command debug: `\"bin/printf-stderr\" \"%s\" \"hello\"`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"zzz\"`,\n",
            "          stderr: `\"hello\"`",
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stderr string contains a given containee.
///
/// Pseudocode:<br>
/// (command ⇒ stderr ⇒ string) contains (expr into string)
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// This uses [`std::String`](https://doc.rust-lang.org/std/string/struct.String.html) method `contains`.
///
/// * The containee can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character contains.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use std::process::Command;
///
/// # fn main() {
/// let mut command = Command::new("bin/printf-stderr");
/// command.args(["%s", "hello"]);
/// let containee = "ell";
/// assert_command_stderr_contains!(command, &containee);
///
/// # let result = panic::catch_unwind(|| {
/// let mut command = Command::new("bin/printf-stderr");
/// command.args(["%s", "hello"]);
/// let containee = "zzz";
/// assert_command_stderr_contains!(command, &containee);
/// # });
/// // assertion failed: `assert_command_stderr_contains!(command, containee)`
/// // https://docs.rs/assertables/8.13.0/assertables/macro.assert_command_stderr_contains.html
/// //    command label: `command`,
/// //    command debug: `\"bin/printf-stderr\" \"%s\" \"hello\"`,
/// //  containee label: `&containee`,
/// //  containee debug: `\"zzz\"`,
/// //    command value: `\"hello\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_command_stderr_contains!(command, containee)`\n",
/// #     "https://docs.rs/assertables/8.13.0/assertables/macro.assert_command_stderr_contains.html\n",
/// #     "   command label: `command`,\n",
/// #     "   command debug: `\"bin/printf-stderr\" \"%s\" \"hello\"`,\n",
/// #     " containee label: `&containee`,\n",
/// #     " containee debug: `\"zzz\"`,\n",
/// #     "          stderr: `\"hello\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stderr_contains`](macro@crate::assert_command_stderr_contains)
/// * [`assert_command_stderr_contains_as_result`](macro@crate::assert_command_stderr_contains_as_result)
/// * [`debug_assert_command_stderr_contains`](macro@crate::debug_assert_command_stderr_contains)
///
#[macro_export]
macro_rules! assert_command_stderr_contains {
    ($command:expr, $containee:expr $(,)?) => {{
        match $crate::assert_command_stderr_contains_as_result!($command, $containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($command:expr, $containee:expr, $($message:tt)+) => {{
        match $crate::assert_command_stderr_contains_as_result!($command, $containee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command stderr string contains a given containee.
///
/// Pseudocode:<br>
/// (command ⇒ stderr ⇒ string) contains (expr into string)
///
/// This macro provides the same statements as [`assert_command_stderr_contains`](macro.assert_command_stderr_contains.html),
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
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_command_stderr_contains`](macro@crate::assert_command_stderr_contains)
/// * [`assert_command_stderr_contains`](macro@crate::assert_command_stderr_contains)
/// * [`debug_assert_command_stderr_contains`](macro@crate::debug_assert_command_stderr_contains)
///
#[macro_export]
macro_rules! debug_assert_command_stderr_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stderr_contains!($($arg)*);
        }
    };
}
