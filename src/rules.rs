use glam::Vec2;

use crate::boid::Boid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rule {
    // Boids try to fly towards the centre of mass of neighboring boids
    Cohesion,

    // Boids try to keep a small distance away from other objects/boids
    Separation,

    // Boids try to match velocity with near boids
    Alignment,
}

impl Rule {
    pub fn velocity_change(self, boid: &Boid, all_boids: &[Boid]) -> Vec2 {
        use Rule::*;

        match self {
            Cohesion => cohesion(boid, all_boids),
            Separation => separation(boid, all_boids),
            Alignment => alignment(boid, all_boids),
        }
    }
}

fn cohesion(boid: &Boid, all_boids: &[Boid]) -> Vec2 {
    let mut perceived_centre = Vec2::ZERO;

    for b in all_boids.iter().filter(|b| *b != boid) {
        perceived_centre += b.position;
    }

    perceived_centre /= (all_boids.len() - 1) as f32;

    (perceived_centre - boid.position) / 100.0
}

fn separation(boid: &Boid, all_boids: &[Boid]) -> Vec2 {
    let mut c = Vec2::ZERO;

    for b in all_boids.iter().filter(|b| *b != boid) {
        if (b.position - boid.position).length() < 100.0 {
            c -= b.position - boid.position;
        }
    }

    c
}

fn alignment(boid: &Boid, all_boids: &[Boid]) -> Vec2 {
    let mut perceived_velocity = Vec2::ZERO;

    for b in all_boids.iter().filter(|b| *b != boid) {
        perceived_velocity += b.velocity;
    }

    perceived_velocity /= (all_boids.len() - 1) as f32;

    (perceived_velocity - boid.velocity) / 8.0
}
