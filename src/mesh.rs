use std::fs::File;
use std::io::Read;

use crate::texture::TextureUv;
use crate::triangle::Face;
use crate::vector::Vector3;

pub struct Mesh {
    vertices: Vec<Vector3>,
    texel_coordinates: Vec<TextureUv>,
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn get_vertices(&self, face: &Face) -> [Vector3; 3] {
        [
            self.vertices[face.a - 1].clone(),
            self.vertices[face.b - 1].clone(),
            self.vertices[face.c - 1].clone(),
        ]
    }

    pub fn get_texel_coordinates(&self, face: &Face) -> [TextureUv; 3] {
        [
            self.texel_coordinates[face.a_uv - 1].clone(),
            self.texel_coordinates[face.b_uv - 1].clone(),
            self.texel_coordinates[face.c_uv - 1].clone(),
        ]
    }
}

pub fn load_test_mesh() -> Mesh {
    Mesh {
        vertices: MESH_VERTICES.to_vec(),
        texel_coordinates: TEXEL_COORDINATES.to_vec(),
        faces: MESH_FACES.to_vec(),
    }
}

pub fn load_obj_mesh(path: &String) -> Mesh {
    let mut vertices = vec![];
    let mut texel_coordinates = vec![];
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
        } else if line.starts_with("vt ") {
            let rest_of_line = &line[3..];
            let mut elements = [0.0; 2];
            for (index, element_str) in rest_of_line.split(" ").enumerate() {
                elements[index] = match element_str.parse() {
                    Ok(float) => float,
                    Err(_) => panic!("Unable to convert vertex to float"),
                };
            }

            texel_coordinates.push(TextureUv {
                u: elements[0],
                v: 1.0 - elements[1], // we use top left coordinates
            });
        } else if line.starts_with("f ") {
            let rest_of_line = &line[2..];
            // most faces are 3 vertices (a triangle), but there is a possibility for a polygon
            let mut coordinate_elements: [usize; 3] = [0; 3];
            let mut texel_elements: [usize; 3] = [0; 3];
            for (element_index, element_str) in
                rest_of_line.split(" ").enumerate()
            {
                let vertex_info: Vec<&str> = element_str.split("/").collect();
                let vertex_index: usize = match vertex_info[0].parse() {
                    Ok(vertex_index) => vertex_index,
                    Err(_) => panic!("Unable to convert index to usize"),
                };
                let texel_index: usize = match vertex_info[1].parse() {
                    Ok(texel_index) => texel_index,
                    Err(_) => panic!("Unable to convert texel index to usize"),
                };
                // TODO: handle vertex texture and normal info
                coordinate_elements[element_index] = vertex_index;
                texel_elements[element_index] = texel_index;
            }

            faces.push(Face {
                a: coordinate_elements[0],
                b: coordinate_elements[1],
                c: coordinate_elements[2],
                a_uv: texel_elements[0],
                b_uv: texel_elements[1],
                c_uv: texel_elements[2],
                color: 0xFFAAAAAA,
            });
        } else {
            // TODO: don't do anything here yet
        }
    }

    Mesh {
        vertices,
        texel_coordinates,
        faces,
    }
}

const MESH_VERTICES: [Vector3; 8] = [
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

const TEXEL_COORDINATES: [TextureUv; 4] = [
    TextureUv { u: 1.0, v: 0.0 },
    TextureUv { u: 0.0, v: 0.0 },
    TextureUv { u: 1.0, v: 1.0 },
    TextureUv { u: 0.0, v: 1.0 },
];

const MESH_FACES: [Face; 12] = [
    // front
    Face {
        a: 1,
        b: 2,
        c: 3,
        a_uv: 4,
        b_uv: 2,
        c_uv: 1,
        color: 0xFFFFFFFF,
    },
    Face {
        a: 1,
        b: 3,
        c: 4,
        a_uv: 4,
        b_uv: 1,
        c_uv: 3,
        color: 0xFFFFFFFF,
    },
    // right
    Face {
        a: 4,
        b: 3,
        c: 5,
        a_uv: 4,
        b_uv: 2,
        c_uv: 1,
        color: 0xFFFFFFFF,
    },
    Face {
        a: 4,
        b: 5,
        c: 6,
        a_uv: 4,
        b_uv: 1,
        c_uv: 3,
        color: 0xFFFFFFFF,
    },
    // back
    Face {
        a: 6,
        b: 5,
        c: 7,
        a_uv: 4,
        b_uv: 2,
        c_uv: 1,
        color: 0xFFFFFFFF,
    },
    Face {
        a: 6,
        b: 7,
        c: 8,
        a_uv: 4,
        b_uv: 1,
        c_uv: 3,
        color: 0xFFFFFFFF,
    },
    // left
    Face {
        a: 8,
        b: 7,
        c: 2,
        a_uv: 4,
        b_uv: 2,
        c_uv: 1,
        color: 0xFFFFFFFF,
    },
    Face {
        a: 8,
        b: 2,
        c: 1,
        a_uv: 4,
        b_uv: 1,
        c_uv: 3,
        color: 0xFFFFFFFF,
    },
    // top
    Face {
        a: 2,
        b: 7,
        c: 5,
        a_uv: 4,
        b_uv: 2,
        c_uv: 1,
        color: 0xFFFFFFFF,
    },
    Face {
        a: 2,
        b: 5,
        c: 3,
        a_uv: 4,
        b_uv: 1,
        c_uv: 3,
        color: 0xFFFFFFFF,
    },
    // bottom
    Face {
        a: 6,
        b: 8,
        c: 1,
        a_uv: 4,
        b_uv: 2,
        c_uv: 1,
        color: 0xFFFFFFFF,
    },
    Face {
        a: 6,
        b: 1,
        c: 4,
        a_uv: 4,
        b_uv: 1,
        c_uv: 3,
        color: 0xFFFFFFFF,
    },
];
