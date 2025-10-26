# Resources

The `Res` and `ResMut` types allow you to borrow and exclusively borrow resources from the game world.
You can think of a resource as a singleton type; there's only one resource value per type.

The `Time` resource we saw before is maintained by Bevy.
It is updated every frame and allows us to access the time delta per frame.
This enables delta time calculation, as we saw in the movement example system:

```rust
fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (position, velocity) in query.iter_mut() {
        *position.transform += *velocity.0 * time.delta_secs();
    }
}
```

This is an important technique, as it lets us write frame rate independent logic. 
If we don't decouple our simulation from the frame rate, it is challenging to ensure our game behaves consistently across various hardware!

## Making our own resource

It's straightforward to make our own custom resource type. 
First, we declare the type and derive `Resource` for it:

```rust
#[derive(Resource)]
struct MySingleton(u64);
```

We now need to insert the resource into our world:

```rust
fn init() {
    App::new()..insert_resource(MyResource(42)).run();
}
```

After this, the resource is available to query from a system. 
Let's exclusively borrow it so that we can alter the value:

```rust
fn increment_my_resource(mut data: ResMut<MySingleton>) {
    data.0 += 1;
}
```