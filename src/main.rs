use fermium::prelude::*;
use gl33::*;
use std::{fs::File, io::Read};

extern crate nalgebra_glm as glm;

type Vertex = [f32; 8];
const VERTEX_COUNT: usize = 16;
const VERTICES: [Vertex; VERTEX_COUNT] = [
    [-0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-0.5, 0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-0.5, -0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, -0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
    // ---
    [-0.25, -0.25, -0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.25, -0.25, -0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-0.25, 0.25, -0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.25, 0.25, -0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-0.25, -0.25, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.25, -0.25, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-0.25, 0.25, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.25, 0.25, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
];

type IndexTriple = [u32; 3];
const INDEX_COUNT: usize = 24;
const INDICES: [IndexTriple; INDEX_COUNT] = [
    //[0, 1, 2],
    //[1, 3, 2],
    //[1, 5, 3],
    //[5, 7, 3],
    //[5, 4, 7],
    //[4, 6, 7],
    //[4, 0, 6],
    //[0, 2, 6],
    //[2, 3, 6],
    //[3, 7, 6],
    //[4, 5, 0],
    //[5, 1, 0],
    // ---
    [2, 1, 0],
    [2, 3, 1],
    [3, 5, 1],
    [3, 7, 5],
    [7, 4, 5],
    [7, 6, 4],
    [6, 0, 4],
    [6, 2, 0],
    [6, 3, 2],
    [6, 7, 3],
    [0, 5, 4],
    [0, 1, 5],
    // ---
    [10, 9, 8],
    [10, 11, 9],
    [11, 13, 9],
    [11, 15, 13],
    [15, 12, 13],
    [15, 14, 12],
    [14, 8, 12],
    [14, 10, 8],
    [14, 11, 10],
    [14, 15, 11],
    [8, 13, 12],
    [8, 9, 13],
];

unsafe fn init_all() -> (*mut SDL_Window, GlFns, u32) {
    assert_eq!(
        SDL_Init(SDL_INIT_EVERYTHING),
        0,
        "SDL/Fermium could not be inited"
    );
    SDL_GL_SetAttribute(SDL_GL_MULTISAMPLEBUFFERS, 1);
    SDL_GL_SetAttribute(SDL_GL_MULTISAMPLESAMPLES, 16);
    let win = SDL_CreateWindow(
        "Window Title\0".as_ptr().cast(),
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        800,
        600,
        (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI | SDL_WINDOW_RESIZABLE).0,
    );
    assert!(!win.is_null(), "Window was null");

    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
    SDL_GL_SetAttribute(
        SDL_GL_CONTEXT_PROFILE_MASK,
        SDL_GL_CONTEXT_PROFILE_CORE.0 as _,
    );
    SDL_GL_SetAttribute(
        SDL_GL_CONTEXT_FLAGS,
        SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG.0 as _,
    );

    let ctx = SDL_GL_CreateContext(win);
    assert!(!ctx.0.is_null(), "GL context was null");

    let gl = GlFns::load_from(&|p| SDL_GL_GetProcAddress(p.cast()))
        .expect("Could not load from proc address");

    gl.Enable(GL_MULTISAMPLE);
    gl.Enable(GL_CULL_FACE);
    gl.Enable(GL_DEPTH_TEST);
    gl.DepthFunc(GL_LEQUAL);

    //gl.FrontFace(GL_CW);

    let mut vao = 0;
    gl.GenVertexArrays(1, &mut vao);
    assert_ne!(vao, 0, "VAO was null");
    gl.BindVertexArray(vao);

    let mut vbo = 0;
    gl.GenBuffers(1, &mut vbo);
    assert_ne!(vbo, 0, "VBO was null");

    let mut ebo = 0;
    gl.GenBuffers(1, &mut ebo);
    assert_ne!(ebo, 0, "EBO was null");

    gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
    gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

    gl.BufferData(
        GL_ARRAY_BUFFER,
        size_of_val(&VERTICES) as isize,
        VERTICES.as_ptr().cast(),
        GL_STATIC_DRAW,
    );
    gl.BufferData(
        GL_ELEMENT_ARRAY_BUFFER,
        size_of_val(&INDICES) as isize,
        INDICES.as_ptr().cast(),
        GL_STATIC_DRAW,
    );

    gl.VertexAttribPointer(0, 3, GL_FLOAT, 0, 32, 0 as *const _);
    gl.VertexAttribPointer(1, 2, GL_FLOAT, 0, 32, 12 as *const _);
    gl.VertexAttribPointer(2, 3, GL_FLOAT, 0, 32, 20 as *const _);
    gl.EnableVertexAttribArray(0);
    gl.EnableVertexAttribArray(1);
    gl.EnableVertexAttribArray(2);

    let vertex_shader = gl.CreateShader(GL_VERTEX_SHADER);
    assert_ne!(vertex_shader, 0, "Vertex shader was null");

    let mut vertex_file = File::open("shader/vert.glsl").expect("Couldn't open vert.glsl");
    let mut vertex_source: String = Default::default();
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

    let fragment_shader = gl.CreateShader(GL_FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0, "Fragment shader was null");

    let mut fragment_file = File::open("shader/frag.glsl").expect("Couldn't open frag.glsl");
    let mut fragment_source: String = Default::default();
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

    gl.DeleteShader(vertex_shader);
    gl.DeleteShader(fragment_shader);

    gl.UseProgram(shader_program);

    (win, gl, shader_program)
}

fn main() {
    unsafe {
        let (win, gl, shader_program) = init_all();

        let mut event = SDL_Event::default();
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);

        let (mut window_w, mut window_h) = (0, 0);

        let view_matrix_pos = gl.GetUniformLocation(shader_program, "view_matrix\0".as_ptr());
        assert_ne!(
            view_matrix_pos, -1,
            "Location \"view_matrix\" does not exist"
        );
        let perspective_matrix_pos =
            gl.GetUniformLocation(shader_program, "perspective_matrix\0".as_ptr());
        assert_ne!(
            perspective_matrix_pos, -1,
            "Location \"perspective_matrix\" does not exist"
        );

        let mut azimuth = 0.0;
        let mut elevation = 0.0;
        let mut distance = 2.0;

        'main_loop: loop {
            while SDL_PollEvent(&mut event) != 0 {
                match event.type_ {
                    SDL_QUIT => break 'main_loop,
                    _ => {}
                }
            }
            let keystate = SDL_GetKeyboardState(std::ptr::null_mut());
            // SDL keyboard input is weird so we have to do some bit wrangling here
            if *keystate.offset(SDL_SCANCODE_A.0 as isize) != 0 {
                azimuth += 1.0 / 60.0;
            }
            if *keystate.offset(SDL_SCANCODE_D.0 as isize) != 0 {
                azimuth -= 1.0 / 60.0;
            }
            if *keystate.offset(SDL_SCANCODE_W.0 as isize) != 0 {
                elevation += 1.0 / 60.0;
            }
            if *keystate.offset(SDL_SCANCODE_S.0 as isize) != 0 {
                elevation -= 1.0 / 60.0;
            }
            if *keystate.offset(SDL_SCANCODE_SPACE.0 as isize) != 0 {
                distance -= 1.0 / 60.0;
            }
            if *keystate.offset(SDL_SCANCODE_LALT.0 as isize) != 0 {
                distance += 1.0 / 60.0;
            }

            let camera_pos = glm::vec3(
                distance * f32::cos(azimuth) * f32::cos(elevation),
                distance * f32::sin(elevation),
                distance * f32::sin(azimuth) * f32::cos(elevation),
            );

            SDL_GetWindowSize(win, &mut window_w, &mut window_h);
            gl.Viewport(0, 0, window_w, window_h);

            let view = glm::look_at(
                &camera_pos,
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 1.0, 0.0),
            );
            let projection = glm::perspective(window_w as f32 / window_h as f32, 1.25, 0.1, 100.0);

            gl.UniformMatrix4fv(view_matrix_pos, 1, 0, view.data.as_slice().as_ptr());
            gl.UniformMatrix4fv(
                perspective_matrix_pos,
                1,
                0,
                projection.data.as_slice().as_ptr(),
            );

            gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            gl.DrawElements(
                GL_TRIANGLES,
                3 * INDEX_COUNT as i32,
                GL_UNSIGNED_INT,
                0 as *const _,
            );

            SDL_GL_SwapWindow(win);
        }

        SDL_Quit()
    }
}
