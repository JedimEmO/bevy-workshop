# Bundles

The argument given to the `spawn()` must be of a type that implements the `Bundle` trait.
A `Bundle` simply describes a set of components that will be added to an entity.

An entity can only have one of each type of component.

Conveniently, `Bundle` is implemented for any tuple of lengths up to 16 where all the members implement `Bundle`,
and all `Component` types implement `Bundle`!
You can even circumvent the length limit by nesting tuples:

```rust
((C1, C2, ... , C16), (C17, C8, ... , C32))
```