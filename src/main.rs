use std::collections::HashSet;

use ggez::{Context, GameResult};
use glam::*;
use rand::Rng;

use boidz::{Boid, Rule};

const WIDTH: usize = 1000;
const HEIGHT: usize = 500;

const DESIRED_FPS: u32 = 8;

struct App {
    boids: Vec<Boid>,
}

impl App {
    fn new() -> GameResult<App> {
        const NUM_BOIDS: usize = 100;

        let mut rules = HashSet::new();
        rules.insert(Rule::Cohesion);
        rules.insert(Rule::Separation);
        rules.insert(Rule::Alignment);

        let mut boids = Vec::with_capacity(NUM_BOIDS);
        let mut rng = rand::thread_rng();

        for _ in 0..NUM_BOIDS {
            let b = Boid {
                position: Vec2::new(
                    rng.gen_range(0..WIDTH) as f32,
                    rng.gen_range(0..HEIGHT) as f32,
                ),
                velocity: Vec2::ZERO,
                rules: rules.clone(),
            };

            boids.push(b);
        }

        Ok(App { boids })
    }
}

impl ggez::event::EventHandler<ggez::GameError> for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            let all_boids = self.boids.clone();
            for boid in self.boids.iter_mut() {
                boid.move_to_new_position(&all_boids)
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = ggez::graphics::Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            5.0,
            2.0,
            ggez::graphics::Color::WHITE,
        )?;

        for boid in &self.boids {
            ggez::graphics::draw(ctx, &circle, (boid.position,))?;
        }

        ggez::graphics::present(ctx)?;
        ggez::timer::yield_now();

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("boidz", "rabuu")
        .window_setup(ggez::conf::WindowSetup::default().title("boidz"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH as f32, HEIGHT as f32));

    let (ctx, event_loop) = cb.build()?;
    let app = App::new()?;

    ggez::event::run(ctx, event_loop, app)
}
