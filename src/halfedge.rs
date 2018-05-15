use std::cmp::{Eq, PartialEq};
use std::rc::Rc;

use pos::{Pos, Pos2};
use vertex::{Vertex, Vertex2};
use facet::{Facet, Facet2};
use utils::Handle;

pub struct HalfEdge<T : Pos> {
    pub vertex: Option<Handle<Vertex<T>>>,
    pub face: Option<Handle<Facet<T>>>,

    pub opposite: Option<Handle<HalfEdge<T>>>,
    pub next: Option<Handle<HalfEdge<T>>>,
}

impl<T : Pos> PartialEq for HalfEdge<T> {
    fn eq(&self, other: &HalfEdge<T>) -> bool {
        self as *const _ == other as *const _
    }
}

impl<T : Pos> Eq for HalfEdge<T> {}

impl<T:Pos> HalfEdge<T> {
    pub fn new() -> HalfEdge<T> {
        HalfEdge {
            vertex: None,
            face: None,
            opposite: None,
            next: None,
        }
    }
}


pub type HalfEdge2 = HalfEdge<Pos2>;
