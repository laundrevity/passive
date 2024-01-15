use crate::constants::{
    ENEMY_BUFFER, ENEMY_SPAWN_FREQ, ENEMY_SPEED, GATE_SPAWN_FREQ, PLAYER_RADIUS, PLAYER_SPEED,
};
use crate::sprite::{Instance, Sprite, Vertex};

use rand::{thread_rng, Rng};
use std::collections::HashSet;
use winit::event::VirtualKeyCode;

pub struct Player {
    coords: (f32, f32),
}

impl Player {
    pub fn new() -> Self {
        Self { coords: (0.0, 0.0) }
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
            instance_pos: [s * self.coords.0, self.coords.1, 0.0],
        }
    }
}

struct Enemy {
    coords: (f32, f32),
}

pub struct Game {
    paused: bool,
    timer: f32,
    last_enemy_time: f32,
    last_gate_time: f32,
    pub keys: HashSet<VirtualKeyCode>,
    enemies_per_wave: u32,

    pub player: Player,
    pub enemies: Vec<Enemy>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            paused: false,
            timer: 0f32,
            last_enemy_time: 0f32,
            last_gate_time: 0f32,
            keys: HashSet::new(),
            enemies_per_wave: 1,
            player: Player::new(),
            enemies: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.paused {
            // move player
            for key in self.keys.iter() {
                match key {
                    VirtualKeyCode::W => self.player.coords.1 += PLAYER_SPEED,
                    VirtualKeyCode::A => self.player.coords.0 -= PLAYER_SPEED,
                    VirtualKeyCode::S => self.player.coords.1 -= PLAYER_SPEED,
                    VirtualKeyCode::D => self.player.coords.0 += PLAYER_SPEED,
                    _ => {}
                }
            }

            self.timer += dt;

            if self.timer > self.last_enemy_time + ENEMY_SPAWN_FREQ {
                println!("spawn enemy");
                self.spawn_enemies();
            }

            if self.timer > self.last_gate_time + GATE_SPAWN_FREQ {
                println!("spawn gate");
                self.spawn_gate();
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    fn spawn_enemies(&mut self) {
        let mut rng = thread_rng();

        let quadrant = rng.gen_range(1..5);
        let mut x_min = 0f32;
        let mut x_max = 0f32;
        let mut y_min = 0f32;
        let mut y_max = 0f32;

        match quadrant {
            1 => {
                x_min = 1f32 - 2f32 * ENEMY_BUFFER;
                x_max = 1f32;
                y_min = 1f32 - 2f32 * ENEMY_BUFFER;
                y_max = 1f32;
            }
            2 => {
                x_min = -1f32;
                x_max = -1f32 + 2f32 * ENEMY_BUFFER;
                y_min = 1f32 - 2f32 * ENEMY_BUFFER;
                y_max = 1f32;
            }
            3 => {
                x_min = -1f32;
                x_max = -1f32 + 2f32 * ENEMY_BUFFER;
                y_min = -1f32;
                y_max = -1f32 + 2f32 * ENEMY_BUFFER;
            }
            _ => {
                x_min = 1f32 - 2f32 * ENEMY_BUFFER;
                x_max = 1f32;
                y_min = -1f32;
                y_max = -1f32 + 2f32 * ENEMY_BUFFER;
            }
        }

        for _ in 0..self.enemies_per_wave {
            let x = rng.gen_range(x_min..x_max);
            let y = rng.gen_range(y_min..y_max);
            self.enemies.push(Enemy { coords: (x, y) })
        }

        self.last_enemy_time = self.timer;
    }

    fn spawn_gate(&mut self) {
        self.last_gate_time = self.timer;
    }
}
