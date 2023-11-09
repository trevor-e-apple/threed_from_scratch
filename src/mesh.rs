use std::fs::File;
use std::io;
use std::io::Read;

use crate::texture::Tex2;
use crate::triangle::Face;
use crate::vector3::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
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
    Face {
        a: 0,
        b: 1,
        c: 2,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 0.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 0.0 },
        color: 0xFFFF0000,
    },
    Face {
        a: 0,
        b: 2,
        c: 3,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 1.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 1.0 },
        color: 0xFFFF0000,
    },
    // right
    Face {
        a: 3,
        b: 2,
        c: 4,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 0.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 0.0 },
        color: 0xFF00FF00,
    },
    Face {
        a: 3,
        b: 4,
        c: 5,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 1.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 1.0 },
        color: 0xFF00FF00,
    },
    // back
    Face {
        a: 5,
        b: 4,
        c: 6,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 0.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 0.0 },
        color: 0xFF0000FF,
    },
    Face {
        a: 5,
        b: 6,
        c: 7,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 1.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 1.0 },
        color: 0xFF0000FF,
    },
    // left
    Face {
        a: 7,
        b: 6,
        c: 1,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 0.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 0.0 },
        color: 0xFFFF00FF,
    },
    Face {
        a: 7,
        b: 1,
        c: 0,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 1.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 1.0 },
        color: 0xFFFF00FF,
    },
    // top
    Face {
        a: 1,
        b: 6,
        c: 4,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 0.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 0.0 },
        color: 0xFF00FFFF,
    },
    Face {
        a: 1,
        b: 4,
        c: 2,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 1.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 1.0 },
        color: 0xFF00FFFF,
    },
    // bottom
    Face {
        a: 5,
        b: 7,
        c: 0,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 0.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 0.0 },
        color: 0xFFFFFF00,
    },
    Face {
        a: 5,
        b: 0,
        c: 3,
        a_uv: Tex2 { u: 0.0, v: 1.0 },
        b_uv: Tex2 { u: 1.0, v: 0.0 },
        c_uv: Tex2 { u: 1.0, v: 1.0 },
        color: 0xFFFFFF00,
    },
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
    let mut texels: Vec<Tex2> = vec![];
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
        } else if line.starts_with("vt ") {
            let mut line_iter = line.split(" ");

            // skip 'vt'
            line_iter.next();

            let mut elements: [f32; 2] = [0.0; 2];
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

            // we use uv coordinates from the bottom left
            texels.push(Tex2 {
                u: elements[0],
                v: 1.0 - elements[1],
            });
        } else if line.starts_with("f ") {
            // face
            let mut line_iter = line.split(" ");
            line_iter.next();
            // only grabbing the first elements right now, which correspond to
            // -- the vertex index
            let mut vertex_indices: [usize; 3] = [0; 3];
            let mut texel_indices: [usize; 3] = [0; 3];
            for (vertex_index, texel_index) in
                vertex_indices.iter_mut().zip(texel_indices.iter_mut())
            {
                let face_text_data: Vec<&str> = match line_iter.next() {
                    Some(text) => text.split("/").collect(),
                    None => return Err(LoadMeshError::FormatError),
                };
                *vertex_index = match face_text_data[0].parse() {
                    Ok(value) => value,
                    Err(_) => return Err(LoadMeshError::FormatError),
                };
                *texel_index = match face_text_data[1].parse() {
                    Ok(value) => value,
                    Err(_) => return Err(LoadMeshError::FormatError),
                };
            }
            faces.push(Face {
                a: vertex_indices[0] - 1,
                b: vertex_indices[1] - 1,
                c: vertex_indices[2] - 1,
                a_uv: texels[texel_indices[0] - 1],
                b_uv: texels[texel_indices[1] - 1],
                c_uv: texels[texel_indices[2] - 1],
                color: 0xFFFFFFFF,
                ..Default::default()
            })
        }
    }

    Ok(Mesh {
        vertices,
        faces,
        translation: Vec3 {
            ..Default::default()
        },
        rotation: Vec3 {
            ..Default::default()
        },
        scale: Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    })
}

pub fn load_cube_mesh() -> Mesh {
    Mesh {
        vertices: CUBE_VERTICES.to_vec(),
        faces: CUBE_FACES.to_vec(),
        translation: Vec3 {
            ..Default::default()
        },
        rotation: Vec3 {
            ..Default::default()
        },
        scale: Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    }
}
