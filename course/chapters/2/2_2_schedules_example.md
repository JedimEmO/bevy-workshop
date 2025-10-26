# Example of schedules

```rust
fn main() {
  App::new()
    .add_systems(Startup, || {
      println!("Starting the application");
    })
    .add_systems(FixedUpdate, (
        || { println!("Fixed update A"); },
        |time: Res<Time>| {
          println!("Fixed timestep - this number should not vary: {}", time.delta_secs());
        },
      )
    )
    .add_systems(Update, 
      |time: Res<Time>| {
        println!("Framerate bound updates, this number may vary: {}", time.delta_secs());
      }
    )
    .run();
}
```