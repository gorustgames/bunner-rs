# bunner-rs

Simple game utilizing [bevy](https://bevyengine.org/) game engine and [Entity Component System](https://en.wikipedia.org/wiki/Entity_component_system) architecture.

## How to run examples
```rust
cargo run --example 01_move_sprite 
```

## Issues

* first water segment: sometimes higher rows in this first segment just don't have any logs at all

## Bevy quick links
* [Learn Bevi](https://bevyengine.org/learn/)
* [Bevy Examples](https://github.com/bevyengine/bevy/tree/latest/examples#examples)
* [Rust Doc](https://docs.rs/bevy/latest/bevy/)
* [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
* [Testing ECS systems](https://github.com/bevyengine/bevy/blob/latest/tests/how_to_test_systems.rs)

## Rust mutability

With Bevy one can get dizzy from all the mutability syntax. Quick overview:
```rust
    a: &T      // immutable binding of immutable reference
mut a: &T      // mutable binding of immutable reference
    a: &mut T  // immutable binding of mutable reference
mut a: &mut T  // mutable binding of mutable reference
```

* The first variant is absolutely immutable (without taking internal mutability of Cell and such into account) - you can neither change what a points to nor the object it currently references.
* The second variant allows you to change a to point somewhere else but it doesn't allow you to change the object it points to.
* The third variant does not allow to change a to point to something else but it allows mutating the value it references.
* And the last variant allows both changing a to reference something else and mutating the value this reference is currently pointing at.

StackOverflow:

* [link1](https://stackoverflow.com/questions/29672373/what-is-difference-between-mut-a-t-and-a-mut-t)
* [link2](https://stackoverflow.com/questions/28587698/whats-the-difference-between-placing-mut-before-a-variable-name-and-after-the)