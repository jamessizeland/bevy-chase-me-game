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
            max_energy: (30.0, 120.0),
            recharge_rate: (10.0, 60.0),
            mass: (1.0, 10.0),
            thrust: (10.0, 20.0),
            lifetime: (30.0, 120.0),
            // lifetime: (5.0, 10.0), // reduce lifetime for testing purposes (to make it easier to test the game
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
        let relative_difficulty =
            (difficulty - min_difficulty) / ((max_difficulty * 2.0) - min_difficulty);
        // have 10 difficulty colours
        if relative_difficulty < 0.1 {
            Color::srgb(255.0, 0.0, 0.0)
        } else if relative_difficulty < 0.2 {
            Color::srgb(255.0, 112.5, 0.0)
        } else if relative_difficulty < 0.3 {
            Color::srgb(255.0, 255.0, 0.0)
        } else if relative_difficulty < 0.4 {
            Color::srgb(112.5, 255.0, 0.0)
        } else if relative_difficulty < 0.5 {
            Color::srgb(0.0, 255.0, 0.0)
        } else if relative_difficulty < 0.6 {
            Color::srgb(0.0, 255.0, 112.5)
        } else if relative_difficulty < 0.7 {
            Color::srgb(0.0, 255.0, 255.0)
        } else if relative_difficulty < 0.8 {
            Color::srgb(0.0, 112.5, 255.0)
        } else if relative_difficulty < 0.9 {
            Color::srgb(0.0, 0.0, 255.0)
        } else {
            Color::srgb(255.0, 255.0, 255.0)
        }
    }
    /// Get the shape of the enemy based on its momentum and energy
    pub fn get_shape(&self, momentum: &Momentum, enemy: &Enemy) -> Path {
        let difficulty = calc_strength(momentum, enemy);
        let (min_difficulty, max_difficulty) = self.get_power_range();
        let relative_difficulty =
            (difficulty - min_difficulty) / ((max_difficulty * 2.0) - min_difficulty);
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
        // Five-pointed star
        let num_points = 5;
        let angle_step = std::f32::consts::PI * 2.0 / num_points as f32;
        let mut star_points = Vec::with_capacity(num_points * 2);

        for i in 0..num_points * 2 {
            let radius_factor = if i % 2 == 0 { 2.0 } else { 1.0 };
            let angle = angle_step * (i as f32 / 2.0);
            let x = radius * radius_factor * angle.cos();
            let y = radius * radius_factor * angle.sin();
            star_points.push(Vec2::new(x, y));
        }
        let star = shapes::Polygon {
            points: star_points,
            closed: true,
        };

        // Crescent moon
        let moon_points = vec![
            Vec2::new(0.0, radius),                 // Top point
            Vec2::new(radius * 0.5, radius * 0.5),  // Top-right point
            Vec2::new(radius * 0.5, -radius * 0.5), // Bottom-right point
            Vec2::new(0.0, -radius),                // Bottom point
        ];
        let _moon = shapes::Polygon {
            points: moon_points,
            closed: true,
        };

        // U shape
        let u_points = vec![
            Vec2::new(-radius, radius),  // Top-left point
            Vec2::new(radius, radius),   // Top-right point
            Vec2::new(radius, -radius),  // Bottom-right point
            Vec2::new(-radius, -radius), // Bottom-left point
        ];
        let _u_shape = shapes::Polygon {
            points: u_points,
            closed: false,
        };

        if relative_difficulty < 0.5 {
            GeometryBuilder::build_as(&star)
        } else {
            GeometryBuilder::build_as(&triangle)
        }
    }
}
