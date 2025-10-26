# Creating the mesh and material

Before we can create our components, we need to make sure we create a `Handle` for holding both the mesh and the
material.
Bevy uses this indirection to more efficiently handle transfer of data into the GPU.
This means that multiple components can reference the same material and mesh, avoiding uploading duplicate data into the
GPU for the render steps.

To create the two resources, we need to request the `Assets` resource for each type.
The `Assets` type is a resource holding a collection of a given type of data.

```rust
fn spawn_geometry_system(mut commands: Commands, mut commands: Commands,
                         mut meshes: ResMut<Assets<Mesh>>,
                         mut materials: ResMut<Assets<StandardMaterial>>) {
}
```

With access to those two collections, we can now create a primitive cube:

```rust
let cube = Cuboid::from_size(Vec3::splat(1.));
let cube_mesh_handle: Handle<Mesh> = meshes.add(cube);
```

The handle we receive is `Clone`, and contains a pointer to the actual mesh data.

Similarly, we can create a standard material:

```rust
let material_red = StandardMaterial::from_color(tailwind::RED_500);
let material_handle: Handle<StandardMaterial> = materials.add(material_red);
```

The standard material is actually a full PBR material, with lots of fancy features.
For now, we make a simple one with only a diffuse color.