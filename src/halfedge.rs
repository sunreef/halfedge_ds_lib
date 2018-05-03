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

//pub struct HalfEdge2 {
    //pub vertex: Option<Handle<Vertex2>>,
    //pub face: Option<Handle<Facet2>>,

    //pub opposite: Option<Handle<HalfEdge2>>,
    //pub next: Option<Handle<HalfEdge2>>,
//}

//impl PartialEq for HalfEdge2 {
    //fn eq(&self, other: &HalfEdge2) -> bool {
        //self as *const _ == other as *const _
    //}
//}

//impl Eq for HalfEdge2 {}

//impl HalfEdge2 {
    //pub fn new() -> HalfEdge2 {
        //HalfEdge2 {
            //vertex: None,
            //face: None,
            //opposite: None,
            //next: None,
        //}
    //}
//}
