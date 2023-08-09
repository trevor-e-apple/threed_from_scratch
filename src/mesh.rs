use std::fs::File;
use std::io;
use std::io::Read;

use crate::triangle::Face;
use crate::vector3::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
}

const CUBE_VERTICES: [Vec3; 8] = [
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    },
    Vec3 {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    },
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    },
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    },
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    },
    Vec3 {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    },
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    },
];

const CUBE_FACES: [Face; 12] = [
    // front
    Face { a: 1, b: 2, c: 3 },
    Face { a: 1, b: 3, c: 4 },
    // right
    Face { a: 4, b: 3, c: 5 },
    Face { a: 4, b: 5, c: 6 },
    // back
    Face { a: 6, b: 5, c: 7 },
    Face { a: 6, b: 7, c: 8 },
    // left
    Face { a: 8, b: 7, c: 2 },
    Face { a: 8, b: 2, c: 1 },
    // top
    Face { a: 2, b: 7, c: 5 },
    Face { a: 2, b: 5, c: 3 },
    // bottom
    Face { a: 6, b: 8, c: 1 },
    Face { a: 6, b: 1, c: 4 },
];

#[derive(Debug)]
pub enum LoadMeshError {
    FormatError,
    IoError(io::Error),
}

pub fn load_mesh(file_path: &String) -> Result<Mesh, LoadMeshError> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(LoadMeshError::IoError(err)),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(err) => return Err(LoadMeshError::IoError(err)),
    };

    let mut vertices: Vec<Vec3> = vec![];
    let mut faces: Vec<Face> = vec![];
    for line in contents.lines() {
        if line.starts_with("v ") {
            // vertex
            let mut line_iter = line.split(" ");
            // skip 'v'
            line_iter.next();

            // fill elements
            let mut elements: [f32; 3] = [0.0; 3];
            for element in &mut elements {
                let element_text = match line_iter.next() {
                    Some(text) => text,
                    None => return Err(LoadMeshError::FormatError),
                };
                *element = match element_text.parse() {
                    Ok(value) => value,
                    Err(_) => return Err(LoadMeshError::FormatError),
                };
            }

            vertices.push(Vec3 {
                x: elements[0],
                y: elements[1],
                z: elements[2],
            });
        } else if line.starts_with("f ") {
            // face
            let mut line_iter = line.split(" ");
            line_iter.next();
            // only grabbing the first elements right now, which correspond to
            // -- the vertex index
            let mut elements: [usize; 3] = [0; 3];
            for element in &mut elements {
                let face_text_data: Vec<&str> = match line_iter.next() {
                    Some(text) => text.split("/").collect(),
                    None => return Err(LoadMeshError::FormatError),
                };
                *element = match face_text_data[0].parse() {
                    Ok(value) => value,
                    Err(_) => return Err(LoadMeshError::FormatError),
                };
            }
            faces.push(Face {
                a: elements[0],
                b: elements[1],
                c: elements[2],
            })
        }
    }

    Ok(Mesh {
        vertices,
        faces,
        rotation: Vec3 {
            ..Default::default()
        },
    })
}

pub fn load_cube_mesh() -> Mesh {
    Mesh {
        vertices: CUBE_VERTICES.to_vec(),
        faces: CUBE_FACES.to_vec(),
        rotation: Vec3 {
            ..Default::default()
        },
    }
}
