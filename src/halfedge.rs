use std::cmp::{Eq, PartialEq};

use vertex::Vertex2;
use facet::Facet2;
use utils::Handle;


pub struct HalfEdge2 {
    pub vertex: Option<Handle<Vertex2>>,
    pub face: Option<Handle<Facet2>>,

    pub opposite: Option<Handle<HalfEdge2>>,
    pub next: Option<Handle<HalfEdge2>>,
}

impl PartialEq for HalfEdge2 {
    fn eq(&self, other: &HalfEdge2) -> bool {
        self as *const _ == other as *const _
    }
}

impl Eq for HalfEdge2 {}

impl HalfEdge2 {
    pub fn new() -> HalfEdge2 {
        HalfEdge2 {
            vertex: None,
            face: None,
            opposite: None,
            next: None,
        }
    }
}
