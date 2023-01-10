/// Assert a a std::io::Read read_to_string() is a match to a regex.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// use std::io::Read;
/// use regex::Regex;
///
/// # fn main() {
/// // Return Ok
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"ell").unwrap();
/// assert_read_to_string_matches_as_result!(reader, matcher);
/// //-> ()
///
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"xyz").unwrap();
/// let x = assert_read_to_string_matches_as_result!(reader, matcher);
/// //-> Err(…)
/// assert!(x.is_err());
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_matches!(left_reader, right_matcher)`\n",
///     "   left_reader label: `reader`,\n",
///     "   left_reader debug: `[]`,\n",
///     " right_matcher label: `matcher`,\n",
///     " right_matcher debug: `xyz`,\n",
///     "                left: `\"hello\"`,\n",
///     "               right: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
/// 
/// * [`assert_read_to_string_matches`]
/// * [`assert_read_to_string_matches_as_result`]
/// * [`debug_assert_read_to_string_matches`]
///
#[macro_export]
macro_rules! assert_read_to_string_matches_as_result {
    ($a_reader:expr, $b_matcher:expr $(,)?) => ({
        let mut a_string = String::new();
        let a_result = $a_reader.read_to_string(&mut a_string);
        if let Err(a_err) = a_result {
            Err(format!(
                concat!(
                    "assertion failed: `assert_read_to_string_matches!(left_reader, right_matcher)`\n",
                    "   left_reader label: `{}`,\n",
                    "   left_reader debug: `{:?}`,\n",
                    " right_matcher label: `{}`,\n",
                    " right_matcher debug: `{:?}`,\n",
                    "            left err: `{:?}`"
                ),
                stringify!($a_reader), $a_reader,
                stringify!($b_matcher), $b_matcher,
                a_err
            ))
        } else {
            let _a_size = a_result.unwrap();
            if $b_matcher.is_match(a_string.as_str()) {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_read_to_string_matches!(left_reader, right_matcher)`\n",
                        "   left_reader label: `{}`,\n",
                        "   left_reader debug: `{:?}`,\n",
                        " right_matcher label: `{}`,\n",
                        " right_matcher debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`",
                    ),
                    stringify!($a_reader), $a_reader,
                    stringify!($b_matcher), $b_matcher,
                    a_string,
                    $b_matcher
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {
    use std::io::Read;
    use regex::Regex;

    #[test]
    fn test_assert_read_to_string_matches_as_result_x_success() {
        let mut reader = "alpha".as_bytes();
        let matcher = Regex::new(r"lph").unwrap();
        let x = assert_read_to_string_matches_as_result!(reader, matcher);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_read_to_string_matches_as_result_x_failure() {
        let mut reader = "alpha".as_bytes();
        let matcher = Regex::new(r"xyz").unwrap();
        let x = assert_read_to_string_matches_as_result!(reader, matcher);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_read_to_string_matches!(left_reader, right_matcher)`\n",
                "   left_reader label: `reader`,\n",
                "   left_reader debug: `[]`,\n",
                " right_matcher label: `matcher`,\n",
                " right_matcher debug: `xyz`,\n",
                "                left: `\"alpha\"`,\n",
                "               right: `xyz`"
            )
        );
    }
}

/// Assert a a std::io::Read read_to_string() is a match to a regex.
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
/// use std::io::Read;
/// use regex::Regex;
///
/// # fn main() {
/// // Return Ok
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"ell").unwrap();
/// assert_read_to_string_matches!(reader, matcher);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut reader = "hello".as_bytes();
/// let matcher = Regex::new(r"xyz").unwrap();
/// assert_read_to_string_matches!(reader, matcher);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_matches!(left_reader, right_matcher)`\n",
///     "   left_reader label: `reader`,\n",
///     "   left_reader debug: `[]`,\n",
///     " right_matcher label: `matcher`,\n",
///     " right_matcher debug: `xyz`,\n",
///     "                left: `\"hello\"`,\n",
///     "               right: `xyz`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
/// 
/// * [`assert_read_to_string_matches`]
/// * [`assert_read_to_string_matches_as_result`]
/// * [`debug_assert_read_to_string_matches`]
///
#[macro_export]
macro_rules! assert_read_to_string_matches {
    ($a_reader:expr, $b_matcher:expr $(,)?) => ({
        match assert_read_to_string_matches_as_result!($a_reader, $b_matcher) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b_matcher:expr, $($message:tt)+) => ({
        match assert_read_to_string_matches_as_result!($a_reader, $b_matcher) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
