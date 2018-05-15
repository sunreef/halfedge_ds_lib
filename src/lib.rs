extern crate svg;
extern crate rand;

#[macro_use]
pub mod utils;
pub mod halfedge;
pub mod vertex;
pub mod facet;
pub mod polyhedron;
pub mod pos;



#[cfg(test)]
mod tests {
    #[test]
    fn test_svg() {
        use polyhedron::Polyhedron2;
        let poly = Polyhedron2::create_triangle();
        poly.draw_svg("./poly.svg", 500u32, 500u32);
    }


}
