# Gotchas with schedules

If you need to access data cleared every frame, such as the `just_pressed()` function on a `Gamepad`,
make sure you do this in the `Update` schedule, not `FixedUpdate`.
Since `FixedUpdate` systems do not execute every frame, the behavior will not be what you expect, since you will be dropping events.


```
Frame | Schedules           | Event                | State
1     | Update, FixedUpdate |                      | just_pressed = false
2     | Update              | User presses button  | just_pressed = true
3     | Update              |                      | just_pressed = false
4     | Update, FixedUpdate |                      | just_pressed = false
```