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
    let local_flock = boid.local_flock(all_boids);
    let mut perceived_centre = Vec2::ZERO;

    if local_flock.is_empty() {
        return perceived_centre;
    }

    for b in &local_flock {
        perceived_centre += b.pos;
    }

    perceived_centre /= local_flock.len() as f32;

    (perceived_centre - boid.pos) / 100.
}

fn separation(boid: &Boid, all_boids: &[Boid]) -> Vec2 {
    let local_flock = boid.local_flock(all_boids);
    let mut c = Vec2::ZERO;

    if local_flock.is_empty() {
        return c;
    }

    for b in &local_flock {
        if b.pos.distance(boid.pos) < 20. {
            c -= b.pos - boid.pos;
        }
    }

    c / 15.
}

fn alignment(boid: &Boid, all_boids: &[Boid]) -> Vec2 {
    let local_flock = boid.local_flock(all_boids);
    let mut perceived_velocity = Vec2::ZERO;

    if local_flock.is_empty() {
        return perceived_velocity;
    }

    for b in &local_flock {
        perceived_velocity += b.vel;
    }

    perceived_velocity /= local_flock.len() as f32;

    (perceived_velocity - boid.vel) / 5.
}
