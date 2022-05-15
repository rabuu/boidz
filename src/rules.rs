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
    pub fn velocity_change(self, boid: &Boid, other_boids: &[Boid]) -> Vec2 {
        use Rule::*;

        match self {
            Cohesion => todo!(),
            Separation => todo!(),
            Alignment => todo!(),
        }
    }
}
