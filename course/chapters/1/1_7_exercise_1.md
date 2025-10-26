# Exercise: bounce the ball

Let's get our hands dirty.

In `exercises/chapter_1/src/bin/simple_movement.rs` you will find the code for this exercise.

To run it:

```shell
cargo run --bin simple-movement
```

It has a lot of scaffolding to spawn meshes and a camera that we haven't learned yet.
Ignore it, for now.

Let's focus on our task; we will make the ball bounce back and forth in the [-1,1] range of the X-axis!
To achieve this, we need to implement this system:

```rust
fn implement_me_system(time: Res<Time> /* something probably goes here */) {
}
```

It has already been registered to run, but feel free to rename it if you want something nicer!

Pay attention to the `BouncyBall` attached to our sphere; it may come in handy when implementing.