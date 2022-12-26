// rendering_system.rs

use crate::{components::*, resources::Gameplay};
use crate::constants::TILE_WIDTH;

use ggez::graphics::Color;
use ggez::{Context, graphics::{self, DrawParam, Image}};
use specs::{Join, ReadStorage, System, Read};
use glam::Vec2;
pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = Vec2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = Vec2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (Read<'a, Gameplay>, ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, positions, renderables) = data;

        // Clear screen, thus setting background
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all renderables sorted by their z for visual layering
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Draw all renderables
        for (position, renderable) in rendering_data.iter() {
            let image = Image::new(self.context, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Render any text
        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);

        // Present context to draw on screen
        graphics::present(self.context).expect("expected to present");
    }
}