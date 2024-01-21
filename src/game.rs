use crate::collision::check_segment_collision;
use crate::constants::{
    ENEMY_BUFFER, ENEMY_SPAWN_FREQ, ENEMY_SPEED, GATE_RADIUS, GATE_SPAWN_FREQ, PLAYER_RADIUS,
    PLAYER_SPEED,
};
use crate::game_object::{Enemy, Gate, Player};

use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::f32::consts::PI;
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
    aspect_ratio: f32,

    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub gates: Vec<Gate>,
}

impl Game {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            paused: false,
            timer: 0f32,
            last_enemy_time: 0f32,
            last_gate_time: 0f32,
            keys: HashSet::new(),
            enemies_per_wave: 1,
            aspect_ratio,

            player: Player::new(),
            enemies: Vec::new(),
            gates: Vec::new(),
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

            #[cfg(target_arch = "wasm32")]
            let sv = rescale((dx, dy), PLAYER_SPEED * 0.5);
            #[cfg(not(target_arch = "wasm32"))]
            let sv = rescale((dx, dy), PLAYER_SPEED);

            self.player.game_object.coords.0 += sv.0;
            self.player.game_object.coords.1 += sv.1;

            // nmove enemies
            for enemy in self.enemies.iter_mut() {
                let (dx, dy) = (
                    self.player.game_object.coords.0 - enemy.game_object.coords.0,
                    self.player.game_object.coords.1 - enemy.game_object.coords.1,
                );

                #[cfg(target_arch = "wasm32")]
                let sv = rescale((dx, dy), ENEMY_SPEED * 0.5);
                #[cfg(not(target_arch = "wasm32"))]
                let sv = rescale((dx, dy), ENEMY_SPEED);
                enemy.game_object.coords.0 += sv.0;
                enemy.game_object.coords.1 += sv.1;
            }

            // move gates
            for gate in self.gates.iter_mut() {
                gate.rotation += dt * gate.spin_speed;

                // println!("dt={}, gate.spin_speed={}", dt, gate.spin_speed);
                // println!("gate rot: {}", gate.rotation);
            }

            self.check_collisions();

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

    fn check_collisions(&mut self) {
        // check player-gate edge collision
        let (px, py) = self.player.game_object.coords;

        for gate in &self.gates {
            // cannot use get_vertices method as that ignores rotation
            // we use rotation for rendering via instances
            let (gx, gy) = gate.game_object.coords;

            // TODO: debug why this is necessary
            // if we remove this, then we get lots of false positives from distant gates with
            // vertices with very similar x coords, e.g.
            // x1=0.19950452 is between x_min=0.19949819 and x_max=0.1995106
            // player coords: (-0.15033174, 0.33800787)
            // PLAYER_RADIUS: 0.05
            // p=(0.19949819, 0.514558), q=(0.1995106, 0.16814788)
            let (dx, dy) = (gx - px, gy - py);
            let skip_check = true;
            if dx * dx + dy * dy <= GATE_RADIUS * GATE_RADIUS || skip_check {
                let theta = gate.rotation;
                let s = 1f32 / self.aspect_ratio;

                let v1 = (
                    s * (gx + GATE_RADIUS * theta.cos()),
                    gy + GATE_RADIUS * theta.sin(),
                );
                let v2 = (
                    s * (gx + GATE_RADIUS * (theta + 2f32 * PI / 3f32).cos()),
                    gy + GATE_RADIUS * (theta + 2f32 * PI / 3f32).sin(),
                );
                let v3 = (
                    s * (gx + GATE_RADIUS * (theta + 4f32 * PI / 3f32).cos()),
                    gy + GATE_RADIUS * (theta + 4f32 * PI / 3f32).sin(),
                );

                let pairs = vec![(v1, v2), (v2, v3), (v3, v1)];

                for (p, q) in pairs {
                    if check_segment_collision(&(s * px, py), &p, &q, PLAYER_RADIUS) {
                        println!("EXPLODE!");
                        self.paused = !self.paused;

                        println!("player coords: ({}, {})", px, py);
                        println!("PLAYER_RADIUS: {}", PLAYER_RADIUS);
                        println!("p={:?}, q={:?}", p, q);
                        println!("gate vertices: {:?}, {:?}, {:?}", v1, v2, v3);
                        println!("gate rotation: {}", gate.rotation);
                        println!("gate center: {:?}", gate.game_object.coords);
                        return;
                    }
                }
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    fn spawn_enemies(&mut self) {
        let mut rng = thread_rng();

        let quadrant = rng.gen_range(1..5);
        let x_min;
        let x_max;
        let y_min;
        let y_max;

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
        let mut rng = thread_rng();

        let x = rng.gen_range(0.0..1.0);
        let y = rng.gen_range(0.0..1.0);

        self.gates.push(Gate::new((x, y)));

        self.last_gate_time = self.timer;
    }
}
