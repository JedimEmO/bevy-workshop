# Schedules

As we saw when introducing systems, we need to let bevy know *when* to run the systems.

The Main schedule provided by Bevy will run the following schedules (this is not a typo; schedules can be nested):

## Startup

Startup schedules run once, at the beginning of the [Main schedule](https://docs.rs/bevy/latest/bevy/prelude/struct.Main.html):

* PreStartup
* Startup
* PostStartup

## Update

* First
* PreUpdate
* RunFixedMainLoop 
  * FixedFirst
  * FixedPreUpdate
  * FixedUpdate
  * FixedPostUpdate
  * FixedLast
* Update
* PostUpdate
* Last

The update systems run every render frame.
The only systems you should put into these schedules are things requiring low latency, such as input handling, UI and audio.

Your other systems should probably run in the fixed update loop.

The schedules in the fixed main loop will run at a fixed time-interval.
They will run 0 or 1 times for each update cycle, depending on the elapsed time.

Physics simulation, AI, game-rules etc. should run in fixed update, as they are not latency sensitive.

## That's a lot, what do I actually use??

In reality, you can get far by using the `Startup`, `FixedUpdate` and `Update` schedules.
They provide most of what you will need, especially when combined with the run conditions we will cover next.

