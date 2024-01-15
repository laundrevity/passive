use crate::constants::{ENEMY_RADIUS, PLAYER_RADIUS};
use crate::sprite::{Instance, Sprite, Vertex};

pub struct GameObject {
    pub coords: (f32, f32),
    pub texture_index: usize,
}

pub struct Player {
    pub game_object: GameObject,
}

pub struct Enemy {
    pub game_object: GameObject,
}

impl Player {
    pub fn new() -> Self {
        Self {
            game_object: GameObject {
                coords: (0.0, 0.0),
                texture_index: 0,
            },
        }
    }
}

impl Enemy {
    pub fn new(coords: (f32, f32)) -> Self {
        Self {
            game_object: GameObject {
                coords,
                texture_index: 1,
            },
        }
    }
}

impl Sprite for Player {
    fn get_vertices(aspect_ratio: f32) -> Vec<Vertex> {
        let s = 1f32 / aspect_ratio;
        let r = PLAYER_RADIUS;
        vec![
            Vertex {
                position: [s * -r, -r, 0.0],
                tex_coords: [0.0, 1.0],
            }, // A
            Vertex {
                position: [s * r, -r, 0.0],
                tex_coords: [1.0, 1.0],
                // tex_coords: [0.5, 1.0],
            }, // B
            Vertex {
                position: [s * r, r, 0.0],
                tex_coords: [1.0, 0.0],
                // tex_coords: [0.5, 0.0],
            }, // C
            Vertex {
                position: [s * -r, r, 0.0],
                tex_coords: [0.0, 0.0],
            }, // D
        ]
    }

    fn get_indices() -> &'static [u16] {
        &[0, 1, 2, 0, 2, 3]
    }

    fn get_instance(&self, aspect_ratio: f32) -> Instance {
        let s = 1f32 / aspect_ratio;

        Instance {
            instance_pos: [
                s * self.game_object.coords.0,
                self.game_object.coords.1,
                0.0,
            ],
        }
    }
}

impl Sprite for Enemy {
    /// Diamond vertices
    ///      D
    ///  A      C
    ///      B   
    /// Texture coords
    /// [0, 0]      [1, 0]
    ///
    /// [0, 1]      [1, 1]
    fn get_vertices(aspect_ratio: f32) -> Vec<Vertex> {
        let s = 1f32 / aspect_ratio;
        let r = ENEMY_RADIUS;
        vec![
            Vertex {
                position: [-s * r, 0.0, 0.0],
                tex_coords: [0.0, 0.5],
            }, // A
            Vertex {
                position: [0.0, -r, 0.0],
                tex_coords: [0.5, 1.0],
            }, // B
            Vertex {
                position: [s * r, 0.0, 0.0],
                tex_coords: [1.0, 0.5],
            }, // C
            Vertex {
                position: [0.0, r, 0.0],
                tex_coords: [0.5, 0.0],
            }, // D
        ]
    }

    fn get_indices() -> &'static [u16] {
        &[0, 1, 2, 0, 2, 3]
    }

    fn get_instance(&self, aspect_ratio: f32) -> Instance {
        let s = 1f32 / aspect_ratio;

        Instance {
            instance_pos: [
                s * self.game_object.coords.0,
                self.game_object.coords.1,
                0.0,
            ],
        }
    }
}
