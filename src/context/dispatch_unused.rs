use super::OxideGLContext;
use crate::gl::gltypes::*;
impl OxideGLContext {
    pub(crate) fn oxidegl_cull_face(&mut self, mode: GLenum) {
        panic!("command oxidegl_cull_face not yet implemented");
    }
    pub(crate) fn oxidegl_front_face(&mut self, mode: GLenum) {
        panic!("command oxidegl_front_face not yet implemented");
    }
    pub(crate) fn oxidegl_hint(&mut self, target: GLenum, mode: GLenum) {
        panic!("command oxidegl_hint not yet implemented");
    }
    pub(crate) fn oxidegl_line_width(&mut self, width: GLfloat) {
        panic!("command oxidegl_line_width not yet implemented");
    }
    pub(crate) fn oxidegl_point_size(&mut self, size: GLfloat) {
        panic!("command oxidegl_point_size not yet implemented");
    }
    pub(crate) fn oxidegl_polygon_mode(&mut self, face: GLenum, mode: GLenum) {
        panic!("command oxidegl_polygon_mode not yet implemented");
    }
    pub(crate) fn oxidegl_scissor(&mut self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        panic!("command oxidegl_scissor not yet implemented");
    }
    pub(crate) fn oxidegl_tex_parameterf(&mut self, target: GLenum, pname: GLenum, param: GLfloat) {
        panic!("command oxidegl_tex_parameterf not yet implemented");
    }
    pub(crate) fn oxidegl_tex_parameterfv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *const GLfloat,
    ) {
        panic!("command oxidegl_tex_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_tex_parameteri(&mut self, target: GLenum, pname: GLenum, param: GLint) {
        panic!("command oxidegl_tex_parameteri not yet implemented");
    }
    pub(crate) fn oxidegl_tex_parameteriv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *const GLint,
    ) {
        panic!("command oxidegl_tex_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_tex_image1_d(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        border: GLint,
        format: GLenum,
        r#type: GLenum,
        pixels: *const GLvoid,
    ) {
        panic!("command oxidegl_tex_image1_d not yet implemented");
    }
    pub(crate) fn oxidegl_tex_image2_d(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        r#type: GLenum,
        pixels: *const GLvoid,
    ) {
        panic!("command oxidegl_tex_image2_d not yet implemented");
    }
    pub(crate) fn oxidegl_draw_buffer(&mut self, buf: GLenum) {
        panic!("command oxidegl_draw_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_clear(&mut self, mask: GLbitfield) {
        println!("command oxidegl_clear not yet implemented");
    }
    pub(crate) fn oxidegl_clear_color(
        &mut self,
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat,
    ) {
        panic!("command oxidegl_clear_color not yet implemented");
    }
    pub(crate) fn oxidegl_clear_stencil(&mut self, s: GLint) {
        panic!("command oxidegl_clear_stencil not yet implemented");
    }
    pub(crate) fn oxidegl_clear_depth(&mut self, depth: GLdouble) {
        panic!("command oxidegl_clear_depth not yet implemented");
    }
    pub(crate) fn oxidegl_stencil_mask(&mut self, mask: GLuint) {
        panic!("command oxidegl_stencil_mask not yet implemented");
    }
    pub(crate) fn oxidegl_color_mask(
        &mut self,
        red: GLboolean,
        green: GLboolean,
        blue: GLboolean,
        alpha: GLboolean,
    ) {
        panic!("command oxidegl_color_mask not yet implemented");
    }
    pub(crate) fn oxidegl_depth_mask(&mut self, flag: GLboolean) {
        panic!("command oxidegl_depth_mask not yet implemented");
    }
    pub(crate) fn oxidegl_disable(&mut self, cap: GLenum) {
        panic!("command oxidegl_disable not yet implemented");
    }
    pub(crate) fn oxidegl_enable(&mut self, cap: GLenum) {
        panic!("command oxidegl_enable not yet implemented");
    }
    pub(crate) fn oxidegl_finish(&mut self) {
        panic!("command oxidegl_finish not yet implemented");
    }
    pub(crate) fn oxidegl_flush(&mut self) {
        panic!("command oxidegl_flush not yet implemented");
    }
    pub(crate) fn oxidegl_blend_func(&mut self, sfactor: GLenum, dfactor: GLenum) {
        panic!("command oxidegl_blend_func not yet implemented");
    }
    pub(crate) fn oxidegl_logic_op(&mut self, opcode: GLenum) {
        panic!("command oxidegl_logic_op not yet implemented");
    }
    pub(crate) fn oxidegl_stencil_func(&mut self, func: GLenum, r#ref: GLint, mask: GLuint) {
        panic!("command oxidegl_stencil_func not yet implemented");
    }
    pub(crate) fn oxidegl_stencil_op(&mut self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        panic!("command oxidegl_stencil_op not yet implemented");
    }
    pub(crate) fn oxidegl_depth_func(&mut self, func: GLenum) {
        panic!("command oxidegl_depth_func not yet implemented");
    }
    pub(crate) fn oxidegl_pixel_storef(&mut self, pname: GLenum, param: GLfloat) {
        panic!("command oxidegl_pixel_storef not yet implemented");
    }
    pub(crate) fn oxidegl_pixel_storei(&mut self, pname: GLenum, param: GLint) {
        panic!("command oxidegl_pixel_storei not yet implemented");
    }
    pub(crate) fn oxidegl_read_buffer(&mut self, src: GLenum) {
        panic!("command oxidegl_read_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_read_pixels(
        &mut self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *mut GLvoid,
    ) {
        panic!("command oxidegl_read_pixels not yet implemented");
    }
    pub(crate) fn oxidegl_get_error(&mut self) -> GLenum {
        panic!("command oxidegl_get_error not yet implemented");
    }
    pub(crate) fn oxidegl_get_tex_image(
        &mut self,
        target: GLenum,
        level: GLint,
        format: GLenum,
        r#type: GLenum,
        pixels: *mut GLvoid,
    ) {
        panic!("command oxidegl_get_tex_image not yet implemented");
    }
    pub(crate) fn oxidegl_get_tex_parameterfv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_tex_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_get_tex_parameteriv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_tex_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_get_tex_level_parameterfv(
        &mut self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_tex_level_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_get_tex_level_parameteriv(
        &mut self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_tex_level_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_is_enabled(&mut self, cap: GLenum) -> GLboolean {
        panic!("command oxidegl_is_enabled not yet implemented");
    }
    pub(crate) fn oxidegl_depth_range(&mut self, n: GLdouble, f: GLdouble) {
        panic!("command oxidegl_depth_range not yet implemented");
    }
    pub(crate) fn oxidegl_viewport(&mut self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        panic!("command oxidegl_viewport not yet implemented");
    }
    pub(crate) fn oxidegl_draw_arrays(&mut self, mode: GLenum, first: GLint, count: GLsizei) {
        panic!("command oxidegl_draw_arrays not yet implemented");
    }
    pub(crate) fn oxidegl_draw_elements(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const GLvoid,
    ) {
        panic!("command oxidegl_draw_elements not yet implemented");
    }
    pub(crate) fn oxidegl_polygon_offset(&mut self, factor: GLfloat, units: GLfloat) {
        panic!("command oxidegl_polygon_offset not yet implemented");
    }
    pub(crate) fn oxidegl_copy_tex_image1_d(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        border: GLint,
    ) {
        panic!("command oxidegl_copy_tex_image1_d not yet implemented");
    }
    pub(crate) fn oxidegl_copy_tex_image2_d(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        panic!("command oxidegl_copy_tex_image2_d not yet implemented");
    }
    pub(crate) fn oxidegl_copy_tex_sub_image1_d(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
    ) {
        panic!("command oxidegl_copy_tex_sub_image1_d not yet implemented");
    }
    pub(crate) fn oxidegl_copy_tex_sub_image2_d(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_copy_tex_sub_image2_d not yet implemented");
    }
    pub(crate) fn oxidegl_tex_sub_image1_d(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const GLvoid,
    ) {
        panic!("command oxidegl_tex_sub_image1_d not yet implemented");
    }
    pub(crate) fn oxidegl_tex_sub_image2_d(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const GLvoid,
    ) {
        panic!("command oxidegl_tex_sub_image2_d not yet implemented");
    }
    pub(crate) fn oxidegl_bind_texture(&mut self, target: GLenum, texture: GLuint) {
        panic!("command oxidegl_bind_texture not yet implemented");
    }
    pub(crate) fn oxidegl_delete_textures(&mut self, n: GLsizei, textures: *const GLuint) {
        panic!("command oxidegl_delete_textures not yet implemented");
    }
    pub(crate) fn oxidegl_gen_textures(&mut self, n: GLsizei, textures: *mut GLuint) {
        panic!("command oxidegl_gen_textures not yet implemented");
    }
    pub(crate) fn oxidegl_is_texture(&mut self, texture: GLuint) -> GLboolean {
        panic!("command oxidegl_is_texture not yet implemented");
    }
    pub(crate) fn oxidegl_draw_range_elements(
        &mut self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        r#type: GLenum,
        indices: *const GLvoid,
    ) {
        panic!("command oxidegl_draw_range_elements not yet implemented");
    }
    pub(crate) fn oxidegl_tex_image3_d(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        r#type: GLenum,
        pixels: *const GLvoid,
    ) {
        panic!("command oxidegl_tex_image3_d not yet implemented");
    }
    pub(crate) fn oxidegl_tex_sub_image3_d(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const GLvoid,
    ) {
        panic!("command oxidegl_tex_sub_image3_d not yet implemented");
    }
    pub(crate) fn oxidegl_copy_tex_sub_image3_d(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_copy_tex_sub_image3_d not yet implemented");
    }
    pub(crate) fn oxidegl_active_texture(&mut self, texture: GLenum) {
        panic!("command oxidegl_active_texture not yet implemented");
    }
    pub(crate) fn oxidegl_sample_coverage(&mut self, value: GLfloat, invert: GLboolean) {
        panic!("command oxidegl_sample_coverage not yet implemented");
    }
    pub(crate) fn oxidegl_compressed_tex_image3_d(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_compressed_tex_image3_d not yet implemented");
    }
    pub(crate) fn oxidegl_compressed_tex_image2_d(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_compressed_tex_image2_d not yet implemented");
    }
    pub(crate) fn oxidegl_compressed_tex_image1_d(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        border: GLint,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_compressed_tex_image1_d not yet implemented");
    }
    pub(crate) fn oxidegl_compressed_tex_sub_image3_d(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_compressed_tex_sub_image3_d not yet implemented");
    }
    pub(crate) fn oxidegl_compressed_tex_sub_image2_d(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_compressed_tex_sub_image2_d not yet implemented");
    }
    pub(crate) fn oxidegl_compressed_tex_sub_image1_d(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_compressed_tex_sub_image1_d not yet implemented");
    }
    pub(crate) fn oxidegl_get_compressed_tex_image(
        &mut self,
        target: GLenum,
        level: GLint,
        img: *mut GLvoid,
    ) {
        panic!("command oxidegl_get_compressed_tex_image not yet implemented");
    }
    pub(crate) fn oxidegl_blend_func_separate(
        &mut self,
        sfactor_r_g_b: GLenum,
        dfactor_r_g_b: GLenum,
        sfactor_alpha: GLenum,
        dfactor_alpha: GLenum,
    ) {
        panic!("command oxidegl_blend_func_separate not yet implemented");
    }
    pub(crate) fn oxidegl_multi_draw_arrays(
        &mut self,
        mode: GLenum,
        first: *const GLint,
        count: *const GLsizei,
        drawcount: GLsizei,
    ) {
        panic!("command oxidegl_multi_draw_arrays not yet implemented");
    }
    pub(crate) fn oxidegl_multi_draw_elements(
        &mut self,
        mode: GLenum,
        count: *const GLsizei,
        r#type: GLenum,
        indices: *mut *const GLvoid,
        drawcount: GLsizei,
    ) {
        panic!("command oxidegl_multi_draw_elements not yet implemented");
    }
    pub(crate) fn oxidegl_point_parameterf(&mut self, pname: GLenum, param: GLfloat) {
        panic!("command oxidegl_point_parameterf not yet implemented");
    }
    pub(crate) fn oxidegl_point_parameterfv(&mut self, pname: GLenum, params: *const GLfloat) {
        panic!("command oxidegl_point_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_point_parameteri(&mut self, pname: GLenum, param: GLint) {
        panic!("command oxidegl_point_parameteri not yet implemented");
    }
    pub(crate) fn oxidegl_point_parameteriv(&mut self, pname: GLenum, params: *const GLint) {
        panic!("command oxidegl_point_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_blend_color(
        &mut self,
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat,
    ) {
        panic!("command oxidegl_blend_color not yet implemented");
    }
    pub(crate) fn oxidegl_blend_equation(&mut self, mode: GLenum) {
        panic!("command oxidegl_blend_equation not yet implemented");
    }
    pub(crate) fn oxidegl_gen_queries(&mut self, n: GLsizei, ids: *mut GLuint) {
        panic!("command oxidegl_gen_queries not yet implemented");
    }
    pub(crate) fn oxidegl_delete_queries(&mut self, n: GLsizei, ids: *const GLuint) {
        panic!("command oxidegl_delete_queries not yet implemented");
    }
    pub(crate) fn oxidegl_is_query(&mut self, id: GLuint) -> GLboolean {
        panic!("command oxidegl_is_query not yet implemented");
    }
    pub(crate) fn oxidegl_begin_query(&mut self, target: GLenum, id: GLuint) {
        panic!("command oxidegl_begin_query not yet implemented");
    }
    pub(crate) fn oxidegl_end_query(&mut self, target: GLenum) {
        panic!("command oxidegl_end_query not yet implemented");
    }
    pub(crate) fn oxidegl_get_queryiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_queryiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_query_objectiv(
        &mut self,
        id: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_query_objectiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_query_objectuiv(
        &mut self,
        id: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) {
        panic!("command oxidegl_get_query_objectuiv not yet implemented");
    }
    pub(crate) fn oxidegl_bind_buffer(&mut self, target: GLenum, buffer: GLuint) {
        panic!("command oxidegl_bind_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_delete_buffers(&mut self, n: GLsizei, buffers: *const GLuint) {
        panic!("command oxidegl_delete_buffers not yet implemented");
    }
    pub(crate) fn oxidegl_gen_buffers(&mut self, n: GLsizei, buffers: *mut GLuint) {
        panic!("command oxidegl_gen_buffers not yet implemented");
    }
    pub(crate) fn oxidegl_is_buffer(&mut self, buffer: GLuint) -> GLboolean {
        panic!("command oxidegl_is_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_buffer_data(
        &mut self,
        target: GLenum,
        size: GLsizeiptr,
        data: *const GLvoid,
        usage: GLenum,
    ) {
        panic!("command oxidegl_buffer_data not yet implemented");
    }
    pub(crate) fn oxidegl_buffer_sub_data(
        &mut self,
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_buffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_get_buffer_sub_data(
        &mut self,
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *mut GLvoid,
    ) {
        panic!("command oxidegl_get_buffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_map_buffer(&mut self, target: GLenum, access: GLenum) -> *mut GLvoid {
        panic!("command oxidegl_map_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_unmap_buffer(&mut self, target: GLenum) -> GLboolean {
        panic!("command oxidegl_unmap_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_get_buffer_parameteriv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_buffer_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_get_buffer_pointerv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut *mut GLvoid,
    ) {
        panic!("command oxidegl_get_buffer_pointerv not yet implemented");
    }
    pub(crate) fn oxidegl_blend_equation_separate(
        &mut self,
        mode_r_g_b: GLenum,
        mode_alpha: GLenum,
    ) {
        panic!("command oxidegl_blend_equation_separate not yet implemented");
    }
    pub(crate) fn oxidegl_draw_buffers(&mut self, n: GLsizei, bufs: *const GLenum) {
        panic!("command oxidegl_draw_buffers not yet implemented");
    }
    pub(crate) fn oxidegl_stencil_op_separate(
        &mut self,
        face: GLenum,
        sfail: GLenum,
        dpfail: GLenum,
        dppass: GLenum,
    ) {
        panic!("command oxidegl_stencil_op_separate not yet implemented");
    }
    pub(crate) fn oxidegl_stencil_func_separate(
        &mut self,
        face: GLenum,
        func: GLenum,
        r#ref: GLint,
        mask: GLuint,
    ) {
        panic!("command oxidegl_stencil_func_separate not yet implemented");
    }
    pub(crate) fn oxidegl_stencil_mask_separate(&mut self, face: GLenum, mask: GLuint) {
        panic!("command oxidegl_stencil_mask_separate not yet implemented");
    }
    pub(crate) fn oxidegl_attach_shader(&mut self, program: GLuint, shader: GLuint) {
        panic!("command oxidegl_attach_shader not yet implemented");
    }
    pub(crate) fn oxidegl_bind_attrib_location(
        &mut self,
        program: GLuint,
        index: GLuint,
        name: *const GLchar,
    ) {
        panic!("command oxidegl_bind_attrib_location not yet implemented");
    }
    pub(crate) fn oxidegl_compile_shader(&mut self, shader: GLuint) {
        panic!("command oxidegl_compile_shader not yet implemented");
    }
    pub(crate) fn oxidegl_create_program(&mut self) -> GLuint {
        panic!("command oxidegl_create_program not yet implemented");
    }
    pub(crate) fn oxidegl_create_shader(&mut self, r#type: GLenum) -> GLuint {
        panic!("command oxidegl_create_shader not yet implemented");
    }
    pub(crate) fn oxidegl_delete_program(&mut self, program: GLuint) {
        panic!("command oxidegl_delete_program not yet implemented");
    }
    pub(crate) fn oxidegl_delete_shader(&mut self, shader: GLuint) {
        panic!("command oxidegl_delete_shader not yet implemented");
    }
    pub(crate) fn oxidegl_detach_shader(&mut self, program: GLuint, shader: GLuint) {
        panic!("command oxidegl_detach_shader not yet implemented");
    }
    pub(crate) fn oxidegl_disable_vertex_attrib_array(&mut self, index: GLuint) {
        panic!("command oxidegl_disable_vertex_attrib_array not yet implemented");
    }
    pub(crate) fn oxidegl_enable_vertex_attrib_array(&mut self, index: GLuint) {
        panic!("command oxidegl_enable_vertex_attrib_array not yet implemented");
    }
    pub(crate) fn oxidegl_get_active_attrib(
        &mut self,
        program: GLuint,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        r#type: *mut GLenum,
        name: *mut GLchar,
    ) {
        panic!("command oxidegl_get_active_attrib not yet implemented");
    }
    pub(crate) fn oxidegl_get_active_uniform(
        &mut self,
        program: GLuint,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        r#type: *mut GLenum,
        name: *mut GLchar,
    ) {
        panic!("command oxidegl_get_active_uniform not yet implemented");
    }
    pub(crate) fn oxidegl_get_attached_shaders(
        &mut self,
        program: GLuint,
        max_count: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint,
    ) {
        panic!("command oxidegl_get_attached_shaders not yet implemented");
    }
    pub(crate) fn oxidegl_get_attrib_location(
        &mut self,
        program: GLuint,
        name: *const GLchar,
    ) -> GLint {
        panic!("command oxidegl_get_attrib_location not yet implemented");
    }
    pub(crate) fn oxidegl_get_programiv(
        &mut self,
        program: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_programiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_info_log(
        &mut self,
        program: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        info_log: *mut GLchar,
    ) {
        panic!("command oxidegl_get_program_info_log not yet implemented");
    }
    pub(crate) fn oxidegl_get_shaderiv(
        &mut self,
        shader: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_shaderiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_shader_info_log(
        &mut self,
        shader: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        info_log: *mut GLchar,
    ) {
        panic!("command oxidegl_get_shader_info_log not yet implemented");
    }
    pub(crate) fn oxidegl_get_shader_source(
        &mut self,
        shader: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar,
    ) {
        panic!("command oxidegl_get_shader_source not yet implemented");
    }
    pub(crate) fn oxidegl_get_uniform_location(
        &mut self,
        program: GLuint,
        name: *const GLchar,
    ) -> GLint {
        panic!("command oxidegl_get_uniform_location not yet implemented");
    }
    pub(crate) fn oxidegl_get_uniformfv(
        &mut self,
        program: GLuint,
        location: GLint,
        params: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_uniformfv not yet implemented");
    }
    pub(crate) fn oxidegl_get_uniformiv(
        &mut self,
        program: GLuint,
        location: GLint,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_uniformiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_attribdv(
        &mut self,
        index: GLuint,
        pname: GLenum,
        params: *mut GLdouble,
    ) {
        panic!("command oxidegl_get_vertex_attribdv not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_attribfv(
        &mut self,
        index: GLuint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_vertex_attribfv not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_attribiv(
        &mut self,
        index: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_vertex_attribiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_attrib_pointerv(
        &mut self,
        index: GLuint,
        pname: GLenum,
        pointer: *mut *mut GLvoid,
    ) {
        panic!("command oxidegl_get_vertex_attrib_pointerv not yet implemented");
    }
    pub(crate) fn oxidegl_is_program(&mut self, program: GLuint) -> GLboolean {
        panic!("command oxidegl_is_program not yet implemented");
    }
    pub(crate) fn oxidegl_is_shader(&mut self, shader: GLuint) -> GLboolean {
        panic!("command oxidegl_is_shader not yet implemented");
    }
    pub(crate) fn oxidegl_link_program(&mut self, program: GLuint) {
        panic!("command oxidegl_link_program not yet implemented");
    }
    pub(crate) fn oxidegl_shader_source(
        &mut self,
        shader: GLuint,
        count: GLsizei,
        string: GLchar,
        length: *const GLint,
    ) {
        panic!("command oxidegl_shader_source not yet implemented");
    }
    pub(crate) fn oxidegl_use_program(&mut self, program: GLuint) {
        panic!("command oxidegl_use_program not yet implemented");
    }
    pub(crate) fn oxidegl_uniform1f(&mut self, location: GLint, v0: GLfloat) {
        panic!("command oxidegl_uniform1f not yet implemented");
    }
    pub(crate) fn oxidegl_uniform2f(&mut self, location: GLint, v0: GLfloat, v1: GLfloat) {
        panic!("command oxidegl_uniform2f not yet implemented");
    }
    pub(crate) fn oxidegl_uniform3f(
        &mut self,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
    ) {
        panic!("command oxidegl_uniform3f not yet implemented");
    }
    pub(crate) fn oxidegl_uniform4f(
        &mut self,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) {
        panic!("command oxidegl_uniform4f not yet implemented");
    }
    pub(crate) fn oxidegl_uniform1i(&mut self, location: GLint, v0: GLint) {
        panic!("command oxidegl_uniform1i not yet implemented");
    }
    pub(crate) fn oxidegl_uniform2i(&mut self, location: GLint, v0: GLint, v1: GLint) {
        panic!("command oxidegl_uniform2i not yet implemented");
    }
    pub(crate) fn oxidegl_uniform3i(&mut self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
        panic!("command oxidegl_uniform3i not yet implemented");
    }
    pub(crate) fn oxidegl_uniform4i(
        &mut self,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint,
    ) {
        panic!("command oxidegl_uniform4i not yet implemented");
    }
    pub(crate) fn oxidegl_uniform1fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform1fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform2fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform2fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform3fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform3fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform4fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform4fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform1iv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        panic!("command oxidegl_uniform1iv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform2iv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        panic!("command oxidegl_uniform2iv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform3iv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        panic!("command oxidegl_uniform3iv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform4iv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        panic!("command oxidegl_uniform4iv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix2fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform_matrix2fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix3fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform_matrix3fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix4fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform_matrix4fv not yet implemented");
    }
    pub(crate) fn oxidegl_validate_program(&mut self, program: GLuint) {
        panic!("command oxidegl_validate_program not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib1d(&mut self, index: GLuint, x: GLdouble) {
        panic!("command oxidegl_vertex_attrib1d not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib1dv(&mut self, index: GLuint, v: *const GLdouble) {
        panic!("command oxidegl_vertex_attrib1dv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib1f(&mut self, index: GLuint, x: GLfloat) {
        panic!("command oxidegl_vertex_attrib1f not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib1fv(&mut self, index: GLuint, v: *const GLfloat) {
        panic!("command oxidegl_vertex_attrib1fv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib1s(&mut self, index: GLuint, x: GLshort) {
        panic!("command oxidegl_vertex_attrib1s not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib1sv(&mut self, index: GLuint, v: *const GLshort) {
        panic!("command oxidegl_vertex_attrib1sv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib2d(&mut self, index: GLuint, x: GLdouble, y: GLdouble) {
        panic!("command oxidegl_vertex_attrib2d not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib2dv(&mut self, index: GLuint, v: *const GLdouble) {
        panic!("command oxidegl_vertex_attrib2dv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib2f(&mut self, index: GLuint, x: GLfloat, y: GLfloat) {
        panic!("command oxidegl_vertex_attrib2f not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib2fv(&mut self, index: GLuint, v: *const GLfloat) {
        panic!("command oxidegl_vertex_attrib2fv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib2s(&mut self, index: GLuint, x: GLshort, y: GLshort) {
        panic!("command oxidegl_vertex_attrib2s not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib2sv(&mut self, index: GLuint, v: *const GLshort) {
        panic!("command oxidegl_vertex_attrib2sv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib3d(
        &mut self,
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
    ) {
        panic!("command oxidegl_vertex_attrib3d not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib3dv(&mut self, index: GLuint, v: *const GLdouble) {
        panic!("command oxidegl_vertex_attrib3dv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib3f(
        &mut self,
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
    ) {
        panic!("command oxidegl_vertex_attrib3f not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib3fv(&mut self, index: GLuint, v: *const GLfloat) {
        panic!("command oxidegl_vertex_attrib3fv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib3s(
        &mut self,
        index: GLuint,
        x: GLshort,
        y: GLshort,
        z: GLshort,
    ) {
        panic!("command oxidegl_vertex_attrib3s not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib3sv(&mut self, index: GLuint, v: *const GLshort) {
        panic!("command oxidegl_vertex_attrib3sv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4_nbv(&mut self, index: GLuint, v: *const GLbyte) {
        panic!("command oxidegl_vertex_attrib4_nbv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4_niv(&mut self, index: GLuint, v: *const GLint) {
        panic!("command oxidegl_vertex_attrib4_niv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4_nsv(&mut self, index: GLuint, v: *const GLshort) {
        panic!("command oxidegl_vertex_attrib4_nsv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4_nub(
        &mut self,
        index: GLuint,
        x: GLubyte,
        y: GLubyte,
        z: GLubyte,
        w: GLubyte,
    ) {
        panic!("command oxidegl_vertex_attrib4_nub not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4_nubv(&mut self, index: GLuint, v: *const GLubyte) {
        panic!("command oxidegl_vertex_attrib4_nubv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4_nuiv(&mut self, index: GLuint, v: *const GLuint) {
        panic!("command oxidegl_vertex_attrib4_nuiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4_nusv(&mut self, index: GLuint, v: *const GLushort) {
        panic!("command oxidegl_vertex_attrib4_nusv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4bv(&mut self, index: GLuint, v: *const GLbyte) {
        panic!("command oxidegl_vertex_attrib4bv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4d(
        &mut self,
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble,
    ) {
        panic!("command oxidegl_vertex_attrib4d not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4dv(&mut self, index: GLuint, v: *const GLdouble) {
        panic!("command oxidegl_vertex_attrib4dv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4f(
        &mut self,
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) {
        panic!("command oxidegl_vertex_attrib4f not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4fv(&mut self, index: GLuint, v: *const GLfloat) {
        panic!("command oxidegl_vertex_attrib4fv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4iv(&mut self, index: GLuint, v: *const GLint) {
        panic!("command oxidegl_vertex_attrib4iv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4s(
        &mut self,
        index: GLuint,
        x: GLshort,
        y: GLshort,
        z: GLshort,
        w: GLshort,
    ) {
        panic!("command oxidegl_vertex_attrib4s not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4sv(&mut self, index: GLuint, v: *const GLshort) {
        panic!("command oxidegl_vertex_attrib4sv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4ubv(&mut self, index: GLuint, v: *const GLubyte) {
        panic!("command oxidegl_vertex_attrib4ubv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4uiv(&mut self, index: GLuint, v: *const GLuint) {
        panic!("command oxidegl_vertex_attrib4uiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib4usv(&mut self, index: GLuint, v: *const GLushort) {
        panic!("command oxidegl_vertex_attrib4usv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_pointer(
        &mut self,
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const GLvoid,
    ) {
        panic!("command oxidegl_vertex_attrib_pointer not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix2x3fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform_matrix2x3fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix3x2fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform_matrix3x2fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix2x4fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform_matrix2x4fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix4x2fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform_matrix4x2fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix3x4fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform_matrix3x4fv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix4x3fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_uniform_matrix4x3fv not yet implemented");
    }
    pub(crate) fn oxidegl_color_maski(
        &mut self,
        index: GLuint,
        r: GLboolean,
        g: GLboolean,
        b: GLboolean,
        a: GLboolean,
    ) {
        panic!("command oxidegl_color_maski not yet implemented");
    }
    pub(crate) fn oxidegl_get_booleani_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLboolean,
    ) {
        panic!("command oxidegl_get_booleani_v not yet implemented");
    }
    pub(crate) fn oxidegl_enablei(&mut self, target: GLenum, index: GLuint) {
        panic!("command oxidegl_enablei not yet implemented");
    }
    pub(crate) fn oxidegl_disablei(&mut self, target: GLenum, index: GLuint) {
        panic!("command oxidegl_disablei not yet implemented");
    }
    pub(crate) fn oxidegl_is_enabledi(&mut self, target: GLenum, index: GLuint) -> GLboolean {
        panic!("command oxidegl_is_enabledi not yet implemented");
    }
    pub(crate) fn oxidegl_begin_transform_feedback(&mut self, primitive_mode: GLenum) {
        panic!("command oxidegl_begin_transform_feedback not yet implemented");
    }
    pub(crate) fn oxidegl_end_transform_feedback(&mut self) {
        panic!("command oxidegl_end_transform_feedback not yet implemented");
    }
    pub(crate) fn oxidegl_transform_feedback_varyings(
        &mut self,
        program: GLuint,
        count: GLsizei,
        varyings: GLchar,
        buffer_mode: GLenum,
    ) {
        panic!("command oxidegl_transform_feedback_varyings not yet implemented");
    }
    pub(crate) fn oxidegl_get_transform_feedback_varying(
        &mut self,
        program: GLuint,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        r#type: *mut GLenum,
        name: *mut GLchar,
    ) {
        panic!("command oxidegl_get_transform_feedback_varying not yet implemented");
    }
    pub(crate) fn oxidegl_clamp_color(&mut self, target: GLenum, clamp: GLenum) {
        panic!("command oxidegl_clamp_color not yet implemented");
    }
    pub(crate) fn oxidegl_begin_conditional_render(&mut self, id: GLuint, mode: GLenum) {
        panic!("command oxidegl_begin_conditional_render not yet implemented");
    }
    pub(crate) fn oxidegl_end_conditional_render(&mut self) {
        panic!("command oxidegl_end_conditional_render not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i_pointer(
        &mut self,
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        stride: GLsizei,
        pointer: *const GLvoid,
    ) {
        panic!("command oxidegl_vertex_attrib_i_pointer not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_attrib_iiv(
        &mut self,
        index: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_vertex_attrib_iiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_attrib_iuiv(
        &mut self,
        index: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) {
        panic!("command oxidegl_get_vertex_attrib_iuiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i1i(&mut self, index: GLuint, x: GLint) {
        panic!("command oxidegl_vertex_attrib_i1i not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i2i(&mut self, index: GLuint, x: GLint, y: GLint) {
        panic!("command oxidegl_vertex_attrib_i2i not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i3i(
        &mut self,
        index: GLuint,
        x: GLint,
        y: GLint,
        z: GLint,
    ) {
        panic!("command oxidegl_vertex_attrib_i3i not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i4i(
        &mut self,
        index: GLuint,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    ) {
        panic!("command oxidegl_vertex_attrib_i4i not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i1ui(&mut self, index: GLuint, x: GLuint) {
        panic!("command oxidegl_vertex_attrib_i1ui not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i2ui(&mut self, index: GLuint, x: GLuint, y: GLuint) {
        panic!("command oxidegl_vertex_attrib_i2ui not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i3ui(
        &mut self,
        index: GLuint,
        x: GLuint,
        y: GLuint,
        z: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_i3ui not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i4ui(
        &mut self,
        index: GLuint,
        x: GLuint,
        y: GLuint,
        z: GLuint,
        w: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_i4ui not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i1iv(&mut self, index: GLuint, v: *const GLint) {
        panic!("command oxidegl_vertex_attrib_i1iv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i2iv(&mut self, index: GLuint, v: *const GLint) {
        panic!("command oxidegl_vertex_attrib_i2iv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i3iv(&mut self, index: GLuint, v: *const GLint) {
        panic!("command oxidegl_vertex_attrib_i3iv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i4iv(&mut self, index: GLuint, v: *const GLint) {
        panic!("command oxidegl_vertex_attrib_i4iv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i1uiv(&mut self, index: GLuint, v: *const GLuint) {
        panic!("command oxidegl_vertex_attrib_i1uiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i2uiv(&mut self, index: GLuint, v: *const GLuint) {
        panic!("command oxidegl_vertex_attrib_i2uiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i3uiv(&mut self, index: GLuint, v: *const GLuint) {
        panic!("command oxidegl_vertex_attrib_i3uiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i4uiv(&mut self, index: GLuint, v: *const GLuint) {
        panic!("command oxidegl_vertex_attrib_i4uiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i4bv(&mut self, index: GLuint, v: *const GLbyte) {
        panic!("command oxidegl_vertex_attrib_i4bv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i4sv(&mut self, index: GLuint, v: *const GLshort) {
        panic!("command oxidegl_vertex_attrib_i4sv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i4ubv(&mut self, index: GLuint, v: *const GLubyte) {
        panic!("command oxidegl_vertex_attrib_i4ubv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i4usv(&mut self, index: GLuint, v: *const GLushort) {
        panic!("command oxidegl_vertex_attrib_i4usv not yet implemented");
    }
    pub(crate) fn oxidegl_get_uniformuiv(
        &mut self,
        program: GLuint,
        location: GLint,
        params: *mut GLuint,
    ) {
        panic!("command oxidegl_get_uniformuiv not yet implemented");
    }
    pub(crate) fn oxidegl_bind_frag_data_location(
        &mut self,
        program: GLuint,
        color: GLuint,
        name: *const GLchar,
    ) {
        panic!("command oxidegl_bind_frag_data_location not yet implemented");
    }
    pub(crate) fn oxidegl_get_frag_data_location(
        &mut self,
        program: GLuint,
        name: *const GLchar,
    ) -> GLint {
        panic!("command oxidegl_get_frag_data_location not yet implemented");
    }
    pub(crate) fn oxidegl_uniform1ui(&mut self, location: GLint, v0: GLuint) {
        panic!("command oxidegl_uniform1ui not yet implemented");
    }
    pub(crate) fn oxidegl_uniform2ui(&mut self, location: GLint, v0: GLuint, v1: GLuint) {
        panic!("command oxidegl_uniform2ui not yet implemented");
    }
    pub(crate) fn oxidegl_uniform3ui(
        &mut self,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) {
        panic!("command oxidegl_uniform3ui not yet implemented");
    }
    pub(crate) fn oxidegl_uniform4ui(
        &mut self,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        panic!("command oxidegl_uniform4ui not yet implemented");
    }
    pub(crate) fn oxidegl_uniform1uiv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_uniform1uiv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform2uiv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_uniform2uiv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform3uiv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_uniform3uiv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform4uiv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_uniform4uiv not yet implemented");
    }
    pub(crate) fn oxidegl_tex_parameter_iiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *const GLint,
    ) {
        panic!("command oxidegl_tex_parameter_iiv not yet implemented");
    }
    pub(crate) fn oxidegl_tex_parameter_iuiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *const GLuint,
    ) {
        panic!("command oxidegl_tex_parameter_iuiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_tex_parameter_iiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_tex_parameter_iiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_tex_parameter_iuiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLuint,
    ) {
        panic!("command oxidegl_get_tex_parameter_iuiv not yet implemented");
    }
    pub(crate) fn oxidegl_clear_bufferiv(
        &mut self,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLint,
    ) {
        panic!("command oxidegl_clear_bufferiv not yet implemented");
    }
    pub(crate) fn oxidegl_clear_bufferuiv(
        &mut self,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_clear_bufferuiv not yet implemented");
    }
    pub(crate) fn oxidegl_clear_bufferfv(
        &mut self,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_clear_bufferfv not yet implemented");
    }
    pub(crate) fn oxidegl_clear_bufferfi(
        &mut self,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        panic!("command oxidegl_clear_bufferfi not yet implemented");
    }

    pub(crate) fn oxidegl_is_renderbuffer(&mut self, renderbuffer: GLuint) -> GLboolean {
        panic!("command oxidegl_is_renderbuffer not yet implemented");
    }
    pub(crate) fn oxidegl_bind_renderbuffer(&mut self, target: GLenum, renderbuffer: GLuint) {
        panic!("command oxidegl_bind_renderbuffer not yet implemented");
    }
    pub(crate) fn oxidegl_delete_renderbuffers(
        &mut self,
        n: GLsizei,
        renderbuffers: *const GLuint,
    ) {
        panic!("command oxidegl_delete_renderbuffers not yet implemented");
    }
    pub(crate) fn oxidegl_gen_renderbuffers(&mut self, n: GLsizei, renderbuffers: *mut GLuint) {
        panic!("command oxidegl_gen_renderbuffers not yet implemented");
    }
    pub(crate) fn oxidegl_renderbuffer_storage(
        &mut self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_renderbuffer_storage not yet implemented");
    }
    pub(crate) fn oxidegl_get_renderbuffer_parameteriv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_renderbuffer_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_is_framebuffer(&mut self, framebuffer: GLuint) -> GLboolean {
        panic!("command oxidegl_is_framebuffer not yet implemented");
    }
    pub(crate) fn oxidegl_bind_framebuffer(&mut self, target: GLenum, framebuffer: GLuint) {
        panic!("command oxidegl_bind_framebuffer not yet implemented");
    }
    pub(crate) fn oxidegl_delete_framebuffers(&mut self, n: GLsizei, framebuffers: *const GLuint) {
        panic!("command oxidegl_delete_framebuffers not yet implemented");
    }
    pub(crate) fn oxidegl_gen_framebuffers(&mut self, n: GLsizei, framebuffers: *mut GLuint) {
        panic!("command oxidegl_gen_framebuffers not yet implemented");
    }
    pub(crate) fn oxidegl_check_framebuffer_status(&mut self, target: GLenum) -> GLenum {
        panic!("command oxidegl_check_framebuffer_status not yet implemented");
    }
    pub(crate) fn oxidegl_framebuffer_texture1_d(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        panic!("command oxidegl_framebuffer_texture1_d not yet implemented");
    }
    pub(crate) fn oxidegl_framebuffer_texture2_d(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        panic!("command oxidegl_framebuffer_texture2_d not yet implemented");
    }
    pub(crate) fn oxidegl_framebuffer_texture3_d(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
        zoffset: GLint,
    ) {
        panic!("command oxidegl_framebuffer_texture3_d not yet implemented");
    }
    pub(crate) fn oxidegl_framebuffer_renderbuffer(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    ) {
        panic!("command oxidegl_framebuffer_renderbuffer not yet implemented");
    }
    pub(crate) fn oxidegl_get_framebuffer_attachment_parameteriv(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_framebuffer_attachment_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_generate_mipmap(&mut self, target: GLenum) {
        panic!("command oxidegl_generate_mipmap not yet implemented");
    }
    pub(crate) fn oxidegl_blit_framebuffer(
        &mut self,
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
        panic!("command oxidegl_blit_framebuffer not yet implemented");
    }
    pub(crate) fn oxidegl_renderbuffer_storage_multisample(
        &mut self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_renderbuffer_storage_multisample not yet implemented");
    }
    pub(crate) fn oxidegl_framebuffer_texture_layer(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) {
        panic!("command oxidegl_framebuffer_texture_layer not yet implemented");
    }
    pub(crate) fn oxidegl_map_buffer_range(
        &mut self,
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) -> *mut GLvoid {
        panic!("command oxidegl_map_buffer_range not yet implemented");
    }
    pub(crate) fn oxidegl_flush_mapped_buffer_range(
        &mut self,
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
    ) {
        panic!("command oxidegl_flush_mapped_buffer_range not yet implemented");
    }
    pub(crate) fn oxidegl_bind_vertex_array(&mut self, array: GLuint) {
        panic!("command oxidegl_bind_vertex_array not yet implemented");
    }
    pub(crate) fn oxidegl_delete_vertex_arrays(&mut self, n: GLsizei, arrays: *const GLuint) {
        panic!("command oxidegl_delete_vertex_arrays not yet implemented");
    }
    pub(crate) fn oxidegl_gen_vertex_arrays(&mut self, n: GLsizei, arrays: *mut GLuint) {
        panic!("command oxidegl_gen_vertex_arrays not yet implemented");
    }
    pub(crate) fn oxidegl_is_vertex_array(&mut self, array: GLuint) -> GLboolean {
        panic!("command oxidegl_is_vertex_array not yet implemented");
    }
    pub(crate) fn oxidegl_draw_arrays_instanced(
        &mut self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    ) {
        panic!("command oxidegl_draw_arrays_instanced not yet implemented");
    }
    pub(crate) fn oxidegl_draw_elements_instanced(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const GLvoid,
        instancecount: GLsizei,
    ) {
        panic!("command oxidegl_draw_elements_instanced not yet implemented");
    }
    pub(crate) fn oxidegl_tex_buffer(
        &mut self,
        target: GLenum,
        internalformat: GLenum,
        buffer: GLuint,
    ) {
        panic!("command oxidegl_tex_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_primitive_restart_index(&mut self, index: GLuint) {
        panic!("command oxidegl_primitive_restart_index not yet implemented");
    }
    pub(crate) fn oxidegl_copy_buffer_sub_data(
        &mut self,
        read_target: GLenum,
        write_target: GLenum,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        panic!("command oxidegl_copy_buffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_get_uniform_indices(
        &mut self,
        program: GLuint,
        uniform_count: GLsizei,
        uniform_names: GLchar,
        uniform_indices: *mut GLuint,
    ) {
        panic!("command oxidegl_get_uniform_indices not yet implemented");
    }
    pub(crate) fn oxidegl_get_active_uniformsiv(
        &mut self,
        program: GLuint,
        uniform_count: GLsizei,
        uniform_indices: *const GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_active_uniformsiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_active_uniform_name(
        &mut self,
        program: GLuint,
        uniform_index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        uniform_name: *mut GLchar,
    ) {
        panic!("command oxidegl_get_active_uniform_name not yet implemented");
    }
    pub(crate) fn oxidegl_get_uniform_block_index(
        &mut self,
        program: GLuint,
        uniform_block_name: *const GLchar,
    ) -> GLuint {
        panic!("command oxidegl_get_uniform_block_index not yet implemented");
    }
    pub(crate) fn oxidegl_get_active_uniform_blockiv(
        &mut self,
        program: GLuint,
        uniform_block_index: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_active_uniform_blockiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_active_uniform_block_name(
        &mut self,
        program: GLuint,
        uniform_block_index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        uniform_block_name: *mut GLchar,
    ) {
        panic!("command oxidegl_get_active_uniform_block_name not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_block_binding(
        &mut self,
        program: GLuint,
        uniform_block_index: GLuint,
        uniform_block_binding: GLuint,
    ) {
        panic!("command oxidegl_uniform_block_binding not yet implemented");
    }
    pub(crate) fn oxidegl_bind_buffer_range(
        &mut self,
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        panic!("command oxidegl_bind_buffer_range not yet implemented");
    }
    pub(crate) fn oxidegl_bind_buffer_base(
        &mut self,
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
    ) {
        panic!("command oxidegl_bind_buffer_base not yet implemented");
    }
    pub(crate) fn oxidegl_get_integeri_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLint,
    ) {
        panic!("command oxidegl_get_integeri_v not yet implemented");
    }
    pub(crate) fn oxidegl_draw_elements_base_vertex(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const GLvoid,
        basevertex: GLint,
    ) {
        panic!("command oxidegl_draw_elements_base_vertex not yet implemented");
    }
    pub(crate) fn oxidegl_draw_range_elements_base_vertex(
        &mut self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        r#type: GLenum,
        indices: *const GLvoid,
        basevertex: GLint,
    ) {
        panic!("command oxidegl_draw_range_elements_base_vertex not yet implemented");
    }
    pub(crate) fn oxidegl_draw_elements_instanced_base_vertex(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const GLvoid,
        instancecount: GLsizei,
        basevertex: GLint,
    ) {
        panic!("command oxidegl_draw_elements_instanced_base_vertex not yet implemented");
    }
    pub(crate) fn oxidegl_multi_draw_elements_base_vertex(
        &mut self,
        mode: GLenum,
        count: *const GLsizei,
        r#type: GLenum,
        indices: *mut *const GLvoid,
        drawcount: GLsizei,
        basevertex: *const GLint,
    ) {
        panic!("command oxidegl_multi_draw_elements_base_vertex not yet implemented");
    }
    pub(crate) fn oxidegl_provoking_vertex(&mut self, mode: GLenum) {
        panic!("command oxidegl_provoking_vertex not yet implemented");
    }
    pub(crate) fn oxidegl_fence_sync(&mut self, condition: GLenum, flags: GLbitfield) -> GLsync {
        panic!("command oxidegl_fence_sync not yet implemented");
    }
    pub(crate) fn oxidegl_is_sync(&mut self, sync: GLsync) -> GLboolean {
        panic!("command oxidegl_is_sync not yet implemented");
    }
    pub(crate) fn oxidegl_delete_sync(&mut self, sync: GLsync) {
        panic!("command oxidegl_delete_sync not yet implemented");
    }
    pub(crate) fn oxidegl_client_wait_sync(
        &mut self,
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> GLenum {
        panic!("command oxidegl_client_wait_sync not yet implemented");
    }
    pub(crate) fn oxidegl_wait_sync(&mut self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
        panic!("command oxidegl_wait_sync not yet implemented");
    }
    pub(crate) fn oxidegl_get_integer64v(&mut self, pname: GLenum, data: *mut GLint64) {
        panic!("command oxidegl_get_integer64v not yet implemented");
    }
    pub(crate) fn oxidegl_get_synciv(
        &mut self,
        sync: GLsync,
        pname: GLenum,
        count: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint,
    ) {
        panic!("command oxidegl_get_synciv not yet implemented");
    }
    pub(crate) fn oxidegl_get_integer64i_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLint64,
    ) {
        panic!("command oxidegl_get_integer64i_v not yet implemented");
    }
    pub(crate) fn oxidegl_get_buffer_parameteri64v(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint64,
    ) {
        panic!("command oxidegl_get_buffer_parameteri64v not yet implemented");
    }
    pub(crate) fn oxidegl_framebuffer_texture(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        panic!("command oxidegl_framebuffer_texture not yet implemented");
    }
    pub(crate) fn oxidegl_tex_image2_d_multisample(
        &mut self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        panic!("command oxidegl_tex_image2_d_multisample not yet implemented");
    }
    pub(crate) fn oxidegl_tex_image3_d_multisample(
        &mut self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        panic!("command oxidegl_tex_image3_d_multisample not yet implemented");
    }
    pub(crate) fn oxidegl_get_multisamplefv(
        &mut self,
        pname: GLenum,
        index: GLuint,
        val: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_multisamplefv not yet implemented");
    }
    pub(crate) fn oxidegl_sample_maski(&mut self, mask_number: GLuint, mask: GLbitfield) {
        panic!("command oxidegl_sample_maski not yet implemented");
    }
    pub(crate) fn oxidegl_bind_frag_data_location_indexed(
        &mut self,
        program: GLuint,
        color_number: GLuint,
        index: GLuint,
        name: *const GLchar,
    ) {
        panic!("command oxidegl_bind_frag_data_location_indexed not yet implemented");
    }
    pub(crate) fn oxidegl_get_frag_data_index(
        &mut self,
        program: GLuint,
        name: *const GLchar,
    ) -> GLint {
        panic!("command oxidegl_get_frag_data_index not yet implemented");
    }
    pub(crate) fn oxidegl_gen_samplers(&mut self, count: GLsizei, samplers: *mut GLuint) {
        panic!("command oxidegl_gen_samplers not yet implemented");
    }
    pub(crate) fn oxidegl_delete_samplers(&mut self, count: GLsizei, samplers: *const GLuint) {
        panic!("command oxidegl_delete_samplers not yet implemented");
    }
    pub(crate) fn oxidegl_is_sampler(&mut self, sampler: GLuint) -> GLboolean {
        panic!("command oxidegl_is_sampler not yet implemented");
    }
    pub(crate) fn oxidegl_bind_sampler(&mut self, unit: GLuint, sampler: GLuint) {
        panic!("command oxidegl_bind_sampler not yet implemented");
    }
    pub(crate) fn oxidegl_sampler_parameteri(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: GLint,
    ) {
        panic!("command oxidegl_sampler_parameteri not yet implemented");
    }
    pub(crate) fn oxidegl_sampler_parameteriv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint,
    ) {
        panic!("command oxidegl_sampler_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_sampler_parameterf(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: GLfloat,
    ) {
        panic!("command oxidegl_sampler_parameterf not yet implemented");
    }
    pub(crate) fn oxidegl_sampler_parameterfv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLfloat,
    ) {
        panic!("command oxidegl_sampler_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_sampler_parameter_iiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint,
    ) {
        panic!("command oxidegl_sampler_parameter_iiv not yet implemented");
    }
    pub(crate) fn oxidegl_sampler_parameter_iuiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLuint,
    ) {
        panic!("command oxidegl_sampler_parameter_iuiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_sampler_parameteriv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_sampler_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_get_sampler_parameter_iiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_sampler_parameter_iiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_sampler_parameterfv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_sampler_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_get_sampler_parameter_iuiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) {
        panic!("command oxidegl_get_sampler_parameter_iuiv not yet implemented");
    }
    pub(crate) fn oxidegl_query_counter(&mut self, id: GLuint, target: GLenum) {
        panic!("command oxidegl_query_counter not yet implemented");
    }
    pub(crate) fn oxidegl_get_query_objecti64v(
        &mut self,
        id: GLuint,
        pname: GLenum,
        params: *mut GLint64,
    ) {
        panic!("command oxidegl_get_query_objecti64v not yet implemented");
    }
    pub(crate) fn oxidegl_get_query_objectui64v(
        &mut self,
        id: GLuint,
        pname: GLenum,
        params: *mut GLuint64,
    ) {
        panic!("command oxidegl_get_query_objectui64v not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_divisor(&mut self, index: GLuint, divisor: GLuint) {
        panic!("command oxidegl_vertex_attrib_divisor not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_p1ui(
        &mut self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_p1ui not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_p1uiv(
        &mut self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_p1uiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_p2ui(
        &mut self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_p2ui not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_p2uiv(
        &mut self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_p2uiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_p3ui(
        &mut self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_p3ui not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_p3uiv(
        &mut self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_p3uiv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_p4ui(
        &mut self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_p4ui not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_p4uiv(
        &mut self,
        index: GLuint,
        r#type: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_p4uiv not yet implemented");
    }
    pub(crate) fn oxidegl_min_sample_shading(&mut self, value: GLfloat) {
        panic!("command oxidegl_min_sample_shading not yet implemented");
    }
    pub(crate) fn oxidegl_blend_equationi(&mut self, buf: GLuint, mode: GLenum) {
        panic!("command oxidegl_blend_equationi not yet implemented");
    }
    pub(crate) fn oxidegl_blend_equation_separatei(
        &mut self,
        buf: GLuint,
        mode_r_g_b: GLenum,
        mode_alpha: GLenum,
    ) {
        panic!("command oxidegl_blend_equation_separatei not yet implemented");
    }
    pub(crate) fn oxidegl_blend_funci(&mut self, buf: GLuint, src: GLenum, dst: GLenum) {
        panic!("command oxidegl_blend_funci not yet implemented");
    }
    pub(crate) fn oxidegl_blend_func_separatei(
        &mut self,
        buf: GLuint,
        src_r_g_b: GLenum,
        dst_r_g_b: GLenum,
        src_alpha: GLenum,
        dst_alpha: GLenum,
    ) {
        panic!("command oxidegl_blend_func_separatei not yet implemented");
    }
    pub(crate) fn oxidegl_draw_arrays_indirect(&mut self, mode: GLenum, indirect: *const GLvoid) {
        panic!("command oxidegl_draw_arrays_indirect not yet implemented");
    }
    pub(crate) fn oxidegl_draw_elements_indirect(
        &mut self,
        mode: GLenum,
        r#type: GLenum,
        indirect: *const GLvoid,
    ) {
        panic!("command oxidegl_draw_elements_indirect not yet implemented");
    }
    pub(crate) fn oxidegl_uniform1d(&mut self, location: GLint, x: GLdouble) {
        panic!("command oxidegl_uniform1d not yet implemented");
    }
    pub(crate) fn oxidegl_uniform2d(&mut self, location: GLint, x: GLdouble, y: GLdouble) {
        panic!("command oxidegl_uniform2d not yet implemented");
    }
    pub(crate) fn oxidegl_uniform3d(
        &mut self,
        location: GLint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
    ) {
        panic!("command oxidegl_uniform3d not yet implemented");
    }
    pub(crate) fn oxidegl_uniform4d(
        &mut self,
        location: GLint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble,
    ) {
        panic!("command oxidegl_uniform4d not yet implemented");
    }
    pub(crate) fn oxidegl_uniform1dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform1dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform2dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform2dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform3dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform3dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform4dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform4dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix2dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform_matrix2dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix3dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform_matrix3dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix4dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform_matrix4dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix2x3dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform_matrix2x3dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix2x4dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform_matrix2x4dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix3x2dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform_matrix3x2dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix3x4dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform_matrix3x4dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix4x2dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform_matrix4x2dv not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_matrix4x3dv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_uniform_matrix4x3dv not yet implemented");
    }
    pub(crate) fn oxidegl_get_uniformdv(
        &mut self,
        program: GLuint,
        location: GLint,
        params: *mut GLdouble,
    ) {
        panic!("command oxidegl_get_uniformdv not yet implemented");
    }
    pub(crate) fn oxidegl_get_subroutine_uniform_location(
        &mut self,
        program: GLuint,
        shadertype: GLenum,
        name: *const GLchar,
    ) -> GLint {
        panic!("command oxidegl_get_subroutine_uniform_location not yet implemented");
    }
    pub(crate) fn oxidegl_get_subroutine_index(
        &mut self,
        program: GLuint,
        shadertype: GLenum,
        name: *const GLchar,
    ) -> GLuint {
        panic!("command oxidegl_get_subroutine_index not yet implemented");
    }
    pub(crate) fn oxidegl_get_active_subroutine_uniformiv(
        &mut self,
        program: GLuint,
        shadertype: GLenum,
        index: GLuint,
        pname: GLenum,
        values: *mut GLint,
    ) {
        panic!("command oxidegl_get_active_subroutine_uniformiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_active_subroutine_uniform_name(
        &mut self,
        program: GLuint,
        shadertype: GLenum,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    ) {
        panic!("command oxidegl_get_active_subroutine_uniform_name not yet implemented");
    }
    pub(crate) fn oxidegl_get_active_subroutine_name(
        &mut self,
        program: GLuint,
        shadertype: GLenum,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    ) {
        panic!("command oxidegl_get_active_subroutine_name not yet implemented");
    }
    pub(crate) fn oxidegl_uniform_subroutinesuiv(
        &mut self,
        shadertype: GLenum,
        count: GLsizei,
        indices: *const GLuint,
    ) {
        panic!("command oxidegl_uniform_subroutinesuiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_uniform_subroutineuiv(
        &mut self,
        shadertype: GLenum,
        location: GLint,
        params: *mut GLuint,
    ) {
        panic!("command oxidegl_get_uniform_subroutineuiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_stageiv(
        &mut self,
        program: GLuint,
        shadertype: GLenum,
        pname: GLenum,
        values: *mut GLint,
    ) {
        panic!("command oxidegl_get_program_stageiv not yet implemented");
    }
    pub(crate) fn oxidegl_patch_parameteri(&mut self, pname: GLenum, value: GLint) {
        panic!("command oxidegl_patch_parameteri not yet implemented");
    }
    pub(crate) fn oxidegl_patch_parameterfv(&mut self, pname: GLenum, values: *const GLfloat) {
        panic!("command oxidegl_patch_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_bind_transform_feedback(&mut self, target: GLenum, id: GLuint) {
        panic!("command oxidegl_bind_transform_feedback not yet implemented");
    }
    pub(crate) fn oxidegl_delete_transform_feedbacks(&mut self, n: GLsizei, ids: *const GLuint) {
        panic!("command oxidegl_delete_transform_feedbacks not yet implemented");
    }
    pub(crate) fn oxidegl_gen_transform_feedbacks(&mut self, n: GLsizei, ids: *mut GLuint) {
        panic!("command oxidegl_gen_transform_feedbacks not yet implemented");
    }
    pub(crate) fn oxidegl_is_transform_feedback(&mut self, id: GLuint) -> GLboolean {
        panic!("command oxidegl_is_transform_feedback not yet implemented");
    }
    pub(crate) fn oxidegl_pause_transform_feedback(&mut self) {
        panic!("command oxidegl_pause_transform_feedback not yet implemented");
    }
    pub(crate) fn oxidegl_resume_transform_feedback(&mut self) {
        panic!("command oxidegl_resume_transform_feedback not yet implemented");
    }
    pub(crate) fn oxidegl_draw_transform_feedback(&mut self, mode: GLenum, id: GLuint) {
        panic!("command oxidegl_draw_transform_feedback not yet implemented");
    }
    pub(crate) fn oxidegl_draw_transform_feedback_stream(
        &mut self,
        mode: GLenum,
        id: GLuint,
        stream: GLuint,
    ) {
        panic!("command oxidegl_draw_transform_feedback_stream not yet implemented");
    }
    pub(crate) fn oxidegl_begin_query_indexed(
        &mut self,
        target: GLenum,
        index: GLuint,
        id: GLuint,
    ) {
        panic!("command oxidegl_begin_query_indexed not yet implemented");
    }
    pub(crate) fn oxidegl_end_query_indexed(&mut self, target: GLenum, index: GLuint) {
        panic!("command oxidegl_end_query_indexed not yet implemented");
    }
    pub(crate) fn oxidegl_get_query_indexediv(
        &mut self,
        target: GLenum,
        index: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_query_indexediv not yet implemented");
    }
    pub(crate) fn oxidegl_release_shader_compiler(&mut self) {
        panic!("command oxidegl_release_shader_compiler not yet implemented");
    }
    pub(crate) fn oxidegl_shader_binary(
        &mut self,
        count: GLsizei,
        shaders: *const GLuint,
        binary_format: GLenum,
        binary: *const GLvoid,
        length: GLsizei,
    ) {
        panic!("command oxidegl_shader_binary not yet implemented");
    }
    pub(crate) fn oxidegl_get_shader_precision_format(
        &mut self,
        shadertype: GLenum,
        precisiontype: GLenum,
        range: *mut GLint,
        precision: *mut GLint,
    ) {
        panic!("command oxidegl_get_shader_precision_format not yet implemented");
    }
    pub(crate) fn oxidegl_depth_rangef(&mut self, n: GLfloat, f: GLfloat) {
        panic!("command oxidegl_depth_rangef not yet implemented");
    }
    pub(crate) fn oxidegl_clear_depthf(&mut self, d: GLfloat) {
        panic!("command oxidegl_clear_depthf not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_binary(
        &mut self,
        program: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        binary_format: *mut GLenum,
        binary: *mut GLvoid,
    ) {
        panic!("command oxidegl_get_program_binary not yet implemented");
    }
    pub(crate) fn oxidegl_program_binary(
        &mut self,
        program: GLuint,
        binary_format: GLenum,
        binary: *const GLvoid,
        length: GLsizei,
    ) {
        panic!("command oxidegl_program_binary not yet implemented");
    }
    pub(crate) fn oxidegl_use_program_stages(
        &mut self,
        pipeline: GLuint,
        stages: GLbitfield,
        program: GLuint,
    ) {
        panic!("command oxidegl_use_program_stages not yet implemented");
    }
    pub(crate) fn oxidegl_active_shader_program(&mut self, pipeline: GLuint, program: GLuint) {
        panic!("command oxidegl_active_shader_program not yet implemented");
    }
    pub(crate) fn oxidegl_create_shader_programv(
        &mut self,
        r#type: GLenum,
        count: GLsizei,
        strings: GLchar,
    ) -> GLuint {
        panic!("command oxidegl_create_shader_programv not yet implemented");
    }
    pub(crate) fn oxidegl_bind_program_pipeline(&mut self, pipeline: GLuint) {
        panic!("command oxidegl_bind_program_pipeline not yet implemented");
    }
    pub(crate) fn oxidegl_delete_program_pipelines(
        &mut self,
        n: GLsizei,
        pipelines: *const GLuint,
    ) {
        panic!("command oxidegl_delete_program_pipelines not yet implemented");
    }
    pub(crate) fn oxidegl_gen_program_pipelines(&mut self, n: GLsizei, pipelines: *mut GLuint) {
        panic!("command oxidegl_gen_program_pipelines not yet implemented");
    }
    pub(crate) fn oxidegl_is_program_pipeline(&mut self, pipeline: GLuint) -> GLboolean {
        panic!("command oxidegl_is_program_pipeline not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_pipelineiv(
        &mut self,
        pipeline: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_program_pipelineiv not yet implemented");
    }
    pub(crate) fn oxidegl_program_parameteri(
        &mut self,
        program: GLuint,
        pname: GLenum,
        value: GLint,
    ) {
        panic!("command oxidegl_program_parameteri not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform1i(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLint,
    ) {
        panic!("command oxidegl_program_uniform1i not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform1iv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        panic!("command oxidegl_program_uniform1iv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform1f(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
    ) {
        panic!("command oxidegl_program_uniform1f not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform1fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform1fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform1d(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLdouble,
    ) {
        panic!("command oxidegl_program_uniform1d not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform1dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform1dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform1ui(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
    ) {
        panic!("command oxidegl_program_uniform1ui not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform1uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_program_uniform1uiv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform2i(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
    ) {
        panic!("command oxidegl_program_uniform2i not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform2iv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        panic!("command oxidegl_program_uniform2iv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform2f(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
    ) {
        panic!("command oxidegl_program_uniform2f not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform2fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform2fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform2d(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLdouble,
        v1: GLdouble,
    ) {
        panic!("command oxidegl_program_uniform2d not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform2dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform2dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform2ui(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
    ) {
        panic!("command oxidegl_program_uniform2ui not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform2uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_program_uniform2uiv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform3i(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
    ) {
        panic!("command oxidegl_program_uniform3i not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform3iv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        panic!("command oxidegl_program_uniform3iv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform3f(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
    ) {
        panic!("command oxidegl_program_uniform3f not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform3fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform3fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform3d(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLdouble,
        v1: GLdouble,
        v2: GLdouble,
    ) {
        panic!("command oxidegl_program_uniform3d not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform3dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform3dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform3ui(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) {
        panic!("command oxidegl_program_uniform3ui not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform3uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_program_uniform3uiv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform4i(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint,
    ) {
        panic!("command oxidegl_program_uniform4i not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform4iv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        panic!("command oxidegl_program_uniform4iv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform4f(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) {
        panic!("command oxidegl_program_uniform4f not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform4fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform4fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform4d(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLdouble,
        v1: GLdouble,
        v2: GLdouble,
        v3: GLdouble,
    ) {
        panic!("command oxidegl_program_uniform4d not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform4dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform4dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform4ui(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        panic!("command oxidegl_program_uniform4ui not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform4uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_program_uniform4uiv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix2fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform_matrix2fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix3fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform_matrix3fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix4fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform_matrix4fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix2dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform_matrix2dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix3dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform_matrix3dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix4dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform_matrix4dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix2x3fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform_matrix2x3fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix3x2fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform_matrix3x2fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix2x4fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform_matrix2x4fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix4x2fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform_matrix4x2fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix3x4fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform_matrix3x4fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix4x3fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_program_uniform_matrix4x3fv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix2x3dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform_matrix2x3dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix3x2dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform_matrix3x2dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix2x4dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform_matrix2x4dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix4x2dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform_matrix4x2dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix3x4dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform_matrix3x4dv not yet implemented");
    }
    pub(crate) fn oxidegl_program_uniform_matrix4x3dv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble,
    ) {
        panic!("command oxidegl_program_uniform_matrix4x3dv not yet implemented");
    }
    pub(crate) fn oxidegl_validate_program_pipeline(&mut self, pipeline: GLuint) {
        panic!("command oxidegl_validate_program_pipeline not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_pipeline_info_log(
        &mut self,
        pipeline: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        info_log: *mut GLchar,
    ) {
        panic!("command oxidegl_get_program_pipeline_info_log not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l1d(&mut self, index: GLuint, x: GLdouble) {
        panic!("command oxidegl_vertex_attrib_l1d not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l2d(&mut self, index: GLuint, x: GLdouble, y: GLdouble) {
        panic!("command oxidegl_vertex_attrib_l2d not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l3d(
        &mut self,
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
    ) {
        panic!("command oxidegl_vertex_attrib_l3d not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l4d(
        &mut self,
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble,
    ) {
        panic!("command oxidegl_vertex_attrib_l4d not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l1dv(&mut self, index: GLuint, v: *const GLdouble) {
        panic!("command oxidegl_vertex_attrib_l1dv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l2dv(&mut self, index: GLuint, v: *const GLdouble) {
        panic!("command oxidegl_vertex_attrib_l2dv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l3dv(&mut self, index: GLuint, v: *const GLdouble) {
        panic!("command oxidegl_vertex_attrib_l3dv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l4dv(&mut self, index: GLuint, v: *const GLdouble) {
        panic!("command oxidegl_vertex_attrib_l4dv not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l_pointer(
        &mut self,
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        stride: GLsizei,
        pointer: *const GLvoid,
    ) {
        panic!("command oxidegl_vertex_attrib_l_pointer not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_attrib_ldv(
        &mut self,
        index: GLuint,
        pname: GLenum,
        params: *mut GLdouble,
    ) {
        panic!("command oxidegl_get_vertex_attrib_ldv not yet implemented");
    }
    pub(crate) fn oxidegl_viewport_arrayv(
        &mut self,
        first: GLuint,
        count: GLsizei,
        v: *const GLfloat,
    ) {
        panic!("command oxidegl_viewport_arrayv not yet implemented");
    }
    pub(crate) fn oxidegl_viewport_indexedf(
        &mut self,
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        w: GLfloat,
        h: GLfloat,
    ) {
        panic!("command oxidegl_viewport_indexedf not yet implemented");
    }
    pub(crate) fn oxidegl_viewport_indexedfv(&mut self, index: GLuint, v: *const GLfloat) {
        panic!("command oxidegl_viewport_indexedfv not yet implemented");
    }
    pub(crate) fn oxidegl_scissor_arrayv(
        &mut self,
        first: GLuint,
        count: GLsizei,
        v: *const GLint,
    ) {
        panic!("command oxidegl_scissor_arrayv not yet implemented");
    }
    pub(crate) fn oxidegl_scissor_indexed(
        &mut self,
        index: GLuint,
        left: GLint,
        bottom: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_scissor_indexed not yet implemented");
    }
    pub(crate) fn oxidegl_scissor_indexedv(&mut self, index: GLuint, v: *const GLint) {
        panic!("command oxidegl_scissor_indexedv not yet implemented");
    }
    pub(crate) fn oxidegl_depth_range_arrayv(
        &mut self,
        first: GLuint,
        count: GLsizei,
        v: *const GLdouble,
    ) {
        panic!("command oxidegl_depth_range_arrayv not yet implemented");
    }
    pub(crate) fn oxidegl_depth_range_indexed(&mut self, index: GLuint, n: GLdouble, f: GLdouble) {
        panic!("command oxidegl_depth_range_indexed not yet implemented");
    }
    pub(crate) fn oxidegl_get_floati_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_floati_v not yet implemented");
    }
    pub(crate) fn oxidegl_get_doublei_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLdouble,
    ) {
        panic!("command oxidegl_get_doublei_v not yet implemented");
    }
    pub(crate) fn oxidegl_draw_arrays_instanced_base_instance(
        &mut self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
        baseinstance: GLuint,
    ) {
        panic!("command oxidegl_draw_arrays_instanced_base_instance not yet implemented");
    }
    pub(crate) fn oxidegl_draw_elements_instanced_base_instance(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const GLvoid,
        instancecount: GLsizei,
        baseinstance: GLuint,
    ) {
        panic!("command oxidegl_draw_elements_instanced_base_instance not yet implemented");
    }
    pub(crate) fn oxidegl_draw_elements_instanced_base_vertex_base_instance(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        r#type: GLenum,
        indices: *const GLvoid,
        instancecount: GLsizei,
        basevertex: GLint,
        baseinstance: GLuint,
    ) {
        panic!(
            "command oxidegl_draw_elements_instanced_base_vertex_base_instance not yet implemented"
        );
    }
    pub(crate) fn oxidegl_get_active_atomic_counter_bufferiv(
        &mut self,
        program: GLuint,
        buffer_index: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_active_atomic_counter_bufferiv not yet implemented");
    }
    pub(crate) fn oxidegl_bind_image_texture(
        &mut self,
        unit: GLuint,
        texture: GLuint,
        level: GLint,
        layered: GLboolean,
        layer: GLint,
        access: GLenum,
        format: GLenum,
    ) {
        panic!("command oxidegl_bind_image_texture not yet implemented");
    }
    pub(crate) fn oxidegl_memory_barrier(&mut self, barriers: GLbitfield) {
        panic!("command oxidegl_memory_barrier not yet implemented");
    }
    pub(crate) fn oxidegl_tex_storage1_d(
        &mut self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
    ) {
        panic!("command oxidegl_tex_storage1_d not yet implemented");
    }
    pub(crate) fn oxidegl_tex_storage2_d(
        &mut self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_tex_storage2_d not yet implemented");
    }
    pub(crate) fn oxidegl_tex_storage3_d(
        &mut self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        panic!("command oxidegl_tex_storage3_d not yet implemented");
    }
    pub(crate) fn oxidegl_draw_transform_feedback_instanced(
        &mut self,
        mode: GLenum,
        id: GLuint,
        instancecount: GLsizei,
    ) {
        panic!("command oxidegl_draw_transform_feedback_instanced not yet implemented");
    }
    pub(crate) fn oxidegl_draw_transform_feedback_stream_instanced(
        &mut self,
        mode: GLenum,
        id: GLuint,
        stream: GLuint,
        instancecount: GLsizei,
    ) {
        panic!("command oxidegl_draw_transform_feedback_stream_instanced not yet implemented");
    }
    pub(crate) fn oxidegl_clear_buffer_data(
        &mut self,
        target: GLenum,
        internalformat: GLenum,
        format: GLenum,
        r#type: GLenum,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_clear_buffer_data not yet implemented");
    }
    pub(crate) fn oxidegl_clear_buffer_sub_data(
        &mut self,
        target: GLenum,
        internalformat: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        format: GLenum,
        r#type: GLenum,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_clear_buffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_dispatch_compute(
        &mut self,
        num_groups_x: GLuint,
        num_groups_y: GLuint,
        num_groups_z: GLuint,
    ) {
        panic!("command oxidegl_dispatch_compute not yet implemented");
    }
    pub(crate) fn oxidegl_dispatch_compute_indirect(&mut self, indirect: GLintptr) {
        panic!("command oxidegl_dispatch_compute_indirect not yet implemented");
    }
    pub(crate) fn oxidegl_copy_image_sub_data(
        &mut self,
        src_name: GLuint,
        src_target: GLenum,
        src_level: GLint,
        src_x: GLint,
        src_y: GLint,
        src_z: GLint,
        dst_name: GLuint,
        dst_target: GLenum,
        dst_level: GLint,
        dst_x: GLint,
        dst_y: GLint,
        dst_z: GLint,
        src_width: GLsizei,
        src_height: GLsizei,
        src_depth: GLsizei,
    ) {
        panic!("command oxidegl_copy_image_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_framebuffer_parameteri(
        &mut self,
        target: GLenum,
        pname: GLenum,
        param: GLint,
    ) {
        panic!("command oxidegl_framebuffer_parameteri not yet implemented");
    }
    pub(crate) fn oxidegl_get_framebuffer_parameteriv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_framebuffer_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_invalidate_tex_sub_image(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        panic!("command oxidegl_invalidate_tex_sub_image not yet implemented");
    }
    pub(crate) fn oxidegl_invalidate_tex_image(&mut self, texture: GLuint, level: GLint) {
        panic!("command oxidegl_invalidate_tex_image not yet implemented");
    }
    pub(crate) fn oxidegl_invalidate_buffer_sub_data(
        &mut self,
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr,
    ) {
        panic!("command oxidegl_invalidate_buffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_invalidate_buffer_data(&mut self, buffer: GLuint) {
        panic!("command oxidegl_invalidate_buffer_data not yet implemented");
    }
    pub(crate) fn oxidegl_invalidate_framebuffer(
        &mut self,
        target: GLenum,
        num_attachments: GLsizei,
        attachments: *const GLenum,
    ) {
        panic!("command oxidegl_invalidate_framebuffer not yet implemented");
    }
    pub(crate) fn oxidegl_invalidate_sub_framebuffer(
        &mut self,
        target: GLenum,
        num_attachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_invalidate_sub_framebuffer not yet implemented");
    }
    pub(crate) fn oxidegl_multi_draw_arrays_indirect(
        &mut self,
        mode: GLenum,
        indirect: *const GLvoid,
        drawcount: GLsizei,
        stride: GLsizei,
    ) {
        panic!("command oxidegl_multi_draw_arrays_indirect not yet implemented");
    }
    pub(crate) fn oxidegl_multi_draw_elements_indirect(
        &mut self,
        mode: GLenum,
        r#type: GLenum,
        indirect: *const GLvoid,
        drawcount: GLsizei,
        stride: GLsizei,
    ) {
        panic!("command oxidegl_multi_draw_elements_indirect not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_interfaceiv(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_program_interfaceiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_resource_index(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        name: *const GLchar,
    ) -> GLuint {
        panic!("command oxidegl_get_program_resource_index not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_resource_name(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    ) {
        panic!("command oxidegl_get_program_resource_name not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_resourceiv(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        index: GLuint,
        prop_count: GLsizei,
        props: *const GLenum,
        count: GLsizei,
        length: *mut GLsizei,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_program_resourceiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_resource_location(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        name: *const GLchar,
    ) -> GLint {
        panic!("command oxidegl_get_program_resource_location not yet implemented");
    }
    pub(crate) fn oxidegl_get_program_resource_location_index(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        name: *const GLchar,
    ) -> GLint {
        panic!("command oxidegl_get_program_resource_location_index not yet implemented");
    }
    pub(crate) fn oxidegl_shader_storage_block_binding(
        &mut self,
        program: GLuint,
        storage_block_index: GLuint,
        storage_block_binding: GLuint,
    ) {
        panic!("command oxidegl_shader_storage_block_binding not yet implemented");
    }
    pub(crate) fn oxidegl_tex_buffer_range(
        &mut self,
        target: GLenum,
        internalformat: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        panic!("command oxidegl_tex_buffer_range not yet implemented");
    }
    pub(crate) fn oxidegl_tex_storage2_d_multisample(
        &mut self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        panic!("command oxidegl_tex_storage2_d_multisample not yet implemented");
    }
    pub(crate) fn oxidegl_tex_storage3_d_multisample(
        &mut self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        panic!("command oxidegl_tex_storage3_d_multisample not yet implemented");
    }
    pub(crate) fn oxidegl_texture_view(
        &mut self,
        texture: GLuint,
        target: GLenum,
        origtexture: GLuint,
        internalformat: GLenum,
        minlevel: GLuint,
        numlevels: GLuint,
        minlayer: GLuint,
        numlayers: GLuint,
    ) {
        panic!("command oxidegl_texture_view not yet implemented");
    }
    pub(crate) fn oxidegl_bind_vertex_buffer(
        &mut self,
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) {
        panic!("command oxidegl_bind_vertex_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_format not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_i_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_i_format not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_l_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_l_format not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_attrib_binding(
        &mut self,
        attribindex: GLuint,
        bindingindex: GLuint,
    ) {
        panic!("command oxidegl_vertex_attrib_binding not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_binding_divisor(&mut self, bindingindex: GLuint, divisor: GLuint) {
        panic!("command oxidegl_vertex_binding_divisor not yet implemented");
    }
    pub(crate) fn oxidegl_debug_message_control(
        &mut self,
        source: GLenum,
        r#type: GLenum,
        severity: GLenum,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean,
    ) {
        panic!("command oxidegl_debug_message_control not yet implemented");
    }
    pub(crate) fn oxidegl_debug_message_insert(
        &mut self,
        source: GLenum,
        r#type: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        buf: *const GLchar,
    ) {
        panic!("command oxidegl_debug_message_insert not yet implemented");
    }
    pub(crate) fn oxidegl_debug_message_callback(
        &mut self,
        callback: GLDEBUGPROC,
        user_param: *const GLvoid,
    ) {
        panic!("command oxidegl_debug_message_callback not yet implemented");
    }
    pub(crate) fn oxidegl_get_debug_message_log(
        &mut self,
        count: GLuint,
        buf_size: GLsizei,
        sources: *mut GLenum,
        types: *mut GLenum,
        ids: *mut GLuint,
        severities: *mut GLenum,
        lengths: *mut GLsizei,
        message_log: *mut GLchar,
    ) -> GLuint {
        panic!("command oxidegl_get_debug_message_log not yet implemented");
    }
    pub(crate) fn oxidegl_push_debug_group(
        &mut self,
        source: GLenum,
        id: GLuint,
        length: GLsizei,
        message: *const GLchar,
    ) {
        panic!("command oxidegl_push_debug_group not yet implemented");
    }
    pub(crate) fn oxidegl_pop_debug_group(&mut self) {
        panic!("command oxidegl_pop_debug_group not yet implemented");
    }
    pub(crate) fn oxidegl_object_label(
        &mut self,
        identifier: GLenum,
        name: GLuint,
        length: GLsizei,
        label: *const GLchar,
    ) {
        panic!("command oxidegl_object_label not yet implemented");
    }
    pub(crate) fn oxidegl_get_object_label(
        &mut self,
        identifier: GLenum,
        name: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) {
        panic!("command oxidegl_get_object_label not yet implemented");
    }
    pub(crate) fn oxidegl_object_ptr_label(
        &mut self,
        ptr: *const GLvoid,
        length: GLsizei,
        label: *const GLchar,
    ) {
        panic!("command oxidegl_object_ptr_label not yet implemented");
    }
    pub(crate) fn oxidegl_get_object_ptr_label(
        &mut self,
        ptr: *const GLvoid,
        buf_size: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) {
        panic!("command oxidegl_get_object_ptr_label not yet implemented");
    }
    pub(crate) fn oxidegl_get_pointerv(&mut self, pname: GLenum, params: *mut *mut GLvoid) {
        panic!("command oxidegl_get_pointerv not yet implemented");
    }
    pub(crate) fn oxidegl_buffer_storage(
        &mut self,
        target: GLenum,
        size: GLsizeiptr,
        data: *const GLvoid,
        flags: GLbitfield,
    ) {
        panic!("command oxidegl_buffer_storage not yet implemented");
    }
    pub(crate) fn oxidegl_clear_tex_image(
        &mut self,
        texture: GLuint,
        level: GLint,
        format: GLenum,
        r#type: GLenum,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_clear_tex_image not yet implemented");
    }
    pub(crate) fn oxidegl_clear_tex_sub_image(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        r#type: GLenum,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_clear_tex_sub_image not yet implemented");
    }
    pub(crate) fn oxidegl_bind_buffers_base(
        &mut self,
        target: GLenum,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
    ) {
        panic!("command oxidegl_bind_buffers_base not yet implemented");
    }
    pub(crate) fn oxidegl_bind_buffers_range(
        &mut self,
        target: GLenum,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        sizes: *const GLsizeiptr,
    ) {
        panic!("command oxidegl_bind_buffers_range not yet implemented");
    }
    pub(crate) fn oxidegl_bind_textures(
        &mut self,
        first: GLuint,
        count: GLsizei,
        textures: *const GLuint,
    ) {
        panic!("command oxidegl_bind_textures not yet implemented");
    }
    pub(crate) fn oxidegl_bind_samplers(
        &mut self,
        first: GLuint,
        count: GLsizei,
        samplers: *const GLuint,
    ) {
        panic!("command oxidegl_bind_samplers not yet implemented");
    }
    pub(crate) fn oxidegl_bind_image_textures(
        &mut self,
        first: GLuint,
        count: GLsizei,
        textures: *const GLuint,
    ) {
        panic!("command oxidegl_bind_image_textures not yet implemented");
    }
    pub(crate) fn oxidegl_bind_vertex_buffers(
        &mut self,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        strides: *const GLsizei,
    ) {
        panic!("command oxidegl_bind_vertex_buffers not yet implemented");
    }
    pub(crate) fn oxidegl_clip_control(&mut self, origin: GLenum, depth: GLenum) {
        panic!("command oxidegl_clip_control not yet implemented");
    }
    pub(crate) fn oxidegl_create_transform_feedbacks(&mut self, n: GLsizei, ids: *mut GLuint) {
        panic!("command oxidegl_create_transform_feedbacks not yet implemented");
    }
    pub(crate) fn oxidegl_transform_feedback_buffer_base(
        &mut self,
        xfb: GLuint,
        index: GLuint,
        buffer: GLuint,
    ) {
        panic!("command oxidegl_transform_feedback_buffer_base not yet implemented");
    }
    pub(crate) fn oxidegl_transform_feedback_buffer_range(
        &mut self,
        xfb: GLuint,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        panic!("command oxidegl_transform_feedback_buffer_range not yet implemented");
    }
    pub(crate) fn oxidegl_get_transform_feedbackiv(
        &mut self,
        xfb: GLuint,
        pname: GLenum,
        param: *mut GLint,
    ) {
        panic!("command oxidegl_get_transform_feedbackiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_transform_feedbacki_v(
        &mut self,
        xfb: GLuint,
        pname: GLenum,
        index: GLuint,
        param: *mut GLint,
    ) {
        panic!("command oxidegl_get_transform_feedbacki_v not yet implemented");
    }
    pub(crate) fn oxidegl_get_transform_feedbacki64_v(
        &mut self,
        xfb: GLuint,
        pname: GLenum,
        index: GLuint,
        param: *mut GLint64,
    ) {
        panic!("command oxidegl_get_transform_feedbacki64_v not yet implemented");
    }
    pub(crate) fn oxidegl_create_buffers(&mut self, n: GLsizei, buffers: *mut GLuint) {
        panic!("command oxidegl_create_buffers not yet implemented");
    }
    pub(crate) fn oxidegl_named_buffer_storage(
        &mut self,
        buffer: GLuint,
        size: GLsizeiptr,
        data: *const GLvoid,
        flags: GLbitfield,
    ) {
        panic!("command oxidegl_named_buffer_storage not yet implemented");
    }
    pub(crate) fn oxidegl_named_buffer_data(
        &mut self,
        buffer: GLuint,
        size: GLsizeiptr,
        data: *const GLvoid,
        usage: GLenum,
    ) {
        panic!("command oxidegl_named_buffer_data not yet implemented");
    }
    pub(crate) fn oxidegl_named_buffer_sub_data(
        &mut self,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_named_buffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_copy_named_buffer_sub_data(
        &mut self,
        read_buffer: GLuint,
        write_buffer: GLuint,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        panic!("command oxidegl_copy_named_buffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_clear_named_buffer_data(
        &mut self,
        buffer: GLuint,
        internalformat: GLenum,
        format: GLenum,
        r#type: GLenum,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_clear_named_buffer_data not yet implemented");
    }
    pub(crate) fn oxidegl_clear_named_buffer_sub_data(
        &mut self,
        buffer: GLuint,
        internalformat: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        format: GLenum,
        r#type: GLenum,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_clear_named_buffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_map_named_buffer(
        &mut self,
        buffer: GLuint,
        access: GLenum,
    ) -> *mut GLvoid {
        panic!("command oxidegl_map_named_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_map_named_buffer_range(
        &mut self,
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) -> *mut GLvoid {
        panic!("command oxidegl_map_named_buffer_range not yet implemented");
    }
    pub(crate) fn oxidegl_unmap_named_buffer(&mut self, buffer: GLuint) -> GLboolean {
        panic!("command oxidegl_unmap_named_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_flush_mapped_named_buffer_range(
        &mut self,
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr,
    ) {
        panic!("command oxidegl_flush_mapped_named_buffer_range not yet implemented");
    }
    pub(crate) fn oxidegl_get_named_buffer_parameteriv(
        &mut self,
        buffer: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_named_buffer_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_get_named_buffer_parameteri64v(
        &mut self,
        buffer: GLuint,
        pname: GLenum,
        params: *mut GLint64,
    ) {
        panic!("command oxidegl_get_named_buffer_parameteri64v not yet implemented");
    }
    pub(crate) fn oxidegl_get_named_buffer_pointerv(
        &mut self,
        buffer: GLuint,
        pname: GLenum,
        params: *mut *mut GLvoid,
    ) {
        panic!("command oxidegl_get_named_buffer_pointerv not yet implemented");
    }
    pub(crate) fn oxidegl_get_named_buffer_sub_data(
        &mut self,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *mut GLvoid,
    ) {
        panic!("command oxidegl_get_named_buffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_create_framebuffers(&mut self, n: GLsizei, framebuffers: *mut GLuint) {
        panic!("command oxidegl_create_framebuffers not yet implemented");
    }
    pub(crate) fn oxidegl_named_framebuffer_renderbuffer(
        &mut self,
        framebuffer: GLuint,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    ) {
        panic!("command oxidegl_named_framebuffer_renderbuffer not yet implemented");
    }
    pub(crate) fn oxidegl_named_framebuffer_parameteri(
        &mut self,
        framebuffer: GLuint,
        pname: GLenum,
        param: GLint,
    ) {
        panic!("command oxidegl_named_framebuffer_parameteri not yet implemented");
    }
    pub(crate) fn oxidegl_named_framebuffer_texture(
        &mut self,
        framebuffer: GLuint,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        panic!("command oxidegl_named_framebuffer_texture not yet implemented");
    }
    pub(crate) fn oxidegl_named_framebuffer_texture_layer(
        &mut self,
        framebuffer: GLuint,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) {
        panic!("command oxidegl_named_framebuffer_texture_layer not yet implemented");
    }
    pub(crate) fn oxidegl_named_framebuffer_draw_buffer(
        &mut self,
        framebuffer: GLuint,
        buf: GLenum,
    ) {
        panic!("command oxidegl_named_framebuffer_draw_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_named_framebuffer_draw_buffers(
        &mut self,
        framebuffer: GLuint,
        n: GLsizei,
        bufs: *const GLenum,
    ) {
        panic!("command oxidegl_named_framebuffer_draw_buffers not yet implemented");
    }
    pub(crate) fn oxidegl_named_framebuffer_read_buffer(
        &mut self,
        framebuffer: GLuint,
        src: GLenum,
    ) {
        panic!("command oxidegl_named_framebuffer_read_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_invalidate_named_framebuffer_data(
        &mut self,
        framebuffer: GLuint,
        num_attachments: GLsizei,
        attachments: *const GLenum,
    ) {
        panic!("command oxidegl_invalidate_named_framebuffer_data not yet implemented");
    }
    pub(crate) fn oxidegl_invalidate_named_framebuffer_sub_data(
        &mut self,
        framebuffer: GLuint,
        num_attachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_invalidate_named_framebuffer_sub_data not yet implemented");
    }
    pub(crate) fn oxidegl_clear_named_framebufferiv(
        &mut self,
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLint,
    ) {
        panic!("command oxidegl_clear_named_framebufferiv not yet implemented");
    }
    pub(crate) fn oxidegl_clear_named_framebufferuiv(
        &mut self,
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLuint,
    ) {
        panic!("command oxidegl_clear_named_framebufferuiv not yet implemented");
    }
    pub(crate) fn oxidegl_clear_named_framebufferfv(
        &mut self,
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLfloat,
    ) {
        panic!("command oxidegl_clear_named_framebufferfv not yet implemented");
    }
    pub(crate) fn oxidegl_clear_named_framebufferfi(
        &mut self,
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        panic!("command oxidegl_clear_named_framebufferfi not yet implemented");
    }
    pub(crate) fn oxidegl_blit_named_framebuffer(
        &mut self,
        read_framebuffer: GLuint,
        draw_framebuffer: GLuint,
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
        panic!("command oxidegl_blit_named_framebuffer not yet implemented");
    }
    pub(crate) fn oxidegl_check_named_framebuffer_status(
        &mut self,
        framebuffer: GLuint,
        target: GLenum,
    ) -> GLenum {
        panic!("command oxidegl_check_named_framebuffer_status not yet implemented");
    }
    pub(crate) fn oxidegl_get_named_framebuffer_parameteriv(
        &mut self,
        framebuffer: GLuint,
        pname: GLenum,
        param: *mut GLint,
    ) {
        panic!("command oxidegl_get_named_framebuffer_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_get_named_framebuffer_attachment_parameteriv(
        &mut self,
        framebuffer: GLuint,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_named_framebuffer_attachment_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_create_renderbuffers(&mut self, n: GLsizei, renderbuffers: *mut GLuint) {
        panic!("command oxidegl_create_renderbuffers not yet implemented");
    }
    pub(crate) fn oxidegl_named_renderbuffer_storage(
        &mut self,
        renderbuffer: GLuint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_named_renderbuffer_storage not yet implemented");
    }
    pub(crate) fn oxidegl_named_renderbuffer_storage_multisample(
        &mut self,
        renderbuffer: GLuint,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_named_renderbuffer_storage_multisample not yet implemented");
    }
    pub(crate) fn oxidegl_get_named_renderbuffer_parameteriv(
        &mut self,
        renderbuffer: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_named_renderbuffer_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_create_textures(
        &mut self,
        target: GLenum,
        n: GLsizei,
        textures: *mut GLuint,
    ) {
        panic!("command oxidegl_create_textures not yet implemented");
    }
    pub(crate) fn oxidegl_texture_buffer(
        &mut self,
        texture: GLuint,
        internalformat: GLenum,
        buffer: GLuint,
    ) {
        panic!("command oxidegl_texture_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_texture_buffer_range(
        &mut self,
        texture: GLuint,
        internalformat: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        panic!("command oxidegl_texture_buffer_range not yet implemented");
    }
    pub(crate) fn oxidegl_texture_storage1_d(
        &mut self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
    ) {
        panic!("command oxidegl_texture_storage1_d not yet implemented");
    }
    pub(crate) fn oxidegl_texture_storage2_d(
        &mut self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_texture_storage2_d not yet implemented");
    }
    pub(crate) fn oxidegl_texture_storage3_d(
        &mut self,
        texture: GLuint,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        panic!("command oxidegl_texture_storage3_d not yet implemented");
    }
    pub(crate) fn oxidegl_texture_storage2_d_multisample(
        &mut self,
        texture: GLuint,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        panic!("command oxidegl_texture_storage2_d_multisample not yet implemented");
    }
    pub(crate) fn oxidegl_texture_storage3_d_multisample(
        &mut self,
        texture: GLuint,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        panic!("command oxidegl_texture_storage3_d_multisample not yet implemented");
    }
    pub(crate) fn oxidegl_texture_sub_image1_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const GLvoid,
    ) {
        panic!("command oxidegl_texture_sub_image1_d not yet implemented");
    }
    pub(crate) fn oxidegl_texture_sub_image2_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const GLvoid,
    ) {
        panic!("command oxidegl_texture_sub_image2_d not yet implemented");
    }
    pub(crate) fn oxidegl_texture_sub_image3_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        r#type: GLenum,
        pixels: *const GLvoid,
    ) {
        panic!("command oxidegl_texture_sub_image3_d not yet implemented");
    }
    pub(crate) fn oxidegl_compressed_texture_sub_image1_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_compressed_texture_sub_image1_d not yet implemented");
    }
    pub(crate) fn oxidegl_compressed_texture_sub_image2_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_compressed_texture_sub_image2_d not yet implemented");
    }
    pub(crate) fn oxidegl_compressed_texture_sub_image3_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        panic!("command oxidegl_compressed_texture_sub_image3_d not yet implemented");
    }
    pub(crate) fn oxidegl_copy_texture_sub_image1_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
    ) {
        panic!("command oxidegl_copy_texture_sub_image1_d not yet implemented");
    }
    pub(crate) fn oxidegl_copy_texture_sub_image2_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_copy_texture_sub_image2_d not yet implemented");
    }
    pub(crate) fn oxidegl_copy_texture_sub_image3_d(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        panic!("command oxidegl_copy_texture_sub_image3_d not yet implemented");
    }
    pub(crate) fn oxidegl_texture_parameterf(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        param: GLfloat,
    ) {
        panic!("command oxidegl_texture_parameterf not yet implemented");
    }
    pub(crate) fn oxidegl_texture_parameterfv(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        param: *const GLfloat,
    ) {
        panic!("command oxidegl_texture_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_texture_parameteri(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        param: GLint,
    ) {
        panic!("command oxidegl_texture_parameteri not yet implemented");
    }
    pub(crate) fn oxidegl_texture_parameter_iiv(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        params: *const GLint,
    ) {
        panic!("command oxidegl_texture_parameter_iiv not yet implemented");
    }
    pub(crate) fn oxidegl_texture_parameter_iuiv(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        params: *const GLuint,
    ) {
        panic!("command oxidegl_texture_parameter_iuiv not yet implemented");
    }
    pub(crate) fn oxidegl_texture_parameteriv(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        param: *const GLint,
    ) {
        panic!("command oxidegl_texture_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_generate_texture_mipmap(&mut self, texture: GLuint) {
        panic!("command oxidegl_generate_texture_mipmap not yet implemented");
    }
    pub(crate) fn oxidegl_bind_texture_unit(&mut self, unit: GLuint, texture: GLuint) {
        panic!("command oxidegl_bind_texture_unit not yet implemented");
    }
    pub(crate) fn oxidegl_get_texture_image(
        &mut self,
        texture: GLuint,
        level: GLint,
        format: GLenum,
        r#type: GLenum,
        buf_size: GLsizei,
        pixels: *mut GLvoid,
    ) {
        panic!("command oxidegl_get_texture_image not yet implemented");
    }
    pub(crate) fn oxidegl_get_compressed_texture_image(
        &mut self,
        texture: GLuint,
        level: GLint,
        buf_size: GLsizei,
        pixels: *mut GLvoid,
    ) {
        panic!("command oxidegl_get_compressed_texture_image not yet implemented");
    }
    pub(crate) fn oxidegl_get_texture_level_parameterfv(
        &mut self,
        texture: GLuint,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_texture_level_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_get_texture_level_parameteriv(
        &mut self,
        texture: GLuint,
        level: GLint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_texture_level_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_get_texture_parameterfv(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_texture_parameterfv not yet implemented");
    }
    pub(crate) fn oxidegl_get_texture_parameter_iiv(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_texture_parameter_iiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_texture_parameter_iuiv(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) {
        panic!("command oxidegl_get_texture_parameter_iuiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_texture_parameteriv(
        &mut self,
        texture: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_get_texture_parameteriv not yet implemented");
    }
    pub(crate) fn oxidegl_create_vertex_arrays(&mut self, n: GLsizei, arrays: *mut GLuint) {
        panic!("command oxidegl_create_vertex_arrays not yet implemented");
    }
    pub(crate) fn oxidegl_disable_vertex_array_attrib(&mut self, vaobj: GLuint, index: GLuint) {
        panic!("command oxidegl_disable_vertex_array_attrib not yet implemented");
    }
    pub(crate) fn oxidegl_enable_vertex_array_attrib(&mut self, vaobj: GLuint, index: GLuint) {
        panic!("command oxidegl_enable_vertex_array_attrib not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_array_element_buffer(&mut self, vaobj: GLuint, buffer: GLuint) {
        panic!("command oxidegl_vertex_array_element_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_array_vertex_buffer(
        &mut self,
        vaobj: GLuint,
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) {
        panic!("command oxidegl_vertex_array_vertex_buffer not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_array_vertex_buffers(
        &mut self,
        vaobj: GLuint,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        strides: *const GLsizei,
    ) {
        panic!("command oxidegl_vertex_array_vertex_buffers not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_array_attrib_binding(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        bindingindex: GLuint,
    ) {
        panic!("command oxidegl_vertex_array_attrib_binding not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_array_attrib_format(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_array_attrib_format not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_array_attrib_i_format(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_array_attrib_i_format not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_array_attrib_l_format(
        &mut self,
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        r#type: GLenum,
        relativeoffset: GLuint,
    ) {
        panic!("command oxidegl_vertex_array_attrib_l_format not yet implemented");
    }
    pub(crate) fn oxidegl_vertex_array_binding_divisor(
        &mut self,
        vaobj: GLuint,
        bindingindex: GLuint,
        divisor: GLuint,
    ) {
        panic!("command oxidegl_vertex_array_binding_divisor not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_arrayiv(
        &mut self,
        vaobj: GLuint,
        pname: GLenum,
        param: *mut GLint,
    ) {
        panic!("command oxidegl_get_vertex_arrayiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_array_indexediv(
        &mut self,
        vaobj: GLuint,
        index: GLuint,
        pname: GLenum,
        param: *mut GLint,
    ) {
        panic!("command oxidegl_get_vertex_array_indexediv not yet implemented");
    }
    pub(crate) fn oxidegl_get_vertex_array_indexed64iv(
        &mut self,
        vaobj: GLuint,
        index: GLuint,
        pname: GLenum,
        param: *mut GLint64,
    ) {
        panic!("command oxidegl_get_vertex_array_indexed64iv not yet implemented");
    }
    pub(crate) fn oxidegl_create_samplers(&mut self, n: GLsizei, samplers: *mut GLuint) {
        panic!("command oxidegl_create_samplers not yet implemented");
    }
    pub(crate) fn oxidegl_create_program_pipelines(&mut self, n: GLsizei, pipelines: *mut GLuint) {
        panic!("command oxidegl_create_program_pipelines not yet implemented");
    }
    pub(crate) fn oxidegl_create_queries(&mut self, target: GLenum, n: GLsizei, ids: *mut GLuint) {
        panic!("command oxidegl_create_queries not yet implemented");
    }
    pub(crate) fn oxidegl_get_query_buffer_objecti64v(
        &mut self,
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr,
    ) {
        panic!("command oxidegl_get_query_buffer_objecti64v not yet implemented");
    }
    pub(crate) fn oxidegl_get_query_buffer_objectiv(
        &mut self,
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr,
    ) {
        panic!("command oxidegl_get_query_buffer_objectiv not yet implemented");
    }
    pub(crate) fn oxidegl_get_query_buffer_objectui64v(
        &mut self,
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr,
    ) {
        panic!("command oxidegl_get_query_buffer_objectui64v not yet implemented");
    }
    pub(crate) fn oxidegl_get_query_buffer_objectuiv(
        &mut self,
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr,
    ) {
        panic!("command oxidegl_get_query_buffer_objectuiv not yet implemented");
    }
    pub(crate) fn oxidegl_memory_barrier_by_region(&mut self, barriers: GLbitfield) {
        panic!("command oxidegl_memory_barrier_by_region not yet implemented");
    }
    pub(crate) fn oxidegl_get_texture_sub_image(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        r#type: GLenum,
        buf_size: GLsizei,
        pixels: *mut GLvoid,
    ) {
        panic!("command oxidegl_get_texture_sub_image not yet implemented");
    }
    pub(crate) fn oxidegl_get_compressed_texture_sub_image(
        &mut self,
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        buf_size: GLsizei,
        pixels: *mut GLvoid,
    ) {
        panic!("command oxidegl_get_compressed_texture_sub_image not yet implemented");
    }
    pub(crate) fn oxidegl_get_graphics_reset_status(&mut self) -> GLenum {
        panic!("command oxidegl_get_graphics_reset_status not yet implemented");
    }
    pub(crate) fn oxidegl_getn_compressed_tex_image(
        &mut self,
        target: GLenum,
        lod: GLint,
        buf_size: GLsizei,
        pixels: *mut GLvoid,
    ) {
        panic!("command oxidegl_getn_compressed_tex_image not yet implemented");
    }
    pub(crate) fn oxidegl_getn_tex_image(
        &mut self,
        target: GLenum,
        level: GLint,
        format: GLenum,
        r#type: GLenum,
        buf_size: GLsizei,
        pixels: *mut GLvoid,
    ) {
        panic!("command oxidegl_getn_tex_image not yet implemented");
    }
    pub(crate) fn oxidegl_getn_uniformdv(
        &mut self,
        program: GLuint,
        location: GLint,
        buf_size: GLsizei,
        params: *mut GLdouble,
    ) {
        panic!("command oxidegl_getn_uniformdv not yet implemented");
    }
    pub(crate) fn oxidegl_getn_uniformfv(
        &mut self,
        program: GLuint,
        location: GLint,
        buf_size: GLsizei,
        params: *mut GLfloat,
    ) {
        panic!("command oxidegl_getn_uniformfv not yet implemented");
    }
    pub(crate) fn oxidegl_getn_uniformiv(
        &mut self,
        program: GLuint,
        location: GLint,
        buf_size: GLsizei,
        params: *mut GLint,
    ) {
        panic!("command oxidegl_getn_uniformiv not yet implemented");
    }
    pub(crate) fn oxidegl_getn_uniformuiv(
        &mut self,
        program: GLuint,
        location: GLint,
        buf_size: GLsizei,
        params: *mut GLuint,
    ) {
        panic!("command oxidegl_getn_uniformuiv not yet implemented");
    }
    pub(crate) fn oxidegl_readn_pixels(
        &mut self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        r#type: GLenum,
        buf_size: GLsizei,
        data: *mut GLvoid,
    ) {
        panic!("command oxidegl_readn_pixels not yet implemented");
    }
    pub(crate) fn oxidegl_texture_barrier(&mut self) {
        panic!("command oxidegl_texture_barrier not yet implemented");
    }
    pub(crate) fn oxidegl_specialize_shader(
        &mut self,
        shader: GLuint,
        p_entry_point: *const GLchar,
        num_specialization_constants: GLuint,
        p_constant_index: *const GLuint,
        p_constant_value: *const GLuint,
    ) {
        panic!("command oxidegl_specialize_shader not yet implemented");
    }
    pub(crate) fn oxidegl_multi_draw_arrays_indirect_count(
        &mut self,
        mode: GLenum,
        indirect: *const GLvoid,
        drawcount: GLintptr,
        maxdrawcount: GLsizei,
        stride: GLsizei,
    ) {
        panic!("command oxidegl_multi_draw_arrays_indirect_count not yet implemented");
    }
    pub(crate) fn oxidegl_multi_draw_elements_indirect_count(
        &mut self,
        mode: GLenum,
        r#type: GLenum,
        indirect: *const GLvoid,
        drawcount: GLintptr,
        maxdrawcount: GLsizei,
        stride: GLsizei,
    ) {
        panic!("command oxidegl_multi_draw_elements_indirect_count not yet implemented");
    }
    pub(crate) fn oxidegl_polygon_offset_clamp(
        &mut self,
        factor: GLfloat,
        units: GLfloat,
        clamp: GLfloat,
    ) {
        panic!("command oxidegl_polygon_offset_clamp not yet implemented");
    }
}
