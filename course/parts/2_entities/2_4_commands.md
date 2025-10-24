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

There are a few things to unpack here.

### Bundles

The argument given to the `spawn()` must be of a type that implements the `Bundle` trait.
A `Bundle` simply describes a set of components that will be added to an entity.

An entity can only have one of each type of component.

Conveniently, `Bundle` is implemented for any tuple of lengths up to 16 where all the members implement `Bundle`, 
and all `Component` types implement `Bundle`!
You can even circumvent the length limit by nesting tuples:

```rust
((C1, C2, ... , C16), (C17, C8, ... , C32))
```

### EntityCommands

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

### Despawning entities

You can also despawn entities using the entity commands.
This will also despawn any children the entity might have. 

```rust
fn get_commands_system(mut commands: Commands, query: Single<(Entity, &BouncyBall)>) {
    let entity_commands = commands.despawn(query.0);
}
```