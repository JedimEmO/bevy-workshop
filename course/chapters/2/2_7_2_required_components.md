# Required Components

We need to quickly mention a concept called `required components` at this juncture.

When we derive (or implement) the `Component` trait, we can annotate it with a set of `required components`.
These are components that Bevy will ensure are attached to the entity for which we spawn the component we are implementing.

Take a look at the definition of the Camera3d component:

```rust
#[derive(Component)]
#[require(Camera, Projection)]
pub struct Camera3d { /* ... */}
```

This means that whenever you attach the `Camera3d` component to an entity, Bevy will ensure that the `Camera` and `Projection` components are also attached to it.

If we then look at the `Camera` component, we see it in turn has more required components:
```rust
#[require(
    Frustum,
    CameraMainTextureUsages,
    VisibleEntities,
    Transform,
    Visibility
)]
pub struct Camera { /* ... */ }
```

Note that if you provide the component manually, Bevy will use the value you provided and not replace it with its own default value.
