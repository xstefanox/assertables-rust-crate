//! Assert a bag is a superbag of another.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a = [1, 1, 1];
//! let b = [1, 1];
//! assert_bag_superbag!(&a, &b);
//! # }
//! ```
//!
//! This implementation uses [`std::collections::BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) to count items and sort them.
//!
//! # Module macros
//!
//! * [`assert_bag_superbag`](macro@crate::assert_bag_superbag)
//! * [`assert_bag_superbag_as_result`](macro@crate::assert_bag_superbag_as_result)
//! * [`debug_assert_bag_superbag`](macro@crate::debug_assert_bag_superbag)

/// Assert a bag is a superbag of another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_superbag_other`](macro.assert_superbag_other.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_bag_superbag`](macro@crate::assert_bag_superbag)
/// * [`assert_bag_superbag_as_result`](macro@crate::assert_bag_superbag_as_result)
/// * [`debug_assert_bag_superbag`](macro@crate::debug_assert_bag_superbag)
///
#[macro_export]
macro_rules! assert_bag_superbag_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a_val, b_val) => {
                let mut a_bag: ::std::collections::BTreeMap<_, usize> =
                    ::std::collections::BTreeMap::new();
                let mut b_bag: ::std::collections::BTreeMap<_, usize> =
                    ::std::collections::BTreeMap::new();
                for x in a_val.into_iter() {
                    let n = a_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in b_val.into_iter() {
                    let n = b_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                if b_val.into_iter().all(|key| {
                    a_bag.contains_key(&key)
                        && b_bag.contains_key(&key)
                        && a_bag.get_key_value(&key) >= b_bag.get_key_value(&key)
                }) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_bag_superbag!(a_bag, b_bag)`\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            "       a: `{:?}`,\n",
                            "       b: `{:?}`"
                        ),
                        stringify!($a),
                        $a,
                        stringify!($b),
                        $b,
                        &a_bag,
                        &b_bag
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_bag_superbag_as_result_x_success() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let result = assert_bag_superbag_as_result!(&a, &b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_bag_superbag_as_result_x_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let result = assert_bag_superbag_as_result!(&a, &b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_bag_superbag!(a_bag, b_bag)`\n",
                " a label: `&a`,\n",
                " a debug: `[1, 1]`,\n",
                " b label: `&b`,\n",
                " b debug: `[2, 2]`,\n",
                "       a: `{1: 2}`,\n",
                "       b: `{2: 2}`"
            )
        );
    }

    #[test]
    fn test_assert_bag_superbag_as_result_x_failure_because_val_count_is_excessive() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let result = assert_bag_superbag_as_result!(&a, &b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_bag_superbag!(a_bag, b_bag)`\n",
                " a label: `&a`,\n",
                " a debug: `[1, 1]`,\n",
                " b label: `&b`,\n",
                " b debug: `[1, 1, 1]`,\n",
                "       a: `{1: 2}`,\n",
                "       b: `{1: 3}`"
            )
        );
    }
}

/// Assert a bag is a superbag of another.
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] in order to print the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// // Return Ok
/// let a = [1, 1, 1];
/// let b = [1, 1];
/// assert_bag_superbag!(&a, &b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [2, 2];
/// assert_bag_superbag!(&a, &b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_bag_superbag!(a_bag, b_bag)`\n",
///     " a label: `&a`,\n",
///     " a debug: `[1, 1]`,\n",
///     " b label: `&b`,\n",
///     " b debug: `[2, 2]`,\n",
///     "       a: `{1: 2}`,\n",
///     "       b: `{2: 2}`"
/// );
///
/// // Panic with custom message
/// let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [2, 2];
/// assert_bag_superbag!(&a, &b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// let result = panic::catch_unwind(|| {
/// assert_bag_superbag!(&a, &b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_bag_superbag!(a_bag, b_bag)`\n",
///     " a label: `&a`,\n",
///     " a debug: `[1, 1]`,\n",
///     " b label: `&b`,\n",
///     " b debug: `[1, 1, 1]`,\n",
///     "       a: `{1: 2}`,\n",
///     "       b: `{1: 3}`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with custom message
/// let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// assert_bag_superbag!(&a, &b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`std::collections::BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) to count items and sort them.
///
/// # Module macros
///
/// * [`assert_bag_superbag`](macro@crate::assert_bag_superbag)
/// * [`assert_bag_superbag_as_result`](macro@crate::assert_bag_superbag_as_result)
/// * [`debug_assert_bag_superbag`](macro@crate::debug_assert_bag_superbag)
///
#[macro_export]
macro_rules! assert_bag_superbag {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_bag_superbag_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_bag_superbag_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a bag is a superbag of another.
///
/// This macro provides the same statements as [`assert_bag_superbag`](macro.assert_bag_superbag.html),
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
/// * [`assert_bag_superbag`](macro@crate::assert_bag_superbag)
/// * [`assert_bag_superbag_as_result`](macro@crate::assert_bag_superbag_as_result)
/// * [`debug_assert_bag_superbag`](macro@crate::debug_assert_bag_superbag)
///
#[macro_export]
macro_rules! debug_assert_bag_superbag {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_bag_superbag!($($arg)*);
        }
    };
}
