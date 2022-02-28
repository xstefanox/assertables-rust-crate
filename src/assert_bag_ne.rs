/// Assert a bag is not equal to another.
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
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// let x = assert_bag_ne_as_result!(&a, &b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let a = [1, 1];
/// let b = [1, 1];
/// let x = assert_bag_ne_as_result!(&a, &b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_bag_ne!(left, right)`\n",
///     "  left: `{1: 2}`,\n",
///     " right: `{1: 2}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```

///
/// This implementation uses [`BTreeMap`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_bag_ne_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                let mut a_bag: ::std::collections::BTreeMap<_, usize> = ::std::collections::BTreeMap::new();
                let mut b_bag: ::std::collections::BTreeMap<_, usize> = ::std::collections::BTreeMap::new();
                for x in a_val.into_iter() {
                    let n = a_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in b_val.into_iter() {
                    let n = b_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                if a_bag != b_bag {
                    Ok(())
                } else {
                    Err(msg_with_left_and_right!("assertion failed", "assert_bag_ne!", &a_bag, &b_bag))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_assert_x_result {

    #[test]
    fn test_assert_bag_ne_as_result_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assert_bag_ne_as_result!(&a, &b);
        assert!(x.is_ok());
    }

    #[test]
    fn test_assert_bag_ne_as_result_x_arity_2_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assert_bag_ne_as_result!(&a, &b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_bag_ne!(left, right)`\n",
                "  left: `{1: 2}`,\n",
                " right: `{1: 2}`"
            )
        );
    }

}

/// Assert a bag is not equal to another.
///
/// * When true, return `()`.
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
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// assert_bag_ne!(&a, &b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [1, 1];
/// assert_bag_ne!(&a, &b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_bag_ne!(left, right)`\n",
///     "  left: `{1: 2}`,\n",
///     " right: `{1: 2}`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```

///
/// This implementation uses [`BTreeMap`] to count items and sort them.
///
#[macro_export]
macro_rules! assert_bag_ne {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_bag_ne_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($arg:tt)+) => ({
        match assert_bag_ne_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {

    #[test]
    fn test_assert_bag_ne_x_arity_2_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assert_bag_ne!(&a, &b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_bag_ne!(left, right)`\n  left: `{1: 2}`,\n right: `{1: 2}`")]
    fn test_assert_bag_ne_x_arity_2_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let _x = assert_bag_ne!(&a, &b);
    }

    #[test]
    fn test_assert_bag_ne_x_arity_3_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assert_bag_ne!(&a, &b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_bag_ne_x_arity_3_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let _x = assert_bag_ne!(&a, &b, "message");
    }

}
