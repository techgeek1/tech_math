use std::ops::{ Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign };
use std::cmp::{ PartialEq, Eq };
use std::fmt;

use {ApproxEq, Clamp, Clamp01};
use consts::{ EPSILON };

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3{ x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    pub const FORWARD: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    pub const RIGHT: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const UP: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }
    
    pub fn clamp_magnitude(&self, max_length: f32) -> Vector3 {
        if self.sqr_magnitude() > max_length * max_length {
            return self.normalized() * max_length
        }
        
        *self
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > EPSILON {
            *self = *self / mag;
        }
        else {
            *self = Vector3::ZERO;
        }
    }

    pub fn normalized(&self) -> Vector3 {
        let mag = self.magnitude();
        if mag > EPSILON {
            return *self / mag;
        }
        
        Vector3::ZERO
    }

    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x
        }
    }
    
    pub fn distance(a: Vector3, b: Vector3) -> f32 {
        (a - b).magnitude()
    }

    pub fn angle(a: Vector3, b: Vector3) -> f32 {
        Vector3::dot(a.normalized(), b.normalized())
            .clamp(-1.0, 1.0)
            .acos()
    }

    pub fn scale(v: Vector3, other: Vector3) -> Vector3 {
        Vector3 {
            x: v.x * other.x,
            y: v.y * other.y,
            z: v.z * other.z
        }
    }
    
    pub fn ortho_normalize(a: &mut Vector3, b: &mut Vector3) {
        a.normalize();

        let mut c = Vector3::cross(*a, *b);
        c.normalize();

        *b = Vector3::cross(*a, *b);
        b.normalize();
    }

    
    pub fn lerp(start: Vector3, end: Vector3, t: f32) -> Vector3 {
        let alpha = t.clamp01();

        Vector3 {
            x: start.x + (end.x - start.x) * alpha,
            y: start.y + (end.y - start.y) * alpha,
            z: start.z + (end.z - start.z) * alpha
        }
    }

    pub fn lerp_unclamped(start: Vector3, end: Vector3, t: f32) -> Vector3 {
        Vector3 {
            x: start.x + (end.x - start.x) * t,
            y: start.y + (end.y - start.y) * t,
            z: start.z + (end.z - start.z) * t
        }
    }

    pub fn project(v: Vector3, normal: Vector3) -> Vector3 {
        let dot = Vector3::dot(normal, normal);
        if dot < EPSILON {
            Vector3::ZERO
        }
        else {
            normal * Vector3::dot(v, normal) / dot
        }
    }
    
    pub fn project_on_segment(point: Vector3, start: Vector3, end: Vector3) -> Vector3 {
        let segment = end - start;
        let proj_point = Vector3::project(point, segment.normalized());
        
        (proj_point - start).clamp_magnitude(segment.magnitude())
    }

    pub fn project_on_plane(v: Vector3, normal: Vector3) -> Vector3 {
        v - Vector3::project(v, normal)
    }

    pub fn reflect(v: Vector3, normal: Vector3) -> Vector3 {
        -2.0 * Vector3::dot(normal, v) * normal + v
    }
}

// Formatting
impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Equality
impl PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

impl Eq for Vector3 {}

impl_op! { ApproxEq,
    fn approx_eq(self: Vector3, other: Vector3) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

// Ops
impl_op! { Add,
    fn add(self: Vector3, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl_op! { Add,
    fn add(self: Vector3, other: f32) -> Vector3 {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}

impl_op! { Sub,
    fn sub(self: Vector3, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl_op! { Sub,
    fn sub(self: Vector3, other: f32) -> Vector3 {
        Vector3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other
        }
    }
}

impl_op! { Mul,
    fn mul(self: Vector3, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl_op! { Mul,
    fn mul(self: Vector3, other: f32) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl_op! { Mul,
    fn mul(self: f32, other: Vector3) -> Vector3 {
        Vector3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self
        }
    }
}

impl_op! { Div,
    fn div(self: Vector3, other: f32) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl_op! { Neg,
    fn neg(self: Vector3) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl_op! { AddAssign,
    fn add_assign(&mut self: Vector3, other: f32) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}
    
impl_op! { SubAssign,
    fn sub_assign(&mut self: Vector3, other: f32) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl_op! { MulAssign,
    fn mul_assign(&mut self: Vector3, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl_op! { DivAssign,
    fn div_assign(&mut self: Vector3, other: f32) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;   
    }
}
