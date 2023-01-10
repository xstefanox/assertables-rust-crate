/// Assert a set is a superset of another.
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
/// let a = [1, 2, 3];
/// let b = [1, 2];
/// let x = assert_set_superset_other_as_result!(&a, &b);
/// //-> Ok(())
/// assert_eq!(x, Ok(()));
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = [1, 2];
/// let b = [1, 2, 3];
/// let x = assert_set_superset_other_as_result!(&a, &b);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_set_superset_other!(left_set, right_set)`\n",
///     "  left_set label: `&a`,\n",
///     "  left_set debug: `[1, 2]`,\n",
///     " right_set label: `&b`,\n",
///     " right_set debug: `[1, 2, 3]`,\n",
///     "            left: `{1, 2}`,\n",
///     "           right: `{1, 2, 3}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`BTreeSet`] to count items and sort them.
///
/// # Related
/// 
/// * [`assert_set_superset`]
/// * [`assert_set_superset_other_as_result`]
/// * [`debug_assert_set_superset`]
///
#[macro_export]
macro_rules! assert_set_superset_other_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let a_set: ::std::collections::BTreeSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::BTreeSet<_> = b_val.into_iter().collect();
                if a_set.is_superset(&b_set) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_set_superset_other!(left_set, right_set)`\n",
                            "  left_set label: `{}`,\n",
                            "  left_set debug: `{:?}`,\n",
                            " right_set label: `{}`,\n",
                            " right_set debug: `{:?}`,\n",
                            "            left: `{:?}`,\n",
                            "           right: `{:?}`"
                        ),
                        stringify!($a), $a,
                        stringify!($b), $b,
                        &a_set,
                        &b_set
                    ))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_set_superset_other_as_result_x_success() {
        let a = [1, 2, 3];
        let b = [1, 2];
        let x = assert_set_superset_other_as_result!(&a, &b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_set_superset_other_as_result_x_failure() {
        let a = [1, 2];
        let b = [1, 2, 3];
        let x = assert_set_superset_other_as_result!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_set_superset_other!(left_set, right_set)`\n",
                "  left_set label: `&a`,\n",
                "  left_set debug: `[1, 2]`,\n",
                " right_set label: `&b`,\n",
                " right_set debug: `[1, 2, 3]`,\n",
                "            left: `{1, 2}`,\n",
                "           right: `{1, 2, 3}`"
            )
        );
    }

}

/// Assert a set is a superset of another.
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
/// let a = [1, 2, 3];
/// let b = [1, 2];
/// assert_set_superset_other!(&a, &b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a = [1, 2];
/// let b = [1, 2, 3];
/// assert_set_superset_other!(&a, &b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_set_superset_other!(left_set, right_set)`\n",
///     "  left_set label: `&a`,\n",
///     "  left_set debug: `[1, 2]`,\n",
///     " right_set label: `&b`,\n",
///     " right_set debug: `[1, 2, 3]`,\n",
///     "            left: `{1, 2}`,\n",
///     "           right: `{1, 2, 3}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`BTreeSet`] to count items and sort them.
///
/// # Related
/// 
/// * [`assert_set_superset`]
/// * [`assert_set_superset_other_as_result`]
/// * [`debug_assert_set_superset`]
///
#[macro_export]
macro_rules! assert_set_superset_other {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_set_superset_other_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_set_superset_other_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a set is a superset of another.
///
/// This macro provides the same statements as [`assert_set_superset`],
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
/// * [`assert_set_superset`]
/// * [`assert_set_superset`]
/// * [`debug_assert_set_superset`]
/// 
#[macro_export]
macro_rules! debug_assert_set_superset_other {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_set_superset_other!($($arg)*);
        }
    };
}