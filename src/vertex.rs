use std::rc::Rc;
use std::cmp::{PartialEq, Eq};

use utils::Handle;
use halfedge::HalfEdge2;
use pos::{Pos, Pos2};

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
                        panic!("Edge has no next edge: this should never be the case.")
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
