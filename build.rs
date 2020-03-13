use gl_generator::{Api, Fallbacks, Profile, Registry};
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() {
    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());
    let mut file_gl_and_gles =
        File::create(&Path::new(&dest).join("gl_and_gles_bindings.rs")).unwrap();
    let mut file_gl = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();
    let mut file_gles = File::create(&Path::new(&dest).join("gles_bindings.rs")).unwrap();

    // OpenGL 3.3 bindings
    let gl_extensions = [
        "GL_APPLE_vertex_array_object",
        "GL_ARB_texture_rectangle",
        "GL_EXT_texture_filter_anisotropic",
        "GL_ARB_texture_storage",
        "GL_ARB_transform_feedback2",
        "GL_ARB_internalformat_query",
        "GL_ARB_invalidate_subdata",
    ];
    let gl_reg = Registry::new(
        Api::Gl,
        (3, 3),
        Profile::Compatibility,
        Fallbacks::All,
        gl_extensions,
    );
    gl_reg
        .write_bindings(gl_generator::StructGenerator, &mut file_gl)
        .unwrap();

    // GLES 3.0 bindings
    let gles_extensions = [
        "GL_EXT_disjoint_timer_query",
        "GL_EXT_texture_filter_anisotropic",
        "GL_OES_texture_half_float",
        "GL_OES_vertex_array_object",
    ];
    let gles_reg = Registry::new(
        Api::Gles2,
        (3, 0),
        Profile::Core,
        Fallbacks::All,
        gles_extensions,
    );
    gles_reg
        .write_bindings(gl_generator::StructGenerator, &mut file_gles)
        .unwrap();

    // OpenGL 3.3 + GLES 3.0 bindings. Used to get all enums
    let gl_reg = gl_reg + gles_reg;
    gl_reg
        .write_bindings(gl_generator::StructGenerator, &mut file_gl_and_gles)
        .unwrap();
}
