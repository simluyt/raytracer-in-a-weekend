use std::{ops, fmt};


pub type Point3 = Vec3;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

impl Vec3 {
    pub fn new(&self) -> Vec3 {
        unimplemented!();
    }

    pub fn length(&self) -> f64 {
        (self.length_squared().sqrt())
    }

    pub fn length_squared(&self) -> f64 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }

}

impl fmt::Display for Vec3 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}



impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Index<i32> for Vec3 {
    type Output = f64;
    fn index(&self, index: i32) -> &Self::Output {
        match index{
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!()
        }
    }
}

impl ops::IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!()
        }
    }
}


impl ops::Add for  Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl ops::AddAssign for  Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Self {
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
        };
    }
}
impl ops::Sub for  Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) ->Self::Output {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl ops::SubAssign for  Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Self {
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z,
        };
    }
}

impl ops::Mul for  Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) ->Self::Output {
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl ops::Mul<f64> for  Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) ->Self::Output {
        Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl ops::Mul<Vec3> for  f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) ->Self::Output {
        other * self
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
        };
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        (1.0/other) * self
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x : self.x / other,
            y : self.y / other,
            z : self.z / other,
        };
    }
}

pub fn cross(this: Vec3, other: Vec3) -> Vec3 {
    Vec3 {
        x : this.y * other.z - this.z * other.y,
        y : this.z * other.x - this.x * other.z,
        z : this.x * other.y - this.y * other.x
    }
}

pub fn dot(this: Vec3, other: Vec3) -> f64 {
    this.x * other.x + this.y * other.y + this.z * other.z
}


pub fn unit(this: Vec3) -> Vec3 {
    this * (1.0/this.length())
}

