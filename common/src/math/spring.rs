#![allow(dead_code)]
use crate::{Vector2D, Vector3D};

/// To get a critically damped spring (one which doesn't "overshoot" the desired
/// 'center' position), use a damping coefficient equal to:
///
///  2.0 * sqrt( mass * stiffness )
///
/// In our case, all of these spring classes are treated as having a mass of 1,
/// so you can ignore that factor.
///
/// Damping values lower than this will yield a spring which oscillates, and
/// values higher than this will yield a very sluggish spring.
///
/// Note that as all these springs are implemented using euler integrations,
/// large differences between 'center' and 'position' values may make them
/// behave eratically when timesteps are large.


#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct Spring {
	stiffness: f32,
	damping: f32,

	pub center: f32,
	pub position: f32,
	pub velocity: f32,
}

impl Spring {
    fn new ( stiffness: f32, damping_factor: f32 ) -> Self {
        Self { stiffness, damping: damping_factor, ..Default::default() }
    }

    fn new_damped ( stiffness: f32 ) -> Self {
        Self { stiffness, damping: 2.0 * stiffness.sqrt(), ..Default::default() }
    }

    pub fn update ( &mut self, timestep: f32 ) -> f32 {
        let delta = self.center - self.position;

        self.velocity *= self.damping * timestep;
        self.velocity += delta * self.stiffness * timestep;

        self.position += self.velocity * timestep;

        self.position
    }
}


#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct Spring2D {
	stiffness: Vector2D,
	damping: f32,
	// damping: VSvector2D,

	pub center: Vector2D,
	pub position: Vector2D,
	pub velocity: Vector2D,
}

impl Spring2D {
    fn new ( stiffness: Vector2D, damping_factor: f32 ) -> Self {
        Self { stiffness, damping: damping_factor, ..Default::default() }
        // Self { stiffness, damping: VSvector2D{ x: damping_factor, y: damping_factor}, ..Default::default() }
    }

    pub fn update ( &mut self, timestep: f32 ) -> &Vector2D {
        let delta = self.center - self.position;

        self.velocity *= self.damping * timestep;
        // self.velocity.x *= self.damping * timestep;
        // self.velocity.y *= self.damping * timestep;

        self.velocity += delta * self.stiffness * timestep;
        // self.velocity.x += delta.x * self.stiffness.x * timestep;
        // self.velocity.y += delta.y * self.stiffness.y * timestep;

        self.position += self.velocity * timestep;

        &self.position
    }
}


#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct Spring3D {
	stiffness: Vector3D,
	damping: f32,
	// damping: VSvector3D,

	center: Vector3D,
	position: Vector3D,
	velocity: Vector3D,
}

impl Spring3D {
    fn new ( stiffness: Vector3D, damping_factor: f32 ) -> Self {
        Self { stiffness, damping: damping_factor, ..Default::default() }
        // Self { stiffness, damping: VSvector3D{ x: damping_factor, y: damping_factor, z: damping_factor}, ..Default::default() }
    }

    pub fn update ( &mut self, timestep: f32 ) -> &Vector3D {
        let delta = self.center - self.position;

        self.velocity *= self.damping * timestep;
        // self.velocity.x *= self.damping * timestep;
        // self.velocity.y *= self.damping * timestep;
        // self.velocity.z *= self.damping * timestep;

        self.velocity += delta * self.stiffness * timestep;
        // self.velocity.x += delta.x * self.stiffness.x * timestep;
        // self.velocity.y += delta.y * self.stiffness.y * timestep;
        // self.velocity.z += delta.z * self.stiffness.z * timestep;

        self.position += self.velocity * timestep;

        &self.position
    }
}
