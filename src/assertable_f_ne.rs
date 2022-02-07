/// Assert one function output is not equal to another function output.
///
/// * When true, return `Ok(())`.
///
/// * Otherwise, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let x = assertable_f_ne!(i32::abs, 1 as i32, -2 as i32);
/// //-> Ok(())
/// assert_eq!(x.unwrap(), ());
///
/// let x = assertable_f_ne!(i32::abs, 1 as i32, -1 as i32);
/// //-> Err("…")
/// // assertable failed: `assertable_f_ne!(function, left, right)`
/// //    left input: `1`,
/// //   right input: `-1`,
/// //   left output: `1`,
/// //  right output: `1`
/// assert_eq!(x.unwrap_err(), "assertable failed: `assertable_f_ne!(function, left, right)`\n   left input: `1`,\n  right input: `-1`,\n  left output: `1`,\n right output: `1`".to_string());
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assertable_f_ne {
    ($function:path, $left:expr, $right:expr $(,)?) => ({
        let left = $function($left);
        let right = $function($right);
        if (left != right) {
            Ok(())
        } else {
            Err(format!("assertable failed: `assertable_f_ne!(function, left, right)`\n   left input: `{:?}`,\n  right input: `{:?}`,\n  left output: `{:?}`,\n right output: `{:?}`", $left, $right, left, right))
        }
    });
    ($function:path, $left:expr, $right:expr, $($arg:tt)+) => ({
        let left = $function($left);
        let right = $function($right);
        if (left != right) {
            Ok(())
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assertable_f_ne_x_arity_2_success() {
        let a = 1;
        let b = -2;
        let x = assertable_f_ne!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_ne_x_arity_2_failure() {
        let a = 1;
        let b = -1;
        let x = assertable_f_ne!(i32::abs, a as i32, b as i32);
        assert_eq!(
            x.unwrap_err(),
            "assertable failed: `assertable_f_ne!(function, left, right)`\n   left input: `1`,\n  right input: `-1`,\n  left output: `1`,\n right output: `1`"
        );
    }

    #[test]
    fn test_assertable_f_ne_x_arity_3_success() {
        let a = 1;
        let b = -2;
        let x = assertable_f_ne!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assertable_f_ne_x_arity_3_failure() {
        let a = 1;
        let b = -1;
        let x = assertable_f_ne!(i32::abs, a as i32, b as i32, "message");
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}