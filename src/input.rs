//! Movement system largely based on The Impatient Programmer's Guide to Bevy and Rust Chapter 3
use bevy::prelude::*;

pub fn read_movement_input(input: &ButtonInput<KeyCode>) -> Vec2 {
    const MOVEMENT_KEYS: [(KeyCode, Vec2); 8] = [
        (KeyCode::ArrowLeft, Vec2::NEG_X),
        (KeyCode::KeyA, Vec2::NEG_X),
        (KeyCode::ArrowRight, Vec2::X),
        (KeyCode::KeyD, Vec2::X),
        (KeyCode::ArrowUp, Vec2::Y),
        (KeyCode::KeyW, Vec2::Y),
        (KeyCode::ArrowDown, Vec2::NEG_Y),
        (KeyCode::KeyS, Vec2::NEG_Y),
    ];

    MOVEMENT_KEYS
        .iter()
        .filter(|(key, _)| input.pressed(*key))
        .map(|(_, dir)| *dir)
        .sum()
}
