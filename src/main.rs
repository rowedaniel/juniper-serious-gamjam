//! Shows how to create graphics that snap to the pixel grid by rendering to a texture in 2D

use bevy::{
    prelude::*,
};

mod render;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (render::setup_camera, setup_mesh))
        .add_systems(Update, (rotate, render::fit_canvas))
        .run();
}


/// Spawns a capsule mesh on the pixel-perfect layer.
fn setup_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::default())),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(25., 0., 2.).with_scale(Vec3::splat(32.)),
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

