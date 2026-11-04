# Code that should compile emits a compile time error in new Rust version (Regression)

Depending on which version of (stable) Rust you try to compile this code, it may :

- Compile fine:

```bash
rustup override set 1.96.0
cargo check
```
(Also work with version `1.97.0`)

- Produce an error that makes absolutely no sens (at compile time) :

```bash
rustup override set 1.98.0
cargo check
```

```rs
error[E0119]: conflicting implementations of trait `IndexGet<_>` for type `CustomIndex`
  --> custom_collection\src\lib.rs:15:1
   |
15 | impl<C> IndexGet<C> for CustomIndex where C: Index<CustomIndex>
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: conflicting implementation in crate `index_get`:
           - impl<I, C> IndexGet<C> for I
             where I: SliceIndex<C>, C: Index<I>;
   = note: downstream crates may implement trait `std::slice::SliceIndex<_>` for type `CustomIndex`
```

This error should not be possible, especially the `downstream crates may implement trait std::slice::SliceIndex<_> for type CustomIndex`.
Due to the [orphan rule](<https://doc.rust-lang.org/reference/items/implementations.html#:~:text=either%20the%20trait%20or%20at%20least%20one%20of%20the%20types%20in%20the%20implementation%20is%20defined%20in%20the%20current%20crate>), it's impossible for a downstream crates to implement a foreign trait (`std::slice::SliceIndex<_>`) on a foreign type (`custom_collection::CustomIndex`).

# Impact

This impacted some of my crates on crate.io that don't compile anymore :

https://crates.io/crates/hexga_generational (It define a similar trait `IndexGet` named `IndexExtension`)
https://crates.io/crates/hexga_core (It impl the trait for a custom index type named `UntypedGenIDOf`)

This is not new, it happen a few months ago.

At first I didn't pay attention because I was mainly working on nightly and assumed it was a nightly bug, so I just rollback to an older version.
But I'm forced to admit that this regression is still present today on stable Rust.
