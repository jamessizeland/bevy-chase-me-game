use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::game::movement::Momentum;

use super::{
    systems::{calc_strength, randomize},
    Enemy,
};

/// The max power of an enemy
#[derive(Resource)]
pub struct EnemyStrengthRange {
    max_speed: (f32, f32),
    max_energy: (f32, f32),
    recharge_rate: (f32, f32),
    mass: (f32, f32),
    thrust: (f32, f32),
    lifetime: (f32, f32),
    radius: (f32, f32),
}

impl Default for EnemyStrengthRange {
    fn default() -> Self {
        EnemyStrengthRange {
            max_speed: (5.0, 12.0),
            max_energy: (50.0, 120.0),
            recharge_rate: (10.0, 60.0),
            mass: (1.0, 10.0),
            thrust: (10.0, 20.0),
            lifetime: (30.0, 120.0),
            radius: (5.0, 15.0),
        }
    }
}

impl EnemyStrengthRange {
    /// Get the max power of the enemy
    pub fn get_power_range(&self) -> (f32, f32) {
        let min = calc_strength(
            &Momentum::new(self.max_speed.0, self.mass.0, self.thrust.0),
            &Enemy::new(self.lifetime.0, self.max_energy.0, self.recharge_rate.0),
        );
        let max = calc_strength(
            &Momentum::new(self.max_speed.1, self.mass.1, self.thrust.1),
            &Enemy::new(self.lifetime.1, self.max_energy.1, self.recharge_rate.1),
        );
        (min, max)
    }
    /// Generate stats for a new enemy, based on the max power of the enemy and a bias that reduces each stat by a percentage
    pub fn get_enemy_stats(&self, bias: f32) -> (Momentum, Enemy) {
        let momentum = Momentum::new(
            randomize(self.max_speed, bias),
            randomize(self.mass, bias),
            randomize(self.thrust, bias),
        );
        let enemy = Enemy::new(
            randomize(self.lifetime, bias),
            randomize(self.max_energy, bias),
            randomize(self.recharge_rate, bias),
        );
        (momentum, enemy)
    }
    /// Generate the radius of the new enemy based on the max radius of the enemy, max mass and its current mass
    pub fn get_radius(&self, mass: f32) -> f32 {
        randomize(self.radius, mass / self.mass.1).clamp(self.radius.0, self.radius.1)
    }
    /// Get the colour of the enemy based on its momentum and energy
    pub fn get_colour(&self, momentum: &Momentum, enemy: &Enemy) -> Color {
        let difficulty = calc_strength(momentum, enemy);
        let (min_difficulty, max_difficulty) = self.get_power_range();
        let relative_difficulty = (difficulty - min_difficulty) / (max_difficulty - min_difficulty);
        // have 10 difficulty colours
        if relative_difficulty < 0.1 {
            Color::RED
        } else if relative_difficulty < 0.2 {
            Color::rgb(1.0, 0.5, 0.0)
        } else if relative_difficulty < 0.3 {
            Color::rgb(1.0, 1.0, 0.0)
        } else if relative_difficulty < 0.4 {
            Color::rgb(0.5, 1.0, 0.0)
        } else if relative_difficulty < 0.5 {
            Color::rgb(0.0, 1.0, 0.0)
        } else if relative_difficulty < 0.6 {
            Color::rgb(0.0, 1.0, 0.5)
        } else if relative_difficulty < 0.7 {
            Color::rgb(0.0, 1.0, 1.0)
        } else if relative_difficulty < 0.8 {
            Color::rgb(0.0, 0.5, 1.0)
        } else if relative_difficulty < 0.9 {
            Color::rgb(0.0, 0.0, 1.0)
        } else {
            Color::BLUE
        }
    }
    /// Get the shape of the enemy based on its momentum and energy
    pub fn get_shape(&self, momentum: &Momentum, enemy: &Enemy) -> Path {
        let difficulty = calc_strength(momentum, enemy);
        let (min_difficulty, max_difficulty) = self.get_power_range();
        let _relative_difficulty =
            (difficulty - min_difficulty) / (max_difficulty - min_difficulty);
        let radius = self.get_radius(momentum.mass);
        // have 4 different shapes based on the difficulty level
        // a circle, a triangle, a square and a pentagon
        let points = vec![
            Vec2::new(0.0, radius),                   // tip of the beak
            Vec2::new(-(radius * 2.0), radius * 2.0), // start of left wing
            Vec2::new(0.0, -radius),                  // tip of the tail
            Vec2::new(radius * 2.0, radius * 2.0),    // start of right wing
        ];
        let triangle = shapes::Polygon {
            points,
            closed: true,
        };
        // GeometryBuilder::build_as(&shapes::Circle {
        //     radius,
        //     center: Vec2::ZERO,
        // })
        GeometryBuilder::build_as(&triangle)
    }
}
