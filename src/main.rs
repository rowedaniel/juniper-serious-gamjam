use bevy::prelude::*;

mod input;
mod render;

#[derive(Component)]
struct Player;
const MOVE_SPEED: f32 = 50.;
const CHARACTER_PIXEL_HEIGHT: u32 = 32;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (render::setup_camera, setup_character))
        .add_systems(Update, (rotate, move_player, render::fit_canvas))
        .run();
}

fn setup_character(
    // mainly from https://medium.com/@thomsmed/animating-sprites-using-bevys-animationplayer-fa715d2c0815
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // # Character
    let character_name = Name::new("Character");
    let character_entity = commands.spawn((Player, character_name.clone())).id();

    let sprite_sheet_num_rows = 1;
    let sprite_sheet_num_columns = 1; // increase when added sprites with animations
    let sprite_sheet_section_size = CHARACTER_PIXEL_HEIGHT;

    let texture_handle = asset_server.load("businessman.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(sprite_sheet_section_size),
        sprite_sheet_num_columns,
        sprite_sheet_num_rows,
        None,
        None,
    );
    let texture_atlas_layout_handle = texture_atlas_layouts.add(texture_atlas_layout);

    commands.entity(character_entity).insert((
        character_name.clone(),
        Sprite::from_atlas_image(
            texture_handle.clone(),
            TextureAtlas {
                layout: texture_atlas_layout_handle.clone(),
                index: 0,
            },
        ),
        Transform::from_xyz(0.0, (CHARACTER_PIXEL_HEIGHT / 2) as f32, 0.0),
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
