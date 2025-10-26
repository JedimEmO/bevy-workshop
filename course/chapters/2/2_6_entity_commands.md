
# EntityCommands

The value returned from `spawn()` is of the type `EntityCommands`.
This will allow you to access the entity's ID through the `id()` function, insert or remove components, insert and remove children and adding observers.

```rust
fn entity_commands_example_system(mut commands: Commands) {
    let mut entity_commands = commands.spawn(Transform::default());

    let child = commands.spawn(());
    entity_commands.add_child(child.id());
    
    // Listen for Attacked events targeted at the entity
    entity_commands.observe(|event: On<Attacked>| {
        println!("HELP! HELP! HELP!");
    });
}
```

You can also get entity commands by using the `entity(id)` function on the `Commands` struct:

```rust
fn get_commands_system(mut commands: Commands, query: Single<(Entity, &BouncyBall)>) {
    let entity_commands = commands.entity(query.0);
}
```

## Despawning entities

You can also despawn entities using the entity commands.
This will also despawn any children the entity might have.

```rust
fn get_commands_system(mut commands: Commands, query: Single<(Entity, &BouncyBall)>) {
    let entity_commands = commands.despawn(query.0);
}
```