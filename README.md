# Merge2

Provides `Merge` trait that can be used to merge structs into single by it's values:

```rust
trait Merge {
    fn merge(&mut self, other: &mut Self);
}
```

`Merge` can be derived for structs:

<!-- should be kept in sync with examples/user.rs -->

```rust
use merge2::Merge;

#[derive(Merge)]
struct User {
    #[merge(skip)]
    pub name: &'static str,
    pub location: Option<&'static str>,
    #[merge(strategy = ::merge2::vec::append)]
    pub groups: Vec<&'static str>,
}

let mut defaults = User {
    name: "",
    location: Some("Internet"),
    groups: vec!["rust"],
};
let mut ferris = User {
    name: "Ferris",
    location: None,
    groups: vec!["mascot"],
};
ferris.merge(&mut defaults);

assert_eq!("Ferris", ferris.name);
assert_eq!(Some("Internet"), ferris.location);
assert_eq!(vec!["mascot", "rust"], ferris.groups);
```

A merge strategy is a function with the signature `fn merge<T>(left: &mut T, right: &mut T)`
that merges `right` into `left`. The `merge2` crate provides strategies
for the most common types, but you can also define your own strategies.

The trait can be used to merge configuration from different sources:
environment variables, multiple configuration files and command-line
arguments, see the `args.rs` example.

## Features

This crate has the following features:

-   `derive` (default): Enables the derive macro for the `Merge` trait using the
    `merge_derive` crate.
-   `num` (default): Enables the merge strategies in the `num` module that
    require the `num_traits` crate.
-   `std` (default): Enables the merge strategies for the `hashmap` and `vec`
    that require the standard library. If this feature is not set,
    `merge2` is a `no_std`.

### Based on the [source code](https://git.sr.ht/~ireas/merge-rs) of the `Merge` crate
