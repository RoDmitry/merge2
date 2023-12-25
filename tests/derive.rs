#![cfg(feature = "derive")]

use merge2::Merge;

fn test<T: std::fmt::Debug + Merge + PartialEq>(expected: T, mut left: T, mut right: T) {
    left.merge(&mut right);
    assert_eq!(expected, left);
}

#[test]
fn test_one_option_field() {
    #[derive(Debug, Merge, PartialEq)]
    struct S {
        field1: Option<usize>,
    }

    impl S {
        pub fn new(field1: Option<usize>) -> S {
            S { field1 }
        }
    }

    test(S::new(Some(1)), S::new(Some(1)), S::new(Some(2)));
    test(S::new(Some(1)), S::new(Some(1)), S::new(None));
    test(S::new(Some(2)), S::new(None), S::new(Some(2)));
    test(S::new(None), S::new(None), S::new(None));
}

#[test]
fn test_two_option_fields() {
    #[derive(Debug, Merge, PartialEq)]
    struct S {
        field1: Option<usize>,
        field2: Option<usize>,
    }

    impl S {
        pub fn new(field1: Option<usize>, field2: Option<usize>) -> S {
            S { field1, field2 }
        }
    }

    // left.field1 == Some(1)
    // right.field1 == Some(2)
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(Some(2), None),
    );
    test(
        S::new(Some(1), Some(2)),
        S::new(Some(1), None),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(Some(2), None),
    );

    // left.field1 == Some(1)
    // right.field1 == None
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(None, Some(2)),
    );
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(None, None),
    );
    test(
        S::new(Some(1), Some(2)),
        S::new(Some(1), None),
        S::new(None, Some(2)),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(None, None),
    );

    // left.field1 == None
    // right.field1 == Some(2)
    test(
        S::new(Some(2), Some(1)),
        S::new(None, Some(1)),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(2), Some(1)),
        S::new(None, Some(1)),
        S::new(Some(2), None),
    );
    test(
        S::new(Some(2), Some(2)),
        S::new(None, None),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(2), None),
        S::new(None, None),
        S::new(Some(2), None),
    );

    // left.field1 == None
    // right.field1 == None
    test(
        S::new(None, Some(1)),
        S::new(None, Some(1)),
        S::new(None, Some(2)),
    );
    test(
        S::new(None, Some(1)),
        S::new(None, Some(1)),
        S::new(None, None),
    );
    test(
        S::new(None, Some(2)),
        S::new(None, None),
        S::new(None, Some(2)),
    );
    test(S::new(None, None), S::new(None, None), S::new(None, None));
}

#[test]
fn test_skip_valid() {
    #[derive(Debug, Merge, PartialEq)]
    struct S {
        field1: Option<usize>,
        #[merge(skip)]
        field2: Option<usize>,
    }

    impl S {
        pub fn new(field1: Option<usize>, field2: Option<usize>) -> S {
            S { field1, field2 }
        }
    }

    // left.field1 == Some(1)
    // right.field1 == Some(2)
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(Some(2), None),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(Some(2), None),
    );

    // left.field1 == Some(1)
    // right.field1 == None
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(None, Some(2)),
    );
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(None, None),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(None, Some(2)),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(None, None),
    );

    // left.field1 == None
    // right.field1 == Some(2)
    test(
        S::new(Some(2), Some(1)),
        S::new(None, Some(1)),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(2), Some(1)),
        S::new(None, Some(1)),
        S::new(Some(2), None),
    );
    test(
        S::new(Some(2), None),
        S::new(None, None),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(2), None),
        S::new(None, None),
        S::new(Some(2), None),
    );

    // left.field1 == None
    // right.field1 == None
    test(
        S::new(None, Some(1)),
        S::new(None, Some(1)),
        S::new(None, Some(2)),
    );
    test(
        S::new(None, Some(1)),
        S::new(None, Some(1)),
        S::new(None, None),
    );
    test(
        S::new(None, None),
        S::new(None, None),
        S::new(None, Some(2)),
    );
    test(S::new(None, None), S::new(None, None), S::new(None, None));
}

#[test]
fn test_skip_invalid() {
    #[derive(Debug, Merge, PartialEq)]
    struct S {
        field1: Option<usize>,
        #[merge(skip)]
        field2: usize,
    }

    impl S {
        pub fn new(field1: Option<usize>, field2: usize) -> S {
            S { field1, field2 }
        }
    }

    // left.field1 == Some(1)
    // right.field1 == Some(2)
    test(S::new(Some(1), 1), S::new(Some(1), 1), S::new(Some(2), 2));
    test(S::new(Some(1), 1), S::new(Some(1), 1), S::new(Some(2), 0));
    test(S::new(Some(1), 0), S::new(Some(1), 0), S::new(Some(2), 2));
    test(S::new(Some(1), 0), S::new(Some(1), 0), S::new(Some(2), 0));

    // left.field1 == Some(1)
    // right.field1 == None
    test(S::new(Some(1), 1), S::new(Some(1), 1), S::new(None, 2));
    test(S::new(Some(1), 1), S::new(Some(1), 1), S::new(None, 0));
    test(S::new(Some(1), 0), S::new(Some(1), 0), S::new(None, 2));
    test(S::new(Some(1), 0), S::new(Some(1), 0), S::new(None, 0));

    // left.field1 == None
    // right.field1 == Some(2)
    test(S::new(Some(2), 1), S::new(None, 1), S::new(Some(2), 2));
    test(S::new(Some(2), 1), S::new(None, 1), S::new(Some(2), 0));
    test(S::new(Some(2), 0), S::new(None, 0), S::new(Some(2), 2));
    test(S::new(Some(2), 0), S::new(None, 0), S::new(Some(2), 0));

    // left.field1 == None
    // right.field1 == None
    test(S::new(None, 1), S::new(None, 1), S::new(None, 2));
    test(S::new(None, 1), S::new(None, 1), S::new(None, 0));
    test(S::new(None, 0), S::new(None, 0), S::new(None, 2));
    test(S::new(None, 0), S::new(None, 0), S::new(None, 0));
}

#[test]
fn test_strategy_usize_add() {
    #[derive(Debug, Merge, PartialEq)]
    struct S {
        #[merge(strategy = add)]
        field1: usize,
    }

    impl S {
        pub fn new(field1: usize) -> S {
            S { field1 }
        }
    }

    fn add(left: &mut usize, right: &mut usize) {
        *left += *right;
    }

    test(S::new(0), S::new(0), S::new(0));
    test(S::new(1), S::new(1), S::new(0));
    test(S::new(1), S::new(0), S::new(1));
    test(S::new(2), S::new(1), S::new(1));
}

#[test]
fn test_strategy_vec_append() {
    #[derive(Debug, Merge, PartialEq)]
    struct S {
        #[merge(strategy = append)]
        field1: Vec<usize>,
    }

    impl S {
        pub fn new(field1: Vec<usize>) -> S {
            S { field1 }
        }
    }

    fn append(left: &mut Vec<usize>, right: &mut Vec<usize>) {
        left.append(right);
    }

    test(
        S::new(vec![0, 1, 2, 3]),
        S::new(vec![0, 1]),
        S::new(vec![2, 3]),
    );
    test(
        S::new(vec![0, 1, 2, 3]),
        S::new(vec![0, 1, 2, 3]),
        S::new(vec![]),
    );
    test(
        S::new(vec![0, 1, 2, 3]),
        S::new(vec![]),
        S::new(vec![0, 1, 2, 3]),
    );
}

#[test]
fn test_unnamed_fields() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(Option<usize>, Option<usize>);

    impl S {
        pub fn new(field1: Option<usize>, field2: Option<usize>) -> S {
            S(field1, field2)
        }
    }

    // left.field1 == Some(1)
    // right.field1 == Some(2)
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(Some(2), None),
    );
    test(
        S::new(Some(1), Some(2)),
        S::new(Some(1), None),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(Some(2), None),
    );

    // left.field1 == Some(1)
    // right.field1 == None
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(None, Some(2)),
    );
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(None, None),
    );
    test(
        S::new(Some(1), Some(2)),
        S::new(Some(1), None),
        S::new(None, Some(2)),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(None, None),
    );

    // left.field1 == None
    // right.field1 == Some(2)
    test(
        S::new(Some(2), Some(1)),
        S::new(None, Some(1)),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(2), Some(1)),
        S::new(None, Some(1)),
        S::new(Some(2), None),
    );
    test(
        S::new(Some(2), Some(2)),
        S::new(None, None),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(2), None),
        S::new(None, None),
        S::new(Some(2), None),
    );

    // left.field1 == None
    // right.field1 == None
    test(
        S::new(None, Some(1)),
        S::new(None, Some(1)),
        S::new(None, Some(2)),
    );
    test(
        S::new(None, Some(1)),
        S::new(None, Some(1)),
        S::new(None, None),
    );
    test(
        S::new(None, Some(2)),
        S::new(None, None),
        S::new(None, Some(2)),
    );
    test(S::new(None, None), S::new(None, None), S::new(None, None));
}

#[test]
fn test_unnamed_fields_skip() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(Option<usize>, #[merge(skip)] Option<usize>);

    impl S {
        pub fn new(field1: Option<usize>, field2: Option<usize>) -> S {
            S(field1, field2)
        }
    }

    // left.field1 == Some(1)
    // right.field1 == Some(2)
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(Some(2), None),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(Some(2), None),
    );

    // left.field1 == Some(1)
    // right.field1 == None
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(None, Some(2)),
    );
    test(
        S::new(Some(1), Some(1)),
        S::new(Some(1), Some(1)),
        S::new(None, None),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(None, Some(2)),
    );
    test(
        S::new(Some(1), None),
        S::new(Some(1), None),
        S::new(None, None),
    );

    // left.field1 == None
    // right.field1 == Some(2)
    test(
        S::new(Some(2), Some(1)),
        S::new(None, Some(1)),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(2), Some(1)),
        S::new(None, Some(1)),
        S::new(Some(2), None),
    );
    test(
        S::new(Some(2), None),
        S::new(None, None),
        S::new(Some(2), Some(2)),
    );
    test(
        S::new(Some(2), None),
        S::new(None, None),
        S::new(Some(2), None),
    );

    // left.field1 == None
    // right.field1 == None
    test(
        S::new(None, Some(1)),
        S::new(None, Some(1)),
        S::new(None, Some(2)),
    );
    test(
        S::new(None, Some(1)),
        S::new(None, Some(1)),
        S::new(None, None),
    );
    test(
        S::new(None, None),
        S::new(None, None),
        S::new(None, Some(2)),
    );
    test(S::new(None, None), S::new(None, None), S::new(None, None));
}

#[test]
#[cfg(all(feature = "num", feature = "std"))]
fn test_default_strategy() {
    #[derive(Debug, Merge, PartialEq)]
    struct N(#[merge(strategy = ::merge2::num::saturating_add)] u8);

    #[derive(Debug, Merge, PartialEq)]
    struct S(
        Option<usize>,
        Option<usize>,
        #[merge(strategy = ::merge2::num::saturating_add)] u8,
        #[merge(strategy = Merge::merge)] N,
    );
}

#[test]
fn test_generics() {
    #[derive(Debug, Merge, PartialEq)]
    struct TupleWithGenerics<A: core::fmt::Display, B: core::fmt::Debug>(Option<A>, Option<B>);

    #[derive(Debug, Merge, PartialEq)]
    struct TupleWithWhere<A, B>(Option<A>, Option<B>)
    where
        A: core::fmt::Display,
        B: core::fmt::Debug;

    #[derive(Debug, Merge, PartialEq)]
    struct TupleWithBoth<A: core::fmt::Display, B>(Option<A>, Option<B>)
    where
        B: core::fmt::Debug;

    #[derive(Debug, Merge, PartialEq)]
    struct StructWithGenerics<A: core::fmt::Display, B: core::fmt::Debug> {
        a: Option<A>,
        b: Option<B>,
    }

    #[derive(Debug, Merge, PartialEq)]
    struct StructWithWhere<A, B>
    where
        A: core::fmt::Display,
        B: core::fmt::Debug,
    {
        a: Option<A>,
        b: Option<B>,
    }

    #[derive(Debug, Merge, PartialEq)]
    struct StructWithBoth<A: core::fmt::Display, B>
    where
        B: core::fmt::Debug,
    {
        a: Option<A>,
        b: Option<B>,
    }
}
