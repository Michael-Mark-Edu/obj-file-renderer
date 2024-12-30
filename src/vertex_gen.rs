// In the future, we should read this data from .obj files instead of having static arrays

use std::{fs::File, io::Read};

// The position of each vertex
pub type VertexPos = [f32; 3];
pub const VERTEX_POS_COUNT: usize = 8;
pub const VERTEX_POS_COORDS: [VertexPos; VERTEX_POS_COUNT] = [
    // Outer cube
    [1.000000, -1.000000, -1.000000],
    [1.000000, -1.000000, 1.000000],
    [-1.000000, -1.000000, 1.000000],
    [-1.000000, -1.000000, -1.000000],
    [1.000000, 1.000000, -1.000000],
    [1.000000, 1.000000, 1.000000],
    [-1.000000, 1.000000, 1.000000],
    [-1.000000, 1.000000, -1.000000],
];

// The texture coordinates to apply to vertices
pub type VertexTex = [f32; 2];
pub const VERTEX_TEX_COUNT: usize = 14;
pub const VERTEX_TEX_COORDS: [VertexTex; VERTEX_TEX_COUNT] = [
    [1.000000, 0.333333],
    [1.000000, 0.666667],
    [0.666667, 0.666667],
    [0.666667, 0.333333],
    [0.666667, 0.000000],
    [0.000000, 0.333333],
    [0.000000, 0.000000],
    [0.333333, 0.000000],
    [0.333333, 1.000000],
    [0.000000, 1.000000],
    [0.000000, 0.666667],
    [0.333333, 0.333333],
    [0.333333, 0.666667],
    [1.000000, 0.000000],
];

// The normal vectors for vertices to have
pub type VertexNormal = [f32; 3];
pub const VERTEX_NORMAL_COUNT: usize = 6;
pub const VERTEX_NORMAL_COORDS: [VertexNormal; VERTEX_NORMAL_COUNT] = [
    [0.000000, -1.000000, 0.000000],
    [0.000000, 1.000000, 0.000000],
    [1.000000, 0.000000, 0.000000],
    [-0.000000, 0.000000, 1.000000],
    [-1.000000, -0.000000, -0.000000],
    [0.000000, 0.000000, -1.000000],
];

// The indices are formatted as [position, texture, normal]
// Each three forms a triangle
pub type Index = [u32; 3];
pub const INDEX_COUNT: usize = 36;
pub const INDICES: [Index; INDEX_COUNT] = [
    [2, 1, 1],
    [3, 2, 1],
    [4, 3, 1],
    [8, 1, 2],
    [7, 4, 2],
    [6, 5, 2],
    [5, 6, 3],
    [6, 7, 3],
    [2, 8, 3],
    [6, 8, 4],
    [7, 5, 4],
    [3, 4, 4],
    [3, 9, 5],
    [7, 10, 5],
    [8, 11, 5],
    [1, 12, 6],
    [4, 13, 6],
    [8, 11, 6],
    [1, 4, 1],
    [2, 1, 1],
    [4, 3, 1],
    [5, 14, 2],
    [8, 1, 2],
    [6, 5, 2],
    [1, 12, 3],
    [5, 6, 3],
    [2, 8, 3],
    [2, 12, 4],
    [6, 8, 4],
    [3, 4, 4],
    [4, 13, 5],
    [3, 9, 5],
    [8, 11, 5],
    [5, 6, 6],
    [1, 12, 6],
    [8, 11, 6],
];

pub type Vertex = [f32; 8];

/// Gets the vertices of the mesh from the indexed data
pub fn get_mesh_data() -> Vec<Vertex> {
    let mut file = File::open("mesh/cube.obj").expect("Couldn't find mesh/cube.obj");
    let mut obj = String::default();
    let _ = file.read_to_string(&mut obj);
    drop(file);
    let lines = obj.lines().map(|line| line.trim());
    for line in lines {
        let mut split = line.split(" ");
        match split.next() {
            Some("v") => {
                println!("Found a vertex");
            }
            Some("vt") => {
                println!("Found a tex");
            }
            Some("vn") => {
                println!("Found a normal");
            }
            _ => {}
        }
    }

    let mut vertices: Vec<Vertex> = vec![];
    for index in INDICES {
        let pos = VERTEX_POS_COORDS.get(index[0] as usize - 1).unwrap();
        let tex = VERTEX_TEX_COORDS.get(index[1] as usize - 1).unwrap();
        let normal = VERTEX_NORMAL_COORDS.get(index[2] as usize - 1).unwrap();

        vertices.push([
            pos[0], pos[1], pos[2], tex[0], tex[1], normal[0], normal[1], normal[2],
        ]);
    }
    vertices
}
