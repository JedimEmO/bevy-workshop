# What is an ECS

Entity Component System describes an architecture where we store objects compositionally rather than as
fixed structures/classes of data coupled with behavior.

The core concept is that we declare what components are attached to an entity,
and we write systems that work upon specific sets of components.

Imagine that we wish to model a particle moving in a 3d space.

If we say each particle is an entity, this entity could be described in terms of having a position and a velocity.

```rust
type EntityID = u64;
struct Position(Vec3);
struct Velocity(Vec3);
```

Let's now say we store this data in an imaginary SQL-like table, 
where each column represents a component (the key for each row is the entity id)

```
| Entity ID | Position         | Velocity         |
| 1         | Vec3(0., 0., 0.) | Vec3(1., 0., 0.) |
| 2         | Vec3(0., 2., 6.) | Vec3(1., 0., 1.) |
```

We could now hallucinate an SQL-like query, where we simply iterate over all the entities that have the position and velocity component,
and increment the position by the velocity:

 ```sql
UPDATE entity SET position = position + velocity WHERE SELECT(position, velocity);
```

Read this as "for every entity that has both position and velocity, add the latter to the former."
