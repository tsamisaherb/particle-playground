# Turbo-Particles

## Description

Flexible, fast particle generation to use in Turbo Projects.

## How it works

Add the `particles.rs` file into your src folder. Then add

```rust
mod particles;
use particles::*;
```

to your `lib.rs` file.

To create particles, you'll need to add a `ParticleManager` to your game state. You can initialize it with `ParticleManager::new()`.

Then call `ParticleManager.update()` and `ParticleManager.draw()` in your go loop. 

## Making a Burst

When you want to create your particles, you use `ParticleManager.create_burst(BurstConfig)`. A Burst Config contains all the details for your burst of particles.

```rust
pub struct BurstConfig {
    pub source: BurstSource,
    pub shape: Shape,
    pub x_velocity: (f32, f32), //min, max
    pub y_velocity: (f32, f32),
    pub lifetime: (f32, f32),
    pub color: u32,
    pub size: (u32, u32),
    pub count: u32,
    pub should_fade_out: bool,
}
```

The tuples are there so you can define a range (e.g. a minimum and maximum). When you create a lot of particles, it generally looks better when there is some variation in the size, speed and direction.

The `BurstSource` is the range of positions that the particles will emit from. This can be a **Rectangle**, **Circle** or a **Point**.

The `Shape` is the shape of the particle itself: **Square**, **Circle** or **Sprite**.

Count is the number of particles in this burst.

Here is an example config for an explosion effect:

```rust
BurstConfig {
    source: BurstSource::Circle {
        center: pos,
        radius: 1.0,
    },
    x_velocity: (-1.0, 1.0),
    y_velocity: (-1.0, 1.0),
    lifetime: (0.4, 0.4),
    color: 0xFF6633ff,
    size: (4, 6),
    count: 10,
    shape: Shape::Square,
    should_fade_out: false,
}
```

### Tips and Tricks

Sometimes you'll want to create multiple smaller bursts to capture certain effects, or to mix different colors into your particle system. In the trail example, there is a constant small burst coming from the back of the moving circle. In the confetti example, there are three identical bursts of different colors

When you're testing particles, build a testing environment where you can trigger the emission over and over again. Take advantage of Turbo's hot reloading to adjust your burst config and keep testing until you have it looking just right.





