use std::rc::Rc;
use std::fs::File;
use std::io::Write;
use std::f64::consts::*;

use simplesvg::*;

use utils::*;
use vertex::{Vertex2, Pos2};
use halfedge::HalfEdge2;
use facet::Facet2;

use std::vec::Vec;

pub struct Polyhedron2 {
    pub vertices: Vec<Handle<Vertex2>>,
    pub edges: Vec<Handle<HalfEdge2>>,
    pub facets: Vec<Handle<Facet2>>,
}

// Basic methods

impl Polyhedron2 {
    pub fn new() -> Polyhedron2 {
        Polyhedron2 {
            vertices: Vec::new(),
            edges: Vec::new(),
            facets: Vec::new(),
        }
    }

    pub fn vertices_size(&self) -> usize {
        self.vertices.len()
    }

    pub fn edges_size(&self) -> usize {
        self.edges.len()
    }

    pub fn facets_size(&self) -> usize {
        self.facets.len()
    }
}

// Various constructors

impl Polyhedron2 {
    pub fn create_triangle() -> Polyhedron2 {
        let v1 = new_handle(Vertex2 {
            edge: None,
            position: Pos2 { x: 0., y: 0. },
        });
        let v2 = new_handle(Vertex2 {
            edge: None,
            position: Pos2 { x: 0., y: 200. },
        });
        let v3 = new_handle(Vertex2 {
            edge: None,
            position: Pos2 { x: 200., y: 0. },
        });

        let f = new_handle(Facet2::new());

        let e1 = new_handle(HalfEdge2 {
            vertex: Some(Rc::clone(&v1)),
            opposite: None,
            next: None,
            face: Some(Rc::clone(&f)),
        });
        let e3 = new_handle(HalfEdge2 {
            vertex: Some(Rc::clone(&v3)),
            opposite: None,
            next: Some(Rc::clone(&e1)),
            face: Some(Rc::clone(&f)),
        });
        let e2 = new_handle(HalfEdge2 {
            vertex: Some(Rc::clone(&v2)),
            opposite: None,
            next: Some(Rc::clone(&e3)),
            face: Some(Rc::clone(&f)),
        });
        e1.borrow_mut().next = Some(Rc::clone(&e2));

        v1.borrow_mut().edge = Some(Rc::clone(&e1));
        v2.borrow_mut().edge = Some(Rc::clone(&e2));
        v3.borrow_mut().edge = Some(Rc::clone(&e3));

        f.borrow_mut().edge = Some(Rc::clone(&e1));

        let mut vertices = Vec::new();
        vertices.push(v1);
        vertices.push(v2);
        vertices.push(v3);

        let mut edges = Vec::new();
        edges.push(e1);
        edges.push(e2);
        edges.push(e3);

        let mut facets = Vec::new();
        facets.push(f);

        Polyhedron2 {
            vertices: vertices,
            edges: edges,
            facets: facets,
        }
    }

    pub fn create_rectangle(corner_x: f32, corner_y: f32, height: f32, width: f32) -> Polyhedron2 {
        let v1 = new_handle(Vertex2 {
            edge: None,
            position: Pos2 { x: corner_x, y: corner_y },
        });
        let v2 = new_handle(Vertex2 {
            edge: None,
            position: Pos2 { x: corner_x, y: corner_y + height },
        });
        let v3 = new_handle(Vertex2 {
            edge: None,
            position: Pos2 { x: corner_x + width, y: corner_y + height },
        });
        let v4 = new_handle(Vertex2 {
            edge: None,
            position: Pos2 { x: corner_x + width, y: corner_y },
        });

        let f = new_handle(Facet2::new());

        let e1 = new_handle(HalfEdge2 {
            vertex: Some(Rc::clone(&v1)),
            opposite: None,
            next: None,
            face: Some(Rc::clone(&f)),
        });
        let e4 = new_handle(HalfEdge2 {
            vertex: Some(Rc::clone(&v4)),
            opposite: None,
            next: Some(Rc::clone(&e1)),
            face: Some(Rc::clone(&f)),
        });
        let e3 = new_handle(HalfEdge2 {
            vertex: Some(Rc::clone(&v3)),
            opposite: None,
            next: Some(Rc::clone(&e4)),
            face: Some(Rc::clone(&f)),
        });
        let e2 = new_handle(HalfEdge2 {
            vertex: Some(Rc::clone(&v2)),
            opposite: None,
            next: Some(Rc::clone(&e3)),
            face: Some(Rc::clone(&f)),
        });
        e1.borrow_mut().next = Some(Rc::clone(&e2));

        v1.borrow_mut().edge = Some(Rc::clone(&e1));
        v2.borrow_mut().edge = Some(Rc::clone(&e2));
        v3.borrow_mut().edge = Some(Rc::clone(&e3));
        v4.borrow_mut().edge = Some(Rc::clone(&e4));

        f.borrow_mut().edge = Some(Rc::clone(&e1));

        let mut vertices = Vec::new();
        vertices.push(v1);
        vertices.push(v2);
        vertices.push(v3);
        vertices.push(v4);

        let mut edges = Vec::new();
        edges.push(e1);
        edges.push(e2);
        edges.push(e3);
        edges.push(e4);

        let mut facets = Vec::new();
        facets.push(f);

        Polyhedron2 {
            vertices: vertices,
            edges: edges,
            facets: facets,
        }
    }

    pub fn create_regular_polygon(center_x: f32, center_y: f32, radius: f32, sides: usize) -> Polyhedron2 {
        let mut vertices = Vec::new();
        let mut edges = Vec::new();
        let mut facets = Vec::new();

        let f = new_handle(Facet2::new());
        facets.push(f);

        let angle = 2.0 * PI as f32 / (sides as f32);
        for i in 0..sides {
            let x = center_x + radius * ((i as f32 * angle).cos());
            let y = center_y - radius * ((i as f32 * angle).sin());

            let vertex = Vertex2 {
                position: Pos2 {
                    x: x,
                    y: y,
                },
                edge: None
            };
            vertices.push(new_handle(vertex));

            let mut edge = HalfEdge2::new();
            edge.vertex = Some(Rc::clone(&vertices[i]));
            edge.face = Some(Rc::clone(&facets[0]));
            edges.push(new_handle(edge));

            vertices[i].borrow_mut().edge = Some(Rc::clone(&edges[i]));
        }

        for i in 0..sides {
            edges[i].borrow_mut().next = Some(Rc::clone(&edges[(i + 1) % sides]));
        }

        facets[0].borrow_mut().edge = Some(Rc::clone(&edges[0]));

        Polyhedron2 {
            vertices: vertices,
            edges: edges,
            facets: facets
        }
    }
}

// Utility operators

impl Polyhedron2 {
    pub fn get_center_position(&self, edge: Handle<HalfEdge2>) -> Pos2 {
        let mut center_x = 0f32;
        let mut center_y = 0f32;
        let mut d = 0i32;

        let mut current_edge = Rc::clone(&edge);
        while {
            let v = get_element!(current_edge, vertex);
            let pos = v.borrow().position;

            center_x += pos.x;
            center_y += pos.y;
            d += 1;

            current_edge = get_element!(current_edge, next);
            edge != current_edge 
        } {}

        center_x /= d as f32;
        center_y /= d as f32;

        Pos2 {
            x: center_x,
            y: center_y,
        }
    }

    pub fn get_area(&self, edge: Handle<HalfEdge2>) -> f32 {
        let mut area = 0f32;

        let mut previous_vertex = get_element!(edge, vertex);
        let mut current_edge = get_element!(edge, next);
        while edge != current_edge {
            let vertex = get_element!(current_edge, vertex);

            let previous_position = previous_vertex.borrow().position;
            let current_position = vertex.borrow().position;

            area += previous_position.x * current_position.y - previous_position.y * current_position.x;

            previous_vertex = vertex;
            current_edge = get_element!(current_edge, next);
        }
        (area * 0.5f32).abs()
    }

    pub fn get_prev_edge(&self, edge: Handle<HalfEdge2>) -> Handle<HalfEdge2> {
        let mut current_edge = get_element!(edge, next);
        while(current_edge != edge) {
            let next_edge = get_element!(current_edge, next);
            if (next_edge == edge) {
                return current_edge;
            }
            current_edge = next_edge;
        }
        edge
    }
}

// Combinatorial Euler operators

impl Polyhedron2 {
    pub fn create_center_vertex(&mut self, edge: Handle<HalfEdge2>) {
        let center_position = self.get_center_position(Rc::clone(&edge));
        let vertex_handle = new_handle(Vertex2 { position: center_position, edge: None});

        let mut new_facets = Vec::new();
        let mut new_edges = Vec::new();

        let facet_handle = get_element!(edge, face);
        let degree = facet_handle.borrow().degree();
        let mut current_edge = get_element!(facet_handle, edge);

        for _i in 0..degree {
            new_facets.push(new_handle(Facet2::new()));
            new_edges.push(new_handle(HalfEdge2::new()));
            new_edges.push(new_handle(HalfEdge2::new()));
        }
        new_facets[0] = facet_handle;

        for i in 0usize..degree as usize {
            new_facets[i].borrow_mut().edge = Some(Rc::clone(&current_edge));

            let next_edge = get_element!(current_edge, next);

            {
                let mut edge_mut = current_edge.borrow_mut();
                edge_mut.next = Some(Rc::clone(&new_edges[(2 * i)]));
                edge_mut.face = Some(Rc::clone(&new_facets[i]));
            }

            {
                let mut edge_mut = new_edges[(2 * i)].borrow_mut();
                edge_mut.next = Some(Rc::clone(&new_edges[(2 * degree as usize + 2 * i - 1) %
                                               (2 * degree as usize)]));
                edge_mut.opposite = Some(Rc::clone(&new_edges[(2 * i + 1)]));
                edge_mut.face = Some(Rc::clone(&new_facets[i]));
                edge_mut.vertex = Some(Rc::clone(&vertex_handle));
            }

            {
                let mut edge_mut = new_edges[(2 * i + 1)].borrow_mut();
                edge_mut.next = Some(Rc::clone(&next_edge));
                edge_mut.opposite = Some(Rc::clone(&new_edges[(2 * i)]));
                edge_mut.face = Some(Rc::clone(&new_facets[(i + 1) % degree as usize]));
                edge_mut.vertex = Some(get_element!(current_edge, vertex));
            }

            current_edge = next_edge;
        }

        vertex_handle.borrow_mut().edge = Some(Rc::clone(&new_edges[0]));

        self.vertices.push(vertex_handle);
        new_facets.remove(0);
        for f in new_facets {
            self.facets.push(f);
        }
        for e in new_edges {
            self.edges.push(e);
        }
    }

    pub fn erase_center_vertex(&mut self, edge: Handle<HalfEdge2>) {
        let face = get_element!(edge, face);
        let vertex = get_element!(edge, vertex);
        let degree = vertex.borrow().degree();

        let mut faces_to_remove = Vec::new();
        let mut edges_to_remove = Vec::new();

        let mut current_edge = Rc::clone(&edge);
        for i in 0..degree {
            let next_edge = get_element!(current_edge, next);
            let opposite_next_edge = get_element!(next_edge, opposite);

            let mut previous_edge;
            let mut temp_edge = get_element!(next_edge, next);
            while  {
                temp_edge.borrow_mut().face = Some(Rc::clone(&face));
                previous_edge = Rc::clone(&temp_edge);
                temp_edge = get_element!(temp_edge, next);
                temp_edge != current_edge
            } {}
            let opposite_edge = get_element!(current_edge, opposite);
            let next_opposite_edge = get_element!(opposite_edge, next);
            previous_edge.borrow_mut().next = Some(next_opposite_edge);

            if i == 0 {
                face.borrow_mut().edge = Some(previous_edge);
            }

            faces_to_remove.push(get_element!(current_edge, face));
            edges_to_remove.push(current_edge);
            edges_to_remove.push(next_edge);

            current_edge = opposite_next_edge;
        }
        faces_to_remove.remove(0);

        for f in faces_to_remove {
            for i in 0..self.facets.len() {
                if self.facets[i] == f {
                    self.facets.remove(i);
                    break;
                }
            }
        }
        for e in edges_to_remove {
            for i in 0..self.edges.len() {
                if self.edges[i] == e {
                    self.edges.remove(i);
                    break;
                }
            }
        }
    }

    pub fn flip_edge(&self, edge: Handle<HalfEdge2>) {
        let face = get_element!(edge, face);
        let vertex = get_element!(edge, vertex);

        let opposite_edge = get_element!(edge, opposite);
        let opposite_face = get_element!(opposite_edge, face);
        let opposite_vertex = get_element!(opposite_edge, vertex);

        assert_eq!(face.borrow().degree(), 3);
        assert_eq!(opposite_face.borrow().degree(), 3);

        let next_edge = get_element!(edge, next);
        let previous_edge = get_element!(next_edge, next);

        let next_opposite_edge = get_element!(opposite_edge, next);
        let previous_opposite_edge = get_element!(next_opposite_edge, next);

        {
            let mut edge_mut = edge.borrow_mut();
            edge_mut.vertex = Some(get_element!(next_edge, vertex));
            edge_mut.next = Some(Rc::clone(&previous_edge));
        }
        {
            let mut edge_mut = opposite_edge.borrow_mut();
            edge_mut.vertex = Some(get_element!(next_opposite_edge, vertex));
            edge_mut.next = Some(Rc::clone(&previous_opposite_edge));
        }

        {
            let mut edge_mut = next_edge.borrow_mut();
            edge_mut.face = Some(Rc::clone(&opposite_face));
            edge_mut.next = Some(Rc::clone(&opposite_edge));
        }
        {
            let mut edge_mut = next_opposite_edge.borrow_mut();
            edge_mut.face = Some(Rc::clone(&face));
            edge_mut.next = Some(Rc::clone(&edge));
        }

        previous_edge.borrow_mut().next = Some(Rc::clone(&next_opposite_edge));
        previous_opposite_edge.borrow_mut().next = Some(Rc::clone(&next_edge));

        face.borrow_mut().edge = Some(Rc::clone(&edge));
        opposite_face.borrow_mut().edge = Some(Rc::clone(&opposite_edge));

        vertex.borrow_mut().edge = Some(Rc::clone(&previous_opposite_edge));
        opposite_vertex.borrow_mut().edge = Some(Rc::clone(&previous_edge));
    }

    pub fn split_facet(&mut self, edge1: Handle<HalfEdge2>, edge2: Handle<HalfEdge2>) {
        let face1 = get_element!(edge1, face);
        assert!(face1 == get_element!(edge2, face));
        let face2 = new_handle(Facet2::new());

        let vertex1 = get_element!(edge1, vertex);
        let vertex2 = get_element!(edge2, vertex);

        let new_edge1 = new_handle(HalfEdge2::new());
        let new_edge2 = new_handle(HalfEdge2::new());

        {
            let mut edge_mut = new_edge1.borrow_mut();
            edge_mut.opposite = Some(Rc::clone(&new_edge2));
            edge_mut.vertex = Some(Rc::clone(&vertex2));
            edge_mut.face = Some(Rc::clone(&face1));
            edge_mut.next = Some(get_element!(edge2, next));
        }
        {
            let mut edge_mut = new_edge2.borrow_mut();
            edge_mut.opposite = Some(Rc::clone(&new_edge1));
            edge_mut.vertex = Some(Rc::clone(&vertex1));
            edge_mut.face = Some(Rc::clone(&face2));
            edge_mut.next = Some(get_element!(edge1, next));
        }

        edge1.borrow_mut().next = Some(Rc::clone(&new_edge1));
        edge2.borrow_mut().next = Some(Rc::clone(&new_edge2));

        face1.borrow_mut().edge = Some(Rc::clone(&edge1));
        face2.borrow_mut().edge = Some(Rc::clone(&edge2));

        let mut current_edge1 = Rc::clone(&new_edge1);
        while {
            current_edge1.borrow_mut().face = Some(Rc::clone(&face1));
            current_edge1 = get_element!(current_edge1, next);
            current_edge1 != new_edge1
        } {}

        let mut current_edge2 = Rc::clone(&new_edge2);
        while {
            current_edge2.borrow_mut().face = Some(Rc::clone(&face2));
            current_edge2 = get_element!(current_edge2, next);
            current_edge2 != new_edge2
        } {}

        self.edges.push(new_edge1);
        self.edges.push(new_edge2);

        self.facets.push(face2);
    }

    pub fn join_facet(&mut self, edge: Handle<HalfEdge2>) {
        let face = get_element!(edge, face);
        let opposite_edge = get_element!(edge, opposite);
        let opposite_face = get_element!(opposite_edge, face);

        let mut previous_edge = Rc::clone(&edge);
        let mut current_edge = Rc::clone(&edge);
        while {
            let next_edge = get_element!(current_edge, next);
            previous_edge = current_edge;
            current_edge = next_edge;
            current_edge != edge
        } {}

        let mut previous_opposite_edge = Rc::clone(&opposite_edge);
        let mut current_opposite_edge = Rc::clone(&opposite_edge);
        while {
            previous_opposite_edge.borrow_mut().face = Some(Rc::clone(&face));
            let next_edge = get_element!(current_opposite_edge, next);
            previous_opposite_edge = current_opposite_edge;
            current_opposite_edge = next_edge;
            current_opposite_edge != opposite_edge
        } {}

        previous_edge.borrow_mut().next = Some(get_element!(opposite_edge, next));
        previous_opposite_edge.borrow_mut().next = Some(get_element!(edge, next));

        for i in 0..self.edges.len() {
            if edge == self.edges[i] {
                self.edges.remove(i);
                break;
            }
        }
        for i in 0..self.edges.len() {
            if opposite_edge == self.edges[i] {
                self.edges.remove(i);
                break;
            }
        }
        for i in 0..self.facets.len() {
            if opposite_face == self.facets[i] {
                self.facets.remove(i);
                break;
            }
        }
    }

    pub fn split_vertex(&mut self, edge1: Handle<HalfEdge2>, edge2: Handle<HalfEdge2>) {
        let vertex1 = get_element!(edge1, vertex);
        let vertex2 = get_element!(edge2, vertex);

        assert!(vertex1 == vertex2);

        let new_vertex = new_handle(Vertex2 { position: vertex1.borrow().position, edge: Some(Rc::clone(&edge2)) });

        let new_edge1 = new_handle(HalfEdge2 { vertex: Some(Rc::clone(&new_vertex)), face: Some(get_element!(edge1, face)), opposite: None, next: Some(get_element!(edge1, next)) });
        let new_edge2 = new_handle(HalfEdge2 { vertex: Some(Rc::clone(&vertex1)), face: Some(get_element!(edge2, face)), opposite: Some(Rc::clone(&new_edge1)), next: Some(get_element!(edge2, next)) });
        new_edge1.borrow_mut().opposite = Some(Rc::clone(&new_edge2));

        let mut current_edge = get_element!(new_edge1, next);
        current_edge = get_element!(current_edge, opposite);
        while(current_edge != edge2) {
            current_edge.borrow_mut().vertex = Some(Rc::clone(&new_vertex));

            current_edge = get_element!(current_edge, next);
            current_edge = get_element!(current_edge, opposite);
        }

        edge1.borrow_mut().next = Some(Rc::clone(&new_edge1));

        edge2.borrow_mut().next = Some(Rc::clone(&new_edge2));
        edge2.borrow_mut().vertex = Some(Rc::clone(&new_vertex));

        self.vertices.push(new_vertex);

        self.edges.push(new_edge1);
        self.edges.push(new_edge2);
    }

    pub fn join_vertex(&mut self, edge: Handle<HalfEdge2>) {
        let opposite_edge = get_element!(edge, opposite);

        let vertex = get_element!(edge, vertex);
        let opposite_vertex = get_element!(opposite_edge, vertex);

        let new_pos = 0.5f32 * (vertex.borrow().position + opposite_vertex.borrow().position);
        vertex.borrow_mut().position = new_pos;

        let face = get_element!(edge, face);
        face.borrow_mut().edge = Some(get_element!(edge, next));

        let opposite_face = get_element!(opposite_edge, face);
        opposite_face.borrow_mut().edge = Some(get_element!(opposite_edge, next));

        let previous_edge = self.get_prev_edge(Rc::clone(&edge));
        let previous_opposite_edge = self.get_prev_edge(Rc::clone(&opposite_edge));

        let mut current_edge = Rc::clone(&opposite_edge);
        while(current_edge != previous_edge) {
            current_edge.borrow_mut().vertex = Some(Rc::clone(&vertex));
            current_edge = get_element!(current_edge, next);
            current_edge = get_element!(current_edge, opposite);
        }

        previous_edge.borrow_mut().next = Some(get_element!(edge, next));
        previous_edge.borrow_mut().vertex = Some(Rc::clone(&vertex));

        previous_opposite_edge.borrow_mut().next = Some(get_element!(opposite_edge, next));

        for i in 0..self.edges.len() {
            if edge == self.edges[i] {
                self.edges.remove(i);
                break;
            }
        }
        for i in 0..self.edges.len() {
            if opposite_edge == self.edges[i] {
                self.edges.remove(i);
                break;
            }
        }
        for i in 0..self.vertices.len() {
            if opposite_vertex == self.vertices[i] {
                self.vertices.remove(i);
                break;
            }
        }





    }
}

// Display and output methods

impl Polyhedron2 {
    pub fn draw_svg(&self, path: &str, height: u32, width: u32) {
        let mut lines: Vec<Fig> = Vec::new();
        let line_style = Attr::default().stroke(ColorAttr::Color(255u8,0u8,0u8));
        let face_style = Attr::default().stroke(ColorAttr::Color(255u8,0u8,0u8)).fill(ColorAttr::Color(0u8, 0u8, 255u8));
        for f in &self.facets {
            let mut face_lines = Vec::new();
            let degree = f.borrow().degree();

            let mut current_edge = get_element!(f, edge);
            for _ in 0..degree {
                let first_vertex = get_element!(current_edge, vertex);
                let first_position = first_vertex.borrow().position;
                let next_edge = get_element!(current_edge, next);
                let second_vertex = get_element!(next_edge, vertex);
                let second_position = second_vertex.borrow().position;
                current_edge = next_edge;

                let mut line = Fig::Line(first_position.x, first_position.y, second_position.x, second_position.y);
                face_lines.push(line);
            }

            let triangle = Fig::Multiple(face_lines).styled(face_style.clone());
            lines.push(triangle);
        }

        let svg = Svg(lines, height, width);

        let mut file = match File::create(&path) {
            Err(_) => panic!("Failed to create output SVG file"),
            Ok(f) => f,
        };
        match write!(file, "{}", svg) {
            Ok(_) => println!("Successfully wrote to {}.", path),   
            Err(e) => panic!("{}", e),
        };
    }

    pub fn save_as_obj(&self, path: &str) {


    }
}
