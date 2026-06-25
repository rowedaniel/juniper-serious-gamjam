// base for characters, mainly based on https://medium.com/@thomsmed/animating-sprites-using-bevys-animationplayer-fa715d2c0815
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Player;

// const CHARACTER_PIXEL_HEIGHT: u32 = 32; // for clown and businessman
const CHARACTER_PIXEL_HEIGHT: u32 = 24; // FOR EXAMPLE

pub fn setup_character(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // # Character
    let character_name = Name::new("Character");
    let character_entity = commands.spawn((Player, character_name.clone())).id();

    let sprite_sheet_num_rows = 1;
    // let sprite_sheet_num_columns = 1; // for non-animated
    let sprite_sheet_num_columns = 7; //for EXAMPLE
    let sprite_sheet_section_size = CHARACTER_PIXEL_HEIGHT;

    let texture_handle = asset_server.load("example-idle-run.png");
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

// ---

pub fn update_texture_atlas_index(
    sprites: Query<&mut Sprite>,
    mut timer: Local<Timer>,
    time: Res<Time>,
) {
    if timer.tick(time.delta()).is_finished() {
        timer.reset();
        timer.set_duration(Duration::from_secs_f32(0.3)); // 0.3 seconds timer duration
    } else {
        return;
    }

    for mut sprite in sprites {
        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        // Jump to next index every 1 seconds (assuming 1..5 are valid indices)
        if texture_atlas.index > 4 {
            texture_atlas.index = 0;
        } else {
            texture_atlas.index += 1;
        }
    }
}
