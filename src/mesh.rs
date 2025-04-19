use std::fs::File;
use std::io::Read;

use crate::triangle::Face;
use crate::vector::Vector3;

pub const MESH_VERTICES: [Vector3; 8] = [
    Vector3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    }, // 1
    Vector3 {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    }, // 2
    Vector3 {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    }, // 3
    Vector3 {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    }, // 4
    Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }, // 5
    Vector3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    }, // 6
    Vector3 {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    }, // 7
    Vector3 {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    }, // 8
];

pub const MESH_FACES: [Face; 12] = [
    // front
    Face { a: 1, b: 2, c: 3, color: 0xFFFF0000 },
    Face { a: 1, b: 3, c: 4, color: 0xFFFF0000 },
    // right
    Face { a: 4, b: 3, c: 5,  color: 0xFF00FF00},
    Face { a: 4, b: 5, c: 6,  color: 0xFF00FF00},
    // back
    Face { a: 6, b: 5, c: 7,  color: 0xFF0000FF },
    Face { a: 6, b: 7, c: 8,  color: 0xFF0000FF },
    // left
    Face { a: 8, b: 7, c: 2, color: 0xFFFFFF00 },
    Face { a: 8, b: 2, c: 1, color: 0xFFFFFF00 },
    // top
    Face { a: 2, b: 7, c: 5, color: 0xFFFF00FF },
    Face { a: 2, b: 5, c: 3, color: 0xFFFF00FF },
    // bottom
    Face { a: 6, b: 8, c: 1, color: 0xFF00FFFF },
    Face { a: 6, b: 1, c: 4, color: 0xFF00FFFF },
];

pub fn load_obj_mesh(path: &String) -> (Vec<Vector3>, Vec<Face>) {
    let mut vertices = vec![];
    let mut faces = vec![];

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => todo!(),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => todo!(),
    }

    for line in contents.lines() {
        if line.starts_with("v ") {
            let rest_of_line = &line[2..];
            let mut elements = [0.0; 4];
            for (index, element_str) in rest_of_line.split(" ").enumerate() {
                elements[index] = match element_str.parse() {
                    Ok(float) => float,
                    Err(_) => panic!("Unable to convert vertex to float"),
                };
            }

            // TODO: what to do with the affine coordinate (w)?
            vertices.push(Vector3 {
                x: elements[0],
                y: elements[1],
                z: elements[2],
            });
        } else if line.starts_with("f ") {
            let rest_of_line = &line[2..];
            // most faces are 3 vertices (a triangle), but there is a possibility for a polygon
            let mut elements: Vec<usize> = Vec::with_capacity(3);
            for element_str in rest_of_line.split(" ") {
                let vertex_info: Vec<&str> = element_str.split("/").collect();
                let vertex_index: usize = match vertex_info[0].parse() {
                    Ok(vertex_index) => vertex_index,
                    Err(_) => panic!("Unable to convert index to usize"),
                };
                // TODO: handle vertex texture and normal info
                elements.push(vertex_index);
            }

            assert!(elements.len() == 3);

            faces.push(Face {
                a: elements[0],
                b: elements[1],
                c: elements[2],
                color: 0xFFAAAAAA
            });
        } else {
            // TODO: don't do anything here yet
        }
    }

    (vertices, faces)
}
