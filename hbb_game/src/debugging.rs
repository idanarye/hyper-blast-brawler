// use amethyst::prelude::*;

use amethyst::core::nalgebra::{Vector3, Point3};
use amethyst::renderer::{DebugLines, Rgba};
use amethyst::shred::{System, Write};

pub struct DebugInfoSystem;

impl<'a> System<'a> for DebugInfoSystem {
    type SystemData = (
        Write<'a, DebugLines>,
        );

    fn run(&mut self, (mut debug_lines,): Self::SystemData) {
        // log::info!("About to draw debug lines");
        debug_lines.draw_direction(
            // Point3::new(100., 100., 0.),
            // Vector3::new(40., 40., 0.),
            Point3::new(0.2, 0.2, 0.),
            Vector3::new(0.4, 0.4, 0.),
            Rgba::GREEN);
        // log::info!("drawn debug lines");
    }
}
