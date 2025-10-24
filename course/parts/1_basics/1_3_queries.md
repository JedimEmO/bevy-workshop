# Queries

Queries are the heart of the ECS.
They let us extract data from components belonging to the same entity.

We will only get entries in the query for entities that have all the queried components.
We can, however, have optional components in the query:

```rust
fn query_optional(query: Query<(&Transform, Option<(&LastGroundContact)>)>) {}
```

If we want to mutate data in a component, we need to ask for mutable references like we saw before

```rust
fn query_mut(mut query: Query<(&mut Transform)>) {}
```

If we know we have exactly one entity matching a query, we can use the `Single` query to avoid iterating:

```rust
fn single_query(q: Single<&BouncyBall>) {
    println!("My bouncy ball! {BouncyBall:?}");
}
```

