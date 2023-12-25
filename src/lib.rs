//! Provides [`Merge`][] trait that can be used to merge structs into single by it's values:
//!
//! ```
//! trait Merge: Sized {
//!     fn merge(&mut self, other: &mut Self);
//! }
//! ```
//!
//! # Usage
//!
//! The [`Merge`][] trait can be used to merge two structs into single by values. The example
//! use case is merging configuration from different sources: environment variables,
//! multiple configuration files and command-line arguments, see the [`args.rs`][] example.
//!
//! `Merge` can be derived for structs. Also you can provide custom merge strategies
//! for any fields that don’t implement `Merge` trait.
//! A merge strategy is a function with the signature `fn merge<T>(left: &mut T, right: &mut T)`
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
//! let mut defaults = User {
//!     name: "",
//!     location: Some("Internet"),
//!     groups: vec!["rust"],
//! };
//! let mut ferris = User {
//!     name: "Ferris",
//!     location: None,
//!     groups: vec!["mascot"],
//! };
//! ferris.merge(&mut defaults);
//!
//! assert_eq!("Ferris", ferris.name);
//! assert_eq!(Some("Internet"), ferris.location);
//! assert_eq!(vec!["mascot", "rust"], ferris.groups);
//! ```
//!
//! [`Merge`]: trait.Merge.html
//! [`args.rs`]: https://github.com/RoDmitry/merge2/blob/main/examples/args.rs

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
/// val.merge(&mut S {
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
/// val.merge(&mut S {
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
pub trait Merge: Sized {
    /// Merge another object into this object.
    fn merge(&mut self, other: &mut Self);
}

// Merge strategies applicable to any types
pub mod any {
    /// Overwrite `left` with `right` regardless of their values.
    #[inline]
    pub fn overwrite<T: Default>(left: &mut T, right: &mut T) {
        *left = core::mem::take(right);
    }

    /// Overwrite `left` with `right` if the value of `left` is equal to the default for the type.
    #[inline]
    pub fn overwrite_default<T: Default + PartialEq>(left: &mut T, right: &mut T) {
        if *left == T::default() {
            core::mem::swap(left, right);
        }
    }

    /// Swap `left` and `right` regardless of their values.
    #[inline]
    pub fn swap<T>(left: &mut T, right: &mut T) {
        core::mem::swap(left, right);
    }
}

impl<T> Merge for Option<T> {
    /// Overwrite `option` only if it is `None`
    #[inline]
    fn merge(&mut self, right: &mut Self) {
        if self.is_none() {
            core::mem::swap(self, right);
        }
    }
}

/// Merge strategies for `Option`
pub mod option {
    /// On conflict, recursively merge the elements.
    #[inline]
    pub fn recursive<T: super::Merge>(left: &mut Option<T>, right: &mut Option<T>) {
        if let Some(mut new) = right.take() {
            if let Some(original) = left {
                original.merge(&mut new);
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
            fn merge(&mut self, _: &mut Self) {}
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
    pub fn overwrite_false(left: &mut bool, right: &mut bool) {
        if !*left {
            *left = *right;
        }
    }

    /// Overwrite left with right if the value of left is true.
    #[inline]
    pub fn overwrite_true(left: &mut bool, right: &mut bool) {
        if *left {
            *left = *right;
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
    pub fn saturating_add<T: num_traits::SaturatingAdd>(left: &mut T, right: &mut T) {
        *left = left.saturating_add(right);
    }
}

/// Merge strategies for types that form a total order.
pub mod ord {
    use core::cmp;

    /// Set `left` as `right` if `left` is Less than `right`, set `right` as Default
    #[inline]
    pub fn max_def<T: cmp::PartialOrd + Default>(left: &mut T, right: &mut T) {
        if cmp::PartialOrd::partial_cmp(left, right) == Some(cmp::Ordering::Less) {
            *left = core::mem::take(right);
        }
    }

    /// Set `left` as `right` if `left` is Greater than `right`, set `right` as Default
    #[inline]
    pub fn min_def<T: cmp::PartialOrd + Default>(left: &mut T, right: &mut T) {
        if cmp::PartialOrd::partial_cmp(left, right) == Some(cmp::Ordering::Greater) {
            *left = core::mem::take(right);
        }
    }

    /// Swap elements if `left` is Less than `right`.
    #[inline]
    pub fn max_swap<T: cmp::PartialOrd>(left: &mut T, right: &mut T) {
        if cmp::PartialOrd::partial_cmp(left, right) == Some(cmp::Ordering::Less) {
            core::mem::swap(left, right);
        }
    }

    /// Swap elements if `left` is Greater than `right`.
    #[inline]
    pub fn min_swap<T: cmp::PartialOrd>(left: &mut T, right: &mut T) {
        if cmp::PartialOrd::partial_cmp(left, right) == Some(cmp::Ordering::Greater) {
            core::mem::swap(left, right);
        }
    }
}

#[cfg(feature = "std")]
impl Merge for &str {
    #[inline]
    fn merge(&mut self, right: &mut Self) {
        if self.is_empty() {
            core::mem::swap(self, right);
        }
    }
}

#[cfg(feature = "std")]
impl Merge for String {
    #[inline]
    fn merge(&mut self, right: &mut Self) {
        if self.is_empty() {
            core::mem::swap(self, right);
        }
    }
}

/// Merge strategies for strings.
///
/// These strategies are only available if the `std` feature is enabled.
#[cfg(feature = "std")]
pub mod string {
    /// Append the contents of right to left.
    #[inline]
    pub fn append(left: &mut String, right: &mut String) {
        let new = core::mem::take(right);
        left.push_str(&new);
    }

    /// Prepend the contents of right to left.
    #[inline]
    pub fn prepend(left: &mut String, right: &mut String) {
        right.push_str(left);
        *left = core::mem::take(right);
    }
}

#[cfg(feature = "std")]
impl<T> Merge for Vec<T> {
    #[inline]
    fn merge(&mut self, right: &mut Self) {
        if self.is_empty() {
            core::mem::swap(self, right);
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
    pub fn append<T>(left: &mut Vec<T>, right: &mut Vec<T>) {
        if left.is_empty() {
            core::mem::swap(left, right);
        } else {
            left.append(right);
        }
    }

    /// Prepend the contents of right to left.
    #[inline]
    pub fn prepend<T>(left: &mut Vec<T>, right: &mut Vec<T>) {
        right.append(left);
        core::mem::swap(left, right);
    }
}

#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
impl<K, V> Merge for HashMap<K, V> {
    #[inline]
    fn merge(&mut self, right: &mut Self) {
        if self.is_empty() {
            core::mem::swap(self, right);
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
    #[inline]
    pub fn merge<K: Eq + Hash, V>(left: &mut HashMap<K, V>, right: &mut HashMap<K, V>) {
        let map = core::mem::take(right);
        for (k, v) in map {
            left.entry(k).or_insert(v);
        }
    }

    /// On conflict, replace elements of `left` with `right`.
    ///
    /// In other words, this gives precedence to `right`.
    #[inline]
    pub fn replace<K: Eq + Hash, V>(left: &mut HashMap<K, V>, right: &mut HashMap<K, V>) {
        left.extend(core::mem::take(right))
    }

    /// On conflict, recursively merge the elements.
    pub fn recursive<K: Eq + Hash, V: super::Merge>(
        left: &mut HashMap<K, V>,
        right: &mut HashMap<K, V>,
    ) {
        use std::collections::hash_map::Entry;

        let map = core::mem::take(right);
        for (k, mut v) in map {
            match left.entry(k) {
                Entry::Occupied(mut existing) => existing.get_mut().merge(&mut v),
                Entry::Vacant(empty) => {
                    empty.insert(v);
                }
            }
        }
    }

    /// Merge recursively elements only if the key is present in `left` and `right`.
    pub fn intersection<K: Eq + Hash, V: super::Merge>(
        left: &mut HashMap<K, V>,
        right: &mut HashMap<K, V>,
    ) {
        use std::collections::hash_map::Entry;

        let map = core::mem::take(right);
        for (k, mut v) in map {
            if let Entry::Occupied(mut existing) = left.entry(k) {
                existing.get_mut().merge(&mut v);
            }
        }
    }
}
