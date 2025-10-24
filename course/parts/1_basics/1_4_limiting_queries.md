# With and Without you

If we want to limit queries without including the queried data, we can use With:

```rust
fn single_query(q: Single<&Transform, With<BouncyBall>>) {
    println!("My bouncy balls transform {BouncyBall:?}");
}
```

And sometimes we wish to not match components from entities that has a given component:

```rust
fn filtered_query(q: Query<&Transform, Without<BouncyBall>>) {}
```

We can even combine these as tuple types!

```rust
fn multi_filtered_query(q: Query<&Transform, (With<Velocity>, Without<BouncyBall>)>) {}
```