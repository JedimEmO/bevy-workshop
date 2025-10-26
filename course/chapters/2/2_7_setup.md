# Setup

You are going to want to perform some actions in the setup schedule.
Create game states and resources, spawn a camera and UI etc.

For our early simple games, we typically want to simply set up the main game environment.
This means:

* Creating a camera to render our world
* Create the level and game play entities
* Set up the player entity with an associated player controller to handle input

The first thing to do is to add a system to the startup schedule:

```rust
fn main() {
    App::new()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {}
```