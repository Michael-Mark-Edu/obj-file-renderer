use fermium::prelude::*;
use gl33::*;
use image::ImageReader;
use std::{f32::consts::PI, fs::File, io::Read};

mod vertex_gen;
use vertex_gen::*;

mod material_gen;

extern crate nalgebra_glm as glm;

unsafe fn is_key_down(keystate: *const u8, code: SDL_Scancode) -> bool {
    *keystate.offset(code.0 as isize) != 0
}

fn main() {
    unsafe {
        // Initialize SDL/Fermium
        assert_eq!(
            SDL_Init(SDL_INIT_EVERYTHING),
            0,
            "SDL/Fermium could not be inited"
        );

        // Create a window
        let win = SDL_CreateWindow(
            "Window Title\0".as_ptr().cast(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            800,
            600,
            (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI | SDL_WINDOW_RESIZABLE).0,
        );
        assert!(!win.is_null(), "Window was null");

        // Set GL attributes
        assert_eq!(SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3), 0);
        assert_eq!(SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3), 0);
        assert_eq!(
            SDL_GL_SetAttribute(
                SDL_GL_CONTEXT_PROFILE_MASK,
                SDL_GL_CONTEXT_PROFILE_CORE.0 as _,
            ),
            0
        );
        assert_eq!(
            SDL_GL_SetAttribute(
                SDL_GL_CONTEXT_FLAGS,
                SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG.0 as _,
            ),
            0
        );
        assert_eq!(SDL_GL_SetAttribute(SDL_GL_MULTISAMPLEBUFFERS, 1), 0);
        assert_eq!(SDL_GL_SetAttribute(SDL_GL_MULTISAMPLESAMPLES, 16), 0);

        let ctx = SDL_GL_CreateContext(win);
        assert!(!ctx.0.is_null(), "GL context was null");

        let gl = GlFns::load_from(&|p| SDL_GL_GetProcAddress(p.cast()))
            .expect("Could not load from proc address");

        // Configuration flags
        gl.Enable(GL_MULTISAMPLE);
        gl.Enable(GL_CULL_FACE);
        gl.Enable(GL_DEPTH_TEST);
        gl.DepthFunc(GL_LEQUAL);

        // Buffer object initialization
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0, "VAO was null");
        gl.BindVertexArray(vao);

        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
        gl.VertexAttribPointer(0, 3, GL_FLOAT, 0, 8 * 4, 0 as *const _);
        gl.VertexAttribPointer(1, 2, GL_FLOAT, 0, 8 * 4, (3 * 4) as *const _);
        gl.VertexAttribPointer(2, 3, GL_FLOAT, 0, 8 * 4, (5 * 4) as *const _);
        gl.EnableVertexAttribArray(0);
        gl.EnableVertexAttribArray(1);
        gl.EnableVertexAttribArray(2);

        // Compile and source vertex shader at shader/vert.glsl
        let vertex_shader = gl.CreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0, "Vertex shader was null");

        let mut vertex_file = File::open("shader/vert.glsl").expect("Couldn't open vert.glsl");
        let mut vertex_source = String::default();
        let vertex_length = vertex_file
            .read_to_string(&mut vertex_source)
            .expect("Read to string failed");
        drop(vertex_file);

        gl.ShaderSource(
            vertex_shader,
            1,
            &(vertex_source.as_bytes().as_ptr().cast()),
            &(vertex_length as i32),
        );

        gl.CompileShader(vertex_shader);

        let mut success = 0;
        gl.GetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl.GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }

        // Compile and source frag shader at shader/vert.glsl
        let fragment_shader = gl.CreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0, "Fragment shader was null");

        let mut fragment_file = File::open("shader/frag.glsl").expect("Couldn't open frag.glsl");
        let mut fragment_source = String::default();
        let fragment_length = fragment_file
            .read_to_string(&mut fragment_source)
            .expect("Read to string failed");
        drop(fragment_file);

        gl.ShaderSource(
            fragment_shader,
            1,
            &(fragment_source.as_bytes().as_ptr().cast()),
            &(fragment_length as i32),
        );

        gl.CompileShader(fragment_shader);

        let mut success = 0;
        gl.GetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl.GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        // Attach shaders to shader program
        let shader_program = gl.CreateProgram();
        gl.AttachShader(shader_program, vertex_shader);
        gl.AttachShader(shader_program, fragment_shader);
        gl.LinkProgram(shader_program);

        let mut success = 0;
        gl.GetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl.GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        // Clean up shaders
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);

        // Set program
        gl.UseProgram(shader_program);

        // Set clear color
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);

        // Load texture
        let diffuse_img = ImageReader::open("texture/container.png")
            .expect("Couldn't find container")
            .decode()
            .unwrap();
        let diffuse_img = diffuse_img.flipv();
        let diffuse_bytes = diffuse_img.as_bytes();

        let mut diffuse: u32 = 0;
        gl.GenTextures(1, &mut diffuse);
        gl.BindTexture(GL_TEXTURE_2D, diffuse);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_BORDER.0 as _);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_BORDER.0 as _);
        gl.TexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MIN_FILTER,
            GL_LINEAR_MIPMAP_LINEAR.0 as _,
        );
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as _);
        gl.TexImage2D(
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
        gl.GenerateMipmap(GL_TEXTURE_2D);

        // Load specular map
        let specular_img = ImageReader::open("texture/container_specular.png")
            .expect("Couldn't find container")
            .decode()
            .unwrap();
        let specular_img = specular_img.flipv();
        let specular_bytes = specular_img.as_bytes();

        let mut specular: u32 = 0;
        gl.GenTextures(1, &mut specular);
        gl.BindTexture(GL_TEXTURE_2D, specular);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_BORDER.0 as _);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_BORDER.0 as _);
        gl.TexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MIN_FILTER,
            GL_LINEAR_MIPMAP_LINEAR.0 as _,
        );
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as _);
        gl.TexImage2D(
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
        gl.GenerateMipmap(GL_TEXTURE_2D);

        // Get mesh and material data
        let (mesh, material) = get_mesh_data("mesh/cube.obj");

        // Get uniform locations
        let diffuse_uniform = gl.GetUniformLocation(shader_program, "diffuse_map\0".as_ptr());
        assert_ne!(
            diffuse_uniform, -1,
            "Uniform \"diffuse_map\" does not exist"
        );
        gl.Uniform1i(diffuse_uniform, 0);

        let specular_uniform = gl.GetUniformLocation(shader_program, "specular_map\0".as_ptr());
        assert_ne!(
            specular_uniform, -1,
            "Uniform \"specular_map\" does not exist"
        );
        gl.Uniform1i(specular_uniform, 1);

        let transform_uniform = gl.GetUniformLocation(shader_program, "transform\0".as_ptr());
        assert_ne!(
            transform_uniform, -1,
            "Uniform \"transform\" does not exist"
        );

        let camera_pos_uniform = gl.GetUniformLocation(shader_program, "camera_pos\0".as_ptr());
        assert_ne!(
            camera_pos_uniform, -1,
            "Uniform \"camera_pos\" does not exist"
        );

        let ambient_uniform = gl.GetUniformLocation(shader_program, "material.ambient\0".as_ptr());
        assert_ne!(
            ambient_uniform, -1,
            "Uniform \"material.ambient\" does not exist"
        );

        let diffuse_uniform = gl.GetUniformLocation(shader_program, "material.diffuse\0".as_ptr());
        assert_ne!(
            diffuse_uniform, -1,
            "Uniform \"material.diffuse\" does not exist"
        );

        let specular_uniform =
            gl.GetUniformLocation(shader_program, "material.specular\0".as_ptr());
        assert_ne!(
            specular_uniform, -1,
            "Uniform \"material.specular\" does not exist"
        );

        let shininess_uniform =
            gl.GetUniformLocation(shader_program, "material.shininess\0".as_ptr());
        assert_ne!(
            shininess_uniform, -1,
            "Uniform \"material.shininess\" does not exist"
        );

        if let Some(material) = material {
            gl.Uniform3f(
                ambient_uniform,
                material.ambient[0],
                material.ambient[1],
                material.ambient[2],
            );
            gl.Uniform3f(
                diffuse_uniform,
                material.diffuse[0],
                material.diffuse[1],
                material.diffuse[2],
            );
            gl.Uniform3f(
                specular_uniform,
                material.specular[0],
                material.specular[1],
                material.specular[2],
            );
            gl.Uniform1f(shininess_uniform, material.shininess);
        } else {
            println!("Couldn't find a material; resorting to sensible defaults");
            gl.Uniform3f(ambient_uniform, 0.1, 0.1, 0.1);
            gl.Uniform3f(diffuse_uniform, 1.0, 1.0, 1.0);
            gl.Uniform3f(specular_uniform, 0.5, 0.5, 0.5);
            gl.Uniform1f(shininess_uniform, 32.0);
        }

        // Cross-frame state variables
        let mut azimuth = PI / 4.0;
        let mut elevation = PI / 4.0;
        let mut distance = 3.0;

        'main_loop: loop {
            let mut event = SDL_Event::default();
            while SDL_PollEvent(&mut event) != 0 {
                match event.type_ {
                    SDL_QUIT => break 'main_loop,
                    _ => {}
                }
            }
            let keystate = SDL_GetKeyboardState(std::ptr::null_mut());
            // SDL keyboard input is weird so we have to do some bit wrangling here
            if is_key_down(keystate, SDL_SCANCODE_A) {
                azimuth += 1.0 / 60.0;
            }
            if is_key_down(keystate, SDL_SCANCODE_D) {
                azimuth -= 1.0 / 60.0;
            }
            if is_key_down(keystate, SDL_SCANCODE_W) {
                elevation += 1.0 / 60.0;
            }
            if is_key_down(keystate, SDL_SCANCODE_S) {
                elevation -= 1.0 / 60.0;
            }
            if is_key_down(keystate, SDL_SCANCODE_SPACE) {
                distance -= 1.0 / 60.0;
            }
            if is_key_down(keystate, SDL_SCANCODE_LALT) {
                distance += 1.0 / 60.0;
            }

            // Limit elevation to prevent teleportation effects
            elevation = f32::clamp(elevation, -PI / 2.1, PI / 2.1);

            let camera_pos = glm::vec3(
                distance * f32::cos(azimuth) * f32::cos(elevation),
                distance * f32::sin(elevation),
                distance * f32::sin(azimuth) * f32::cos(elevation),
            );
            gl.Uniform3f(camera_pos_uniform, camera_pos.x, camera_pos.y, camera_pos.z);

            let (mut window_w, mut window_h) = (0, 0);
            SDL_GetWindowSize(win, &mut window_w, &mut window_h);
            gl.Viewport(0, 0, window_w, window_h);

            let view = glm::look_at(
                &camera_pos,
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 1.0, 0.0),
            );
            let projection = glm::perspective(window_w as f32 / window_h as f32, 1.25, 0.1, 100.0);

            // Although matrix multiplication is faster on the GPU, I do it on the CPU since
            // otherwise I'd have to compute this multiplication for each vertex. Here I only
            // have to do it once for all vertices
            let transform = projection * view;
            gl.UniformMatrix4fv(transform_uniform, 1, 0, transform.data.as_slice().as_ptr());

            gl.ActiveTexture(GL_TEXTURE0);
            gl.BindTexture(GL_TEXTURE_2D, diffuse);
            gl.ActiveTexture(GL_TEXTURE1);
            gl.BindTexture(GL_TEXTURE_2D, specular);

            gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
            gl.BufferData(
                GL_ARRAY_BUFFER,
                (mesh.len() * (4 * 8)) as isize,
                mesh.as_ptr().cast(),
                GL_STATIC_DRAW,
            );

            gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            gl.DrawArrays(GL_TRIANGLES, 0, (3 * mesh.len()) as i32);

            SDL_GL_SwapWindow(win);
        }

        SDL_Quit()
    }
}
