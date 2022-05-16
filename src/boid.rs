use std::collections::HashSet;

use glam::Vec2;

use crate::Rule;

#[derive(Debug, Clone, PartialEq)]
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Boid {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Boid { pos, vel }
    }

    pub fn update(&mut self, rules: &HashSet<Rule>, all_boids: &[Boid], area: (f32, f32)) {
        let mut velocity_change = Vec2::ZERO;

        for rule in rules {
            velocity_change += rule.velocity_change(self, all_boids);
        }

        velocity_change /= 100.0;

        self.vel += velocity_change;
        self.pos += self.vel;

        self.pos.x %= area.0;
        self.pos.y %= area.1;
    }

    pub fn local_flock<'a>(&self, all_boids: &'a [Boid]) -> Vec<&'a Boid> {
        const PERCEPTION: f32 = 100.0;

        all_boids
            .iter()
            .filter(|b| *b != self && self.pos.distance(b.pos) <= PERCEPTION)
            .collect()
    }
}
