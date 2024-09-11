use eframe::CreationContext;
use egui::{Color32, Painter, Pos2};
use rand::prelude::*;
use std::time::SystemTime;

use tracing::*;

//pub mod helpers;

const MAX_PARTICLES: usize = 1000;

//const DEFAULT_PARTICLES: usize = 100;

const ENABLE_WRAP: bool = false;

pub struct MyApp {
    sim: ParticleSim,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            sim: ParticleSim::new(),
        }
    }
}

impl MyApp {
    pub fn new(_: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ctx.available_rect();

            self.sim.width = available.max.x - available.min.x;
            self.sim.height = available.max.y - available.min.y;

            let painter = ui.painter();

            self.sim.draw(painter);

            ctx.request_repaint();

            let mut particle_count: usize = self.sim.particle_count();

            ui.add(
                egui::Slider::new(&mut particle_count, 0..=MAX_PARTICLES)
                    .text("Number of particles"),
            );
            ui.add(egui::Slider::new(&mut self.sim.sim_speed, 0.0..=10.0).text("Simulation Speed"));
            ui.add(egui::Slider::new(&mut self.sim.g, 0.0..=100.0).text("G"));

            self.sim.set_particle_count(particle_count);
            self.sim.step();
        });
    }
}

// struct Pos<T, const N: usize> {
//     xy: [T; N]
// }
//
// type Pos2 = Pos<u32, 2>;
//

pub struct ParticleType {
    pub color: Color32,
}

pub struct ParticleSim {
    particles: Vec<Particle>,
    particle_types: Vec<ParticleType>,
    pub width: f32,
    pub height: f32,
    pub sim_speed: f32,
    pub g: f32,
    #[cfg(not(target_arch = "wasm32"))]
    last_step: SystemTime,
    #[cfg(target_arch = "wasm32")]
    last_step: wasm_timer::SystemTime,
}
//use std::fmt::{Debug, Write};

impl Debug for ParticleSim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        write!(f, "size_x: {}", self.width)?;
        write!(f, ", size_y: {}", self.height)?;
        write!(f, ", sim_speed: {}", self.sim_speed)?;
        write!(f, "}}")
    }
}

impl Default for ParticleSim {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
fn current_time() -> wasm_timer::SystemTime {
    wasm_timer::SystemTime::now()
}

#[cfg(not(target_arch = "wasm32"))]
fn current_time() -> SystemTime {
    std::time::SystemTime::now()
}

impl ParticleSim {
    pub fn new() -> ParticleSim {
        ParticleSim {
            particles: Vec::new(),
            particle_types: vec![
                ParticleType {
                    color: Color32::RED,
                },
                ParticleType {
                    color: Color32::BLUE,
                },
            ],
            width: 1.0,
            height: 1.0,
            sim_speed: 1.0,
            g: 10.0,
            last_step: current_time(),
        }
    }

    pub fn set_particle_count(&mut self, num_particles: usize) {
        if num_particles == self.particles.len() {
            return;
        }

        info!("setting number of particles to {num_particles}");

        if num_particles > self.particles.len() {
            let mut rng = rand::thread_rng();
            for _ in 0..(num_particles - self.particles.len()) {
                self.particles.push(Particle {
                    particle_type: rng.gen::<u8>() % self.particle_types.len() as u8,
                    x: rng.gen::<f32>() * self.width,
                    y: rng.gen::<f32>() * self.height,
                    // velocity_x: rng.gen::<f32>() % 5.0,
                    // velocity_y: rng.gen::<f32>() % 5.0,
                    velocity_x: 0.0,
                    velocity_y: 0.0,
                });
            }
        } else {
            self.particles.truncate(num_particles);
        }
    }

    pub fn with_particles(num_particles: usize) -> ParticleSim {
        let particles = Vec::with_capacity(num_particles);
        let mut sim = ParticleSim::new();
        sim.particles = particles;
        sim.set_particle_count(num_particles);
        sim
    }

    pub fn draw(&self, painter: &Painter) {
        for particle in self.particles.iter() {
            painter.circle_filled(
                Pos2::new(particle.x, particle.y),
                3.0,
                self.particle_types[particle.particle_type as usize].color,
            );
        }
    }

    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    pub fn step(&mut self) {
        let now = current_time();
        let last = self.last_step;

        let delta = now
            .duration_since(last)
            .expect("failed to get elapsed time")
            .as_micros();
        if delta == 0 {
            return;
        }
        let delta = delta as f32 / 1_000_000.0;

        self.last_step = now;

        /*
        const DELTA_SCALE: u32 = 1_000;
        let delta_scale = |n: i32| {
                (
                    ((n * self.sim_speed as i32) * (DELTA_SCALE as i32) ) / (delta as i32)
                ) / (DELTA_SCALE as i32)
        };*/

        let scale = |n: f32| n * self.sim_speed;

        for i in 0..self.particles.len() {
            let mut particle = self.particles[i];
            let Particle {
                x,
                y,
                velocity_x,
                velocity_y,
                ..
            } = &mut particle;

            let mut fx = 0.0;
            let mut fy = 0.0;

            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }
                let particle2 = &self.particles[j];
                let (x2, y2) = (particle2.x, particle2.y);

                let (r12_x, r12_y);

                if ENABLE_WRAP {
                    let (mut cx, mut cy) = (*x - self.width, *y - self.height);
                    if *x < self.width / 2.0 {
                        cx = *x + self.width;
                    }
                    if *y < self.height / 2.0 {
                        cy = *y + self.height;
                    }

                    r12_x = std::cmp::min_by(*x - x2, cx - x2, |a, b| a.partial_cmp(b).unwrap());
                    r12_y = std::cmp::min_by(*y - y2, cy - y2, |a, b| a.partial_cmp(b).unwrap());
                } else {
                    r12_x = *x - x2;
                    r12_y = *y - y2;
                }

                let r12_magnitude = ((r12_x * r12_x) + (r12_y * r12_y)).sqrt();
                let c = -self.g / (r12_magnitude * r12_magnitude * r12_magnitude);

                if r12_magnitude > 5.0 {
                    fx += c * r12_x;
                    fy += c * r12_y;
                }
            }

            *velocity_x += fx * delta;
            *velocity_y += fy * delta;
            if ENABLE_WRAP {
                *x = (*x + scale(*velocity_x)).rem_euclid(self.width);
                *y = (*y + scale(*velocity_y)).rem_euclid(self.height);
            } else {
                *x += scale(*velocity_x);
                *y += scale(*velocity_y);
            }

            self.particles[i] = particle;
        }
    }
}

// fn force() -> {
//
// }
//

// All values are 1000x the actual v
// 1000x
#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub particle_type: u8,
    // units
    pub x: f32,
    pub y: f32,
    // units per second
    pub velocity_x: f32,
    pub velocity_y: f32,
}
