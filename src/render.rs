use bevy::{
    camera::visibility::RenderLayers, camera::RenderTarget, color::palettes::css::GRAY, prelude::*,
    render::render_resource::TextureFormat, window::WindowResized,
};

/// In-game resolution width.
pub const RES_WIDTH: u32 = 160;

/// In-game resolution height.
pub const RES_HEIGHT: u32 = 90;

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

/// Render layers for high-resolution rendering.
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
pub struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
pub struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
pub struct OuterCamera;

#[derive(Component)]
pub struct Rotate;


pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // This Image serves as a canvas representing the low-resolution game screen
    let canvas =
        Image::new_target_texture(RES_WIDTH, RES_HEIGHT, TextureFormat::Bgra8UnormSrgb, None);

    let image_handle = images.add(canvas);

    // This camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera2d,
        Camera {
            // Render before the "main pass" camera
            order: -1,
            clear_color: ClearColorConfig::Custom(GRAY.into()),
            ..default()
        },
        RenderTarget::Image(image_handle.clone().into()),
        Msaa::Off,
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));

    // Spawn the canvas
    commands.spawn((Sprite::from_image(image_handle), Canvas, HIGH_RES_LAYERS));

    // The "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2d, Msaa::Off, OuterCamera, HIGH_RES_LAYERS));
}

/// Scales camera projection to fit the window (integer multiples only).
pub fn fit_canvas(
    mut resize_messages: MessageReader<WindowResized>,
    mut projection: Single<&mut Projection, With<OuterCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    for window_resized in resize_messages.read() {
        let h_scale = window_resized.width / RES_WIDTH as f32;
        let v_scale = window_resized.height / RES_HEIGHT as f32;
        projection.scale = 1. / h_scale.min(v_scale).floor();
    }
}
