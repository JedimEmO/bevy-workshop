# 3D Camera

The `Camera3D` component will set up the 3d rendering path, as well as a default perspective projection.
The defaults are useful for quickly getting a 3D environment up and running.

Here's a simple example:

```rust
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 1., 1.).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}
```

This will create an entity holding all the default required camera components,
but with an overridden transform.

Also pay attention to the `looking_at()` helper function on `Transform`, it's a useful utility!
