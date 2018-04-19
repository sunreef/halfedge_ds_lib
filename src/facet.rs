use std::cmp::{Eq, PartialEq};

use std::rc::Rc;
use utils::Handle;
use halfedge::HalfEdge2;

pub struct Facet2 {
    pub edge: Option<Handle<HalfEdge2>>,
}

impl Facet2 {
    pub fn degree(&self) -> i32 {
        if let Some(ref original_edge) = self.edge {
            let mut current_edge = Rc::clone(&original_edge);
            let mut count = 0;
            while {
                      let new_edge = match &current_edge.borrow().next {
                          &Some(ref edge) => Rc::clone(&edge),
                          &None => panic!("Edge has no next edge"),
                      };
                      current_edge = new_edge;

                      count += 1;
                      current_edge != *original_edge
                  } {}
            count

        } else {
            panic!("A facet has no associated edge.");
        }
    }

    pub fn new() -> Facet2 {
        Facet2 { edge: None }
    }
}

impl PartialEq for Facet2 {
    fn eq(&self, other: &Facet2) -> bool {
        self as *const _ == other as *const _
    }
}

impl Eq for Facet2 {}
