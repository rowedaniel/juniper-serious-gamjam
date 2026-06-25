use bevy::prelude::*;

mod character;
mod input;
mod render;

const MOVE_SPEED: f32 = 50.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (render::setup_camera, character::setup_character))
        .add_systems(
            Update,
            (
                rotate,
                move_player,
                render::fit_canvas,
                character::update_texture_atlas_index,
            ),
        )
        .run();
}

/// Rotates entities to demonstrate grid snapping.
fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<render::Rotate>>) {
    for mut transform in &mut transforms {
        let dt = time.delta_secs();
        transform.rotate_z(dt);
    }
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform, With<character::Player>>,
) {
    let direction = input::read_movement_input(&input);

    // Handle movement
    if direction != Vec2::ZERO {
        for mut transform in &mut transforms {
            let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
            transform.translation += delta.extend(0.0);
        }
    }
}
