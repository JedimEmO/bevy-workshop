# Commands

Your systems rarely directly interact with the world.
Instead, you issue commands that will be processed at the end of the frame.

To access the command queue, request the `Commands` type to your system:

```rust
fn my_world_changing_system(mut commands: Commands) {}
```

## Spawning entities

One of the most common things you'll want to do with commands, is to spawn entities.

The `spawn()` function will create a new entity, and assign the component bundle passed as its parameter to the entity:

```rust
#[derive(Component)]
struct MyComponent;

fn my_spawner(mut commands: Commands) {
    let entity_commands: EntityCommands = commands.spawn((
        Transform::default(),
        MyComponent
    ));
}
```


