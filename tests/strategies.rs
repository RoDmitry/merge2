#![cfg(feature = "derive")]

use merge2::Merge;

fn test<T: std::fmt::Debug + Merge + PartialEq>(expected: T, mut left: T, mut right: T) {
    left.merge(&mut right);
    assert_eq!(expected, left);
}

#[test]
fn test_any_overwrite() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = ::merge2::any::overwrite)] u8);

    test(S(2), S(1), S(2));
    test(S(2), S(0), S(2));
    test(S(0), S(1), S(0));
    test(S(0), S(0), S(0));
}

#[test]
fn test_any_overwrite_default() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = ::merge2::any::overwrite_default)] u8);

    test(S(1), S(1), S(2));
    test(S(2), S(0), S(2));
    test(S(1), S(1), S(0));
    test(S(0), S(0), S(0));
}

#[test]
fn test_any_swap() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = ::merge2::any::swap)] u8);

    test(S(2), S(1), S(2));
    test(S(2), S(0), S(2));
    test(S(0), S(1), S(0));
    test(S(0), S(0), S(0));
}

#[test]
fn test_option_overwrite_none() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(Option<u8>);

    test(S(Some(1)), S(Some(1)), S(Some(2)));
    test(S(Some(2)), S(None), S(Some(2)));
    test(S(None), S(None), S(None));
}

#[test]
#[cfg(all(feature = "num", feature = "std"))]
fn test_option_recursive() {
    #[derive(Debug, Merge, PartialEq)]
    struct N(#[merge(strategy = ::merge2::num::saturating_add)] u8);

    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = ::merge2::option::recursive)] Option<N>);

    test(S(Some(N(3))), S(Some(N(1))), S(Some(N(2))));
    test(S(Some(N(1))), S(Some(N(1))), S(None));
    test(S(Some(N(1))), S(None), S(Some(N(1))));
    test(S(None), S(None), S(None));
}

#[test]
fn test_bool_overwrite_false() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = ::merge2::bool::overwrite_false)] bool);

    test(S(false), S(false), S(false));
    test(S(true), S(false), S(true));
    test(S(true), S(true), S(false));
    test(S(true), S(true), S(true));
}

#[test]
fn test_bool_overwrite_true() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = ::merge2::bool::overwrite_true)] bool);

    test(S(false), S(false), S(false));
    test(S(false), S(false), S(true));
    test(S(false), S(true), S(false));
    test(S(true), S(true), S(true));
}

#[cfg(feature = "num")]
#[test]
fn test_num_saturating_add() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = ::merge2::num::saturating_add)] u8);

    test(S(0), S(0), S(0));
    test(S(1), S(0), S(1));
    test(S(255), S(255), S(10));
    test(S(40), S(30), S(10));
}

#[test]
fn test_ord_max() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = ::merge2::ord::max)] u8);

    test(S(2), S(1), S(2));
    test(S(2), S(2), S(1));
    test(S(2), S(2), S(2));
    test(S(2), S(2), S(0));
    test(S(2), S(0), S(2));
    test(S(33), S(33), S(11));
}

#[test]
fn test_ord_min() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = ::merge2::ord::min)] u8);

    test(S(1), S(1), S(2));
    test(S(1), S(2), S(1));
    test(S(2), S(2), S(2));
    test(S(0), S(2), S(0));
    test(S(0), S(0), S(2));
    test(S(11), S(33), S(11));
}

#[cfg(feature = "std")]
mod string {
    use super::test;
    use crate::Merge;

    #[test]
    fn test_string_overwrite_empty() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(String);

        test(S("".to_owned()), S("".to_owned()), S("".to_owned()));
        test(S("1".to_owned()), S("".to_owned()), S("1".to_owned()));
        test(S("0".to_owned()), S("0".to_owned()), S("1".to_owned()));
        test(S("255".to_owned()), S("255".to_owned()), S("10".to_owned()));
    }

    #[test]
    fn test_string_append() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = ::merge2::string::append)] String);

        test(S("".to_owned()), S("".to_owned()), S("".to_owned()));
        test(S("1".to_owned()), S("".to_owned()), S("1".to_owned()));
        test(S("01".to_owned()), S("0".to_owned()), S("1".to_owned()));
        test(
            S("25510".to_owned()),
            S("255".to_owned()),
            S("10".to_owned()),
        );
        test(
            S("01234".to_owned()),
            S("012".to_owned()),
            S("34".to_owned()),
        );
        test(
            S("34012".to_owned()),
            S("34".to_owned()),
            S("012".to_owned()),
        );
    }

    #[test]
    fn test_string_prepend() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = ::merge2::string::prepend)] String);

        test(S("".to_owned()), S("".to_owned()), S("".to_owned()));
        test(S("1".to_owned()), S("".to_owned()), S("1".to_owned()));
        test(S("10".to_owned()), S("0".to_owned()), S("1".to_owned()));
        test(
            S("10255".to_owned()),
            S("255".to_owned()),
            S("10".to_owned()),
        );
        test(
            S("34012".to_owned()),
            S("012".to_owned()),
            S("34".to_owned()),
        );
        test(
            S("01234".to_owned()),
            S("34".to_owned()),
            S("012".to_owned()),
        );
    }
}

#[cfg(feature = "std")]
mod vec {
    use super::test;
    use crate::Merge;

    #[test]
    fn test_overwrite_empty() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(Vec<u8>);

        test(S(vec![]), S(vec![]), S(vec![]));
        test(S(vec![1]), S(vec![]), S(vec![1]));
        test(S(vec![0]), S(vec![0]), S(vec![1]));
        test(S(vec![255]), S(vec![255]), S(vec![10]));
    }

    #[test]
    fn test_append() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = ::merge2::vec::append)] Vec<u8>);

        test(S(vec![]), S(vec![]), S(vec![]));
        test(S(vec![1]), S(vec![]), S(vec![1]));
        test(S(vec![0, 1]), S(vec![0]), S(vec![1]));
        test(S(vec![255, 10]), S(vec![255]), S(vec![10]));
        test(S(vec![0, 1, 2, 3, 4]), S(vec![0, 1, 2]), S(vec![3, 4]));
        test(S(vec![3, 4, 0, 1, 2]), S(vec![3, 4]), S(vec![0, 1, 2]));
    }

    #[test]
    fn test_prepend() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = ::merge2::vec::prepend)] Vec<u8>);

        test(S(vec![]), S(vec![]), S(vec![]));
        test(S(vec![1]), S(vec![]), S(vec![1]));
        test(S(vec![1, 0]), S(vec![0]), S(vec![1]));
        test(S(vec![10, 255]), S(vec![255]), S(vec![10]));
        test(S(vec![3, 4, 0, 1, 2]), S(vec![0, 1, 2]), S(vec![3, 4]));
        test(S(vec![0, 1, 2, 3, 4]), S(vec![3, 4]), S(vec![0, 1, 2]));
    }
}

#[cfg(feature = "std")]
mod hashmap {
    use super::test;
    use crate::Merge;
    use std::collections::HashMap;

    /// A macro to create a HashMap.
    ///
    /// Example:
    ///
    /// ```
    /// let letters = map!{"a" => "b", "c" => "d"};
    /// ```
    ///
    /// Trailing commas are allowed.
    /// Commas between elements are required (even if the expression is a block).
    macro_rules! map {
        ($( $key: expr => $val: expr ),* $(,)*) => {{
            let mut map = HashMap::default();
            $( map.insert($key, $val); )*
            map
        }}
    }

    #[test]
    fn test_merge() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = ::merge2::hashmap::merge)] HashMap<u8, u8>);

        test(S(map! {1 => 1}), S(map! {1 => 1}), S(map! {1 => 2}));
        test(S(map! {1 => 2}), S(map! {1 => 2}), S(map! {1 => 1}));
        test(S(map! {0 => 1, 1 => 2}), S(map! {0 => 1}), S(map! {1 => 2}));
    }

    #[test]
    fn test_replace() {
        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = ::merge2::hashmap::replace)] HashMap<u8, u8>);

        test(S(map! {1 => 2}), S(map! {1 => 1}), S(map! {1 => 2}));
        test(S(map! {1 => 1}), S(map! {1 => 2}), S(map! {1 => 1}));
        test(S(map! {0 => 1, 1 => 2}), S(map! {0 => 1}), S(map! {1 => 2}));
    }

    #[test]
    #[cfg(feature = "num")]
    fn test_recursive() {
        #[derive(Debug, Merge, PartialEq)]
        struct N(#[merge(strategy = ::merge2::num::saturating_add)] u8);

        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = ::merge2::hashmap::recursive)] HashMap<u8, N>);

        test(
            S(map! {1 => N(3)}),
            S(map! {1 => N(1)}),
            S(map! {1 => N(2)}),
        );
        test(
            S(map! {1 => N(3)}),
            S(map! {1 => N(2)}),
            S(map! {1 => N(1)}),
        );
        test(
            S(map! {0 => N(1), 1 => N(2)}),
            S(map! {0 => N(1)}),
            S(map! {1 => N(2)}),
        );
    }

    #[test]
    #[cfg(feature = "num")]
    fn test_intersection() {
        #[derive(Debug, Merge, PartialEq)]
        struct N(#[merge(strategy = ::merge2::num::saturating_add)] u8);

        #[derive(Debug, Merge, PartialEq)]
        struct S(#[merge(strategy = ::merge2::hashmap::intersection)] HashMap<u8, N>);

        test(
            S(map! {1 => N(3)}),
            S(map! {1 => N(1)}),
            S(map! {1 => N(2)}),
        );
        test(
            S(map! {1 => N(3)}),
            S(map! {1 => N(2)}),
            S(map! {1 => N(1)}),
        );
        test(
            S(map! {0 => N(1)}),
            S(map! {0 => N(1)}),
            S(map! {1 => N(2)}),
        );
    }
}
