use fermium::prelude::*;
use gl33::*;
use std::{fs::File, io::Read};

type Vertex = [f32; 3];
const VERTEX_COUNT: usize = 6;
const VERTICES: [Vertex; VERTEX_COUNT] = [
    [-0.5, -0.8, 0.0],
    [0.5, -0.5, 0.0],
    [0.0, 0.0, 0.0],
    [-0.5, 0.8, 0.0],
    [0.5, 0.5, 0.0],
    [0.0, 0.0, 0.0],
];

unsafe fn init_all() -> (*mut SDL_Window, GlFns, u32) {
    assert_eq!(
        SDL_Init(SDL_INIT_EVERYTHING),
        0,
        "SDL/Fermium could not be inited"
    );
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

    let mut vao = 0;
    gl.GenVertexArrays(1, &mut vao);
    assert_ne!(vao, 0, "VAO was null");
    gl.BindVertexArray(vao);

    let mut vbo = 0;
    gl.GenBuffers(1, &mut vbo);
    assert_ne!(vbo, 0, "VBO was null");

    gl.BindBuffer(GL_ARRAY_BUFFER, vbo);

    gl.BufferData(
        GL_ARRAY_BUFFER,
        size_of_val(&VERTICES) as isize,
        VERTICES.as_ptr().cast(),
        GL_STATIC_DRAW,
    );
    gl.VertexAttribPointer(
        0,
        3,
        GL_FLOAT,
        0,
        size_of::<Vertex>().try_into().unwrap(),
        0 as *const _,
    );
    gl.EnableVertexAttribArray(0);

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
        let window_size = gl.GetUniformLocation(shader_program, "window_size\0".as_ptr());
        assert_ne!(window_size, -1, "Location \"window_size\" does not exist");

        let (mut mouse_x, mut mouse_y) = (0, 0);
        let mouse_pos = gl.GetUniformLocation(shader_program, "mouse_pos\0".as_ptr());
        assert_ne!(mouse_pos, -1, "Location \"mouse_pos\" does not exist");

        'main_loop: loop {
            while SDL_PollEvent(&mut event) != 0 {
                match event.type_ {
                    SDL_QUIT => break 'main_loop,
                    _ => {}
                }
            }

            SDL_GetWindowSize(win, &mut window_w, &mut window_h);
            gl.Uniform2i(window_size, window_w, window_h);
            gl.Viewport(0, 0, window_w, window_h);

            SDL_GetMouseState(&mut mouse_x, &mut mouse_y);
            gl.Uniform2i(mouse_pos, mouse_x, mouse_y);

            gl.Clear(GL_COLOR_BUFFER_BIT);
            gl.DrawArrays(GL_TRIANGLES, 0, VERTEX_COUNT as _);

            SDL_GL_SwapWindow(win);
        }

        SDL_Quit()
    }
}
