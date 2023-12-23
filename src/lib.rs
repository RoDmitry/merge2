//! Provides [`Merge`][], a trait for objects that can be merged.
//!
//! # Usage
//!
//! ```
//! trait Merge {
//!     fn merge(&mut self, other: Self);
//! }
//! ```
//!
//! The [`Merge`][] trait can be used to merge two objects of the same type into one.  The intended
//! use case is merging configuration from different sources, for example environment variables,
//! multiple configuration files and command-line arguments, see the [`args.rs`][] example.
//!
//! `Merge` can be derived for structs. When deriving the `Merge` trait for a struct, you can
//! provide custom merge strategies for the fields that don’t implement Merge`.
//! A merge strategy is a function with the signature `fn merge<T>(left: &mut T, right: T)`
//! that merges `right` into `left`. The submodules of this crate provide strategies for the
//! most common types, but you can also define your own strategies.
//!
//! ## Features
//!
//! This crate has the following features:
//!
//! - `derive` (default):  Enables the derive macro for the `Merge` trait using the `merge_derive`
//!   crate.
//! - `num` (default): Enables the merge strategies in the `num` module that require the
//!   `num_traits` crate.
//! - `std` (default): Enables the merge strategies in the `hashmap` and `vec` modules that require
//!    the standard library.  If this feature is not set, `merge2` is a `no_std`.
//!
//! # Example
//!
//! ```
//! use merge2::Merge;
//!
//! #[derive(Merge)]
//! struct User {
//!     // Fields with the skip attribute are skipped by Merge
//!     #[merge(skip)]
//!     pub name: &'static str,
//!     pub location: Option<&'static str>,
//!
//!     // The strategy attribute is used to customize the merge behavior
//!     #[merge(strategy = ::merge2::vec::append)]
//!     pub groups: Vec<&'static str>,
//! }
//!
//! let defaults = User {
//!     name: "",
//!     location: Some("Internet"),
//!     groups: vec!["rust"],
//! };
//! let mut ferris = User {
//!     name: "Ferris",
//!     location: None,
//!     groups: vec!["mascot"],
//! };
//! ferris.merge(defaults);
//!
//! assert_eq!("Ferris", ferris.name);
//! assert_eq!(Some("Internet"), ferris.location);
//! assert_eq!(vec!["mascot", "rust"], ferris.groups);
//! ```
//!
//! [`Merge`]: trait.Merge.html
//! [`args.rs`]: https://git.sr.ht/~ireas/merge-rs/tree/master/examples/args.rs

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "derive")]
pub use merge2_derive::*;

/// A trait for objects that can be merged.
///
/// # Deriving
///
/// `Merge` can be derived for structs if the `derive` feature is enabled.  The generated
/// implementation calls the `merge` method for all fields, or the merge strategy function if set.
/// You can use these field attributes to configure the generated implementation:
/// - `skip`: Skip this field in the `merge` method.
/// - `strategy = f`: Call `f(self.field, other.field)` instead of calling the `merge` function for
///    this field.
///
/// You can also set a default strategy for all fields by setting the `strategy` attribute for the
/// struct.
///
/// # Examples
///
/// Deriving `Merge` for a struct:
///
/// ```
/// use merge2::Merge;
///
/// #[derive(Debug, PartialEq, Merge)]
/// struct S {
///     option: Option<usize>,
///
///     #[merge(skip)]
///     s: String,
///
///     #[merge(strategy = ::merge2::bool::overwrite_false)]
///     flag: bool,
/// }
///
/// let mut val = S {
///     option: None,
///     s: "some ignored value".to_owned(),
///     flag: false,
/// };
/// val.merge(S {
///     option: Some(42),
///     s: "some other ignored value".to_owned(),
///     flag: true,
/// });
/// assert_eq!(S {
///     option: Some(42),
///     s: "some ignored value".to_owned(),
///     flag: true,
/// }, val);
/// ```
///
/// Setting a default merge strategy:
///
/// ```
/// use merge2::Merge;
///
/// #[derive(Debug, PartialEq, Merge)]
/// struct S {
///     option1: Option<usize>,
///     option2: Option<usize>,
///     option3: Option<usize>,
/// }
///
/// let mut val = S {
///     option1: None,
///     option2: Some(1),
///     option3: None,
/// };
/// val.merge(S {
///     option1: Some(2),
///     option2: Some(2),
///     option3: None,
/// });
/// assert_eq!(S {
///     option1: Some(2),
///     option2: Some(1),
///     option3: None,
/// }, val);
/// ```
pub trait Merge {
    /// Merge another object into this object.
    fn merge(&mut self, other: Self);
}

impl<T> Merge for Option<T> {
    /// Overwrite `option` only if it is `None`
    #[inline]
    fn merge(&mut self, right: Self) {
        if self.is_none() {
            *self = right;
        }
    }
}

/// Merge strategies for `Option`
pub mod option {
    /// On conflict, recursively merge the elements.
    #[inline]
    pub fn recursive<T: super::Merge>(left: &mut Option<T>, right: Option<T>) {
        if let Some(new) = right {
            if let Some(original) = left {
                original.merge(new);
            } else {
                *left = Some(new);
            }
        }
    }
}

macro_rules! skip_merge {
    ($typ: ident) => {
        impl Merge for $typ {
            #[inline(always)]
            fn merge(&mut self, _: Self) {}
        }
    };
    ($($typ: ident),*) => {
        $(skip_merge!($typ);)*
    };
}

skip_merge!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128, f32, f64, bool);

/// Merge strategies for boolean types.
pub mod bool {
    /// Overwrite left with right if the value of left is false.
    #[inline]
    pub fn overwrite_false(left: &mut bool, right: bool) {
        if !*left {
            *left = right;
        }
    }

    /// Overwrite left with right if the value of left is true.
    #[inline]
    pub fn overwrite_true(left: &mut bool, right: bool) {
        if *left {
            *left = right;
        }
    }
}

/// Merge strategies for numeric types.
///
/// These strategies are only available if the `num` feature is enabled.
#[cfg(feature = "num")]
pub mod num {
    /// Set left to the saturated some of left and right.
    #[inline]
    pub fn saturating_add<T: num_traits::SaturatingAdd>(left: &mut T, right: T) {
        *left = left.saturating_add(&right);
    }

    /// Overwrite left with right if the value of left is zero.
    #[inline]
    pub fn overwrite_zero<T: num_traits::Zero>(left: &mut T, right: T) {
        if left.is_zero() {
            *left = right;
        }
    }
}

/// Merge strategies for types that form a total order.
pub mod ord {
    use core::cmp;

    /// Set left to the maximum of left and right.
    #[inline]
    pub fn max<T: cmp::Ord>(left: &mut T, right: T) {
        if cmp::Ord::cmp(left, &right) == cmp::Ordering::Less {
            *left = right;
        }
    }

    /// Set left to the minimum of left and right.
    #[inline]
    pub fn min<T: cmp::Ord>(left: &mut T, right: T) {
        if cmp::Ord::cmp(left, &right) == cmp::Ordering::Greater {
            *left = right;
        }
    }
}

#[cfg(feature = "std")]
impl Merge for &str {
    #[inline]
    fn merge(&mut self, right: Self) {
        if self.is_empty() {
            *self = right;
        }
    }
}

#[cfg(feature = "std")]
impl Merge for String {
    #[inline]
    fn merge(&mut self, right: Self) {
        if self.is_empty() {
            *self = right;
        }
    }
}

#[cfg(feature = "std")]
impl<T> Merge for Vec<T> {
    #[inline]
    fn merge(&mut self, right: Self) {
        if self.is_empty() {
            *self = right;
        }
    }
}

/// Merge strategies for vectors.
///
/// These strategies are only available if the `std` feature is enabled.
#[cfg(feature = "std")]
pub mod vec {
    /// Append the contents of right to left.
    #[inline]
    pub fn append<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        if left.is_empty() {
            *left = right;
        } else {
            left.append(&mut right);
        }
    }

    /// Prepend the contents of right to left.
    #[inline]
    pub fn prepend<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        right.append(left);
        *left = right;
    }
}

#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
impl<K, V> Merge for HashMap<K, V> {
    #[inline]
    fn merge(&mut self, right: Self) {
        if self.is_empty() {
            *self = right;
        }
    }
}

/// Merge strategies for hash maps.
///
/// These strategies are only available if the `std` feature is enabled.
#[cfg(feature = "std")]
pub mod hashmap {
    use super::HashMap;
    use std::hash::Hash;

    /// On conflict, merge elements from `right` to `left`.
    ///
    /// In other words, this gives precedence to `left`.
    pub fn merge<K: Eq + Hash, V>(left: &mut HashMap<K, V>, right: HashMap<K, V>) {
        for (k, v) in right {
            left.entry(k).or_insert(v);
        }
    }

    /// On conflict, replace elements of `left` with `right`.
    ///
    /// In other words, this gives precedence to `right`.
    #[inline]
    pub fn replace<K: Eq + Hash, V>(left: &mut HashMap<K, V>, right: HashMap<K, V>) {
        left.extend(right)
    }

    /// On conflict, recursively merge the elements.
    pub fn recursive<K: Eq + Hash, V: crate::Merge>(left: &mut HashMap<K, V>, right: HashMap<K, V>) {
        use std::collections::hash_map::Entry;

        for (k, v) in right {
            match left.entry(k) {
                Entry::Occupied(mut existing) => existing.get_mut().merge(v),
                Entry::Vacant(empty) => {
                    empty.insert(v);
                }
            }
        }
    }
}
