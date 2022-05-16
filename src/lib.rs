mod boid;
mod rules;

use std::collections::HashSet;

pub use boid::Boid;
use glam::Vec2;
pub use rules::Rule;

#[derive(Debug, Clone)]
pub struct Simulation {
    pub boids: Vec<Boid>,
    pub rules: HashSet<Rule>,
}

impl Simulation {
    pub fn random_boids(n: usize, area: (f32, f32)) -> Self {
        let mut boids = Vec::with_capacity(n);

        for _ in 0..n {
            let pos = Vec2::new(
                rand::random::<f32>() * area.0,
                rand::random::<f32>() * area.1,
            );

            let boid = Boid::new(pos, Vec2::ONE);
            boids.push(boid);
        }

        let mut default_rules = HashSet::with_capacity(3);
        default_rules.insert(Rule::Cohesion);
        default_rules.insert(Rule::Separation);
        default_rules.insert(Rule::Alignment);

        Simulation {
            boids,
            rules: default_rules,
        }
    }

    pub fn update(&mut self, area: (f32, f32)) {
        let all_boids = self.boids.clone();
        for boid in self.boids.iter_mut() {
            boid.update(&self.rules, &all_boids, area);
        }
    }
}
