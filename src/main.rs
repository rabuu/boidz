use glam::Vec2;

const WIDTH: f32 = 1080.0;
const HEIGHT: f32 = 900.0;
const DESIRED_FPS: u32 = 60;

struct App {
    simulation: boidz::Simulation,
}

impl App {
    fn new() -> ggez::GameResult<App> {
        let simulation = boidz::Simulation::random_boids(100, (WIDTH, HEIGHT));

        Ok(App { simulation })
    }
}

impl ggez::event::EventHandler<ggez::GameError> for App {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            self.simulation.update((WIDTH, HEIGHT));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = ggez::graphics::Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            5.0,
            2.0,
            ggez::graphics::Color::WHITE,
        )?;

        for boid in &self.simulation.boids {
            ggez::graphics::draw(ctx, &circle, (boid.pos,))?;
        }

        ggez::graphics::present(ctx)?;
        ggez::timer::yield_now();

        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("boidz", "rabuu")
        .window_setup(ggez::conf::WindowSetup::default().title("boidz"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH, HEIGHT));

    let (ctx, event_loop) = cb.build()?;
    let app = App::new()?;

    ggez::event::run(ctx, event_loop, app)
}
