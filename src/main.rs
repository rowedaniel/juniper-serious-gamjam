use bevy::prelude::*;

mod input;
mod render;

#[derive(Component)]
struct Player;
const MOVE_SPEED: f32 = 50.;

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
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform, With<Player>>,
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
