use gl33::global_loader::*;
use gl33::*;
use image::ImageReader;
use std::{fs::File, io::Read};

pub struct Material {
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
    pub ambient_map: u32,
    pub diffuse_map: u32,
    pub specular_map: u32,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            ambient: [0.1, 0.1, 0.1],
            diffuse: [0.8, 0.8, 0.8],
            specular: [0.5, 0.5, 0.5],
            shininess: 32.0,
            ambient_map: 0,
            diffuse_map: 0,
            specular_map: 0,
        }
    }
}

pub fn get_material(filepath: &str, material_name: &str) -> Material {
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

    let mut material = if found_material {
        let mut ambient = [0.0, 0.0, 0.0];
        let mut diffuse = [1.0, 1.0, 1.0];
        let mut specular = [1.0, 1.0, 1.0];
        let mut shininess = 0.0;
        let mut ambient_map = 0;
        let mut diffuse_map = 0;
        let mut specular_map = 0;
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
                Some("map_Ka") => unsafe {
                    // Load texture
                    let ambient_img =
                        ImageReader::open(format!("texture/{}", split.next().unwrap()))
                            .expect("Couldn't find container")
                            .decode()
                            .unwrap();
                    let ambient_img = ambient_img.flipv();
                    let ambient_bytes = ambient_img.as_bytes();

                    let mut ambient: u32 = 0;
                    glGenTextures(1, &mut ambient);
                    glBindTexture(GL_TEXTURE_2D, ambient);
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_BORDER.0 as _);
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_BORDER.0 as _);
                    glTexParameteri(
                        GL_TEXTURE_2D,
                        GL_TEXTURE_MIN_FILTER,
                        GL_LINEAR_MIPMAP_LINEAR.0 as _,
                    );
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as _);
                    glTexImage2D(
                        GL_TEXTURE_2D,
                        0,
                        GL_RGBA.0 as _,
                        ambient_img.width() as _,
                        ambient_img.height() as _,
                        0,
                        GL_RGBA,
                        GL_UNSIGNED_BYTE,
                        ambient_bytes.as_ptr() as _,
                    );
                    glGenerateMipmap(GL_TEXTURE_2D);
                    ambient_map = ambient;
                },
                Some("map_Kd") => unsafe {
                    // Load texture
                    let diffuse_img =
                        ImageReader::open(format!("texture/{}", split.next().unwrap()))
                            .expect("Couldn't find container")
                            .decode()
                            .unwrap();
                    let diffuse_img = diffuse_img.flipv();
                    let diffuse_bytes = diffuse_img.as_bytes();

                    let mut diffuse: u32 = 0;
                    glGenTextures(1, &mut diffuse);
                    glBindTexture(GL_TEXTURE_2D, diffuse);
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_BORDER.0 as _);
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_BORDER.0 as _);
                    glTexParameteri(
                        GL_TEXTURE_2D,
                        GL_TEXTURE_MIN_FILTER,
                        GL_LINEAR_MIPMAP_LINEAR.0 as _,
                    );
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as _);
                    glTexImage2D(
                        GL_TEXTURE_2D,
                        0,
                        GL_RGBA.0 as _,
                        diffuse_img.width() as _,
                        diffuse_img.height() as _,
                        0,
                        GL_RGBA,
                        GL_UNSIGNED_BYTE,
                        diffuse_bytes.as_ptr() as _,
                    );
                    glGenerateMipmap(GL_TEXTURE_2D);
                    diffuse_map = diffuse;
                },
                Some("map_Ks") => unsafe {
                    // Load specular map
                    let specular_img =
                        ImageReader::open(format!("texture/{}", split.next().unwrap()))
                            .expect("Couldn't find container")
                            .decode()
                            .unwrap();
                    let specular_img = specular_img.flipv();
                    let specular_bytes = specular_img.as_bytes();

                    let mut specular: u32 = 0;
                    glGenTextures(1, &mut specular);
                    glBindTexture(GL_TEXTURE_2D, specular);
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_BORDER.0 as _);
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_BORDER.0 as _);
                    glTexParameteri(
                        GL_TEXTURE_2D,
                        GL_TEXTURE_MIN_FILTER,
                        GL_LINEAR_MIPMAP_LINEAR.0 as _,
                    );
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as _);
                    glTexImage2D(
                        GL_TEXTURE_2D,
                        0,
                        GL_RGBA.0 as _,
                        specular_img.width() as _,
                        specular_img.height() as _,
                        0,
                        GL_RGBA,
                        GL_UNSIGNED_BYTE,
                        specular_bytes.as_ptr() as _,
                    );
                    glGenerateMipmap(GL_TEXTURE_2D);
                    specular_map = specular;
                },
                _ => {}
            }
        }
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
            ambient_map,
            diffuse_map,
            specular_map,
        }
    } else {
        Material::default()
    };

    if material.ambient_map == 0 || material.diffuse_map == 0 || material.specular_map == 0 {
        unsafe {
            let mut white: u32 = 0;
            glGenTextures(1, &mut white);
            glBindTexture(GL_TEXTURE_2D, white);
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGB.0 as _,
                1,
                1,
                0,
                GL_RGB,
                GL_UNSIGNED_BYTE,
                [255u8, 255u8, 255u8].as_ptr() as _,
            );
            if material.ambient_map == 0 {
                material.ambient_map = white;
            }
            if material.diffuse_map == 0 {
                material.diffuse_map = white;
            }
            if material.specular_map == 0 {
                material.specular_map = white;
            }
        }
    }

    material
}
