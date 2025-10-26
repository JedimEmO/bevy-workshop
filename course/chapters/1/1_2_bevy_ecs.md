# Bevy ECS

As you may have guessed, the Bevy ECS system works much the same as our imaginary SQL from before.

* Entities are spawned with associated components.
* Each component is a simple Rust struct, for which the `Component` trait is implemented/derived.

We declare operations on entities by way of systems in bevy.
A system is simply a function requesting some data from the game world.

The simple movement query from the previous slide would look like this:

```rust
#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Velocity(Vec3);

fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (position, velocity) in query.iter_mut() {
        *position.transform += *velocity.0 * time.delta_secs();
    }
}
```

Pay attention to the `Query` and `Res` types, they are crucial.
This is how you request data from the world.

By sheer magic and unicorn tears, all we have to do is tell Bevy when to run this function, and it will resolve anything
else.
It will register the function to run at the requested schedule (a fixed timestep update interval in this case),
and extract all the required data pass it as arguments to the function.

```rust
fn main() {
    App::new()
        .add_systems(FixedUpdate, movement_system)
        .run();
}
```
