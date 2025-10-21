# Transforms

Bevy provides a lot of built-in components and default systems.

One of these core systems involves transforms and global transforms.

A `Transform` component describes an entity's translation, rotation and scale in its local coordinate system.
Children of entities with transforms are transformed relative to their parents.
This makes it a bit tricky to resolve their absolute world position.

```rust
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
```

Luckily, bevy also attaches a `GlobalTransform` component to all entities that have a `Transform`.
It is updated to always compute the correct absolute position based on its parents.

Understanding how this is done requires a deeper understanding of systems than we currently have covered. 

Don't modify the `GlobalTransform` component unless you really know what you're doing!