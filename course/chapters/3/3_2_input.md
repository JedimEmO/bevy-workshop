# Input

We will now introduce our first third party crate.

Bevy does provide input handling resources and primitives,
but the `leafwing input manager` crate provides a slightly higher level and more ergonomic API.

## Action enum

With leafwing, we start out by defining the input actions we wish to have in an enum.
This enum should derive the `ActionLike` trait:

```rust
#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
enum GameActions {
    Fire,
    #[actionlike(Axis)]
    Rotation,
}
```

This enum should represent the input actions the user can make in our game.
The `#[actionlike(Axis)]` means the action represents a numeric range along a single axis.
There is also `DualAxis`, which, as you would think, represents x,y numeric ranges.

We now need to add an `InputManager` plugin for the specific action enum to our application:

```rust
app.add_plugins(InputManagerPlugin::<GameActions>::default ());
```

Note that if we have multiple action enums, we'll need to add another plugin for all the action types in our game.

## Setting up the input map

We now need to bind concrete inputs to our actions.
The way to do this with leafwing is to create an `InputMap`:

```rust
let input_map = InputMap::default ()
.with(GameActions::Fire, GamepadButton::South)
.with_axis(
GameActions::Rotation,
GamepadControlAxis::LEFT_X.with_deadzone_symmetric(0.15),
);
```

You can even provide multiple input bindings per action:

```rust
let input_map = InputMap::default ()
.with(GameActions::Fire, GamepadButton::South)
.with(GameActions::Fire, KeyCode::Space);
```

## Processing input actions

The last thing to do before we can process inputs, is to attach our input map to an entity.
As it happens, the `InputMap` type is a component:

```rust
let mut player = commands.spawn((
Player,
// ...
input_map
));
```

We can now write a query over input actions from our input map:

```rust
fn input_handling_system(
    query: Single<(&ActionState<GameActions>, &mut Transform), With<Player>>,
) { /* ... */ }
```

We can now check for button presses and read axis information:

```rust

fn input_handling_system(
    query: Single<(&ActionState<GameActions>, &mut Transform), With<Player>>,
) {
    let (action, mut transform) = query.into_inner();

    if action.just_pressed(&GameActions::Fire) {
        println!("Fire was triggered");
    }

    if let Some(axis_info) = action.axis_data(&GameActions::Rotation) {
        println!("Axis value: {}", axis_info.value);
    }
}
```

## Example application

This chapters example application has a small input example.
It's the starting point of a small shooter.

Here's the full input processing function for this game:

```rust

fn input_handling_system(
    mut commands: Commands,
    game_resources: ResMut<GameResources>,
    query: Single<(&ActionState<GameActions>, &mut Transform), With<Player>>,
) {
    let (action, mut transform) = query.into_inner();

    if action.just_pressed(&GameActions::Fire) {
        // spawn the bullet at the tip of the gun
        let mut transform = *transform;

        let forward = -transform.forward().as_vec3().normalize();

        transform.translation += forward * 0.5;

        commands.spawn((
            Mesh3d(game_resources.bullet_mesh.clone()),
            MeshMaterial3d(game_resources.bullet_material.clone()),
            transform,
            Velocity(forward * 10.),
        ));
    }

    if let Some(axis_info) = action.axis_data(&GameActions::Rotation) {
        let rot = -PI * (1. + axis_info.value) / 2.;
        transform.rotation = Quat::from_axis_angle(Vec3::Y, rot - PI / 2.);
    }
}
```

You can run it with the following command:

```shell
cargo run --bin chapter-3-examples -- input
```

You can find more examples of how to use leafing from their examples directory:

[https://github.com/Leafwing-Studios/leafwing-input-manager/tree/main/examples](https://github.com/Leafwing-Studios/leafwing-input-manager/tree/main/examples)
