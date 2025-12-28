use crate::color::*;
use crate::math::vector::*;

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct Spline1D {
    pub start: f32,
    pub start_vel: f32,
    pub end: f32,
    pub end_vel: f32,
}

impl Spline1D {
    // fn new() -> Self {
    //     Self { start: 0.0, start_vel: 0.0, end: 0.0, end_vel: 0.0 }
    // }

    pub fn position_at_time(&self, t: f32) -> f32 {
        assert!(
            (0. ..=1.).contains(&t),
            "! requesting postion for time outside range 0-1"
        );
        let t_squared = t * t;
        let t_cubed = t * t * t;
        let a = (2. * t_cubed) - (3. * t_squared) + 1.;
        let b = t_cubed - (2. * t_squared) + t;
        let c = (-2. * t_cubed) + (3. * t_squared);
        let d = t_cubed - t_squared;
        (a * self.start) + (b * self.start_vel) + (c * self.end) + (d * self.end_vel)
    }

    pub fn velocity_at_time(&self, t: f32) -> f32 {
        assert!(
            (0. ..=1.).contains(&t),
            "! requesting velocity for time outside range 0-1"
        );
        let t_squared = t * t;
        let a = (6. * t_squared) - (6. * t);
        let b = (3. * t_squared) - (4. * t) + 1.;
        let c = (-6. * t_squared) + (6. * t);
        let d = (3. * t_squared) - (2. * t);
        (a * self.start) + (b * self.start_vel) + (c * self.end) + (d * self.end_vel)
    }

    pub fn get_slice(&self, t1: f32, t2: f32) -> Self {
        let new_duration = t2 - t1;
        let velocity_conversion = new_duration;
        Self {
            start: self.position_at_time(t1),
            start_vel: self.velocity_at_time(t1) * velocity_conversion,
            end: self.position_at_time(t2),
            end_vel: self.velocity_at_time(t2) * velocity_conversion,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Default)] // should this be copy?
pub struct Spline2D {
    pub start: Vector2D,
    pub start_vel: Vector2D,
    pub end: Vector2D,
    pub end_vel: Vector2D,
}

impl Spline2D {
    // fn new() -> Self {
    //     Self { start: VSvector3D::ZERO, start_vel: VSvector3D::ZERO, end: VSvector3D::ZERO, end_vel: VSvector3D::ZERO }
    // }

    pub fn position_at_time(&self, t: f32) -> Vector2D {
        assert!(
            (0. ..=1.).contains(&t),
            "! requesting postion for time outside range 0-1"
        );
        let t_squared = t * t;
        let t_cubed = t * t * t;
        let a = (2. * t_cubed) - (3. * t_squared) + 1.;
        let b = t_cubed - (2. * t_squared) + t;
        let c = (-2. * t_cubed) + (3. * t_squared);
        let d = t_cubed - t_squared;
        (a * self.start) + (b * self.start_vel) + (c * self.end) + (d * self.end_vel)
    }

    pub fn velocity_at_time(&self, t: f32) -> Vector2D {
        assert!(
            (0. ..=1.).contains(&t),
            "! requesting velocity for time outside range 0-1"
        );
        let t_squared = t * t;
        let a = (6. * t_squared) - (6. * t);
        let b = (3. * t_squared) - (4. * t) + 1.;
        let c = (-6. * t_squared) + (6. * t);
        let d = (3. * t_squared) - (2. * t);
        (a * self.start) + (b * self.start_vel) + (c * self.end) + (d * self.end_vel)
    }

    pub fn get_slice(&self, t1: f32, t2: f32) -> Self {
        let new_duration = t2 - t1;
        let velocity_conversion = new_duration;
        Self {
            start: self.position_at_time(t1),
            start_vel: self.velocity_at_time(t1) * velocity_conversion,
            end: self.position_at_time(t2),
            end_vel: self.velocity_at_time(t2) * velocity_conversion,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Default)] // should this be copy?
pub struct Spline3D {
    pub start: Vector3D,
    pub start_vel: Vector3D,
    pub end: Vector3D,
    pub end_vel: Vector3D,
}

impl Spline3D {
    // fn new() -> Self {
    //     Self { start: VSvector3D::ZERO, start_vel: VSvector3D::ZERO, end: VSvector3D::ZERO, end_vel: VSvector3D::ZERO }
    // }

    pub fn position_at_time(&self, t: f32) -> Vector3D {
        assert!(
            (0. ..=1.).contains(&t),
            "! requesting postion for time outside range 0-1"
        );
        let t_squared = t * t;
        let t_cubed = t * t * t;
        let a = (2. * t_cubed) - (3. * t_squared) + 1.;
        let b = t_cubed - (2. * t_squared) + t;
        let c = (-2. * t_cubed) + (3. * t_squared);
        let d = t_cubed - t_squared;
        (a * self.start) + (b * self.start_vel) + (c * self.end) + (d * self.end_vel)
    }

    pub fn velocity_at_time(&self, t: f32) -> Vector3D {
        assert!(
            (0. ..=1.).contains(&t),
            "! requesting velocity for time outside range 0-1"
        );
        let t_squared = t * t;
        let a = (6. * t_squared) - (6. * t);
        let b = (3. * t_squared) - (4. * t) + 1.;
        let c = (-6. * t_squared) + (6. * t);
        let d = (3. * t_squared) - (2. * t);
        (a * self.start) + (b * self.start_vel) + (c * self.end) + (d * self.end_vel)
    }

    pub fn get_slice(&self, t1: f32, t2: f32) -> Self {
        let new_duration = t2 - t1;
        let velocity_conversion = new_duration;
        Self {
            start: self.position_at_time(t1),
            start_vel: self.velocity_at_time(t1) * velocity_conversion,
            end: self.position_at_time(t2),
            end_vel: self.velocity_at_time(t2) * velocity_conversion,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Default)] // should this be copy?
pub struct ColorSpline {
    pub start: VScolor,
    pub start_vel: VScolor,
    pub end: VScolor,
    pub end_vel: VScolor,
}

impl ColorSpline {
    // fn new() -> Self {
    //     Self { start: VSvector3D::ZERO, start_vel: VSvector3D::ZERO, end: VSvector3D::ZERO, end_vel: VSvector3D::ZERO }
    // }

    pub fn new(start: VScolor, middle: VScolor, end: VScolor) -> Self {
        Self {
            start,
            start_vel: middle - start,
            end,
            end_vel: end - middle,
        }
    }

    pub fn middle(&self) -> VScolor {
        self.start + self.start_vel
    }

    pub fn color_at_time(&self, t: f32) -> VScolor {
        assert!(
            (0. ..=1.).contains(&t),
            "! requesting postion for time outside range 0-1"
        );
        let t_squared = t * t;
        let t_cubed = t * t * t;
        let a = (2. * t_cubed) - (3. * t_squared) + 1.;
        let b = t_cubed - (2. * t_squared) + t;
        let c = (-2. * t_cubed) + (3. * t_squared);
        let d = t_cubed - t_squared;
        (self.start * a) + (self.start_vel * b) + (self.end * c) + (self.end_vel * d)
    }

    pub fn velocity_at_time(&self, t: f32) -> VScolor {
        assert!(
            (0. ..=1.).contains(&t),
            "! requesting velocity for time outside range 0-1"
        );
        let t_squared = t * t;
        let a = (6. * t_squared) - (6. * t);
        let b = (3. * t_squared) - (4. * t) + 1.;
        let c = (-6. * t_squared) + (6. * t);
        let d = (3. * t_squared) - (2. * t);
        (self.start * a) + (self.start_vel * b) + (self.end * c) + (self.end_vel * d)
    }

    pub fn get_slice(&self, t1: f32, t2: f32) -> Self {
        let new_duration = t2 - t1;
        let velocity_conversion = new_duration;
        Self {
            start: self.color_at_time(t1),
            start_vel: self.velocity_at_time(t1) * velocity_conversion,
            end: self.color_at_time(t2),
            end_vel: self.velocity_at_time(t2) * velocity_conversion,
        }
    }
}
