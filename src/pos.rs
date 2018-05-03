use std::ops::{Index, Add, Sub, Mul, Div};
use std::cmp::{PartialEq, Eq};

pub trait Pos: Copy + Clone + Sized + Index<usize> + Eq + Add + Sub + Mul<f32> + Div<f32>{}

#[derive(Copy, Clone)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}

impl Pos for Pos2 {}

impl Index<usize> for Pos2 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Accessing position out of bounds."),
        }
    }
}

impl PartialEq for Pos2 {
    fn eq(&self, other: &Pos2) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl Eq for Pos2 {}

impl Add for Pos2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Pos2 {
            x : self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Pos2 {
            x : self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<f32> for Pos2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Pos2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<f32> for Pos2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Pos2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Pos2> for f32 {
    type Output = Pos2;
    fn mul(self, rhs: Pos2) -> Self::Output {
        Pos2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }

}
