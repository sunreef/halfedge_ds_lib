use std::rc::Rc;
use std::ops::{Index, Add, Sub, Mul, Div};
use std::cmp::{PartialEq, Eq};

use utils::Handle;
use halfedge::HalfEdge2;

pub trait Pos: Index<usize> + Eq{}

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
            _ => panic!("Accesing position out of bounds."),
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

pub struct Vertex<T: Pos> {
    pub position: T,
    pub edge: Option<Handle<HalfEdge2>>,
}

impl<T: Pos> PartialEq for Vertex<T> {
    fn eq(&self, other: &Vertex<T>) -> bool {
        self as *const _ == other as *const _
    }
}

impl<T: Pos> Eq for Vertex<T> {}

impl<T: Pos> Vertex<T> {
    pub fn degree(&self) -> i32 {
        if let Some(ref original_edge) = self.edge {
            let mut current_edge = Rc::clone(&original_edge);
            let mut count = 0;

            while {
                let new_edge = match &current_edge.borrow().next {
                    &Some(ref edge) => Rc::clone(&edge),
                    &None => {
                        panic!("Edge has no next edge: probably trying to find the degree of a border vertex.")
                    }
                };
                current_edge = match &new_edge.borrow().opposite {
                    &Some(ref edge) => Rc::clone(&edge),
                    &None => {
                        panic!("Edge has no opposite edge: probably trying to find the degree of a border vertex.")
                    }
                };
                count += 1;
                current_edge != *original_edge
            } {}
            count

        } else {
            panic!("A vertex has no associated edge.");
        }
    }
}

pub type Vertex2 = Vertex<Pos2>;
