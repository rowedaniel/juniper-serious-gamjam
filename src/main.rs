//! Shows how to create graphics that snap to the pixel grid by rendering to a texture in 2D

use bevy::{material::key, math::VectorSpace, prelude::*};

mod render;

#[derive(Component)]
struct Player;
const MOVE_SPEED: f32 = 1.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (render::setup_camera, setup_mesh))
        .add_systems(Update, (rotate, move_player, render::fit_canvas))
        .run();
}

/// Spawns a capsule mesh on the pixel-perfect layer.
fn setup_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Capsule2d::default())),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(8.)),
        render::Rotate,
        render::PIXEL_PERFECT_LAYERS,
    ));
}

/// Rotates entities to demonstrate grid snapping.
fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<render::Rotate>>) {
    for mut transform in &mut transforms {
        let dt = time.delta_secs();
        transform.rotate_z(dt);
    }
}

fn move_player(
    mut transforms: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in transforms.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        } // up
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        } // down
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        } // right
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        } // left

        if 0.0 < direction.length() {
            transform.translation += MOVE_SPEED * direction.normalize();
        }
    }
}
