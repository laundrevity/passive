use std::f32::consts::PI;

use crate::constants::{ENEMY_RADIUS, GATE_RADIUS, NUM_TEXTURES, PLAYER_RADIUS};
use crate::sprite::{Instance, Sprite, Vertex};

fn t(x: f32, texture_index: f32) -> f32 {
    let x_min = texture_index / (NUM_TEXTURES as f32);
    let x_max = (texture_index + 1f32) / (NUM_TEXTURES as f32);
    x_min + x * (x_max - x_min)
}

pub struct GameObject {
    pub coords: (f32, f32),
}

pub struct Player {
    pub game_object: GameObject,
}

pub struct Enemy {
    pub game_object: GameObject,
}

pub struct Gate {
    pub game_object: GameObject,
    pub rotation: f32,
    pub spin_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            game_object: GameObject { coords: (0.0, 0.0) },
        }
    }
}

impl Enemy {
    pub fn new(coords: (f32, f32)) -> Self {
        Self {
            game_object: GameObject { coords },
        }
    }
}

impl Gate {
    pub fn new(coords: (f32, f32)) -> Self {
        Self {
            game_object: GameObject { coords },
            rotation: 0.0,
            spin_speed: 1.0,
        }
    }
}

impl Sprite for Player {
    fn get_vertices() -> Vec<Vertex> {
        let r = PLAYER_RADIUS;
        let i = 0f32;

        vec![
            Vertex {
                position: [-r, -r, 0.0],
                tex_coords: [t(0.0, i), 1.0],
            }, // A
            Vertex {
                position: [r, -r, 0.0],
                tex_coords: [t(1.0, i), 1.0],
                // tex_coords: [0.5, 1.0],
            }, // B
            Vertex {
                position: [r, r, 0.0],
                tex_coords: [t(1.0, i), 0.0],
                // tex_coords: [0.5, 0.0],
            }, // C
            Vertex {
                position: [-r, r, 0.0],
                tex_coords: [t(0.0, i), 0.0],
            }, // D
        ]
    }

    fn get_indices() -> &'static [u16] {
        &[0, 1, 2, 0, 2, 3]
    }

    fn get_instance(&self) -> Instance {
        Instance {
            instance_pos: [self.game_object.coords.0, self.game_object.coords.1, 0.0],
            theta: 0.0,
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
    fn get_vertices() -> Vec<Vertex> {
        let r = ENEMY_RADIUS;
        let i = 1f32;

        vec![
            Vertex {
                position: [-r, 0.0, 0.0],
                tex_coords: [t(0.0, i), 0.5],
            }, // A
            Vertex {
                position: [0.0, -r, 0.0],
                tex_coords: [t(0.5, i), 1.0],
            }, // B
            Vertex {
                position: [r, 0.0, 0.0],
                tex_coords: [t(1.0, i), 0.5],
            }, // C
            Vertex {
                position: [0.0, r, 0.0],
                tex_coords: [t(0.5, i), 0.0],
            }, // D
        ]
    }

    fn get_indices() -> &'static [u16] {
        &[0, 1, 2, 0, 2, 3]
    }

    fn get_instance(&self) -> Instance {
        Instance {
            instance_pos: [self.game_object.coords.0, self.game_object.coords.1, 0.0],
            theta: 0.0,
        }
    }
}

impl Sprite for Gate {
    /// Vertex points:    
    ///     B      
    ///         A
    ///     C    
    /// Texture coords:
    /// [0,0]    [1,0]
    ///
    /// [0,1]    [1,1]
    fn get_vertices() -> Vec<Vertex> {
        let r = GATE_RADIUS;
        let dt = 2f32 * PI / 3f32;
        let i = 2f32;

        vec![
            Vertex {
                position: [r * 0f32.cos(), r * 0f32.sin(), 0.0],
                // height of an equilateral triangle with base 1 is sqrt(3)/2
                tex_coords: [t(0.5, i), 1f32 - 3f32.sqrt() / 2f32],
            }, // A
            Vertex {
                position: [r * dt.cos(), r * dt.sin(), 0.0],
                tex_coords: [t(0.0, i), 1.0],
            }, // B
            Vertex {
                position: [r * (2f32 * dt).cos(), r * (2f32 * dt).sin(), 0.0],
                tex_coords: [t(1.0, i), 1.0],
            },
        ]
    }

    fn get_indices() -> &'static [u16] {
        &[0, 1, 2, /* pad */ 0]
    }

    fn get_instance(&self) -> Instance {
        Instance {
            instance_pos: [self.game_object.coords.0, self.game_object.coords.1, 0.0],
            theta: self.rotation,
            // theta: 0.0,
        }
    }
}
