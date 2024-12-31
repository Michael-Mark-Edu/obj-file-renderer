use std::{fs::File, io::Read};

pub struct Material {
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
}

pub fn get_material(filepath: &str, material_name: &str) -> Option<Material> {
    let mut file = File::open(filepath).expect(format!("Couldn't find file {filepath}").as_str());
    let mut obj = String::default();
    let _ = file.read_to_string(&mut obj);
    drop(file);

    let mut lines = obj.lines();
    let mut found_material = false;
    while let Some(line) = lines.next() {
        if line == format!("newmtl {material_name}") {
            found_material = true;
            break;
        }
    }
    if !found_material {
        return None;
    }

    let mut ambient = [0.0, 0.0, 0.0];
    let mut diffuse = [1.0, 1.0, 1.0];
    let mut specular = [1.0, 1.0, 1.0];
    let mut shininess = 0.0;
    'line_iter: for line in lines {
        let mut split = line.split(" ");
        match split.next() {
            Some("newmtl") => break 'line_iter,
            Some("Ka") => {
                ambient = [
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                ]
            }
            Some("Kd") => {
                diffuse = [
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                ]
            }
            Some("Ks") => {
                specular = [
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                ]
            }
            Some("Ns") => shininess = split.next().unwrap().parse::<f32>().unwrap(),
            _ => {}
        }
    }

    Some(Material {
        ambient,
        diffuse,
        specular,
        shininess,
    })
}
