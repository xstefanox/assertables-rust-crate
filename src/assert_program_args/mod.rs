//! Assert macros for comparing programs with arguments.
//!
//! These macros help with calling external programs with arguments, then
//! capturing the standard output stream and standard error stream.
//!
//! These macros have corresponding macros in the module [`assert_command`](module@crate::assert_command).
//!
//! Compare command using program and arguments to standard output:
//!
//! * [`assert_program_args_stdout_eq!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stdout_eq) ≈ command using program1 and args1 to stdout = command2 with program2 and args2 to stdout
//!
//! * [`assert_program_args_stdout_eq_expr!(program, args, expr)`](macro@crate::assert_program_args_stdout_eq_expr) ≈ command using program and args to stdout = expr
//!
//! * [`assert_program_args_stdout_contains!(program, args, containee)`](macro@crate::assert_program_args_stdout_contains) ≈ command using program and args to stdout contains containee
//!
//! * [`assert_program_args_stdout_is_match!(program, args, matcher)`](macro@crate::assert_program_args_stdout_is_match) ≈ matcher is match with command using program and args
//!
//! Compare command using program and arguments to standard output:
//!
//! * [`assert_program_args_stderr_eq!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stderr_eq) ≈ command using program1 and args1 to stderr = command2 with program2 and args2 to stderr
//!
//! * [`assert_program_args_stderr_eq_expr!(program, args, expr)`](macro@crate::assert_program_args_stderr_eq_expr) ≈ command using program and args to stderr = expr
//!
//! * [`assert_program_args_stderr_contains!(program, args, containee)`](macro@crate::assert_program_args_stderr_contains) ≈ command using program and args to stderr contains containee
//!
//! * [`assert_program_args_stderr_is_match!(program, args, matcher)`](macro@crate::assert_program_args_stderr_is_match) ≈ matcher is match with command using program and args
//!
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a_program = "bin/printf-stdout";
//! let a_args = ["%s", "hello"];
//! let b_program = "bin/printf-stdout";
//! let b_args = ["%s%s%s%s%s", "h", "e", "l", "l", "o"];
//! assert_program_args_stdout_eq!(&a_program, &a_args, &b_program, &b_args);
//! # }
//! ```

#[macro_export]
macro_rules! assert_program_args_impl_prep {
    ($program:expr, $args:expr $(,)?) => ({
        match (&$program, &$args) {
            (program, args) => {
                let mut command = ::std::process::Command::new(program);
                command.args(args.into_iter());
                command.output()
            }
        }
    });
}

// stdout
pub mod assert_program_args_stdout_eq;
pub mod assert_program_args_stdout_eq_expr;
pub mod assert_program_args_stdout_ge;
pub mod assert_program_args_stdout_ge_expr;
pub mod assert_program_args_stdout_gt;
pub mod assert_program_args_stdout_gt_expr;
pub mod assert_program_args_stdout_le;
pub mod assert_program_args_stdout_le_expr;
pub mod assert_program_args_stdout_lt;
pub mod assert_program_args_stdout_lt_expr;
pub mod assert_program_args_stdout_ne;
pub mod assert_program_args_stdout_ne_expr;

// stdout searches
pub mod assert_program_args_stdout_contains;
pub mod assert_program_args_stdout_is_match;

// stderr
pub mod assert_program_args_stderr_eq;
pub mod assert_program_args_stderr_eq_expr;
pub mod assert_program_args_stderr_ge;
pub mod assert_program_args_stderr_ge_expr;
pub mod assert_program_args_stderr_gt;
pub mod assert_program_args_stderr_gt_expr;
pub mod assert_program_args_stderr_le;
pub mod assert_program_args_stderr_le_expr;
pub mod assert_program_args_stderr_lt;
pub mod assert_program_args_stderr_lt_expr;
pub mod assert_program_args_stderr_ne;
pub mod assert_program_args_stderr_ne_expr;

// stderr searches
pub mod assert_program_args_stderr_contains;
pub mod assert_program_args_stderr_is_match;
