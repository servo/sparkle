/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod gl {
    pub use self::ffi::types::*;
    pub use self::ffi::*;
    use std::ffi::{CStr, CString};
    use std::mem::size_of;
    use std::os::raw::{c_char, c_int, c_void};
    use std::ptr;
    use std::rc::Rc;
    use std::str;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum GlType {
        Gl,
        Gles,
    }

    pub enum Gl {
        Gl(self::ffi_gl::Gl),
        Gles(self::ffi_gles::Gles2),
    }

    pub enum TexImageSource<'a> {
        Pixels(Option<&'a [u8]>),
        BufferOffset(i64),
    }

    impl Gl {
        pub fn get_type(&self) -> GlType {
            match self {
                Gl::Gl(..) => GlType::Gl,
                Gl::Gles(..) => GlType::Gles,
            }
        }

        pub fn gl_fns(gl: self::ffi_gl::Gl) -> Rc<Gl> {
            Rc::new(Gl::Gl(gl))
        }

        pub fn gles_fns(gl: self::ffi_gles::Gles2) -> Rc<Gl> {
            Rc::new(Gl::Gles(gl))
        }

        pub fn gen_framebuffers(&self, n: GLsizei) -> Vec<GLuint> {
            let mut ids = vec![0 as GLuint; n as usize];
            match self {
                Gl::Gl(gl) => unsafe { gl.GenFramebuffers(n, ids.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GenFramebuffers(n, ids.as_mut_ptr()) },
            }
            ids
        }

        pub fn gen_textures(&self, n: GLsizei) -> Vec<GLuint> {
            let mut ids = vec![0 as GLuint; n as usize];
            match self {
                Gl::Gl(gl) => unsafe { gl.GenTextures(n, ids.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GenTextures(n, ids.as_mut_ptr()) },
            }
            ids
        }

        pub fn gen_renderbuffers(&self, n: GLsizei) -> Vec<GLuint> {
            let mut ids = vec![0 as GLuint; n as usize];
            match self {
                Gl::Gl(gl) => unsafe { gl.GenRenderbuffers(n, ids.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GenRenderbuffers(n, ids.as_mut_ptr()) },
            }
            ids
        }

        pub fn gen_buffers(&self, n: GLsizei) -> Vec<GLuint> {
            let mut ids = vec![0 as GLuint; n as usize];
            match self {
                Gl::Gl(gl) => unsafe { gl.GenBuffers(n, ids.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GenBuffers(n, ids.as_mut_ptr()) },
            }
            ids
        }

        pub fn gen_vertex_arrays(&self, n: GLsizei) -> Vec<GLuint> {
            let mut ids = vec![0 as GLuint; n as usize];
            match self {
                Gl::Gl(gl) => unsafe { gl.GenVertexArrays(n, ids.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GenVertexArrays(n, ids.as_mut_ptr()) },
            }
            ids
        }

        pub fn shader_source(&self, shader: GLuint, strings: &[&[u8]]) {
            let pointers: Vec<*const u8> =
                strings.iter().map(|string| (*string).as_ptr()).collect();
            let lengths: Vec<GLint> = strings.iter().map(|string| string.len() as GLint).collect();
            let len = pointers.len() as GLsizei;
            let pointers = pointers.as_ptr() as *const *const GLchar;
            match self {
                Gl::Gl(gl) => unsafe { gl.ShaderSource(shader, len, pointers, lengths.as_ptr()) },
                Gl::Gles(gles) => unsafe {
                    gles.ShaderSource(shader, len, pointers, lengths.as_ptr())
                },
            }
        }

        pub fn create_program(&self) -> GLuint {
            match self {
                Gl::Gl(gl) => unsafe { gl.CreateProgram() },
                Gl::Gles(gles) => unsafe { gles.CreateProgram() },
            }
        }

        pub fn tex_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            internal_format: GLint,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
            format: GLenum,
            ty: GLenum,
            source: TexImageSource,
        ) {
            let data = match source {
                TexImageSource::Pixels(pixels) => {
                    pixels.map(|d| d.as_ptr()).unwrap_or(ptr::null()) as *const _
                },
                TexImageSource::BufferOffset(offset) => unsafe {
                    let mut buffer = [0];
                    self.get_integer_v(PIXEL_UNPACK_BUFFER_BINDING, &mut buffer);
                    assert!(buffer[0] != 0);
                    offset as *const _
                }
            };
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.TexImage2D(
                        target,
                        level,
                        internal_format,
                        width,
                        height,
                        border,
                        format,
                        ty,
                        data,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.TexImage2D(
                        target,
                        level,
                        internal_format,
                        width,
                        height,
                        border,
                        format,
                        ty,
                        data,
                    )
                },
            }
        }

        pub fn tex_sub_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            ty: GLenum,
            data: &[u8],
        ) {
            let data = data.as_ptr() as *const c_void;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.TexSubImage2D(
                        target, level, xoffset, yoffset, width, height, format, ty, data,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.TexSubImage2D(
                        target, level, xoffset, yoffset, width, height, format, ty, data,
                    )
                },
            }
        }

        pub fn copy_tex_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            internal_format: GLenum,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.CopyTexImage2D(target, level, internal_format, x, y, width, height, border)
                },
                Gl::Gles(gles) => unsafe {
                    gles.CopyTexImage2D(target, level, internal_format, x, y, width, height, border)
                },
            }
        }

        pub fn copy_tex_sub_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height)
                },
                Gl::Gles(gles) => unsafe {
                    gles.CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height)
                },
            }
        }

        pub fn compressed_tex_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            internal_format: GLenum,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
            data: &[u8],
        ) {
            let len = data.len() as GLsizei;
            let data = data.as_ptr() as *const c_void;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.CompressedTexImage2D(
                        target,
                        level,
                        internal_format,
                        width,
                        height,
                        border,
                        len,
                        data,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.CompressedTexImage2D(
                        target,
                        level,
                        internal_format,
                        width,
                        height,
                        border,
                        len,
                        data,
                    )
                },
            }
        }

        pub fn compressed_tex_sub_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            data: &[u8],
        ) {
            let len = data.len() as GLsizei;
            let data = data.as_ptr() as *const c_void;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.CompressedTexSubImage2D(
                        target, level, xoffset, yoffset, width, height, format, len, data,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.CompressedTexSubImage2D(
                        target, level, xoffset, yoffset, width, height, format, len, data,
                    )
                },
            }
        }

        pub fn tex_storage_2d(
            &self,
            target: GLenum,
            levels: GLsizei,
            internal_format: GLenum,
            width: GLsizei,
            height: GLsizei,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.TexStorage2D(target, levels, internal_format, width, height)
                },
                Gl::Gles(gles) => unsafe {
                    gles.TexStorage2D(target, levels, internal_format, width, height)
                },
            }
        }

        pub fn tex_storage_3d(
            &self,
            target: GLenum,
            levels: GLsizei,
            internal_format: GLenum,
            width: GLsizei,
            height: GLsizei,
            depth: GLsizei,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.TexStorage3D(target, levels, internal_format, width, height, depth)
                },
                Gl::Gles(gles) => unsafe {
                    gles.TexStorage3D(target, levels, internal_format, width, height, depth)
                },
            }
        }

        pub fn generate_mipmap(&self, target: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.GenerateMipmap(target) },
                Gl::Gles(gles) => unsafe { gles.GenerateMipmap(target) },
            }
        }

        pub fn active_texture(&self, texture: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ActiveTexture(texture) },
                Gl::Gles(gles) => unsafe { gles.ActiveTexture(texture) },
            }
        }

        pub fn attach_shader(&self, program: GLuint, shader: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.AttachShader(program, shader) },
                Gl::Gles(gles) => unsafe { gles.AttachShader(program, shader) },
            }
        }

        pub fn create_shader(&self, shader_type: GLenum) -> GLuint {
            match self {
                Gl::Gl(gl) => unsafe { gl.CreateShader(shader_type) },
                Gl::Gles(gles) => unsafe { gles.CreateShader(shader_type) },
            }
        }

        pub fn delete_shader(&self, shader: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DeleteShader(shader) },
                Gl::Gles(gles) => unsafe { gles.DeleteShader(shader) },
            }
        }

        pub fn detach_shader(&self, program: GLuint, shader: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DetachShader(program, shader) },
                Gl::Gles(gles) => unsafe { gles.DetachShader(program, shader) },
            }
        }

        pub fn bind_buffer(&self, target: GLenum, buffer: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BindBuffer(target, buffer) },
                Gl::Gles(gles) => unsafe { gles.BindBuffer(target, buffer) },
            }
        }

        pub fn delete_buffers(&self, buffers: &[GLuint]) {
            let len = buffers.len() as GLsizei;
            let buffers = buffers.as_ptr();
            match self {
                Gl::Gl(gl) => unsafe { gl.DeleteBuffers(len, buffers) },
                Gl::Gles(gles) => unsafe { gles.DeleteBuffers(len, buffers) },
            }
        }

        pub fn copy_buffer_sub_data(
            &self,
            read_target: u32,
            write_target: u32,
            read_offset: isize,
            write_offset: isize,
            size: isize,
        ) {
            match self {
                Gl::Gl(gl) => unsafe { gl.CopyBufferSubData(read_target, write_target, read_offset, write_offset, size) },
                Gl::Gles(gles) => unsafe { gles.CopyBufferSubData(read_target, write_target, read_offset, write_offset, size) },
            }
        }

        pub fn map_buffer_range(
            &self,
            target: GLenum,
            offset: GLintptr,
            length: GLsizeiptr,
            access: GLbitfield,
        ) -> *mut c_void {
            match self {
                Gl::Gl(gl) => unsafe { gl.MapBufferRange(target, offset, length, access) },
                Gl::Gles(gles) => unsafe { gles.MapBufferRange(target, offset, length, access) },
            }
        }

        pub fn unmap_buffer(&self, target: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.UnmapBuffer(target); },
                Gl::Gles(gles) => unsafe { gles.UnmapBuffer(target); },
            }
        }

        pub fn link_program(&self, program: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.LinkProgram(program) },
                Gl::Gles(gles) => unsafe { gles.LinkProgram(program) },
            }
        }

        pub fn validate_program(&self, program: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ValidateProgram(program) },
                Gl::Gles(gles) => unsafe { gles.ValidateProgram(program) },
            }
        }

        pub fn delete_program(&self, program: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DeleteProgram(program) },
                Gl::Gles(gles) => unsafe { gles.DeleteProgram(program) },
            }
        }

        pub fn delete_vertex_arrays(&self, vertex_arrays: &[GLuint]) {
            let len = vertex_arrays.len() as GLsizei;
            match self {
                Gl::Gl(gl) => unsafe { gl.DeleteVertexArrays(len, vertex_arrays.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.DeleteVertexArrays(len, vertex_arrays.as_ptr()) },
            }
        }

        pub fn bind_vertex_array(&self, vao: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BindVertexArray(vao) },
                Gl::Gles(gles) => unsafe { gles.BindVertexArray(vao) },
            }
        }

        pub fn enable_vertex_attrib_array(&self, index: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.EnableVertexAttribArray(index) },
                Gl::Gles(gles) => unsafe { gles.EnableVertexAttribArray(index) },
            }
        }

        pub fn disable_vertex_attrib_array(&self, index: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DisableVertexAttribArray(index) },
                Gl::Gles(gles) => unsafe { gles.DisableVertexAttribArray(index) },
            }
        }

        pub fn vertex_attrib_4f(
            &self,
            index: GLuint,
            x: GLfloat,
            y: GLfloat,
            z: GLfloat,
            w: GLfloat,
        ) {
            match self {
                Gl::Gl(gl) => unsafe { gl.VertexAttrib4f(index, x, y, z, w) },
                Gl::Gles(gles) => unsafe { gles.VertexAttrib4f(index, x, y, z, w) },
            }
        }

        pub fn vertex_attrib_4i(
            &self,
            index: GLuint,
            x: GLint,
            y: GLint,
            z: GLint,
            w: GLint,
        ) {
            match self {
                Gl::Gl(gl) => unsafe { gl.VertexAttribI4i(index, x, y, z, w) },
                Gl::Gles(gles) => unsafe { gles.VertexAttribI4i(index, x, y, z, w) },
            }
        }

        pub fn vertex_attrib_4ui(
            &self,
            index: GLuint,
            x: GLuint,
            y: GLuint,
            z: GLuint,
            w: GLuint,
        ) {
            match self {
                Gl::Gl(gl) => unsafe { gl.VertexAttribI4ui(index, x, y, z, w) },
                Gl::Gles(gles) => unsafe { gles.VertexAttribI4ui(index, x, y, z, w) },
            }
        }

        pub fn vertex_attrib_pointer_f32(
            &self,
            index: GLuint,
            size: GLint,
            normalized: bool,
            stride: GLsizei,
            offset: GLuint,
        ) {
            self.vertex_attrib_pointer(index, size, ffi::FLOAT, normalized, stride, offset)
        }

        pub fn vertex_attrib_pointer(
            &self,
            index: GLuint,
            size: GLint,
            type_: GLenum,
            normalized: bool,
            stride: GLsizei,
            offset: GLuint,
        ) {
            let normalized = normalized as GLboolean;
            let offset = offset as *const GLvoid;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.VertexAttribPointer(index, size, type_, normalized, stride, offset)
                },
                Gl::Gles(gles) => unsafe {
                    gles.VertexAttribPointer(index, size, type_, normalized, stride, offset)
                },
            }
        }

        pub fn vertex_attrib_divisor(&self, index: GLuint, divisor: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.VertexAttribDivisor(index, divisor) },
                Gl::Gles(gles) => unsafe { gles.VertexAttribDivisor(index, divisor) },
            }
        }

        pub fn bind_attrib_location(&self, program: GLuint, index: GLuint, name: &str) {
            let c_string = CString::new(name).unwrap();
            match self {
                Gl::Gl(gl) => unsafe { gl.BindAttribLocation(program, index, c_string.as_ptr()) },
                Gl::Gles(gles) => unsafe {
                    gles.BindAttribLocation(program, index, c_string.as_ptr())
                },
            }
        }

        pub unsafe fn get_uniform_iv(
            &self,
            program: GLuint,
            location: GLint,
            result: &mut [GLint],
        ) {
            match self {
                Gl::Gl(gl) => gl.GetUniformiv(program, location, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetUniformiv(program, location, result.as_mut_ptr()),
            }
        }

        pub unsafe fn get_uniform_uiv(
            &self,
            program: GLuint,
            location: GLint,
            result: &mut [GLuint],
        ) {
            match self {
                Gl::Gl(gl) => gl.GetUniformuiv(program, location, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetUniformuiv(program, location, result.as_mut_ptr()),
            }
        }

        pub unsafe fn get_uniform_fv(
            &self,
            program: GLuint,
            location: GLint,
            result: &mut [GLfloat],
        ) {
            match self {
                Gl::Gl(gl) => gl.GetUniformfv(program, location, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetUniformfv(program, location, result.as_mut_ptr()),
            }
        }

        pub fn hint(&self, param_name: GLenum, param_val: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Hint(param_name, param_val) },
                Gl::Gles(gles) => unsafe { gles.Hint(param_name, param_val) },
            }
        }

        pub fn blend_color(&self, r: f32, g: f32, b: f32, a: f32) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BlendColor(r, g, b, a) },
                Gl::Gles(gles) => unsafe { gles.BlendColor(r, g, b, a) },
            }
        }

        pub fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BlendFunc(sfactor, dfactor) },
                Gl::Gles(gles) => unsafe { gles.BlendFunc(sfactor, dfactor) },
            }
        }

        pub fn blend_func_separate(
            &self,
            src_rgb: GLenum,
            dest_rgb: GLenum,
            src_alpha: GLenum,
            dest_alpha: GLenum,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.BlendFuncSeparate(src_rgb, dest_rgb, src_alpha, dest_alpha)
                },
                Gl::Gles(gles) => unsafe {
                    gles.BlendFuncSeparate(src_rgb, dest_rgb, src_alpha, dest_alpha)
                },
            }
        }

        pub fn blend_equation(&self, mode: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BlendEquation(mode) },
                Gl::Gles(gles) => unsafe { gles.BlendEquation(mode) },
            }
        }

        pub fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BlendEquationSeparate(mode_rgb, mode_alpha) },
                Gl::Gles(gles) => unsafe { gles.BlendEquationSeparate(mode_rgb, mode_alpha) },
            }
        }

        pub fn color_mask(&self, r: bool, g: bool, b: bool, a: bool) {
            let (r, g, b, a) = (
                r as GLboolean,
                g as GLboolean,
                b as GLboolean,
                a as GLboolean,
            );
            match self {
                Gl::Gl(gl) => unsafe { gl.ColorMask(r, g, b, a) },
                Gl::Gles(gles) => unsafe { gles.ColorMask(r, g, b, a) },
            }
        }

        pub fn cull_face(&self, mode: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.CullFace(mode) },
                Gl::Gles(gles) => unsafe { gles.CullFace(mode) },
            }
        }

        pub fn front_face(&self, mode: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.FrontFace(mode) },
                Gl::Gles(gles) => unsafe { gles.FrontFace(mode) },
            }
        }

        pub fn depth_func(&self, func: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DepthFunc(func) },
                Gl::Gles(gles) => unsafe { gles.DepthFunc(func) },
            }
        }

        pub fn depth_mask(&self, flag: bool) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DepthMask(flag as GLboolean) },
                Gl::Gles(gles) => unsafe { gles.DepthMask(flag as GLboolean) },
            }
        }

        pub fn depth_range(&self, near: f64, far: f64) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DepthRange(near, far) },
                Gl::Gles(gles) => unsafe { gles.DepthRangef(near as f32, far as f32) },
            }
        }

        pub fn line_width(&self, width: GLfloat) {
            match self {
                Gl::Gl(gl) => unsafe { gl.LineWidth(width) },
                Gl::Gles(gles) => unsafe { gles.LineWidth(width) },
            }
        }

        pub fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {
            match self {
                Gl::Gl(gl) => unsafe { gl.PolygonOffset(factor, units) },
                Gl::Gles(gles) => unsafe { gles.PolygonOffset(factor, units) },
            }
        }

        pub fn sample_coverage(&self, value: GLclampf, invert: bool) {
            match self {
                Gl::Gl(gl) => unsafe { gl.SampleCoverage(value, invert as GLboolean) },
                Gl::Gles(gles) => unsafe { gles.SampleCoverage(value, invert as GLboolean) },
            }
        }

        pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ClearColor(r, g, b, a) },
                Gl::Gles(gles) => unsafe { gles.ClearColor(r, g, b, a) },
            }
        }

        pub fn clear_depth(&self, depth: f64) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ClearDepth(depth) },
                Gl::Gles(gles) => unsafe { gles.ClearDepthf(depth as f32) },
            }
        }

        pub fn clear_stencil(&self, s: GLint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ClearStencil(s) },
                Gl::Gles(gles) => unsafe { gles.ClearStencil(s) },
            }
        }

        pub fn clear(&self, buffer_mask: GLbitfield) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Clear(buffer_mask) },
                Gl::Gles(gles) => unsafe { gles.Clear(buffer_mask) },
            }
        }

        pub fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Scissor(x, y, width, height) },
                Gl::Gles(gles) => unsafe { gles.Scissor(x, y, width, height) },
            }
        }

        pub fn stencil_op(&self, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.StencilOp(sfail, dpfail, dppass) },
                Gl::Gles(gles) => unsafe { gles.StencilOp(sfail, dpfail, dppass) },
            }
        }

        pub fn stencil_op_separate(
            &self,
            face: GLenum,
            sfail: GLenum,
            dpfail: GLenum,
            dppass: GLenum,
        ) {
            match self {
                Gl::Gl(gl) => unsafe { gl.StencilOpSeparate(face, sfail, dpfail, dppass) },
                Gl::Gles(gles) => unsafe { gles.StencilOpSeparate(face, sfail, dpfail, dppass) },
            }
        }

        pub fn stencil_mask(&self, mask: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.StencilMask(mask) },
                Gl::Gles(gles) => unsafe { gles.StencilMask(mask) },
            }
        }

        pub fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.StencilMaskSeparate(face, mask) },
                Gl::Gles(gles) => unsafe { gles.StencilMaskSeparate(face, mask) },
            }
        }

        pub fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.StencilFunc(func, ref_, mask) },
                Gl::Gles(gles) => unsafe { gles.StencilFunc(func, ref_, mask) },
            }
        }

        pub fn stencil_func_separate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.StencilFuncSeparate(face, func, ref_, mask) },
                Gl::Gles(gles) => unsafe { gles.StencilFuncSeparate(face, func, ref_, mask) },
            }
        }

        pub fn is_enabled(&self, cap: GLenum) -> bool {
            TRUE == match self {
                Gl::Gl(gl) => unsafe { gl.IsEnabled(cap) },
                Gl::Gles(gles) => unsafe { gles.IsEnabled(cap) },
            }
        }

        pub fn enable(&self, cap: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Enable(cap) },
                Gl::Gles(gles) => unsafe { gles.Enable(cap) },
            }
        }

        pub fn disable(&self, cap: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Disable(cap) },
                Gl::Gles(gles) => unsafe { gles.Disable(cap) },
            }
        }

        pub fn finish(&self) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Finish() },
                Gl::Gles(gles) => unsafe { gles.Finish() },
            }
        }

        pub fn flush(&self) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Flush() },
                Gl::Gles(gles) => unsafe { gles.Flush() },
            }
        }

        pub fn get_string(&self, which: GLenum) -> String {
            let llstr = match self {
                Gl::Gl(gl) => unsafe { gl.GetString(which) },
                Gl::Gles(gles) => unsafe { gles.GetString(which) },
            };
            if !llstr.is_null() {
                unsafe {
                    str::from_utf8_unchecked(CStr::from_ptr(llstr as *const c_char).to_bytes())
                        .to_string()
                }
            } else {
                "".to_string()
            }
        }

        pub fn get_string_i(&self, which: GLenum, index: GLuint) -> String {
            let llstr = match self {
                Gl::Gl(gl) => unsafe { gl.GetStringi(which, index) },
                Gl::Gles(gles) => unsafe { gles.GetStringi(which, index) },
            };
            if !llstr.is_null() {
                unsafe {
                    str::from_utf8_unchecked(CStr::from_ptr(llstr as *const c_char).to_bytes())
                        .to_string()
                }
            } else {
                "".to_string()
            }
        }

        pub unsafe fn get_shader_iv(&self, shader: GLuint, pname: GLenum, result: &mut [GLint]) {
            assert!(!result.is_empty());
            match self {
                Gl::Gl(gl) => gl.GetShaderiv(shader, pname, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetShaderiv(shader, pname, result.as_mut_ptr()),
            }
        }

        pub fn get_shader_precision_format(
            &self,
            shader_type: GLuint,
            precision_type: GLuint,
        ) -> (GLint, GLint, GLint) {
            match self {
                Gl::Gl(..) => {
                    // gl.GetShaderPrecisionFormat is not available until OpenGL 4.1.
                    // Fall back to OpenGL standard precision that most desktop hardware support.
                    match precision_type {
                        ffi::LOW_FLOAT | ffi::MEDIUM_FLOAT | ffi::HIGH_FLOAT => {
                            // Fallback to IEEE 754 single precision
                            // Range: from -2^127 to 2^127
                            // Significand precision: 23 bits
                            (127, 127, 23)
                        }
                        ffi::LOW_INT | ffi::MEDIUM_INT | ffi::HIGH_INT => {
                            // Fallback to single precision integer
                            // Range: from -2^24 to 2^24
                            // Precision: For integer formats this value is always 0
                            (24, 24, 0)
                        }
                        _ => (0, 0, 0),
                    }
                }
                Gl::Gles(gles) => {
                    let (mut range, mut precision) = match precision_type {
                        // These values are for a 32-bit twos-complement integer format.
                        ffi::LOW_INT | ffi::MEDIUM_INT | ffi::HIGH_INT => ([31, 30], 0),

                        // These values are for an IEEE single-precision floating-point format.
                        ffi::LOW_FLOAT | ffi::MEDIUM_FLOAT | ffi::HIGH_FLOAT => ([127, 127], 23),

                        _ => unreachable!("invalid precision"),
                    };
                    // This function is sometimes defined even though it's really just
                    // a stub, so we need to set range and precision as if it weren't
                    // defined before calling it. Suppress any error that might occur.
                    unsafe {
                        gles.GetShaderPrecisionFormat(
                            shader_type,
                            precision_type,
                            range.as_mut_ptr(),
                            &mut precision,
                        );
                        let _ = gles.GetError();
                    }
                    (range[0], range[1], precision)
                }
            }
        }

        pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Viewport(x, y, width, height) },
                Gl::Gles(gles) => unsafe { gles.Viewport(x, y, width, height) },
            }
        }

        pub fn get_framebuffer_attachment_parameter_iv(
            &self,
            target: GLenum,
            attachment: GLenum,
            pname: GLenum,
        ) -> GLint {
            let mut result = 0;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetFramebufferAttachmentParameteriv(target, attachment, pname, &mut result)
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetFramebufferAttachmentParameteriv(target, attachment, pname, &mut result)
                },
            }
            result
        }

        pub fn get_internal_format_iv(
            &self,
            target: GLenum,
            internalformat: GLenum,
            pname: GLenum,
            result: &mut [GLint],
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetInternalformativ(target, internalformat, pname, result.len() as _, result.as_mut_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetInternalformativ(target, internalformat, pname, result.len() as _, result.as_mut_ptr())
                },
            }
        }

        pub fn get_renderbuffer_parameter_iv(&self, target: GLenum, pname: GLenum) -> GLint {
            let mut result = 0;
            match self {
                Gl::Gl(gl) => unsafe { gl.GetRenderbufferParameteriv(target, pname, &mut result) },
                Gl::Gles(gles) => unsafe {
                    gles.GetRenderbufferParameteriv(target, pname, &mut result)
                },
            }
            result
        }

        pub fn delete_renderbuffers(&self, buffers: &[GLuint]) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.DeleteRenderbuffers(buffers.len() as GLsizei, buffers.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.DeleteRenderbuffers(buffers.len() as GLsizei, buffers.as_ptr())
                },
            }
        }

        pub fn delete_textures(&self, textures: &[GLuint]) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.DeleteTextures(textures.len() as GLsizei, textures.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.DeleteTextures(textures.len() as GLsizei, textures.as_ptr())
                },
            }
        }

        pub fn delete_framebuffers(&self, framebuffers: &[GLuint]) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.DeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.DeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr())
                },
            }
        }

        pub fn bind_renderbuffer(&self, target: GLenum, renderbuffer: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BindRenderbuffer(target, renderbuffer) },
                Gl::Gles(gles) => unsafe { gles.BindRenderbuffer(target, renderbuffer) },
            }
        }

        pub fn is_renderbuffer(&self, renderbuffer: GLuint) -> bool {
            TRUE == match self {
                Gl::Gl(gl) => unsafe { gl.IsRenderbuffer(renderbuffer) },
                Gl::Gles(gles) => unsafe { gles.IsRenderbuffer(renderbuffer) },
            }
        }

        pub fn bind_framebuffer(&self, target: GLenum, framebuffer: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BindFramebuffer(target, framebuffer) },
                Gl::Gles(gles) => unsafe { gles.BindFramebuffer(target, framebuffer) },
            }
        }

        pub fn is_framebuffer(&self, framebuffer: GLuint) -> bool {
            TRUE == match self {
                Gl::Gl(gl) => unsafe { gl.IsFramebuffer(framebuffer) },
                Gl::Gles(gles) => unsafe { gles.IsFramebuffer(framebuffer) },
            }
        }

        pub fn bind_texture(&self, target: GLenum, texture: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BindTexture(target, texture) },
                Gl::Gles(gles) => unsafe { gles.BindTexture(target, texture) },
            }
        }

        pub fn is_texture(&self, texture: GLuint) -> bool {
            TRUE == match self {
                Gl::Gl(gl) => unsafe { gl.IsTexture(texture) },
                Gl::Gles(gles) => unsafe { gles.IsTexture(texture) },
            }
        }

        pub fn is_shader(&self, shader: GLuint) -> bool {
            TRUE == match self {
                Gl::Gl(gl) => unsafe { gl.IsShader(shader) },
                Gl::Gles(gles) => unsafe { gles.IsShader(shader) },
            }
        }

        pub unsafe fn buffer_data(
            &self,
            target: GLenum,
            size: GLsizeiptr,
            data: *const GLvoid,
            usage: GLenum,
        ) {
            match self {
                Gl::Gl(gl) => gl.BufferData(target, size, data, usage),
                Gl::Gles(gles) => gles.BufferData(target, size, data, usage),
            }
        }

        pub unsafe fn buffer_sub_data(
            &self,
            target: GLenum,
            offset: isize,
            size: GLsizeiptr,
            data: *const GLvoid,
        ) {
            match self {
                Gl::Gl(gl) => gl.BufferSubData(target, offset, size, data),
                Gl::Gles(gles) => gles.BufferSubData(target, offset, size, data),
            }
        }

        pub fn read_buffer(&self, buffer: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ReadBuffer(buffer) },
                Gl::Gles(gles) => unsafe { gles.ReadBuffer(buffer) },
            }
        }

        pub fn draw_buffers(&self, bufs: &[GLenum]) {
            let len = bufs.len() as GLsizei;
            match self {
                Gl::Gl(gl) => unsafe { gl.DrawBuffers(len, bufs.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.DrawBuffers(len, bufs.as_ptr()) },
            }
        }

        pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DrawArrays(mode, first, count) },
                Gl::Gles(gles) => unsafe { gles.DrawArrays(mode, first, count) },
            }
        }

        pub fn draw_arrays_instanced(
            &self,
            mode: GLenum,
            first: GLint,
            count: GLsizei,
            primcount: GLsizei,
        ) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DrawArraysInstanced(mode, first, count, primcount) },
                Gl::Gles(gles) => unsafe {
                    gles.DrawArraysInstanced(mode, first, count, primcount)
                },
            }
        }

        pub fn draw_elements(
            &self,
            mode: GLenum,
            count: GLsizei,
            element_type: GLenum,
            indices_offset: GLuint,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.DrawElements(mode, count, element_type, indices_offset as *const c_void)
                },
                Gl::Gles(gles) => unsafe {
                    gles.DrawElements(mode, count, element_type, indices_offset as *const c_void)
                },
            }
        }

        pub fn draw_elements_instanced(
            &self,
            mode: GLenum,
            count: GLsizei,
            element_type: GLenum,
            indices_offset: GLuint,
            primcount: GLsizei,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.DrawElementsInstanced(
                        mode,
                        count,
                        element_type,
                        indices_offset as *const c_void,
                        primcount,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.DrawElementsInstanced(
                        mode,
                        count,
                        element_type,
                        indices_offset as *const c_void,
                        primcount,
                    )
                },
            }
        }

        pub fn framebuffer_renderbuffer(
            &self,
            target: GLenum,
            attachment: GLenum,
            renderbuffertarget: GLenum,
            renderbuffer: GLuint,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer)
                },
                Gl::Gles(gles) => unsafe {
                    gles.FramebufferRenderbuffer(
                        target,
                        attachment,
                        renderbuffertarget,
                        renderbuffer,
                    )
                },
            }
        }

        pub fn framebuffer_texture_2d(
            &self,
            target: GLenum,
            attachment: GLenum,
            textarget: GLenum,
            texture: GLuint,
            level: GLint,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.FramebufferTexture2D(target, attachment, textarget, texture, level)
                },
                Gl::Gles(gles) => unsafe {
                    gles.FramebufferTexture2D(target, attachment, textarget, texture, level)
                },
            }
        }

        pub fn framebuffer_texture_layer(
            &self,
            target: GLenum,
            attachment: GLenum,
            texture: GLuint,
            level: GLint,
            layer: GLint,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.FramebufferTextureLayer(
                        target,
                        attachment,
                        texture,
                        level,
                        layer,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.FramebufferTextureLayer(
                        target,
                        attachment,
                        texture,
                        level,
                        layer,
                    )
                },
            }
        }

        pub fn invalidate_framebuffer(&self, target: GLenum, attachments: &[GLenum]) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.InvalidateFramebuffer(
                        target,
                        attachments.len() as GLsizei,
                        attachments.as_ptr(),
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.InvalidateFramebuffer(
                        target,
                        attachments.len() as GLsizei,
                        attachments.as_ptr(),
                    )
                },
            }
        }

        pub fn invalidate_sub_framebuffer(
            &self,
            target: GLenum,
            attachments: &[GLenum],
            x: i32,
            y: i32,
            width: GLsizei,
            height: GLsizei,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.InvalidateSubFramebuffer(
                        target,
                        attachments.len() as GLsizei,
                        attachments.as_ptr(),
                        x,
                        y,
                        width,
                        height,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.InvalidateSubFramebuffer(
                        target,
                        attachments.len() as GLsizei,
                        attachments.as_ptr(),
                        x,
                        y,
                        width,
                        height,
                    )
                },
            }
        }

        pub fn renderbuffer_storage(
            &self,
            target: GLenum,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.RenderbufferStorage(target, internalformat, width, height)
                },
                Gl::Gles(gles) => unsafe {
                    gles.RenderbufferStorage(target, internalformat, width, height)
                },
            }
        }

        pub fn renderbuffer_storage_multisample(
            &self,
            target: GLenum,
            samples: GLsizei,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.RenderbufferStorageMultisample(target, samples, internalformat, width, height)
                },
                Gl::Gles(gles) => unsafe {
                    gles.RenderbufferStorageMultisample(target, samples, internalformat, width, height)
                },
            }
        }

        pub fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
            match self {
                Gl::Gl(gl) => unsafe { gl.CheckFramebufferStatus(target) },
                Gl::Gles(gles) => unsafe { gles.CheckFramebufferStatus(target) },
            }
        }

        pub fn get_error(&self) -> GLenum {
            match self {
                Gl::Gl(gl) => unsafe { gl.GetError() },
                Gl::Gles(gles) => unsafe { gles.GetError() },
            }
        }

        pub fn tex_parameter_i(&self, target: GLenum, pname: GLenum, param: GLint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.TexParameteri(target, pname, param) },
                Gl::Gles(gles) => unsafe { gles.TexParameteri(target, pname, param) },
            }
        }

        pub fn tex_parameter_f(&self, target: GLenum, pname: GLenum, param: GLfloat) {
            match self {
                Gl::Gl(gl) => unsafe { gl.TexParameterf(target, pname, param) },
                Gl::Gles(gles) => unsafe { gles.TexParameterf(target, pname, param) },
            }
        }

        pub fn get_tex_parameter_iv(&self, target: GLenum, pname: GLenum) -> GLint {
            let mut result = 0;
            match self {
                Gl::Gl(gl) => unsafe { gl.GetTexParameteriv(target, pname, &mut result) },
                Gl::Gles(gles) => unsafe { gles.GetTexParameteriv(target, pname, &mut result) },
            };
            result
        }

        pub fn get_tex_parameter_fv(&self, target: GLenum, pname: GLenum) -> GLfloat {
            let mut result = 0.;
            match self {
                Gl::Gl(gl) => unsafe { gl.GetTexParameterfv(target, pname, &mut result) },
                Gl::Gles(gles) => unsafe { gles.GetTexParameterfv(target, pname, &mut result) },
            };
            result
        }

        pub fn get_active_attrib(&self, program: GLuint, index: GLuint) -> (i32, u32, String) {
            let mut buf_size = [0];
            unsafe {
                self.get_program_iv(program, ffi::ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut buf_size);
            }
            let mut name = vec![0u8; buf_size[0] as usize];
            let mut length = 0 as GLsizei;
            let mut size = 0 as i32;
            let mut type_ = 0 as u32;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetActiveAttrib(
                        program,
                        index,
                        buf_size[0],
                        &mut length,
                        &mut size,
                        &mut type_,
                        name.as_mut_ptr() as *mut GLchar,
                    );
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetActiveAttrib(
                        program,
                        index,
                        buf_size[0],
                        &mut length,
                        &mut size,
                        &mut type_,
                        name.as_mut_ptr() as *mut GLchar,
                    );
                },
            }
            name.truncate(if length > 0 { length as usize } else { 0 });
            (size, type_, String::from_utf8(name).unwrap())
        }

        pub fn get_active_uniform(&self, program: GLuint, index: GLuint) -> (i32, u32, String) {
            let mut buf_size = [0];
            unsafe {
                self.get_program_iv(program, ffi::ACTIVE_UNIFORM_MAX_LENGTH, &mut buf_size);
            }
            let mut name = vec![0 as u8; buf_size[0] as usize];
            let mut length: GLsizei = 0;
            let mut size: i32 = 0;
            let mut type_: u32 = 0;

            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetActiveUniform(
                        program,
                        index,
                        buf_size[0],
                        &mut length,
                        &mut size,
                        &mut type_,
                        name.as_mut_ptr() as *mut GLchar,
                    );
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetActiveUniform(
                        program,
                        index,
                        buf_size[0],
                        &mut length,
                        &mut size,
                        &mut type_,
                        name.as_mut_ptr() as *mut GLchar,
                    );
                },
            }

            name.truncate(if length > 0 { length as usize } else { 0 });

            (size, type_, String::from_utf8(name).unwrap())
        }

        pub fn get_attrib_location(&self, program: GLuint, name: &str) -> c_int {
            let name = CString::new(name).unwrap();
            match self {
                Gl::Gl(gl) => unsafe { gl.GetAttribLocation(program, name.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GetAttribLocation(program, name.as_ptr()) },
            }
        }

        pub fn get_frag_data_location(&self, program: GLuint, name: &str) -> c_int {
            let name = CString::new(name).unwrap();
            match self {
                Gl::Gl(gl) => unsafe { gl.GetFragDataLocation(program, name.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GetFragDataLocation(program, name.as_ptr()) },
            }
        }

        pub fn get_uniform_location(&self, program: GLuint, name: &str) -> c_int {
            let name = CString::new(name).unwrap();
            match self {
                Gl::Gl(gl) => unsafe { gl.GetUniformLocation(program, name.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GetUniformLocation(program, name.as_ptr()) },
            }
        }

        pub fn get_uniform_block_index(&self, program: GLuint, name: &str) -> GLuint {
            let name = CString::new(name).unwrap();
            match self {
                Gl::Gl(gl) => unsafe { gl.GetUniformBlockIndex(program, name.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GetUniformBlockIndex(program, name.as_ptr()) },
            }
        }

        pub fn get_uniform_indices(&self, program: GLuint, names: &[&str]) -> Vec<GLuint> {
            let count = names.len() as GLsizei;
            let c_names = names
                .iter()
                .map(|&name| std::ffi::CString::new(name).unwrap())
                .collect::<Vec<_>>();
            let c_name_ptrs = c_names
                .iter()
                .map(|name| name.as_ptr())
                .collect::<Vec<_>>();

            let mut indices = vec![0 as GLuint; names.len()];
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetUniformIndices(
                        program,
                        count,
                        c_name_ptrs.as_ptr(),
                        indices.as_mut_ptr(),
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetUniformIndices(
                        program,
                        count,
                        c_name_ptrs.as_ptr(),
                        indices.as_mut_ptr(),
                    )
                },
            }
            indices
        }

        pub fn get_active_uniforms_iv(
            &self,
            program: GLuint,
            uniforms: &[GLuint],
            pname: GLenum,
        ) -> Vec<GLint> {
            let mut results = vec![0 as GLint; uniforms.len()];
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetActiveUniformsiv(
                        program,
                        uniforms.len() as GLsizei,
                        uniforms.as_ptr(),
                        pname,
                        results.as_mut_ptr(),
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetActiveUniformsiv(
                        program,
                        uniforms.len() as GLsizei,
                        uniforms.as_ptr(),
                        pname,
                        results.as_mut_ptr(),
                    )
                },
            }
            results
        }

        pub fn get_active_uniform_block_iv(
            &self,
            program: GLuint,
            index: GLuint,
            pname: GLenum,
        ) -> Vec<GLint> {
            let buf_size = match pname {
                ffi::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES => {
                    self.get_active_uniform_block_iv(
                        program,
                        index,
                        ffi::UNIFORM_BLOCK_ACTIVE_UNIFORMS,
                    )[0] as usize
                },
                _ => 1,
            };
            let mut results = vec![0 as i32; buf_size];
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetActiveUniformBlockiv(
                        program,
                        index,
                        pname,
                        results.as_mut_ptr(),
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetActiveUniformBlockiv(
                        program,
                        index,
                        pname,
                        results.as_mut_ptr(),
                    )
                },
            }
            results
        }

        pub fn get_active_uniform_block_name(&self, program: GLuint, index: GLuint) -> String {
            let buf_size = self.get_active_uniform_block_iv(program, index, ffi::UNIFORM_BLOCK_NAME_LENGTH)[0];
            let mut name = vec![0 as u8; buf_size as usize];
            let mut length: GLsizei = 0;

            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetActiveUniformBlockName(
                        program,
                        index,
                        buf_size,
                        &mut length,
                        name.as_mut_ptr() as *mut GLchar,
                    );
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetActiveUniformBlockName(
                        program,
                        index,
                        buf_size,
                        &mut length,
                        name.as_mut_ptr() as *mut GLchar,
                    );
                },
            }

            name.truncate(if length > 0 { length as usize } else { 0 });
            String::from_utf8(name).unwrap()
        }

        pub fn uniform_block_binding(
            &self,
            program: GLuint,
            uniform_block_index: GLuint,
            uniform_block_binding: GLuint,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformBlockBinding(
                        program,
                        uniform_block_index,
                        uniform_block_binding,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformBlockBinding(
                        program,
                        uniform_block_index,
                        uniform_block_binding,
                    )
                },
            }
        }

        pub fn bind_buffer_base(&self, program: GLenum, index: GLuint, buffer: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BindBufferBase(program, index, buffer) },
                Gl::Gles(gles) => unsafe { gles.BindBufferBase(program, index, buffer) },
            }
        }

        pub fn bind_buffer_range(
            &self,
            program: GLenum,
            index: GLuint,
            buffer: GLuint,
            offset: GLintptr,
            size: GLsizeiptr,
        ) {
            assert!(offset >= 0);
            assert!(size >= 0);
            match self {
                Gl::Gl(gl) => unsafe { gl.BindBufferRange(program, index, buffer, offset, size) },
                Gl::Gles(gles) => unsafe { gles.BindBufferRange(program, index, buffer, offset, size) },
            }
        }

        pub fn get_program_info_log(&self, program: GLuint) -> String {
            let mut max_len = [0];
            unsafe {
                self.get_program_iv(program, ffi::INFO_LOG_LENGTH, &mut max_len);
            }
            if max_len[0] == 0 {
                return String::new();
            }
            let mut result = vec![0u8; max_len[0] as usize];
            let mut result_len = 0 as GLsizei;
            let max_len = max_len[0] as GLsizei;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetProgramInfoLog(
                        program,
                        max_len,
                        &mut result_len,
                        result.as_mut_ptr() as *mut GLchar,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetProgramInfoLog(
                        program,
                        max_len,
                        &mut result_len,
                        result.as_mut_ptr() as *mut GLchar,
                    )
                },
            }
            result.truncate(if result_len > 0 {
                result_len as usize
            } else {
                0
            });
            String::from_utf8(result).unwrap()
        }

        pub unsafe fn get_program_iv(&self, program: GLuint, pname: GLenum, result: &mut [GLint]) {
            assert!(!result.is_empty());
            match self {
                Gl::Gl(gl) => gl.GetProgramiv(program, pname, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetProgramiv(program, pname, result.as_mut_ptr()),
            }
        }

        pub unsafe fn get_vertex_attrib_fv(
            &self,
            index: GLuint,
            pname: GLenum,
            result: &mut [GLfloat],
        ) {
            assert!(!result.is_empty());
            match self {
                Gl::Gl(gl) => gl.GetVertexAttribfv(index, pname, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetVertexAttribfv(index, pname, result.as_mut_ptr()),
            }
        }

        pub fn get_shader_info_log(&self, shader: GLuint) -> String {
            let mut max_len = [0];
            unsafe {
                self.get_shader_iv(shader, ffi::INFO_LOG_LENGTH, &mut max_len);
            }
            if max_len[0] == 0 {
                return String::new();
            }
            let mut result = vec![0u8; max_len[0] as usize];
            let mut result_len = 0 as GLsizei;
            let max_len = max_len[0] as GLsizei;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.GetShaderInfoLog(
                        shader,
                        max_len,
                        &mut result_len,
                        result.as_mut_ptr() as *mut GLchar,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.GetShaderInfoLog(
                        shader,
                        max_len,
                        &mut result_len,
                        result.as_mut_ptr() as *mut GLchar,
                    )
                },
            }
            result.truncate(if result_len > 0 {
                result_len as usize
            } else {
                0
            });
            String::from_utf8(result).unwrap()
        }

        pub unsafe fn get_integer_v(&self, name: GLenum, result: &mut [GLint]) {
            assert!(!result.is_empty());
            match self {
                Gl::Gl(gl) => gl.GetIntegerv(name, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetIntegerv(name, result.as_mut_ptr()),
            }
        }

        pub unsafe fn get_integer64_v(&self, name: GLenum, result: &mut [GLint64]) {
            assert!(!result.is_empty());
            match self {
                Gl::Gl(gl) => gl.GetInteger64v(name, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetInteger64v(name, result.as_mut_ptr()),
            }
        }

        pub unsafe fn get_integeri_v(&self, name: GLenum, index: GLuint, result: &mut [GLint]) {
            assert!(!result.is_empty());
            match self {
                Gl::Gl(gl) => gl.GetIntegeri_v(name, index, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetIntegeri_v(name, index, result.as_mut_ptr()),
            }
        }

        pub unsafe fn get_integer64i_v(&self, name: GLenum, index: GLuint, result: &mut [GLint64]) {
            assert!(!result.is_empty());
            match self {
                Gl::Gl(gl) => gl.GetInteger64i_v(name, index, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetInteger64i_v(name, index, result.as_mut_ptr()),
            }
        }

        pub unsafe fn get_boolean_v(&self, name: GLenum, result: &mut [GLboolean]) {
            assert!(!result.is_empty());
            match self {
                Gl::Gl(gl) => gl.GetBooleanv(name, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetBooleanv(name, result.as_mut_ptr()),
            }
        }

        pub unsafe fn get_float_v(&self, name: GLenum, result: &mut [GLfloat]) {
            assert!(!result.is_empty());
            match self {
                Gl::Gl(gl) => gl.GetFloatv(name, result.as_mut_ptr()),
                Gl::Gles(gles) => gles.GetFloatv(name, result.as_mut_ptr()),
            }
        }

        pub fn compile_shader(&self, shader: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.CompileShader(shader) },
                Gl::Gles(gles) => unsafe { gles.CompileShader(shader) },
            }
        }

        pub fn pixel_store_i(&self, name: GLenum, param: GLint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.PixelStorei(name, param) },
                Gl::Gles(gles) => unsafe { gles.PixelStorei(name, param) },
            }
        }

        pub fn read_pixels_into_buffer(
            &self,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            pixel_type: GLenum,
            buffer: &mut [u8],
        ) {
            // Assumes that the user properly allocated the size for buffer.
            assert_eq!(
                calculate_length(width, height, format, pixel_type),
                buffer.len()
            );

            // We don't want any alignment padding on pixel rows.
            self.pixel_store_i(ffi::PACK_ALIGNMENT, 1);

            match self {
                Gl::Gl(gl) => unsafe {
                    gl.ReadPixels(
                        x,
                        y,
                        width,
                        height,
                        format,
                        pixel_type,
                        buffer.as_mut_ptr() as *mut _,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.ReadPixels(
                        x,
                        y,
                        width,
                        height,
                        format,
                        pixel_type,
                        buffer.as_mut_ptr() as *mut _,
                    )
                },
            }
        }

        /// Used when a WebGLBuffer object is bound to PIXEL_PACK_BUFFER.
        /// Reads the current pixel buffer into the bound buffer object
        /// at the provided offset. Unsafe because no validation is performed
        /// to ensure that there is actually a buffer object bound; GL
        /// will write at an invalid pointer value in this case.
        pub unsafe fn read_pixels_into_pixel_pack_buffer(
            &self,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            pixel_type: GLenum,
            buffer_byte_offset: usize,
        ) {
            match self {
                Gl::Gl(gl) =>
                    gl.ReadPixels(
                        x,
                        y,
                        width,
                        height,
                        format,
                        pixel_type,
                        buffer_byte_offset as *mut _,
                    ),
                Gl::Gles(gles) =>
                    gles.ReadPixels(
                        x,
                        y,
                        width,
                        height,
                        format,
                        pixel_type,
                        buffer_byte_offset as *mut _,
                    ),
            }
        }

        pub fn read_pixels(
            &self,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            pixel_type: GLenum,
        ) -> Vec<u8> {
            let len = calculate_length(width, height, format, pixel_type);
            let mut pixels: Vec<u8> = Vec::new();
            pixels.reserve(len);
            unsafe {
                pixels.set_len(len);
            }
            self.read_pixels_into_buffer(x, y, width, height, format, pixel_type, &mut pixels[..]);
            pixels
        }

        pub fn fence_sync(&self, condition: GLenum, flags: GLbitfield) -> GLsync {
            match self {
                Gl::Gl(gl) => unsafe { gl.FenceSync(condition, flags) as GLsync },
                Gl::Gles(gles) => unsafe { gles.FenceSync(condition, flags) as GLsync },
            }
        }

        pub fn client_wait_sync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
            match self {
                Gl::Gl(gl) => unsafe { gl.ClientWaitSync(sync as *const _, flags, timeout) },
                Gl::Gles(gles) => unsafe { gles.ClientWaitSync(sync as *const _, flags, timeout) },
            }
        }

        pub fn wait_sync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
            match self {
                Gl::Gl(gl) => unsafe { gl.WaitSync(sync as *const _, flags, timeout) },
                Gl::Gles(gles) => unsafe { gles.WaitSync(sync as *const _, flags, timeout) },
            };
        }

        pub fn get_sync_iv(&self, sync: GLsync, pname: GLenum) -> Vec<GLint> {
            let mut result = vec![0 as GLint];
            match self {
                Gl::Gl(gl) => unsafe { gl.GetSynciv(sync as *const _, pname, result.len() as _, ptr::null_mut(), result.as_mut_ptr()); },
                Gl::Gles(gles) => unsafe { gles.GetSynciv(sync as *const _, pname, result.len() as _, ptr::null_mut(), result.as_mut_ptr()); },
            };
            result
        }

        pub fn is_sync(&self, sync: GLsync) -> bool {
            TRUE == match self {
                Gl::Gl(gl) => unsafe { gl.IsSync(sync as *const _) as GLboolean },
                Gl::Gles(gles) =>  unsafe { gles.IsSync(sync as *const _) as GLboolean },
            }
        }

        pub fn delete_sync(&self, sync: GLsync) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DeleteSync(sync as *const _) },
                Gl::Gles(gles) => unsafe { gles.DeleteSync(sync as *const _) },
            }
        }

        pub fn uniform_1f(&self, location: GLint, v0: GLfloat) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform1f(location, v0) },
                Gl::Gles(gles) => unsafe { gles.Uniform1f(location, v0) },
            }
        }

        pub fn uniform_1fv(&self, location: GLint, values: &[f32]) {
            let len = values.len() as GLsizei;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform1fv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform1fv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_1i(&self, location: GLint, v0: GLint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform1i(location, v0) },
                Gl::Gles(gles) => unsafe { gles.Uniform1i(location, v0) },
            }
        }

        pub fn uniform_1iv(&self, location: GLint, values: &[i32]) {
            let len = values.len() as GLsizei;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform1iv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform1iv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_1ui(&self, location: GLint, v0: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform1ui(location, v0) },
                Gl::Gles(gles) => unsafe { gles.Uniform1ui(location, v0) },
            }
        }

        pub fn uniform_1uiv(&self, location: GLint, values: &[u32]) {
            let len = values.len() as GLsizei;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform1uiv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform1uiv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform2f(location, v0, v1) },
                Gl::Gles(gles) => unsafe { gles.Uniform2f(location, v0, v1) },
            }
        }

        pub fn uniform_2fv(&self, location: GLint, values: &[f32]) {
            let len = values.len() as GLsizei / 2;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform2fv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform2fv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_2i(&self, location: GLint, v0: GLint, v1: GLint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform2i(location, v0, v1) },
                Gl::Gles(gles) => unsafe { gles.Uniform2i(location, v0, v1) },
            }
        }

        pub fn uniform_2iv(&self, location: GLint, values: &[i32]) {
            let len = values.len() as GLsizei / 2;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform2iv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform2iv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform2ui(location, v0, v1) },
                Gl::Gles(gles) => unsafe { gles.Uniform2ui(location, v0, v1) },
            }
        }

        pub fn uniform_2uiv(&self, location: GLint, values: &[u32]) {
            let len = values.len() as GLsizei / 2;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform2uiv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform2uiv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform3f(location, v0, v1, v2) },
                Gl::Gles(gles) => unsafe { gles.Uniform3f(location, v0, v1, v2) },
            }
        }

        pub fn uniform_3fv(&self, location: GLint, values: &[f32]) {
            let len = values.len() as GLsizei / 3;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform3fv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform3fv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform3i(location, v0, v1, v2) },
                Gl::Gles(gles) => unsafe { gles.Uniform3i(location, v0, v1, v2) },
            }
        }

        pub fn uniform_3iv(&self, location: GLint, values: &[i32]) {
            let len = values.len() as GLsizei / 3;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform3iv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform3iv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform3ui(location, v0, v1, v2) },
                Gl::Gles(gles) => unsafe { gles.Uniform3ui(location, v0, v1, v2) },
            }
        }

        pub fn uniform_3uiv(&self, location: GLint, values: &[u32]) {
            let len = values.len() as GLsizei / 3;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform3uiv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform3uiv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_4f(&self, location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform4f(location, x, y, z, w) },
                Gl::Gles(gles) => unsafe { gles.Uniform4f(location, x, y, z, w) },
            }
        }

        pub fn uniform_4i(&self, location: GLint, x: GLint, y: GLint, z: GLint, w: GLint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform4i(location, x, y, z, w) },
                Gl::Gles(gles) => unsafe { gles.Uniform4i(location, x, y, z, w) },
            }
        }

        pub fn uniform_4iv(&self, location: GLint, values: &[i32]) {
            let len = values.len() as GLsizei / 4;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform4iv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform4iv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_4ui(&self, location: GLint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform4ui(location, x, y, z, w) },
                Gl::Gles(gles) => unsafe { gles.Uniform4ui(location, x, y, z, w) },
            }
        }

        pub fn uniform_4uiv(&self, location: GLint, values: &[u32]) {
            let len = values.len() as GLsizei / 4;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform4uiv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform4uiv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_4fv(&self, location: GLint, values: &[f32]) {
            let len = values.len() as GLsizei / 4;
            match self {
                Gl::Gl(gl) => unsafe { gl.Uniform4fv(location, len, values.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.Uniform4fv(location, len, values.as_ptr()) },
            }
        }

        pub fn uniform_matrix_2fv(&self, location: GLint, transpose: bool, values: &[f32]) {
            let len = values.len() as GLsizei / 4;
            let transpose = transpose as GLboolean;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformMatrix2fv(location, len, transpose, values.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformMatrix2fv(location, len, transpose, values.as_ptr())
                },
            }
        }

        pub fn uniform_matrix_3fv(&self, location: GLint, transpose: bool, values: &[f32]) {
            let len = values.len() as GLsizei / 9;
            let transpose = transpose as GLboolean;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformMatrix3fv(location, len, transpose, values.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformMatrix3fv(location, len, transpose, values.as_ptr())
                },
            }
        }

        pub fn uniform_matrix_4fv(&self, location: GLint, transpose: bool, values: &[f32]) {
            let len = values.len() as GLsizei / 16;
            let transpose = transpose as GLboolean;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformMatrix4fv(location, len, transpose, values.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformMatrix4fv(location, len, transpose, values.as_ptr())
                },
            }
        }

        pub fn uniform_matrix_3x2fv(&self, location: GLint, transpose: bool, values: &[f32]) {
            let len = values.len() as GLsizei / (3 * 2);
            let transpose = transpose as GLboolean;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformMatrix3x2fv(location, len, transpose, values.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformMatrix3x2fv(location, len, transpose, values.as_ptr())
                },
            }
        }

        pub fn uniform_matrix_4x2fv(&self, location: GLint, transpose: bool, values: &[f32]) {
            let len = values.len() as GLsizei / (4 * 2);
            let transpose = transpose as GLboolean;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformMatrix4x2fv(location, len, transpose, values.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformMatrix4x2fv(location, len, transpose, values.as_ptr())
                },
            }
        }

        pub fn uniform_matrix_2x3fv(&self, location: GLint, transpose: bool, values: &[f32]) {
            let len = values.len() as GLsizei / (2 * 3);
            let transpose = transpose as GLboolean;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformMatrix2x3fv(location, len, transpose, values.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformMatrix2x3fv(location, len, transpose, values.as_ptr())
                },
            }
        }

        pub fn uniform_matrix_4x3fv(&self, location: GLint, transpose: bool, values: &[f32]) {
            let len = values.len() as GLsizei / (4 * 3);
            let transpose = transpose as GLboolean;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformMatrix4x3fv(location, len, transpose, values.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformMatrix4x3fv(location, len, transpose, values.as_ptr())
                },
            }
        }

        pub fn uniform_matrix_2x4fv(&self, location: GLint, transpose: bool, values: &[f32]) {
            let len = values.len() as GLsizei / (2 * 4);
            let transpose = transpose as GLboolean;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformMatrix2x4fv(location, len, transpose, values.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformMatrix2x4fv(location, len, transpose, values.as_ptr())
                },
            }
        }

        pub fn uniform_matrix_3x4fv(&self, location: GLint, transpose: bool, values: &[f32]) {
            let len = values.len() as GLsizei / (3 * 4);
            let transpose = transpose as GLboolean;
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.UniformMatrix3x4fv(location, len, transpose, values.as_ptr())
                },
                Gl::Gles(gles) => unsafe {
                    gles.UniformMatrix3x4fv(location, len, transpose, values.as_ptr())
                },
            }
        }


        pub fn use_program(&self, program: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.UseProgram(program) },
                Gl::Gles(gles) => unsafe { gles.UseProgram(program) },
            }
        }

        pub fn blit_framebuffer(
            &self,
            src_x0: GLint,
            src_y0: GLint,
            src_x1: GLint,
            src_y1: GLint,
            dst_x0: GLint,
            dst_y0: GLint,
            dst_x1: GLint,
            dst_y1: GLint,
            mask: GLbitfield,
            filter: GLenum,
        ) {
            match self {
                Gl::Gl(gl) => unsafe {
                    gl.BlitFramebuffer(
                        src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
                    )
                },
                Gl::Gles(gles) => unsafe {
                    gles.BlitFramebuffer(
                        src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
                    )
                },
            }
        }

        pub fn gen_queries(&self, n: GLsizei) -> Vec<GLuint> {
            if let Gl::Gles(gles) = self {
                if !gles.GenQueriesEXT.is_loaded() {
                    return Vec::new();
                }
            }
            let mut result = vec![0 as GLuint; n as usize];
            match self {
                Gl::Gl(gl) => unsafe { gl.GenQueries(n, result.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GenQueriesEXT(n, result.as_mut_ptr()) },
            };
            result
        }

        pub fn begin_query(&self, target: GLenum, id: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BeginQuery(target, id) },
                Gl::Gles(gles) => {
                    if gles.BeginQueryEXT.is_loaded() {
                        unsafe { gles.BeginQueryEXT(target, id) }
                    }
                },
            }
        }

        pub fn end_query(&self, target: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.EndQuery(target) },
                Gl::Gles(gles) => {
                    if gles.EndQueryEXT.is_loaded() {
                        unsafe { gles.EndQueryEXT(target) }
                    }
                },
            }
        }

        pub fn delete_queries(&self, ids: &[GLuint]) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DeleteQueries(ids.len() as GLsizei, ids.as_ptr()) },
                Gl::Gles(gles) => {
                    if gles.DeleteQueriesEXT.is_loaded() {
                        unsafe { gles.DeleteQueriesEXT(ids.len() as GLsizei, ids.as_ptr()) }
                    }
                },
            }
        }

        pub fn is_query(&self, id: GLuint) -> bool {
            TRUE == match self {
                Gl::Gl(gl) => unsafe { gl.IsQuery(id) },
                Gl::Gles(gles) => {
                    match gles.IsQueryEXT.is_loaded() {
                        true => unsafe { gles.IsQueryEXT(id) },
                        false => FALSE,
                    }
                },
            }
        }

        pub fn get_query_iv(&self, target: GLenum, pname: GLenum) -> i32 {
            let mut result = 0;
            match self {
                Gl::Gl(gl) => unsafe { gl.GetQueryiv(target, pname, &mut result) },
                Gl::Gles(gles) => {
                    if gles.GetQueryivEXT.is_loaded() {
                        unsafe { gles.GetQueryivEXT(target, pname, &mut result) }
                    }
                },
            };
            result
        }

        pub fn get_query_object_iv(&self, id: GLuint, pname: GLenum) -> i32 {
            let mut result = 0;
            match self {
                Gl::Gl(gl) => unsafe { gl.GetQueryObjectiv(id, pname, &mut result) },
                Gl::Gles(gles) => {
                    if gles.GetQueryObjectivEXT.is_loaded() {
                        unsafe { gles.GetQueryObjectivEXT(id, pname, &mut result) }
                    }
                },
            }
            result
        }

        pub fn get_query_object_uiv(&self, id: GLuint, pname: GLenum) -> u32 {
            let mut result = 0;
            match self {
                Gl::Gl(gl) => unsafe { gl.GetQueryObjectuiv(id, pname, &mut result) },
                Gl::Gles(gles) => {
                    if gles.GetQueryObjectuivEXT.is_loaded() {
                        unsafe { gles.GetQueryObjectuivEXT(id, pname, &mut result) }
                    }
                },
            }
            result
        }

        pub fn get_query_object_i64v(&self, id: GLuint, pname: GLenum) -> i64 {
            let mut result = 0;
            match self {
                Gl::Gl(gl) => unsafe { gl.GetQueryObjecti64v(id, pname, &mut result) },
                Gl::Gles(gles) => {
                    if gles.GetQueryObjecti64vEXT.is_loaded() {
                        unsafe { gles.GetQueryObjecti64vEXT(id, pname, &mut result) }
                    }
                },
            }
            result
        }

        pub fn get_query_object_ui64v(&self, id: GLuint, pname: GLenum) -> u64 {
            let mut result = 0;
            match self {
                Gl::Gl(gl) => unsafe { gl.GetQueryObjectui64v(id, pname, &mut result) },
                Gl::Gles(gles) => {
                    if gles.GetQueryObjectui64vEXT.is_loaded() {
                        unsafe { gles.GetQueryObjectui64vEXT(id, pname, &mut result) }
                    }
                },
            }
            result
        }

        pub fn gen_samplers(&self, n: GLsizei) -> Vec<GLuint> {
            let mut result = vec![0 as GLuint; n as usize];
            match self {
                Gl::Gl(gl) => unsafe { gl.GenSamplers(n, result.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GenSamplers(n, result.as_mut_ptr()) },
            };
            result
        }

        pub fn delete_samplers(&self, samplers: &[GLuint]) {
            match self {
                Gl::Gl(gl) => unsafe { gl.DeleteSamplers(samplers.len() as GLsizei, samplers.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.DeleteSamplers(samplers.len() as GLsizei, samplers.as_ptr()) },
            }
        }

        pub fn is_sampler(&self, sampler: GLuint) -> bool {
            TRUE == match self {
                Gl::Gl(gl) => unsafe { gl.IsSampler(sampler) },
                Gl::Gles(gles) => unsafe { gles.IsSampler(sampler) },
            }
        }

        pub fn bind_sampler(&self, target: GLenum, sampler: GLuint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BindSampler(target, sampler) },
                Gl::Gles(gles) => unsafe { gles.BindSampler(target, sampler) },
            }
        }

        pub fn get_sampler_parameter_iv(&self, sampler: GLuint, pname: GLenum) -> Vec<GLint> {
            let mut result = vec![0 as GLint];
            match self {
                Gl::Gl(gl) => unsafe { gl.GetSamplerParameteriv(sampler, pname, result.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GetSamplerParameteriv(sampler, pname, result.as_mut_ptr()) },
            }
            result
        }

        pub fn get_sampler_parameter_fv(&self, sampler: GLuint, pname: GLenum) -> Vec<GLfloat> {
            let mut result = vec![0.0_f32 as GLfloat];
            match self {
                Gl::Gl(gl) => unsafe { gl.GetSamplerParameterfv(sampler, pname, result.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GetSamplerParameterfv(sampler, pname, result.as_mut_ptr()) },
            }
            result
        }

        pub fn sampler_parameter_i(&self, sampler: GLuint, pname: GLenum, param: GLint) {
            match self {
                Gl::Gl(gl) => unsafe { gl.SamplerParameteri(sampler, pname, param) },
                Gl::Gles(gles) => unsafe { gles.SamplerParameteri(sampler, pname, param) },
            }
        }

        pub fn sampler_parameter_f(&self, sampler: GLuint, pname: GLenum, param: GLfloat) {
            match self {
                Gl::Gl(gl) => unsafe { gl.SamplerParameterf(sampler, pname, param) },
                Gl::Gles(gles) => unsafe { gles.SamplerParameterf(sampler, pname, param) },
            }
        }

        pub fn sampler_parameter_iv(&self, sampler: GLuint, pname: GLenum, params: &[GLint]) {
            assert!(!params.is_empty());
            match self {
                Gl::Gl(gl) => unsafe { gl.SamplerParameteriv(sampler, pname, params.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.SamplerParameteriv(sampler, pname, params.as_ptr()) },
            }
        }

        pub fn sampler_parameter_fv(&self, sampler: GLuint, pname: GLenum, params: &[GLfloat]) {
            assert!(!params.is_empty());
            match self {
                Gl::Gl(gl) => unsafe { gl.SamplerParameterfv(sampler, pname, params.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.SamplerParameterfv(sampler, pname, params.as_ptr()) },
            }
        }

        pub fn gen_transform_feedbacks(&self) -> u32 {
            let mut ids = vec![0 as GLuint];
            match self {
                Gl::Gl(gl) => unsafe { gl.GenTransformFeedbacks(ids.len() as _, ids.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GenTransformFeedbacks(ids.len() as _, ids.as_mut_ptr()) },
            }
            ids[0]
        }

        pub fn delete_transform_feedbacks(&self, id: GLuint) {
            let ids = vec![id];
            match self {
                Gl::Gl(gl) => unsafe { gl.DeleteTransformFeedbacks(ids.len() as _, ids.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.DeleteTransformFeedbacks(ids.len() as _, ids.as_ptr()) },
            }
        }

        pub fn is_transform_feedback(&self, id: GLuint) -> bool {
            TRUE == match self {
                Gl::Gl(gl) => unsafe { gl.IsTransformFeedback(id) },
                Gl::Gles(gles) => unsafe { gles.IsTransformFeedback(id) },
            }
        }

        pub fn bind_transform_feedback(&self, target: GLenum, id: u32) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BindTransformFeedback(target, id) },
                Gl::Gles(gles) => unsafe { gles.BindTransformFeedback(target, id) },
            }
        }

        pub fn begin_transform_feedback(&self, mode: GLenum) {
            match self {
                Gl::Gl(gl) => unsafe { gl.BeginTransformFeedback(mode) },
                Gl::Gles(gles) => unsafe { gles.BeginTransformFeedback(mode) },
            }
        }

        pub fn end_transform_feedback(&self) {
            match self {
                Gl::Gl(gl) => unsafe { gl.EndTransformFeedback() },
                Gl::Gles(gles) => unsafe { gles.EndTransformFeedback() },
            }
        }

        pub fn pause_transform_feedback(&self) {
            match self {
                Gl::Gl(gl) => unsafe { gl.PauseTransformFeedback() },
                Gl::Gles(gles) => unsafe { gles.PauseTransformFeedback() },
            }
        }

        pub fn resume_transform_feedback(&self) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ResumeTransformFeedback() },
                Gl::Gles(gles) => unsafe { gles.ResumeTransformFeedback() },
            }
        }

        pub fn get_transform_feedback_varying(&self, program: GLuint, index: GLuint) -> (i32, u32, String) {
            let mut length = 0;
            let buf_size = 128;
            let mut name = vec![0 as c_char; buf_size as usize];
            let mut size = 0;
            let mut ty = 0;
            match self {
                Gl::Gl(gl) => unsafe { gl.GetTransformFeedbackVarying(program, index, buf_size, &mut length, &mut size, &mut ty, name.as_mut_ptr()) },
                Gl::Gles(gles) => unsafe { gles.GetTransformFeedbackVarying(program, index, buf_size, &mut length, &mut size, &mut ty, name.as_mut_ptr()) },
            }
            let name: &[u8] = unsafe { std::slice::from_raw_parts(name.as_ptr() as _, length as usize) };
            let name = String::from_utf8(name.to_vec()).unwrap();
            (size, ty, name)
        }

        pub fn transform_feedback_varyings(&self, program: GLuint, varyings: &[String], buffer_mode: GLenum) {
            let c_varyings = varyings
                .iter()
                .map(|varying| {
                    std::ffi::CString::new("_u".to_owned() + varying.as_str()).unwrap()
                })
                .collect::<Vec<_>>();
            let pointers: Vec<*const c_char> =
                c_varyings.iter().map(|p| p.as_ptr()).collect();
            match self {
                Gl::Gl(gl) => unsafe { gl.TransformFeedbackVaryings(program, varyings.len() as _, pointers.as_ptr() as _, buffer_mode) },
                Gl::Gles(gles) => unsafe { gles.TransformFeedbackVaryings(program, varyings.len() as _, pointers.as_ptr() as _, buffer_mode) },
            }
        }

        pub fn clear_buffer_iv(&self, buffer: GLenum, draw_buffer: GLint, value: &[GLint]) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ClearBufferiv(buffer, draw_buffer, value.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.ClearBufferiv(buffer, draw_buffer, value.as_ptr()) },
            }
        }

        pub fn clear_buffer_uiv(&self, buffer: GLenum, draw_buffer: GLint, value: &[GLuint]) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ClearBufferuiv(buffer, draw_buffer, value.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.ClearBufferuiv(buffer, draw_buffer, value.as_ptr()) },
            }
        }

        pub fn clear_buffer_fv(&self, buffer: GLenum, draw_buffer: GLint, value: &[GLfloat]) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ClearBufferfv(buffer, draw_buffer, value.as_ptr()) },
                Gl::Gles(gles) => unsafe { gles.ClearBufferfv(buffer, draw_buffer, value.as_ptr()) },
            }
        }

        pub fn clear_buffer_fi(
            &self,
            buffer: GLenum,
            draw_buffer: GLint,
            depth: GLfloat,
            stencil: GLint,
        ) {
            match self {
                Gl::Gl(gl) => unsafe { gl.ClearBufferfi(buffer, draw_buffer, depth, stencil) },
                Gl::Gles(gles) => unsafe { gles.ClearBufferfi(buffer, draw_buffer, depth, stencil) },
            }
        }
    }

    fn calculate_length(
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        pixel_type: GLenum,
    ) -> usize {
        let colors = match format {
            ffi::RED => 1,
            ffi::RGB => 3,
            ffi::BGR => 3,

            ffi::RGBA => 4,
            ffi::BGRA => 4,

            ffi::ALPHA => 1,
            ffi::R16 => 1,
            ffi::LUMINANCE => 1,
            ffi::DEPTH_COMPONENT => 1,
            _ => panic!("unsupported format: {:?}", format),
        };
        let depth = match pixel_type {
            ffi::UNSIGNED_BYTE => 1,
            ffi::UNSIGNED_SHORT => 2,
            ffi::SHORT => 2,
            ffi::FLOAT => 4,
            _ => panic!("unsupported pixel_type: {:?}", pixel_type),
        };

        (width * height * colors * depth) as usize
    }

    pub fn buffer_data<T>(gl_: &Gl, target: GLenum, data: &[T], usage: GLenum) {
        unsafe {
            gl_.buffer_data(
                target,
                (data.len() * size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage,
            )
        }
    }

    pub fn buffer_sub_data<T>(gl_: &Gl, target: GLenum, offset: isize, data: &[T]) {
        unsafe {
            gl_.buffer_sub_data(
                target,
                offset,
                (data.len() * size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
            );
        }
    }

    pub mod ffi {
        include!(concat!(env!("OUT_DIR"), "/gl_and_gles_bindings.rs"));
    }

    pub mod ffi_gl {
        include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
    }

    pub mod ffi_gles {
        include!(concat!(env!("OUT_DIR"), "/gles_bindings.rs"));
    }
}
