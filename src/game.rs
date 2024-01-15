use crate::constants::{
    ENEMY_BUFFER, ENEMY_SPAWN_FREQ, ENEMY_SPEED, GATE_SPAWN_FREQ, PLAYER_SPEED,
};
use crate::game_object::{Enemy, Player};

use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::f32::EPSILON;
use winit::event::VirtualKeyCode;

fn rescale(v: (f32, f32), speed: f32) -> (f32, f32) {
    let d = (v.0 * v.0 + v.1 * v.1).sqrt();
    (v.0 * speed / (d + EPSILON), v.1 * speed / (d + EPSILON))
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
            let mut dx = 0f32;
            let mut dy = 0f32;

            for key in self.keys.iter() {
                match key {
                    VirtualKeyCode::W => {
                        dy += 1.0;
                    }
                    VirtualKeyCode::A => {
                        dx -= 1.0;
                    }
                    VirtualKeyCode::S => {
                        dy -= 1.0;
                    }
                    VirtualKeyCode::D => {
                        dx += 1.0;
                    }
                    _ => {}
                }
            }

            let sv = rescale((dx, dy), PLAYER_SPEED);
            self.player.game_object.coords.0 += sv.0;
            self.player.game_object.coords.1 += sv.1;

            for enemy in self.enemies.iter_mut() {
                let (dx, dy) = (
                    self.player.game_object.coords.0 - enemy.game_object.coords.0,
                    self.player.game_object.coords.1 - enemy.game_object.coords.1,
                );

                let sv = rescale((dx, dy), ENEMY_SPEED);
                enemy.game_object.coords.0 += sv.0;
                enemy.game_object.coords.1 += sv.1;
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
            self.enemies.push(Enemy::new((x, y)));
        }

        self.enemies_per_wave += 1;
        self.last_enemy_time = self.timer;
    }

    fn spawn_gate(&mut self) {
        self.last_gate_time = self.timer;
    }
}
