use std::{fs::File, io::Read};

pub type VertexPos = [f32; 3];
pub type VertexTex = [f32; 2];
pub type VertexNormal = [f32; 3];
pub type Index = [u32; 3];
pub type Vertex = [f32; 8];

/// Gets the vertices of the mesh from the indexed data
pub fn get_mesh_data(filepath: &str) -> Vec<Vertex> {
    let mut file = File::open(filepath).expect(format!("Couldn't find file {filepath}").as_str());
    let mut obj = String::default();
    let _ = file.read_to_string(&mut obj);
    drop(file);

    let mut vertex_positions: Vec<VertexPos> = vec![];
    let mut vertex_uvs: Vec<VertexTex> = vec![];
    let mut vertex_normals: Vec<VertexNormal> = vec![];
    let mut face_indices: Vec<Index> = vec![];

    let lines = obj.lines().map(|line| line.trim());
    for line in lines {
        let mut split = line.split(" ");
        match split.next() {
            Some("v") => {
                vertex_positions.push([
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                ]);
            }
            Some("vt") => {
                vertex_uvs.push([
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                ]);
            }
            Some("vn") => {
                vertex_normals.push([
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                ]);
            }
            Some("f") => {
                // Assume face data is clockwise
                let mut indices = vec![];
                for triplet in split {
                    let slashed = triplet.split("/");
                    let mut triple = [0, 0, 0];
                    let mut i = 0;
                    for index in slashed {
                        if let Ok(parsed) = index.parse::<u32>() {
                            triple[i] = parsed;
                        }
                        i += 1;
                    }
                    indices.push(triple);
                }
                match indices.len() {
                    x if x < 3 => panic!(),
                    3 => {
                        for index in indices {
                            face_indices.push(index);
                        }
                    }
                    4 => {
                        face_indices.push(indices[0]);
                        face_indices.push(indices[1]);
                        face_indices.push(indices[2]);
                        face_indices.push(indices[2]);
                        face_indices.push(indices[3]);
                        face_indices.push(indices[0]);
                    }
                    x if x > 4 => {
                        todo!();
                    }
                    _ => unreachable!(),
                }
            }
            _ => {}
        }
    }

    let mut vertices: Vec<Vertex> = vec![];
    for index in face_indices {
        let pos = vertex_positions[index[0] as usize - 1];
        let tex = vertex_uvs[index[1] as usize - 1];
        let normal = vertex_normals[index[2] as usize - 1];

        vertices.push([
            pos[0], pos[1], pos[2], tex[0], tex[1], normal[0], normal[1], normal[2],
        ]);
    }
    vertices
}
