// input_system.rs

use crate::components::*;
use crate::constants::*;
use crate::resources::InputQueue;
use ggez::event::KeyCode;
use specs::{world::Index, Entities, Join, ReadStorage, System, Write, WriteStorage};

use std::collections::HashMap;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // Data
    type SystemData = (Write<'a, InputQueue>, Entities<'a>, WriteStorage<'a, Position>, ReadStorage<'a, Player>, ReadStorage<'a, Movable>, ReadStorage<'a, Immovable>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, movables, immovables) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                // Get movables and immovables
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                .join()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();

                // Iterate from current pos to end of map to check moves
                let (start, end, is_x) = match key {
                    KeyCode::Up | KeyCode::W => (position.y, 0, false),
                    KeyCode::Down | KeyCode::S => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left | KeyCode::A => (position.x, 0, true),
                    KeyCode::Right | KeyCode::D => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    // Find a movable
                    // - If it exists, try move and continue
                    // - If it doesn't, continue and try to find immovable
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => {
                            // Find immovable
                            // - If it exists, stop and don't move anything
                            // - If it doesn't, stop cause there's a gap
                            match immov.get(&pos) {
                                Some(_id) => to_move.clear(),
                                None => break,
                            }
                        }
                    }
                }
            }  
        }

        // Actually move what needs to be moved
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::Up | KeyCode::W => position.y -= 1,
                    KeyCode::Down | KeyCode::S => position.y += 1,
                    KeyCode::Left | KeyCode::A => position.x -= 1,
                    KeyCode::Right | KeyCode::D => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}