use amethyst::prelude::*;
use amethyst::core::transform::Transform;
// use amethyst::core::nalgebra::{Vector3};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::renderer::{
    Camera,
    PngFormat,
    Projection,
    SpriteRender,
    SpriteSheet,
    SpriteSheetFormat,
    SpriteSheetHandle,
    Texture,
    TextureMetadata,
    TextureHandle,
    DebugLines,
    DebugLinesParams,
};

pub struct HyperBlastBrawlerGame;

impl HyperBlastBrawlerGame {
    const HEIGHT: f32 = 1000.0;
    const WIDTH: f32 = 1000.0;
}

impl SimpleState for HyperBlastBrawlerGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("Game start");
        let world = data.world;

        add_debug_lines(world);

        let texture_handle = load_texture("texture/sheet.png", world);
        let spritesheet_handle = load_spritesheet("resources/spritesheet.ron", texture_handle, world);

        initialize_camera(world);
        add_dummy(world, &spritesheet_handle);
    }
}

fn add_debug_lines(world: &mut World) {
    log::info!("adding debug lines");
    world.add_resource(DebugLines::new().with_capacity(100));
    world.add_resource(DebugLinesParams {
        line_width: 800.,
    });
}

fn load_texture(name: impl Into<String>, world: &World) -> TextureHandle {
    let loader = world.read_resource::<Loader>();
    loader.load(name, PngFormat, TextureMetadata::srgb(), (), &world.read_resource::<AssetStorage<Texture>>())
}
fn load_spritesheet(name: impl Into<String>, texture_handle: TextureHandle, world: &World) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();
    let spritesheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(name, SpriteSheetFormat, texture_handle, (), &spritesheet_storage)
}

// fn load_spritesheet(name: impl Into<String>, world: &World) -> TextureHandle {

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    world.create_entity()
        .with(Camera::from(Projection::orthographic(0.0, HyperBlastBrawlerGame::WIDTH, 0.0, HyperBlastBrawlerGame::HEIGHT)))
        .with(transform)
        .build();
}

fn add_dummy(world: &mut World, spritesheet_handle: &SpriteSheetHandle) {
    let mut transform = Transform::default();
    transform.set_xyz(100.0, 500.0, 0.0);

    world.create_entity()
        .with(transform)
        .with(SpriteRender {
            sprite_sheet: spritesheet_handle.clone(),
            sprite_number: 0,
        })
        .build();
}
