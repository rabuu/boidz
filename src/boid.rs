use std::collections::HashSet;

use glam::Vec2;

use crate::rules::Rule;

#[derive(Debug, Clone, PartialEq)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    rules: HashSet<Rule>,
}

impl Boid {
    /// Shorthand constructor
    pub fn new(position: Vec2, velocity: Vec2, rules: HashSet<Rule>) -> Self {
        Boid {
            position,
            velocity,
            rules,
        }
    }

    pub fn move_to_new_position(&mut self, other_boids: &[Boid]) {
        let mut velocity_change = Vec2::ZERO;

        for rule in &self.rules {
            velocity_change += rule.velocity_change(&self, other_boids);
        }

        self.velocity += velocity_change;
        self.position += self.velocity;
    }
}
