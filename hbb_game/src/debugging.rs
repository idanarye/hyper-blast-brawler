// use amethyst::prelude::*;

use amethyst::core::transform::Transform;
use amethyst::renderer::{DebugLines, Rgba, SpriteRender, SpriteSheet};
use amethyst::assets::{AssetStorage};
use amethyst::ecs::{
    System,
    Read,
    Write,
    ReadStorage,
    Join,
};

pub struct DebugInfoSystem;

impl<'a> System<'a> for DebugInfoSystem {
    type SystemData = (
        Write<'a, DebugLines>,
        ReadStorage<'a, Transform>,
        Read<'a, AssetStorage<SpriteSheet>>,
        ReadStorage<'a, SpriteRender>,
        );

    fn run(&mut self, (mut debug_lines, transforms, sprite_sheets, sprite_renders): Self::SystemData) {
        for (transform, sprite_render) in (&transforms, &sprite_renders).join() {
            let translation = transform.translation();
            let sprite_sheet = sprite_sheets.get(&sprite_render.sprite_sheet);
            let sprite = if let Some(sprite_sheet) = sprite_sheet {
                &sprite_sheet.sprites[sprite_render.sprite_number]
            } else {
                continue;
            };

            use amethyst::core::nalgebra::{Point3, Vector3, Matrix3};
            let frame: Vec<Point3<f32>> = vec![
                [-0.5, -0.5, 0.0].into(),
                [0.5, -0.5, 0.0].into(),
                [0.5, 0.5, 0.0].into(),
                [-0.5, 0.5, 0.0].into(),
            ];

            let scale = Matrix3::from_diagonal(&Vector3::new(sprite.width + 2., sprite.height + 2., 1.));
            let frame: Vec<_> = frame.into_iter().map(|v| scale * v + translation).collect();

            let mut prev = frame.last().unwrap();
            for point in frame.iter() {
                debug_lines.draw_line(*prev, *point, Rgba::GREEN);
                prev = point;
            }
        }
    }
}
