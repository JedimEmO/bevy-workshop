# Solution

We can solve this by querying for the single bouncy ball we have in our world.
We then do some simple math to bounce the ball back and forth using the x_velocity value of the `BouncyBall` component.

```rust
fn implement_me_system(time: Res<Time>, mut query: Single<(&mut BouncyBall, &mut Transform)>) {
    let (mut bouncy_ball, mut transform) = query.into_inner();

    transform.translation.x += bouncy_ball.x_velocity * time.delta_secs();

    if transform.translation.x.abs() > 1.0 {
        let overshoot = transform.translation.x.fract();

        bouncy_ball.x_velocity *= -1.0;

        transform.translation.x = transform.translation.x.signum() * (1.0 - overshoot);
    }
}
```