// GL Commands
use super::gltypes::*;
use crate::context::with_ctx;

#[no_mangle]
unsafe extern "C" fn glActiveShaderProgram(pipeline: GLuint, program: GLuint) {
    with_ctx(|mut state| state.oxidegl_active_shader_program(pipeline, program))
}
#[no_mangle]
unsafe extern "C" fn glActiveTexture(texture: GLenum) {
    with_ctx(|mut state| state.oxidegl_active_texture(texture))
}
#[no_mangle]
unsafe extern "C" fn glAttachShader(program: GLuint, shader: GLuint) {
    with_ctx(|mut state| state.oxidegl_attach_shader(program, shader))
}
#[no_mangle]
unsafe extern "C" fn glBeginConditionalRender(id: GLuint, mode: GLenum) {
    with_ctx(|mut state| state.oxidegl_begin_conditional_render(id, mode))
}
#[no_mangle]
unsafe extern "C" fn glBeginQuery(target: GLenum, id: GLuint) {
    with_ctx(|mut state| state.oxidegl_begin_query(target, id))
}
#[no_mangle]
unsafe extern "C" fn glBeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint) {
    with_ctx(|mut state| state.oxidegl_begin_query_indexed(target, index, id))
}
#[no_mangle]
unsafe extern "C" fn glBeginTransformFeedback(primitiveMode: GLenum) {
    with_ctx(|mut state| state.oxidegl_begin_transform_feedback(primitiveMode))
}
#[no_mangle]
unsafe extern "C" fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
    with_ctx(|mut state| state.oxidegl_bind_attrib_location(program, index, name))
}
#[no_mangle]
unsafe extern "C" fn glBindBuffer(target: GLenum, buffer: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_buffer(target, buffer))
}
#[no_mangle]
unsafe extern "C" fn glBindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_buffer_base(target, index, buffer))
}
#[no_mangle]
unsafe extern "C" fn glBindBufferRange(
    target: GLenum,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    with_ctx(|mut state| state.oxidegl_bind_buffer_range(target, index, buffer, offset, size))
}
#[no_mangle]
unsafe extern "C" fn glBindBuffersBase(
    target: GLenum,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_bind_buffers_base(target, first, count, buffers))
}
#[no_mangle]
unsafe extern "C" fn glBindBuffersRange(
    target: GLenum,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    sizes: *const GLsizeiptr,
) {
    with_ctx(|mut state| {
        state.oxidegl_bind_buffers_range(target, first, count, buffers, offsets, sizes)
    })
}
#[no_mangle]
unsafe extern "C" fn glBindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) {
    with_ctx(|mut state| state.oxidegl_bind_frag_data_location(program, color, name))
}
#[no_mangle]
unsafe extern "C" fn glBindFragDataLocationIndexed(
    program: GLuint,
    colorNumber: GLuint,
    index: GLuint,
    name: *const GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_bind_frag_data_location_indexed(program, colorNumber, index, name)
    })
}
#[no_mangle]
unsafe extern "C" fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_framebuffer(target, framebuffer))
}
#[no_mangle]
unsafe extern "C" fn glBindImageTexture(
    unit: GLuint,
    texture: GLuint,
    level: GLint,
    layered: GLboolean,
    layer: GLint,
    access: GLenum,
    format: GLenum,
) {
    with_ctx(|mut state| {
        state.oxidegl_bind_image_texture(unit, texture, level, layered, layer, access, format)
    })
}
#[no_mangle]
unsafe extern "C" fn glBindImageTextures(first: GLuint, count: GLsizei, textures: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_image_textures(first, count, textures))
}
#[no_mangle]
unsafe extern "C" fn glBindProgramPipeline(pipeline: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_program_pipeline(pipeline))
}
#[no_mangle]
unsafe extern "C" fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_renderbuffer(target, renderbuffer))
}
#[no_mangle]
unsafe extern "C" fn glBindSampler(unit: GLuint, sampler: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_sampler(unit, sampler))
}
#[no_mangle]
unsafe extern "C" fn glBindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_samplers(first, count, samplers))
}
#[no_mangle]
unsafe extern "C" fn glBindTexture(target: GLenum, texture: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_texture(target, texture))
}
#[no_mangle]
unsafe extern "C" fn glBindTextures(first: GLuint, count: GLsizei, textures: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_textures(first, count, textures))
}
#[no_mangle]
unsafe extern "C" fn glBindTextureUnit(unit: GLuint, texture: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_texture_unit(unit, texture))
}
#[no_mangle]
unsafe extern "C" fn glBindTransformFeedback(target: GLenum, id: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_transform_feedback(target, id))
}
#[no_mangle]
unsafe extern "C" fn glBindVertexArray(array: GLuint) {
    with_ctx(|mut state| state.oxidegl_bind_vertex_array(array))
}
#[no_mangle]
unsafe extern "C" fn glBindVertexBuffer(
    bindingindex: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    stride: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_bind_vertex_buffer(bindingindex, buffer, offset, stride))
}
#[no_mangle]
unsafe extern "C" fn glVertexArrayVertexBuffer(
    vaobj: GLuint,
    bindingindex: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    stride: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_array_vertex_buffer(vaobj, bindingindex, buffer, offset, stride)
    })
}
#[no_mangle]
unsafe extern "C" fn glBindVertexBuffers(
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    strides: *const GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_bind_vertex_buffers(first, count, buffers, offsets, strides))
}
#[no_mangle]
unsafe extern "C" fn glVertexArrayVertexBuffers(
    vaobj: GLuint,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    strides: *const GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_array_vertex_buffers(vaobj, first, count, buffers, offsets, strides)
    })
}
#[no_mangle]
unsafe extern "C" fn glBlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    with_ctx(|mut state| state.oxidegl_blend_color(red, green, blue, alpha))
}
#[no_mangle]
unsafe extern "C" fn glBlendEquation(mode: GLenum) {
    with_ctx(|mut state| state.oxidegl_blend_equation(mode))
}
#[no_mangle]
unsafe extern "C" fn glBlendEquationi(buf: GLuint, mode: GLenum) {
    with_ctx(|mut state| state.oxidegl_blend_equationi(buf, mode))
}
#[no_mangle]
unsafe extern "C" fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) {
    with_ctx(|mut state| state.oxidegl_blend_equation_separate(modeRGB, modeAlpha))
}
#[no_mangle]
unsafe extern "C" fn glBlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) {
    with_ctx(|mut state| state.oxidegl_blend_equation_separatei(buf, modeRGB, modeAlpha))
}
#[no_mangle]
unsafe extern "C" fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) {
    with_ctx(|mut state| state.oxidegl_blend_func(sfactor, dfactor))
}
#[no_mangle]
unsafe extern "C" fn glBlendFunci(buf: GLuint, src: GLenum, dst: GLenum) {
    with_ctx(|mut state| state.oxidegl_blend_funci(buf, src, dst))
}
#[no_mangle]
unsafe extern "C" fn glBlendFuncSeparate(
    sfactorRGB: GLenum,
    dfactorRGB: GLenum,
    sfactorAlpha: GLenum,
    dfactorAlpha: GLenum,
) {
    with_ctx(|mut state| {
        state.oxidegl_blend_func_separate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
    })
}
#[no_mangle]
unsafe extern "C" fn glBlendFuncSeparatei(
    buf: GLuint,
    srcRGB: GLenum,
    dstRGB: GLenum,
    srcAlpha: GLenum,
    dstAlpha: GLenum,
) {
    with_ctx(|mut state| {
        state.oxidegl_blend_func_separatei(buf, srcRGB, dstRGB, srcAlpha, dstAlpha)
    })
}
#[no_mangle]
unsafe extern "C" fn glBlitFramebuffer(
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLbitfield,
    filter: GLenum,
) {
    with_ctx(|mut state| {
        state.oxidegl_blit_framebuffer(
            srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glBlitNamedFramebuffer(
    readFramebuffer: GLuint,
    drawFramebuffer: GLuint,
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLbitfield,
    filter: GLenum,
) {
    with_ctx(|mut state| {
        state.oxidegl_blit_named_framebuffer(
            readFramebuffer,
            drawFramebuffer,
            srcX0,
            srcY0,
            srcX1,
            srcY1,
            dstX0,
            dstY0,
            dstX1,
            dstY1,
            mask,
            filter,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glBufferData(
    target: GLenum,
    size: GLsizeiptr,
    data: *const GLvoid,
    usage: GLenum,
) {
    with_ctx(|mut state| state.oxidegl_buffer_data(target, size, data, usage))
}
#[no_mangle]
unsafe extern "C" fn glNamedBufferData(
    buffer: GLuint,
    size: GLsizeiptr,
    data: *const GLvoid,
    usage: GLenum,
) {
    with_ctx(|mut state| state.oxidegl_named_buffer_data(buffer, size, data, usage))
}
#[no_mangle]
unsafe extern "C" fn glBufferStorage(
    target: GLenum,
    size: GLsizeiptr,
    data: *const GLvoid,
    flags: GLbitfield,
) {
    with_ctx(|mut state| state.oxidegl_buffer_storage(target, size, data, flags))
}
#[no_mangle]
unsafe extern "C" fn glNamedBufferStorage(
    buffer: GLuint,
    size: GLsizeiptr,
    data: *const GLvoid,
    flags: GLbitfield,
) {
    with_ctx(|mut state| state.oxidegl_named_buffer_storage(buffer, size, data, flags))
}
#[no_mangle]
unsafe extern "C" fn glBufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_buffer_sub_data(target, offset, size, data))
}
#[no_mangle]
unsafe extern "C" fn glNamedBufferSubData(
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_named_buffer_sub_data(buffer, offset, size, data))
}
#[no_mangle]
unsafe extern "C" fn glCheckFramebufferStatus(target: GLenum) -> GLenum {
    with_ctx(|mut state| state.oxidegl_check_framebuffer_status(target))
}
#[no_mangle]
unsafe extern "C" fn glCheckNamedFramebufferStatus(framebuffer: GLuint, target: GLenum) -> GLenum {
    with_ctx(|mut state| state.oxidegl_check_named_framebuffer_status(framebuffer, target))
}
#[no_mangle]
unsafe extern "C" fn glClampColor(target: GLenum, clamp: GLenum) {
    with_ctx(|mut state| state.oxidegl_clamp_color(target, clamp))
}
#[no_mangle]
unsafe extern "C" fn glClear(mask: GLbitfield) {
    with_ctx(|mut state| state.oxidegl_clear(mask))
}
#[no_mangle]
unsafe extern "C" fn glClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
    with_ctx(|mut state| state.oxidegl_clear_bufferiv(buffer, drawbuffer, value))
}
#[no_mangle]
unsafe extern "C" fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_clear_bufferuiv(buffer, drawbuffer, value))
}
#[no_mangle]
unsafe extern "C" fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_clear_bufferfv(buffer, drawbuffer, value))
}
#[no_mangle]
unsafe extern "C" fn glClearBufferfi(
    buffer: GLenum,
    drawbuffer: GLint,
    depth: GLfloat,
    stencil: GLint,
) {
    with_ctx(|mut state| state.oxidegl_clear_bufferfi(buffer, drawbuffer, depth, stencil))
}
#[no_mangle]
unsafe extern "C" fn glClearNamedFramebufferiv(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_clear_named_framebufferiv(framebuffer, buffer, drawbuffer, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glClearNamedFramebufferuiv(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_clear_named_framebufferuiv(framebuffer, buffer, drawbuffer, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glClearNamedFramebufferfv(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_clear_named_framebufferfv(framebuffer, buffer, drawbuffer, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glClearNamedFramebufferfi(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    depth: GLfloat,
    stencil: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_clear_named_framebufferfi(framebuffer, buffer, drawbuffer, depth, stencil)
    })
}
#[no_mangle]
unsafe extern "C" fn glClearBufferData(
    target: GLenum,
    internalformat: GLenum,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_clear_buffer_data(target, internalformat, format, r#type, data)
    })
}
#[no_mangle]
unsafe extern "C" fn glClearNamedBufferData(
    buffer: GLuint,
    internalformat: GLenum,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_clear_named_buffer_data(buffer, internalformat, format, r#type, data)
    })
}
#[no_mangle]
unsafe extern "C" fn glClearBufferSubData(
    target: GLenum,
    internalformat: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_clear_buffer_sub_data(
            target,
            internalformat,
            offset,
            size,
            format,
            r#type,
            data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glClearNamedBufferSubData(
    buffer: GLuint,
    internalformat: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_clear_named_buffer_sub_data(
            buffer,
            internalformat,
            offset,
            size,
            format,
            r#type,
            data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    with_ctx(|mut state| state.oxidegl_clear_color(red, green, blue, alpha))
}
#[no_mangle]
unsafe extern "C" fn glClearDepth(depth: GLdouble) {
    with_ctx(|mut state| state.oxidegl_clear_depth(depth))
}
#[no_mangle]
unsafe extern "C" fn glClearDepthf(d: GLfloat) {
    with_ctx(|mut state| state.oxidegl_clear_depthf(d))
}
#[no_mangle]
unsafe extern "C" fn glClearStencil(s: GLint) {
    with_ctx(|mut state| state.oxidegl_clear_stencil(s))
}
#[no_mangle]
unsafe extern "C" fn glClearTexImage(
    texture: GLuint,
    level: GLint,
    format: GLenum,
    r#type: GLenum,
    data: *const GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_clear_tex_image(texture, level, format, r#type, data))
}
#[no_mangle]
unsafe extern "C" fn glClearTexSubImage(
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
    with_ctx(|mut state| {
        state.oxidegl_clear_tex_sub_image(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glClientWaitSync(
    sync: GLsync,
    flags: GLbitfield,
    timeout: GLuint64,
) -> GLenum {
    with_ctx(|mut state| state.oxidegl_client_wait_sync(sync, flags, timeout))
}
#[no_mangle]
unsafe extern "C" fn glClipControl(origin: GLenum, depth: GLenum) {
    with_ctx(|mut state| state.oxidegl_clip_control(origin, depth))
}
#[no_mangle]
unsafe extern "C" fn glColorMask(
    red: GLboolean,
    green: GLboolean,
    blue: GLboolean,
    alpha: GLboolean,
) {
    with_ctx(|mut state| state.oxidegl_color_mask(red, green, blue, alpha))
}
#[no_mangle]
unsafe extern "C" fn glCompileShader(shader: GLuint) {
    with_ctx(|mut state| state.oxidegl_compile_shader(shader))
}
#[no_mangle]
unsafe extern "C" fn glCompressedTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_compressed_tex_image1_d(
            target,
            level,
            internalformat,
            width,
            border,
            imageSize,
            data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCompressedTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_compressed_tex_image2_d(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            imageSize,
            data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCompressedTexImage3D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_compressed_tex_image3_d(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            imageSize,
            data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCompressedTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_compressed_tex_sub_image1_d(
            target, level, xoffset, width, format, imageSize, data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCompressedTextureSubImage1D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_compressed_texture_sub_image1_d(
            texture, level, xoffset, width, format, imageSize, data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCompressedTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_compressed_tex_sub_image2_d(
            target, level, xoffset, yoffset, width, height, format, imageSize, data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCompressedTextureSubImage2D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_compressed_texture_sub_image2_d(
            texture, level, xoffset, yoffset, width, height, format, imageSize, data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCompressedTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_compressed_tex_sub_image3_d(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCompressedTextureSubImage3D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_compressed_texture_sub_image3_d(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize,
            data,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyBufferSubData(
    readTarget: GLenum,
    writeTarget: GLenum,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
) {
    with_ctx(|mut state| {
        state.oxidegl_copy_buffer_sub_data(readTarget, writeTarget, readOffset, writeOffset, size)
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyNamedBufferSubData(
    readBuffer: GLuint,
    writeBuffer: GLuint,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
) {
    with_ctx(|mut state| {
        state.oxidegl_copy_named_buffer_sub_data(
            readBuffer,
            writeBuffer,
            readOffset,
            writeOffset,
            size,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyImageSubData(
    srcName: GLuint,
    srcTarget: GLenum,
    srcLevel: GLint,
    srcX: GLint,
    srcY: GLint,
    srcZ: GLint,
    dstName: GLuint,
    dstTarget: GLenum,
    dstLevel: GLint,
    dstX: GLint,
    dstY: GLint,
    dstZ: GLint,
    srcWidth: GLsizei,
    srcHeight: GLsizei,
    srcDepth: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_copy_image_sub_data(
            srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX,
            dstY, dstZ, srcWidth, srcHeight, srcDepth,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    border: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_copy_tex_image1_d(target, level, internalformat, x, y, width, border)
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_copy_tex_image2_d(target, level, internalformat, x, y, width, height, border)
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_copy_tex_sub_image1_d(target, level, xoffset, x, y, width))
}
#[no_mangle]
unsafe extern "C" fn glCopyTextureSubImage1D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_copy_texture_sub_image1_d(texture, level, xoffset, x, y, width)
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_copy_tex_sub_image2_d(target, level, xoffset, yoffset, x, y, width, height)
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyTextureSubImage2D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_copy_texture_sub_image2_d(
            texture, level, xoffset, yoffset, x, y, width, height,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyTexSubImage3D(
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
    with_ctx(|mut state| {
        state.oxidegl_copy_tex_sub_image3_d(
            target, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCopyTextureSubImage3D(
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
    with_ctx(|mut state| {
        state.oxidegl_copy_texture_sub_image3_d(
            texture, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glCreateBuffers(n: GLsizei, buffers: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_create_buffers(n, buffers))
}
#[no_mangle]
unsafe extern "C" fn glCreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_create_framebuffers(n, framebuffers))
}
#[no_mangle]
unsafe extern "C" fn glCreateProgram() -> GLuint {
    with_ctx(|mut state| state.oxidegl_create_program())
}
#[no_mangle]
unsafe extern "C" fn glCreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_create_program_pipelines(n, pipelines))
}
#[no_mangle]
unsafe extern "C" fn glCreateQueries(target: GLenum, n: GLsizei, ids: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_create_queries(target, n, ids))
}
#[no_mangle]
unsafe extern "C" fn glCreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_create_renderbuffers(n, renderbuffers))
}
#[no_mangle]
unsafe extern "C" fn glCreateSamplers(n: GLsizei, samplers: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_create_samplers(n, samplers))
}
#[no_mangle]
unsafe extern "C" fn glCreateShader(r#type: GLenum) -> GLuint {
    with_ctx(|mut state| state.oxidegl_create_shader(r#type))
}
#[no_mangle]
unsafe extern "C" fn glCreateShaderProgramv(
    r#type: GLenum,
    count: GLsizei,
    strings: GLchar,
) -> GLuint {
    with_ctx(|mut state| state.oxidegl_create_shader_programv(r#type, count, strings))
}
#[no_mangle]
unsafe extern "C" fn glCreateTextures(target: GLenum, n: GLsizei, textures: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_create_textures(target, n, textures))
}
#[no_mangle]
unsafe extern "C" fn glCreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_create_transform_feedbacks(n, ids))
}
#[no_mangle]
unsafe extern "C" fn glCreateVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_create_vertex_arrays(n, arrays))
}
#[no_mangle]
unsafe extern "C" fn glCullFace(mode: GLenum) {
    with_ctx(|mut state| state.oxidegl_cull_face(mode))
}
#[no_mangle]
unsafe extern "C" fn glDebugMessageCallback(callback: GLDEBUGPROC, userParam: *const GLvoid) {
    with_ctx(|mut state| state.oxidegl_debug_message_callback(callback, userParam))
}
#[no_mangle]
unsafe extern "C" fn glDebugMessageControl(
    source: GLenum,
    r#type: GLenum,
    severity: GLenum,
    count: GLsizei,
    ids: *const GLuint,
    enabled: GLboolean,
) {
    with_ctx(|mut state| {
        state.oxidegl_debug_message_control(source, r#type, severity, count, ids, enabled)
    })
}
#[no_mangle]
unsafe extern "C" fn glDebugMessageInsert(
    source: GLenum,
    r#type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    buf: *const GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_debug_message_insert(source, r#type, id, severity, length, buf)
    })
}
#[no_mangle]
unsafe extern "C" fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_buffers(n, buffers))
}
#[no_mangle]
unsafe extern "C" fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_framebuffers(n, framebuffers))
}
#[no_mangle]
unsafe extern "C" fn glDeleteProgram(program: GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_program(program))
}
#[no_mangle]
unsafe extern "C" fn glDeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_program_pipelines(n, pipelines))
}
#[no_mangle]
unsafe extern "C" fn glDeleteQueries(n: GLsizei, ids: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_queries(n, ids))
}
#[no_mangle]
unsafe extern "C" fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_renderbuffers(n, renderbuffers))
}
#[no_mangle]
unsafe extern "C" fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_samplers(count, samplers))
}
#[no_mangle]
unsafe extern "C" fn glDeleteShader(shader: GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_shader(shader))
}
#[no_mangle]
unsafe extern "C" fn glDeleteSync(sync: GLsync) {
    with_ctx(|mut state| state.oxidegl_delete_sync(sync))
}
#[no_mangle]
unsafe extern "C" fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_textures(n, textures))
}
#[no_mangle]
unsafe extern "C" fn glDeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_transform_feedbacks(n, ids))
}
#[no_mangle]
unsafe extern "C" fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_delete_vertex_arrays(n, arrays))
}
#[no_mangle]
unsafe extern "C" fn glDepthFunc(func: GLenum) {
    with_ctx(|mut state| state.oxidegl_depth_func(func))
}
#[no_mangle]
unsafe extern "C" fn glDepthMask(flag: GLboolean) {
    with_ctx(|mut state| state.oxidegl_depth_mask(flag))
}
#[no_mangle]
unsafe extern "C" fn glDepthRange(n: GLdouble, f: GLdouble) {
    with_ctx(|mut state| state.oxidegl_depth_range(n, f))
}
#[no_mangle]
unsafe extern "C" fn glDepthRangef(n: GLfloat, f: GLfloat) {
    with_ctx(|mut state| state.oxidegl_depth_rangef(n, f))
}
#[no_mangle]
unsafe extern "C" fn glDepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_depth_range_arrayv(first, count, v))
}
#[no_mangle]
unsafe extern "C" fn glDepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) {
    with_ctx(|mut state| state.oxidegl_depth_range_indexed(index, n, f))
}
#[no_mangle]
unsafe extern "C" fn glDetachShader(program: GLuint, shader: GLuint) {
    with_ctx(|mut state| state.oxidegl_detach_shader(program, shader))
}
#[no_mangle]
unsafe extern "C" fn glDispatchCompute(
    num_groups_x: GLuint,
    num_groups_y: GLuint,
    num_groups_z: GLuint,
) {
    with_ctx(|mut state| state.oxidegl_dispatch_compute(num_groups_x, num_groups_y, num_groups_z))
}
#[no_mangle]
unsafe extern "C" fn glDispatchComputeIndirect(indirect: GLintptr) {
    with_ctx(|mut state| state.oxidegl_dispatch_compute_indirect(indirect))
}
#[no_mangle]
unsafe extern "C" fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    with_ctx(|mut state| state.oxidegl_draw_arrays(mode, first, count))
}
#[no_mangle]
unsafe extern "C" fn glDrawArraysIndirect(mode: GLenum, indirect: *const GLvoid) {
    with_ctx(|mut state| state.oxidegl_draw_arrays_indirect(mode, indirect))
}
#[no_mangle]
unsafe extern "C" fn glDrawArraysInstanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instancecount: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_draw_arrays_instanced(mode, first, count, instancecount))
}
#[no_mangle]
unsafe extern "C" fn glDrawArraysInstancedBaseInstance(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instancecount: GLsizei,
    baseinstance: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_draw_arrays_instanced_base_instance(
            mode,
            first,
            count,
            instancecount,
            baseinstance,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glDrawBuffer(buf: GLenum) {
    with_ctx(|mut state| state.oxidegl_draw_buffer(buf))
}
#[no_mangle]
unsafe extern "C" fn glNamedFramebufferDrawBuffer(framebuffer: GLuint, buf: GLenum) {
    with_ctx(|mut state| state.oxidegl_named_framebuffer_draw_buffer(framebuffer, buf))
}
#[no_mangle]
unsafe extern "C" fn glDrawBuffers(n: GLsizei, bufs: *const GLenum) {
    with_ctx(|mut state| state.oxidegl_draw_buffers(n, bufs))
}
#[no_mangle]
unsafe extern "C" fn glNamedFramebufferDrawBuffers(
    framebuffer: GLuint,
    n: GLsizei,
    bufs: *const GLenum,
) {
    with_ctx(|mut state| state.oxidegl_named_framebuffer_draw_buffers(framebuffer, n, bufs))
}
#[no_mangle]
unsafe extern "C" fn glDrawElements(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_draw_elements(mode, count, r#type, indices))
}
#[no_mangle]
unsafe extern "C" fn glDrawElementsBaseVertex(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    basevertex: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_draw_elements_base_vertex(mode, count, r#type, indices, basevertex)
    })
}
#[no_mangle]
unsafe extern "C" fn glDrawElementsIndirect(mode: GLenum, r#type: GLenum, indirect: *const GLvoid) {
    with_ctx(|mut state| state.oxidegl_draw_elements_indirect(mode, r#type, indirect))
}
#[no_mangle]
unsafe extern "C" fn glDrawElementsInstanced(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    instancecount: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_draw_elements_instanced(mode, count, r#type, indices, instancecount)
    })
}
#[no_mangle]
unsafe extern "C" fn glDrawElementsInstancedBaseInstance(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    instancecount: GLsizei,
    baseinstance: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_draw_elements_instanced_base_instance(
            mode,
            count,
            r#type,
            indices,
            instancecount,
            baseinstance,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glDrawElementsInstancedBaseVertex(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    instancecount: GLsizei,
    basevertex: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_draw_elements_instanced_base_vertex(
            mode,
            count,
            r#type,
            indices,
            instancecount,
            basevertex,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glDrawElementsInstancedBaseVertexBaseInstance(
    mode: GLenum,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    instancecount: GLsizei,
    basevertex: GLint,
    baseinstance: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_draw_elements_instanced_base_vertex_base_instance(
            mode,
            count,
            r#type,
            indices,
            instancecount,
            basevertex,
            baseinstance,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glDrawRangeElements(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_draw_range_elements(mode, start, end, count, r#type, indices)
    })
}
#[no_mangle]
unsafe extern "C" fn glDrawRangeElementsBaseVertex(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    r#type: GLenum,
    indices: *const GLvoid,
    basevertex: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_draw_range_elements_base_vertex(
            mode, start, end, count, r#type, indices, basevertex,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glDrawTransformFeedback(mode: GLenum, id: GLuint) {
    with_ctx(|mut state| state.oxidegl_draw_transform_feedback(mode, id))
}
#[no_mangle]
unsafe extern "C" fn glDrawTransformFeedbackInstanced(
    mode: GLenum,
    id: GLuint,
    instancecount: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_draw_transform_feedback_instanced(mode, id, instancecount))
}
#[no_mangle]
unsafe extern "C" fn glDrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint) {
    with_ctx(|mut state| state.oxidegl_draw_transform_feedback_stream(mode, id, stream))
}
#[no_mangle]
unsafe extern "C" fn glDrawTransformFeedbackStreamInstanced(
    mode: GLenum,
    id: GLuint,
    stream: GLuint,
    instancecount: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_draw_transform_feedback_stream_instanced(mode, id, stream, instancecount)
    })
}
#[no_mangle]
unsafe extern "C" fn glEnable(cap: GLenum) {
    with_ctx(|mut state| state.oxidegl_enable(cap))
}
#[no_mangle]
unsafe extern "C" fn glDisableVertexAttribArray(index: GLuint) {
    with_ctx(|mut state| state.oxidegl_disable_vertex_attrib_array(index))
}
#[no_mangle]
unsafe extern "C" fn glEnableVertexAttribArray(index: GLuint) {
    with_ctx(|mut state| state.oxidegl_enable_vertex_attrib_array(index))
}
#[no_mangle]
unsafe extern "C" fn glDisableVertexArrayAttrib(vaobj: GLuint, index: GLuint) {
    with_ctx(|mut state| state.oxidegl_disable_vertex_array_attrib(vaobj, index))
}
#[no_mangle]
unsafe extern "C" fn glEnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) {
    with_ctx(|mut state| state.oxidegl_enable_vertex_array_attrib(vaobj, index))
}
#[no_mangle]
unsafe extern "C" fn glFenceSync(condition: GLenum, flags: GLbitfield) -> GLsync {
    with_ctx(|mut state| state.oxidegl_fence_sync(condition, flags))
}
#[no_mangle]
unsafe extern "C" fn glFinish() {
    with_ctx(|mut state| state.oxidegl_finish())
}
#[no_mangle]
unsafe extern "C" fn glFlush() {
    with_ctx(|mut state| state.oxidegl_flush())
}
#[no_mangle]
unsafe extern "C" fn glFlushMappedBufferRange(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
) {
    with_ctx(|mut state| state.oxidegl_flush_mapped_buffer_range(target, offset, length))
}
#[no_mangle]
unsafe extern "C" fn glFlushMappedNamedBufferRange(
    buffer: GLuint,
    offset: GLintptr,
    length: GLsizeiptr,
) {
    with_ctx(|mut state| state.oxidegl_flush_mapped_named_buffer_range(buffer, offset, length))
}
#[no_mangle]
unsafe extern "C" fn glFramebufferParameteri(target: GLenum, pname: GLenum, param: GLint) {
    with_ctx(|mut state| state.oxidegl_framebuffer_parameteri(target, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glNamedFramebufferParameteri(
    framebuffer: GLuint,
    pname: GLenum,
    param: GLint,
) {
    with_ctx(|mut state| state.oxidegl_named_framebuffer_parameteri(framebuffer, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glFramebufferRenderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_framebuffer_renderbuffer(target, attachment, renderbuffertarget, renderbuffer)
    })
}
#[no_mangle]
unsafe extern "C" fn glNamedFramebufferRenderbuffer(
    framebuffer: GLuint,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_named_framebuffer_renderbuffer(
            framebuffer,
            attachment,
            renderbuffertarget,
            renderbuffer,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glFramebufferTexture1D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_framebuffer_texture1_d(target, attachment, textarget, texture, level)
    })
}
#[no_mangle]
unsafe extern "C" fn glFramebufferTexture2D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_framebuffer_texture2_d(target, attachment, textarget, texture, level)
    })
}
#[no_mangle]
unsafe extern "C" fn glFramebufferTexture3D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
    zoffset: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_framebuffer_texture3_d(target, attachment, textarget, texture, level, zoffset)
    })
}
#[no_mangle]
unsafe extern "C" fn glFramebufferTexture(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
) {
    with_ctx(|mut state| state.oxidegl_framebuffer_texture(target, attachment, texture, level))
}
#[no_mangle]
unsafe extern "C" fn glNamedFramebufferTexture(
    framebuffer: GLuint,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_named_framebuffer_texture(framebuffer, attachment, texture, level)
    })
}
#[no_mangle]
unsafe extern "C" fn glFramebufferTextureLayer(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_framebuffer_texture_layer(target, attachment, texture, level, layer)
    })
}
#[no_mangle]
unsafe extern "C" fn glNamedFramebufferTextureLayer(
    framebuffer: GLuint,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_named_framebuffer_texture_layer(
            framebuffer,
            attachment,
            texture,
            level,
            layer,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glFrontFace(mode: GLenum) {
    with_ctx(|mut state| state.oxidegl_front_face(mode))
}
#[no_mangle]
unsafe extern "C" fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_gen_buffers(n, buffers))
}
#[no_mangle]
unsafe extern "C" fn glGenerateMipmap(target: GLenum) {
    with_ctx(|mut state| state.oxidegl_generate_mipmap(target))
}
#[no_mangle]
unsafe extern "C" fn glGenerateTextureMipmap(texture: GLuint) {
    with_ctx(|mut state| state.oxidegl_generate_texture_mipmap(texture))
}
#[no_mangle]
unsafe extern "C" fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_gen_framebuffers(n, framebuffers))
}
#[no_mangle]
unsafe extern "C" fn glGenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_gen_program_pipelines(n, pipelines))
}
#[no_mangle]
unsafe extern "C" fn glGenQueries(n: GLsizei, ids: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_gen_queries(n, ids))
}
#[no_mangle]
unsafe extern "C" fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_gen_renderbuffers(n, renderbuffers))
}
#[no_mangle]
unsafe extern "C" fn glGenSamplers(count: GLsizei, samplers: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_gen_samplers(count, samplers))
}
#[no_mangle]
unsafe extern "C" fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_gen_textures(n, textures))
}
#[no_mangle]
unsafe extern "C" fn glGenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_gen_transform_feedbacks(n, ids))
}
#[no_mangle]
unsafe extern "C" fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_gen_vertex_arrays(n, arrays))
}
#[no_mangle]
unsafe extern "C" fn glGetBooleanv(pname: GLenum, data: *mut GLboolean) {
    with_ctx(|mut state| state.oxidegl_get_booleanv(pname, data))
}
#[no_mangle]
unsafe extern "C" fn glGetActiveAtomicCounterBufferiv(
    program: GLuint,
    bufferIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_atomic_counter_bufferiv(program, bufferIndex, pname, params)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetActiveAttrib(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    r#type: *mut GLenum,
    name: *mut GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_attrib(program, index, bufSize, length, size, r#type, name)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetActiveSubroutineName(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_subroutine_name(program, shadertype, index, bufSize, length, name)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetActiveSubroutineUniformiv(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    pname: GLenum,
    values: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_subroutine_uniformiv(program, shadertype, index, pname, values)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetActiveSubroutineUniformName(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_subroutine_uniform_name(
            program, shadertype, index, bufSize, length, name,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glGetActiveUniform(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    r#type: *mut GLenum,
    name: *mut GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_uniform(program, index, bufSize, length, size, r#type, name)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetActiveUniformBlockiv(
    program: GLuint,
    uniformBlockIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_uniform_blockiv(program, uniformBlockIndex, pname, params)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetActiveUniformBlockName(
    program: GLuint,
    uniformBlockIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformBlockName: *mut GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_uniform_block_name(
            program,
            uniformBlockIndex,
            bufSize,
            length,
            uniformBlockName,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glGetActiveUniformName(
    program: GLuint,
    uniformIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformName: *mut GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_uniform_name(program, uniformIndex, bufSize, length, uniformName)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetActiveUniformsiv(
    program: GLuint,
    uniformCount: GLsizei,
    uniformIndices: *const GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_active_uniformsiv(program, uniformCount, uniformIndices, pname, params)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetAttachedShaders(
    program: GLuint,
    maxCount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
) {
    with_ctx(|mut state| state.oxidegl_get_attached_shaders(program, maxCount, count, shaders))
}
#[no_mangle]
unsafe extern "C" fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
    with_ctx(|mut state| state.oxidegl_get_attrib_location(program, name))
}
#[no_mangle]
unsafe extern "C" fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_buffer_parameteriv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) {
    with_ctx(|mut state| state.oxidegl_get_buffer_parameteri64v(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetNamedBufferParameteriv(
    buffer: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_named_buffer_parameteriv(buffer, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetNamedBufferParameteri64v(
    buffer: GLuint,
    pname: GLenum,
    params: *mut GLint64,
) {
    with_ctx(|mut state| state.oxidegl_get_named_buffer_parameteri64v(buffer, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetBufferPointerv(target: GLenum, pname: GLenum, params: *mut *mut GLvoid) {
    with_ctx(|mut state| state.oxidegl_get_buffer_pointerv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetNamedBufferPointerv(
    buffer: GLuint,
    pname: GLenum,
    params: *mut *mut GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_get_named_buffer_pointerv(buffer, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetBufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_get_buffer_sub_data(target, offset, size, data))
}
#[no_mangle]
unsafe extern "C" fn glGetNamedBufferSubData(
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_get_named_buffer_sub_data(buffer, offset, size, data))
}
#[no_mangle]
unsafe extern "C" fn glGetCompressedTexImage(target: GLenum, level: GLint, img: *mut GLvoid) {
    with_ctx(|mut state| state.oxidegl_get_compressed_tex_image(target, level, img))
}
#[no_mangle]
unsafe extern "C" fn glGetCompressedTextureImage(
    texture: GLuint,
    level: GLint,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_compressed_texture_image(texture, level, bufSize, pixels)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetnCompressedTexImage(
    target: GLenum,
    lod: GLint,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_getn_compressed_tex_image(target, lod, bufSize, pixels))
}
#[no_mangle]
unsafe extern "C" fn glGetCompressedTextureSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_compressed_texture_sub_image(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glGetDebugMessageLog(
    count: GLuint,
    bufSize: GLsizei,
    sources: *mut GLenum,
    types: *mut GLenum,
    ids: *mut GLuint,
    severities: *mut GLenum,
    lengths: *mut GLsizei,
    messageLog: *mut GLchar,
) -> GLuint {
    with_ctx(|mut state| {
        state.oxidegl_get_debug_message_log(
            count, bufSize, sources, types, ids, severities, lengths, messageLog,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glGetError() -> GLenum {
    with_ctx(|mut state| state.oxidegl_get_error())
}
#[no_mangle]
unsafe extern "C" fn glGetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint {
    with_ctx(|mut state| state.oxidegl_get_frag_data_index(program, name))
}
#[no_mangle]
unsafe extern "C" fn glGetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint {
    with_ctx(|mut state| state.oxidegl_get_frag_data_location(program, name))
}
#[no_mangle]
unsafe extern "C" fn glGetFramebufferAttachmentParameteriv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_framebuffer_attachment_parameteriv(target, attachment, pname, params)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetNamedFramebufferAttachmentParameteriv(
    framebuffer: GLuint,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_named_framebuffer_attachment_parameteriv(
            framebuffer,
            attachment,
            pname,
            params,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glGetFramebufferParameteriv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_framebuffer_parameteriv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetNamedFramebufferParameteriv(
    framebuffer: GLuint,
    pname: GLenum,
    param: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_named_framebuffer_parameteriv(framebuffer, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glGetGraphicsResetStatus() -> GLenum {
    with_ctx(|mut state| state.oxidegl_get_graphics_reset_status())
}
#[no_mangle]
unsafe extern "C" fn glGetInternalformativ(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    count: GLsizei,
    params: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_internalformativ(target, internalformat, pname, count, params)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetInternalformati64v(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    count: GLsizei,
    params: *mut GLint64,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_internalformati64v(target, internalformat, pname, count, params)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) {
    with_ctx(|mut state| state.oxidegl_get_multisamplefv(pname, index, val))
}
#[no_mangle]
unsafe extern "C" fn glGetObjectLabel(
    identifier: GLenum,
    name: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
) {
    with_ctx(|mut state| state.oxidegl_get_object_label(identifier, name, bufSize, length, label))
}
#[no_mangle]
unsafe extern "C" fn glGetObjectPtrLabel(
    ptr: *const GLvoid,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
) {
    with_ctx(|mut state| state.oxidegl_get_object_ptr_label(ptr, bufSize, length, label))
}
#[no_mangle]
unsafe extern "C" fn glGetPointerv(pname: GLenum, params: *mut *mut GLvoid) {
    with_ctx(|mut state| state.oxidegl_get_pointerv(pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_programiv(program, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetProgramBinary(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    binaryFormat: *mut GLenum,
    binary: *mut GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_program_binary(program, bufSize, length, binaryFormat, binary)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetProgramInfoLog(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
) {
    with_ctx(|mut state| state.oxidegl_get_program_info_log(program, bufSize, length, infoLog))
}
#[no_mangle]
unsafe extern "C" fn glGetProgramInterfaceiv(
    program: GLuint,
    programInterface: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_program_interfaceiv(program, programInterface, pname, params)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_program_pipelineiv(pipeline, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetProgramPipelineInfoLog(
    pipeline: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_program_pipeline_info_log(pipeline, bufSize, length, infoLog)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetProgramResourceiv(
    program: GLuint,
    programInterface: GLenum,
    index: GLuint,
    propCount: GLsizei,
    props: *const GLenum,
    count: GLsizei,
    length: *mut GLsizei,
    params: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_program_resourceiv(
            program,
            programInterface,
            index,
            propCount,
            props,
            count,
            length,
            params,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glGetProgramResourceIndex(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
) -> GLuint {
    with_ctx(|mut state| state.oxidegl_get_program_resource_index(program, programInterface, name))
}
#[no_mangle]
unsafe extern "C" fn glGetProgramResourceLocation(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
) -> GLint {
    with_ctx(|mut state| {
        state.oxidegl_get_program_resource_location(program, programInterface, name)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetProgramResourceLocationIndex(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
) -> GLint {
    with_ctx(|mut state| {
        state.oxidegl_get_program_resource_location_index(program, programInterface, name)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetProgramResourceName(
    program: GLuint,
    programInterface: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_program_resource_name(
            program,
            programInterface,
            index,
            bufSize,
            length,
            name,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glGetProgramStageiv(
    program: GLuint,
    shadertype: GLenum,
    pname: GLenum,
    values: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_program_stageiv(program, shadertype, pname, values))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryIndexediv(
    target: GLenum,
    index: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_query_indexediv(target, index, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_queryiv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_query_objectiv(id, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetRenderbufferParameteriv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_renderbuffer_parameteriv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetNamedRenderbufferParameteriv(
    renderbuffer: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_named_renderbuffer_parameteriv(renderbuffer, pname, params)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat) {
    with_ctx(|mut state| state.oxidegl_get_sampler_parameterfv(sampler, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_shaderiv(shader, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetShaderInfoLog(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
) {
    with_ctx(|mut state| state.oxidegl_get_shader_info_log(shader, bufSize, length, infoLog))
}
#[no_mangle]
unsafe extern "C" fn glGetShaderPrecisionFormat(
    shadertype: GLenum,
    precisiontype: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_shader_precision_format(shadertype, precisiontype, range, precision)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetShaderSource(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
) {
    with_ctx(|mut state| state.oxidegl_get_shader_source(shader, bufSize, length, source))
}
#[no_mangle]
unsafe extern "C" fn glGetString(name: GLenum) -> *const GLubyte {
    with_ctx(|mut state| state.oxidegl_get_string(name))
}
#[no_mangle]
unsafe extern "C" fn glGetSubroutineIndex(
    program: GLuint,
    shadertype: GLenum,
    name: *const GLchar,
) -> GLuint {
    with_ctx(|mut state| state.oxidegl_get_subroutine_index(program, shadertype, name))
}
#[no_mangle]
unsafe extern "C" fn glGetSubroutineUniformLocation(
    program: GLuint,
    shadertype: GLenum,
    name: *const GLchar,
) -> GLint {
    with_ctx(|mut state| state.oxidegl_get_subroutine_uniform_location(program, shadertype, name))
}
#[no_mangle]
unsafe extern "C" fn glGetSynciv(
    sync: GLsync,
    pname: GLenum,
    count: GLsizei,
    length: *mut GLsizei,
    values: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_synciv(sync, pname, count, length, values))
}
#[no_mangle]
unsafe extern "C" fn glGetTexImage(
    target: GLenum,
    level: GLint,
    format: GLenum,
    r#type: GLenum,
    pixels: *mut GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_get_tex_image(target, level, format, r#type, pixels))
}
#[no_mangle]
unsafe extern "C" fn glGetTextureImage(
    texture: GLuint,
    level: GLint,
    format: GLenum,
    r#type: GLenum,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_texture_image(texture, level, format, r#type, bufSize, pixels)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetnTexImage(
    target: GLenum,
    level: GLint,
    format: GLenum,
    r#type: GLenum,
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_getn_tex_image(target, level, format, r#type, bufSize, pixels)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetTexLevelParameterfv(
    target: GLenum,
    level: GLint,
    pname: GLenum,
    params: *mut GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_get_tex_level_parameterfv(target, level, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTexLevelParameteriv(
    target: GLenum,
    level: GLint,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_tex_level_parameteriv(target, level, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTextureLevelParameterfv(
    texture: GLuint,
    level: GLint,
    pname: GLenum,
    params: *mut GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_get_texture_level_parameterfv(texture, level, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTextureLevelParameteriv(
    texture: GLuint,
    level: GLint,
    pname: GLenum,
    params: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_texture_level_parameteriv(texture, level, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    with_ctx(|mut state| state.oxidegl_get_tex_parameterfv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_tex_parameteriv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_tex_parameter_iiv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_get_tex_parameter_iuiv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTextureParameterfv(texture: GLuint, pname: GLenum, params: *mut GLfloat) {
    with_ctx(|mut state| state.oxidegl_get_texture_parameterfv(texture, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTextureParameterIiv(texture: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_texture_parameter_iiv(texture, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTextureParameterIuiv(
    texture: GLuint,
    pname: GLenum,
    params: *mut GLuint,
) {
    with_ctx(|mut state| state.oxidegl_get_texture_parameter_iuiv(texture, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTextureParameteriv(texture: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_texture_parameteriv(texture, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetTextureSubImage(
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
    bufSize: GLsizei,
    pixels: *mut GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_texture_sub_image(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type,
            bufSize, pixels,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glGetTransformFeedbackiv(xfb: GLuint, pname: GLenum, param: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_transform_feedbackiv(xfb, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glGetTransformFeedbackVarying(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLsizei,
    r#type: *mut GLenum,
    name: *mut GLchar,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_transform_feedback_varying(
            program, index, bufSize, length, size, r#type, name,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) {
    with_ctx(|mut state| state.oxidegl_get_uniformfv(program, location, params))
}
#[no_mangle]
unsafe extern "C" fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_uniformiv(program, location, params))
}
#[no_mangle]
unsafe extern "C" fn glGetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_get_uniformuiv(program, location, params))
}
#[no_mangle]
unsafe extern "C" fn glGetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) {
    with_ctx(|mut state| state.oxidegl_get_uniformdv(program, location, params))
}
#[no_mangle]
unsafe extern "C" fn glGetnUniformdv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_getn_uniformdv(program, location, bufSize, params))
}
#[no_mangle]
unsafe extern "C" fn glGetnUniformfv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_getn_uniformfv(program, location, bufSize, params))
}
#[no_mangle]
unsafe extern "C" fn glGetnUniformiv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_getn_uniformiv(program, location, bufSize, params))
}
#[no_mangle]
unsafe extern "C" fn glGetnUniformuiv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLuint,
) {
    with_ctx(|mut state| state.oxidegl_getn_uniformuiv(program, location, bufSize, params))
}
#[no_mangle]
unsafe extern "C" fn glGetUniformBlockIndex(
    program: GLuint,
    uniformBlockName: *const GLchar,
) -> GLuint {
    with_ctx(|mut state| state.oxidegl_get_uniform_block_index(program, uniformBlockName))
}
#[no_mangle]
unsafe extern "C" fn glGetUniformIndices(
    program: GLuint,
    uniformCount: GLsizei,
    uniformNames: GLchar,
    uniformIndices: *mut GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_get_uniform_indices(program, uniformCount, uniformNames, uniformIndices)
    })
}
#[no_mangle]
unsafe extern "C" fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    with_ctx(|mut state| state.oxidegl_get_uniform_location(program, name))
}
#[no_mangle]
unsafe extern "C" fn glGetUniformSubroutineuiv(
    shadertype: GLenum,
    location: GLint,
    params: *mut GLuint,
) {
    with_ctx(|mut state| state.oxidegl_get_uniform_subroutineuiv(shadertype, location, params))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexArrayIndexed64iv(
    vaobj: GLuint,
    index: GLuint,
    pname: GLenum,
    param: *mut GLint64,
) {
    with_ctx(|mut state| state.oxidegl_get_vertex_array_indexed64iv(vaobj, index, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexArrayiv(vaobj: GLuint, pname: GLenum, param: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_vertex_arrayiv(vaobj, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) {
    with_ctx(|mut state| state.oxidegl_get_vertex_attribdv(index, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) {
    with_ctx(|mut state| state.oxidegl_get_vertex_attribfv(index, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_vertex_attribiv(index, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_vertex_attrib_iiv(index, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_get_vertex_attrib_iuiv(index, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble) {
    with_ctx(|mut state| state.oxidegl_get_vertex_attrib_ldv(index, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexAttribPointerv(
    index: GLuint,
    pname: GLenum,
    pointer: *mut *mut GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_get_vertex_attrib_pointerv(index, pname, pointer))
}
#[no_mangle]
unsafe extern "C" fn glHint(target: GLenum, mode: GLenum) {
    with_ctx(|mut state| state.oxidegl_hint(target, mode))
}
#[no_mangle]
unsafe extern "C" fn glInvalidateBufferData(buffer: GLuint) {
    with_ctx(|mut state| state.oxidegl_invalidate_buffer_data(buffer))
}
#[no_mangle]
unsafe extern "C" fn glInvalidateBufferSubData(
    buffer: GLuint,
    offset: GLintptr,
    length: GLsizeiptr,
) {
    with_ctx(|mut state| state.oxidegl_invalidate_buffer_sub_data(buffer, offset, length))
}
#[no_mangle]
unsafe extern "C" fn glInvalidateFramebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: *const GLenum,
) {
    with_ctx(|mut state| state.oxidegl_invalidate_framebuffer(target, numAttachments, attachments))
}
#[no_mangle]
unsafe extern "C" fn glInvalidateNamedFramebufferData(
    framebuffer: GLuint,
    numAttachments: GLsizei,
    attachments: *const GLenum,
) {
    with_ctx(|mut state| {
        state.oxidegl_invalidate_named_framebuffer_data(framebuffer, numAttachments, attachments)
    })
}
#[no_mangle]
unsafe extern "C" fn glInvalidateSubFramebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: *const GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_invalidate_sub_framebuffer(
            target,
            numAttachments,
            attachments,
            x,
            y,
            width,
            height,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glInvalidateNamedFramebufferSubData(
    framebuffer: GLuint,
    numAttachments: GLsizei,
    attachments: *const GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_invalidate_named_framebuffer_sub_data(
            framebuffer,
            numAttachments,
            attachments,
            x,
            y,
            width,
            height,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glInvalidateTexImage(texture: GLuint, level: GLint) {
    with_ctx(|mut state| state.oxidegl_invalidate_tex_image(texture, level))
}
#[no_mangle]
unsafe extern "C" fn glInvalidateTexSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_invalidate_tex_sub_image(
            texture, level, xoffset, yoffset, zoffset, width, height, depth,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glIsBuffer(buffer: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_buffer(buffer))
}
#[no_mangle]
unsafe extern "C" fn glIsEnabled(cap: GLenum) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_enabled(cap))
}
#[no_mangle]
unsafe extern "C" fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_framebuffer(framebuffer))
}
#[no_mangle]
unsafe extern "C" fn glIsProgram(program: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_program(program))
}
#[no_mangle]
unsafe extern "C" fn glIsProgramPipeline(pipeline: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_program_pipeline(pipeline))
}
#[no_mangle]
unsafe extern "C" fn glIsQuery(id: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_query(id))
}
#[no_mangle]
unsafe extern "C" fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_renderbuffer(renderbuffer))
}
#[no_mangle]
unsafe extern "C" fn glIsSampler(sampler: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_sampler(sampler))
}
#[no_mangle]
unsafe extern "C" fn glIsShader(shader: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_shader(shader))
}
#[no_mangle]
unsafe extern "C" fn glIsSync(sync: GLsync) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_sync(sync))
}
#[no_mangle]
unsafe extern "C" fn glIsTexture(texture: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_texture(texture))
}
#[no_mangle]
unsafe extern "C" fn glIsTransformFeedback(id: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_transform_feedback(id))
}
#[no_mangle]
unsafe extern "C" fn glIsVertexArray(array: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_vertex_array(array))
}
#[no_mangle]
unsafe extern "C" fn glLineWidth(width: GLfloat) {
    with_ctx(|mut state| state.oxidegl_line_width(width))
}
#[no_mangle]
unsafe extern "C" fn glLinkProgram(program: GLuint) {
    with_ctx(|mut state| state.oxidegl_link_program(program))
}
#[no_mangle]
unsafe extern "C" fn glLogicOp(opcode: GLenum) {
    with_ctx(|mut state| state.oxidegl_logic_op(opcode))
}
#[no_mangle]
unsafe extern "C" fn glMapBuffer(target: GLenum, access: GLenum) -> *mut GLvoid {
    with_ctx(|mut state| state.oxidegl_map_buffer(target, access))
}
#[no_mangle]
unsafe extern "C" fn glMapNamedBuffer(buffer: GLuint, access: GLenum) -> *mut GLvoid {
    with_ctx(|mut state| state.oxidegl_map_named_buffer(buffer, access))
}
#[no_mangle]
unsafe extern "C" fn glMapBufferRange(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
) -> *mut GLvoid {
    with_ctx(|mut state| state.oxidegl_map_buffer_range(target, offset, length, access))
}
#[no_mangle]
unsafe extern "C" fn glMapNamedBufferRange(
    buffer: GLuint,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
) -> *mut GLvoid {
    with_ctx(|mut state| state.oxidegl_map_named_buffer_range(buffer, offset, length, access))
}
#[no_mangle]
unsafe extern "C" fn glMemoryBarrier(barriers: GLbitfield) {
    with_ctx(|mut state| state.oxidegl_memory_barrier(barriers))
}
#[no_mangle]
unsafe extern "C" fn glMemoryBarrierByRegion(barriers: GLbitfield) {
    with_ctx(|mut state| state.oxidegl_memory_barrier_by_region(barriers))
}
#[no_mangle]
unsafe extern "C" fn glMinSampleShading(value: GLfloat) {
    with_ctx(|mut state| state.oxidegl_min_sample_shading(value))
}
#[no_mangle]
unsafe extern "C" fn glMultiDrawArrays(
    mode: GLenum,
    first: *const GLint,
    count: *const GLsizei,
    drawcount: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_multi_draw_arrays(mode, first, count, drawcount))
}
#[no_mangle]
unsafe extern "C" fn glMultiDrawArraysIndirect(
    mode: GLenum,
    indirect: *const GLvoid,
    drawcount: GLsizei,
    stride: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_multi_draw_arrays_indirect(mode, indirect, drawcount, stride)
    })
}
#[no_mangle]
unsafe extern "C" fn glMultiDrawElements(
    mode: GLenum,
    count: *const GLsizei,
    r#type: GLenum,
    indices: *mut *const GLvoid,
    drawcount: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_multi_draw_elements(mode, count, r#type, indices, drawcount))
}
#[no_mangle]
unsafe extern "C" fn glMultiDrawElementsBaseVertex(
    mode: GLenum,
    count: *const GLsizei,
    r#type: GLenum,
    indices: *mut *const GLvoid,
    drawcount: GLsizei,
    basevertex: *const GLint,
) {
    with_ctx(|mut state| {
        state.oxidegl_multi_draw_elements_base_vertex(
            mode, count, r#type, indices, drawcount, basevertex,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glMultiDrawElementsIndirect(
    mode: GLenum,
    r#type: GLenum,
    indirect: *const GLvoid,
    drawcount: GLsizei,
    stride: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_multi_draw_elements_indirect(mode, r#type, indirect, drawcount, stride)
    })
}
#[no_mangle]
unsafe extern "C" fn glObjectLabel(
    identifier: GLenum,
    name: GLuint,
    length: GLsizei,
    label: *const GLchar,
) {
    with_ctx(|mut state| state.oxidegl_object_label(identifier, name, length, label))
}
#[no_mangle]
unsafe extern "C" fn glObjectPtrLabel(ptr: *const GLvoid, length: GLsizei, label: *const GLchar) {
    with_ctx(|mut state| state.oxidegl_object_ptr_label(ptr, length, label))
}
#[no_mangle]
unsafe extern "C" fn glPatchParameteri(pname: GLenum, value: GLint) {
    with_ctx(|mut state| state.oxidegl_patch_parameteri(pname, value))
}
#[no_mangle]
unsafe extern "C" fn glPauseTransformFeedback() {
    with_ctx(|mut state| state.oxidegl_pause_transform_feedback())
}
#[no_mangle]
unsafe extern "C" fn glPixelStoref(pname: GLenum, param: GLfloat) {
    with_ctx(|mut state| state.oxidegl_pixel_storef(pname, param))
}
#[no_mangle]
unsafe extern "C" fn glPointParameterf(pname: GLenum, param: GLfloat) {
    with_ctx(|mut state| state.oxidegl_point_parameterf(pname, param))
}
#[no_mangle]
unsafe extern "C" fn glPointSize(size: GLfloat) {
    with_ctx(|mut state| state.oxidegl_point_size(size))
}
#[no_mangle]
unsafe extern "C" fn glPolygonMode(face: GLenum, mode: GLenum) {
    with_ctx(|mut state| state.oxidegl_polygon_mode(face, mode))
}
#[no_mangle]
unsafe extern "C" fn glPolygonOffset(factor: GLfloat, units: GLfloat) {
    with_ctx(|mut state| state.oxidegl_polygon_offset(factor, units))
}
#[no_mangle]
unsafe extern "C" fn glPopDebugGroup() {
    with_ctx(|mut state| state.oxidegl_pop_debug_group())
}
#[no_mangle]
unsafe extern "C" fn glPrimitiveRestartIndex(index: GLuint) {
    with_ctx(|mut state| state.oxidegl_primitive_restart_index(index))
}
#[no_mangle]
unsafe extern "C" fn glProgramBinary(
    program: GLuint,
    binaryFormat: GLenum,
    binary: *const GLvoid,
    length: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_program_binary(program, binaryFormat, binary, length))
}
#[no_mangle]
unsafe extern "C" fn glProgramParameteri(program: GLuint, pname: GLenum, value: GLint) {
    with_ctx(|mut state| state.oxidegl_program_parameteri(program, pname, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform1i(program: GLuint, location: GLint, v0: GLint) {
    with_ctx(|mut state| state.oxidegl_program_uniform1i(program, location, v0))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform1iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform1iv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) {
    with_ctx(|mut state| state.oxidegl_program_uniform1f(program, location, v0))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform1fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform1fv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) {
    with_ctx(|mut state| state.oxidegl_program_uniform1ui(program, location, v0))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform1uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform1uiv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint) {
    with_ctx(|mut state| state.oxidegl_program_uniform2i(program, location, v0, v1))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform2iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform2iv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform2f(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform2f(program, location, v0, v1))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform2fv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) {
    with_ctx(|mut state| state.oxidegl_program_uniform2ui(program, location, v0, v1))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform2uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform2uiv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform3i(
    program: GLuint,
    location: GLint,
    v0: GLint,
    v1: GLint,
    v2: GLint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform3i(program, location, v0, v1, v2))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform3iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform3iv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform3f(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform3f(program, location, v0, v1, v2))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform3fv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform3ui(
    program: GLuint,
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform3ui(program, location, v0, v1, v2))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform3uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform3uiv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform4i(
    program: GLuint,
    location: GLint,
    v0: GLint,
    v1: GLint,
    v2: GLint,
    v3: GLint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform4i(program, location, v0, v1, v2, v3))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform4iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform4iv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform4f(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
    v3: GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform4f(program, location, v0, v1, v2, v3))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform4fv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform4ui(
    program: GLuint,
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
    v3: GLuint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform4ui(program, location, v0, v1, v2, v3))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform4uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform4uiv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix2fv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix3fv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix4fv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix2x3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix2x3fv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix3x2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix3x2fv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix2x4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix2x4fv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix4x2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix4x2fv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix3x4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix3x4fv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix4x3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix4x3fv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProvokingVertex(mode: GLenum) {
    with_ctx(|mut state| state.oxidegl_provoking_vertex(mode))
}
#[no_mangle]
unsafe extern "C" fn glPushDebugGroup(
    source: GLenum,
    id: GLuint,
    length: GLsizei,
    message: *const GLchar,
) {
    with_ctx(|mut state| state.oxidegl_push_debug_group(source, id, length, message))
}
#[no_mangle]
unsafe extern "C" fn glQueryCounter(id: GLuint, target: GLenum) {
    with_ctx(|mut state| state.oxidegl_query_counter(id, target))
}
#[no_mangle]
unsafe extern "C" fn glReadBuffer(src: GLenum) {
    with_ctx(|mut state| state.oxidegl_read_buffer(src))
}
#[no_mangle]
unsafe extern "C" fn glNamedFramebufferReadBuffer(framebuffer: GLuint, src: GLenum) {
    with_ctx(|mut state| state.oxidegl_named_framebuffer_read_buffer(framebuffer, src))
}
#[no_mangle]
unsafe extern "C" fn glReadPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *mut GLvoid,
) {
    with_ctx(|mut state| state.oxidegl_read_pixels(x, y, width, height, format, r#type, pixels))
}
#[no_mangle]
unsafe extern "C" fn glReadnPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    r#type: GLenum,
    bufSize: GLsizei,
    data: *mut GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_readn_pixels(x, y, width, height, format, r#type, bufSize, data)
    })
}
#[no_mangle]
unsafe extern "C" fn glReleaseShaderCompiler() {
    with_ctx(|mut state| state.oxidegl_release_shader_compiler())
}
#[no_mangle]
unsafe extern "C" fn glRenderbufferStorage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_renderbuffer_storage(target, internalformat, width, height))
}
#[no_mangle]
unsafe extern "C" fn glNamedRenderbufferStorage(
    renderbuffer: GLuint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_named_renderbuffer_storage(renderbuffer, internalformat, width, height)
    })
}
#[no_mangle]
unsafe extern "C" fn glRenderbufferStorageMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_renderbuffer_storage_multisample(
            target,
            samples,
            internalformat,
            width,
            height,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glNamedRenderbufferStorageMultisample(
    renderbuffer: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_named_renderbuffer_storage_multisample(
            renderbuffer,
            samples,
            internalformat,
            width,
            height,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glResumeTransformFeedback() {
    with_ctx(|mut state| state.oxidegl_resume_transform_feedback())
}
#[no_mangle]
unsafe extern "C" fn glSampleCoverage(value: GLfloat, invert: GLboolean) {
    with_ctx(|mut state| state.oxidegl_sample_coverage(value, invert))
}
#[no_mangle]
unsafe extern "C" fn glSampleMaski(maskNumber: GLuint, mask: GLbitfield) {
    with_ctx(|mut state| state.oxidegl_sample_maski(maskNumber, mask))
}
#[no_mangle]
unsafe extern "C" fn glSamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) {
    with_ctx(|mut state| state.oxidegl_sampler_parameterf(sampler, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    with_ctx(|mut state| state.oxidegl_scissor(x, y, width, height))
}
#[no_mangle]
unsafe extern "C" fn glScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint) {
    with_ctx(|mut state| state.oxidegl_scissor_arrayv(first, count, v))
}
#[no_mangle]
unsafe extern "C" fn glScissorIndexed(
    index: GLuint,
    left: GLint,
    bottom: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_scissor_indexed(index, left, bottom, width, height))
}
#[no_mangle]
unsafe extern "C" fn glScissorIndexedv(index: GLuint, v: *const GLint) {
    with_ctx(|mut state| state.oxidegl_scissor_indexedv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glShaderBinary(
    count: GLsizei,
    shaders: *const GLuint,
    binaryFormat: GLenum,
    binary: *const GLvoid,
    length: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_shader_binary(count, shaders, binaryFormat, binary, length))
}
#[no_mangle]
unsafe extern "C" fn glShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: GLchar,
    length: *const GLint,
) {
    with_ctx(|mut state| state.oxidegl_shader_source(shader, count, string, length))
}
#[no_mangle]
unsafe extern "C" fn glShaderStorageBlockBinding(
    program: GLuint,
    storageBlockIndex: GLuint,
    storageBlockBinding: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_shader_storage_block_binding(program, storageBlockIndex, storageBlockBinding)
    })
}
#[no_mangle]
unsafe extern "C" fn glStencilFunc(func: GLenum, r#ref: GLint, mask: GLuint) {
    with_ctx(|mut state| state.oxidegl_stencil_func(func, r#ref, mask))
}
#[no_mangle]
unsafe extern "C" fn glStencilFuncSeparate(face: GLenum, func: GLenum, r#ref: GLint, mask: GLuint) {
    with_ctx(|mut state| state.oxidegl_stencil_func_separate(face, func, r#ref, mask))
}
#[no_mangle]
unsafe extern "C" fn glStencilMask(mask: GLuint) {
    with_ctx(|mut state| state.oxidegl_stencil_mask(mask))
}
#[no_mangle]
unsafe extern "C" fn glStencilMaskSeparate(face: GLenum, mask: GLuint) {
    with_ctx(|mut state| state.oxidegl_stencil_mask_separate(face, mask))
}
#[no_mangle]
unsafe extern "C" fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) {
    with_ctx(|mut state| state.oxidegl_stencil_op(fail, zfail, zpass))
}
#[no_mangle]
unsafe extern "C" fn glStencilOpSeparate(
    face: GLenum,
    sfail: GLenum,
    dpfail: GLenum,
    dppass: GLenum,
) {
    with_ctx(|mut state| state.oxidegl_stencil_op_separate(face, sfail, dpfail, dppass))
}
#[no_mangle]
unsafe extern "C" fn glTexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) {
    with_ctx(|mut state| state.oxidegl_tex_buffer(target, internalformat, buffer))
}
#[no_mangle]
unsafe extern "C" fn glTextureBuffer(texture: GLuint, internalformat: GLenum, buffer: GLuint) {
    with_ctx(|mut state| state.oxidegl_texture_buffer(texture, internalformat, buffer))
}
#[no_mangle]
unsafe extern "C" fn glTexBufferRange(
    target: GLenum,
    internalformat: GLenum,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    with_ctx(|mut state| {
        state.oxidegl_tex_buffer_range(target, internalformat, buffer, offset, size)
    })
}
#[no_mangle]
unsafe extern "C" fn glTextureBufferRange(
    texture: GLuint,
    internalformat: GLenum,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    with_ctx(|mut state| {
        state.oxidegl_texture_buffer_range(texture, internalformat, buffer, offset, size)
    })
}
#[no_mangle]
unsafe extern "C" fn glTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    border: GLint,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_tex_image1_d(
            target,
            level,
            internalformat,
            width,
            border,
            format,
            r#type,
            pixels,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTexImage2D(
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
    with_ctx(|mut state| {
        state.oxidegl_tex_image2_d(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            r#type,
            pixels,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTexImage2DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    with_ctx(|mut state| {
        state.oxidegl_tex_image2_d_multisample(
            target,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTexImage3D(
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
    with_ctx(|mut state| {
        state.oxidegl_tex_image3_d(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            r#type,
            pixels,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTexImage3DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    with_ctx(|mut state| {
        state.oxidegl_tex_image3_d_multisample(
            target,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    with_ctx(|mut state| state.oxidegl_tex_parameterf(target, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    with_ctx(|mut state| state.oxidegl_tex_parameteri(target, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glTexStorage1D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_tex_storage1_d(target, levels, internalformat, width))
}
#[no_mangle]
unsafe extern "C" fn glTextureStorage1D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
) {
    with_ctx(|mut state| state.oxidegl_texture_storage1_d(texture, levels, internalformat, width))
}
#[no_mangle]
unsafe extern "C" fn glTexStorage2D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_tex_storage2_d(target, levels, internalformat, width, height)
    })
}
#[no_mangle]
unsafe extern "C" fn glTextureStorage2D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_texture_storage2_d(texture, levels, internalformat, width, height)
    })
}
#[no_mangle]
unsafe extern "C" fn glTexStorage2DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    with_ctx(|mut state| {
        state.oxidegl_tex_storage2_d_multisample(
            target,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTextureStorage2DMultisample(
    texture: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    with_ctx(|mut state| {
        state.oxidegl_texture_storage2_d_multisample(
            texture,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTexStorage3D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_tex_storage3_d(target, levels, internalformat, width, height, depth)
    })
}
#[no_mangle]
unsafe extern "C" fn glTextureStorage3D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_texture_storage3_d(texture, levels, internalformat, width, height, depth)
    })
}
#[no_mangle]
unsafe extern "C" fn glTexStorage3DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    with_ctx(|mut state| {
        state.oxidegl_tex_storage3_d_multisample(
            target,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTextureStorage3DMultisample(
    texture: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    with_ctx(|mut state| {
        state.oxidegl_texture_storage3_d_multisample(
            texture,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_tex_sub_image1_d(target, level, xoffset, width, format, r#type, pixels)
    })
}
#[no_mangle]
unsafe extern "C" fn glTextureSubImage1D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_texture_sub_image1_d(texture, level, xoffset, width, format, r#type, pixels)
    })
}
#[no_mangle]
unsafe extern "C" fn glTexSubImage2D(
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
    with_ctx(|mut state| {
        state.oxidegl_tex_sub_image2_d(
            target, level, xoffset, yoffset, width, height, format, r#type, pixels,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTextureSubImage2D(
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
    with_ctx(|mut state| {
        state.oxidegl_texture_sub_image2_d(
            texture, level, xoffset, yoffset, width, height, format, r#type, pixels,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTexSubImage3D(
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
    with_ctx(|mut state| {
        state.oxidegl_tex_sub_image3_d(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, pixels,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTextureSubImage3D(
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
    with_ctx(|mut state| {
        state.oxidegl_texture_sub_image3_d(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, pixels,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTextureBarrier() {
    with_ctx(|mut state| state.oxidegl_texture_barrier())
}
#[no_mangle]
unsafe extern "C" fn glTextureView(
    texture: GLuint,
    target: GLenum,
    origtexture: GLuint,
    internalformat: GLenum,
    minlevel: GLuint,
    numlevels: GLuint,
    minlayer: GLuint,
    numlayers: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_texture_view(
            texture,
            target,
            origtexture,
            internalformat,
            minlevel,
            numlevels,
            minlayer,
            numlayers,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glTransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint) {
    with_ctx(|mut state| state.oxidegl_transform_feedback_buffer_base(xfb, index, buffer))
}
#[no_mangle]
unsafe extern "C" fn glTransformFeedbackBufferRange(
    xfb: GLuint,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    with_ctx(|mut state| {
        state.oxidegl_transform_feedback_buffer_range(xfb, index, buffer, offset, size)
    })
}
#[no_mangle]
unsafe extern "C" fn glTransformFeedbackVaryings(
    program: GLuint,
    count: GLsizei,
    varyings: GLchar,
    bufferMode: GLenum,
) {
    with_ctx(|mut state| {
        state.oxidegl_transform_feedback_varyings(program, count, varyings, bufferMode)
    })
}
#[no_mangle]
unsafe extern "C" fn glUniform1f(location: GLint, v0: GLfloat) {
    with_ctx(|mut state| state.oxidegl_uniform1f(location, v0))
}
#[no_mangle]
unsafe extern "C" fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    with_ctx(|mut state| state.oxidegl_uniform2f(location, v0, v1))
}
#[no_mangle]
unsafe extern "C" fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    with_ctx(|mut state| state.oxidegl_uniform3f(location, v0, v1, v2))
}
#[no_mangle]
unsafe extern "C" fn glUniform4f(
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
    v3: GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform4f(location, v0, v1, v2, v3))
}
#[no_mangle]
unsafe extern "C" fn glUniform1i(location: GLint, v0: GLint) {
    with_ctx(|mut state| state.oxidegl_uniform1i(location, v0))
}
#[no_mangle]
unsafe extern "C" fn glUniform2i(location: GLint, v0: GLint, v1: GLint) {
    with_ctx(|mut state| state.oxidegl_uniform2i(location, v0, v1))
}
#[no_mangle]
unsafe extern "C" fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    with_ctx(|mut state| state.oxidegl_uniform3i(location, v0, v1, v2))
}
#[no_mangle]
unsafe extern "C" fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    with_ctx(|mut state| state.oxidegl_uniform4i(location, v0, v1, v2, v3))
}
#[no_mangle]
unsafe extern "C" fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_uniform1fv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_uniform2fv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_uniform3fv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_uniform4fv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint) {
    with_ctx(|mut state| state.oxidegl_uniform1iv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint) {
    with_ctx(|mut state| state.oxidegl_uniform2iv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint) {
    with_ctx(|mut state| state.oxidegl_uniform3iv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint) {
    with_ctx(|mut state| state.oxidegl_uniform4iv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix2fv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix3fv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix4fv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix2x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix2x3fv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix3x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix3x2fv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix2x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix2x4fv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix4x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix4x2fv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix3x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix3x4fv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix4x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix4x3fv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform1ui(location: GLint, v0: GLuint) {
    with_ctx(|mut state| state.oxidegl_uniform1ui(location, v0))
}
#[no_mangle]
unsafe extern "C" fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint) {
    with_ctx(|mut state| state.oxidegl_uniform2ui(location, v0, v1))
}
#[no_mangle]
unsafe extern "C" fn glUniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
    with_ctx(|mut state| state.oxidegl_uniform3ui(location, v0, v1, v2))
}
#[no_mangle]
unsafe extern "C" fn glUniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
    with_ctx(|mut state| state.oxidegl_uniform4ui(location, v0, v1, v2, v3))
}
#[no_mangle]
unsafe extern "C" fn glUniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_uniform1uiv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_uniform2uiv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_uniform3uiv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_uniform4uiv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformBlockBinding(
    program: GLuint,
    uniformBlockIndex: GLuint,
    uniformBlockBinding: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_uniform_block_binding(program, uniformBlockIndex, uniformBlockBinding)
    })
}
#[no_mangle]
unsafe extern "C" fn glUniformSubroutinesuiv(
    shadertype: GLenum,
    count: GLsizei,
    indices: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_uniform_subroutinesuiv(shadertype, count, indices))
}
#[no_mangle]
unsafe extern "C" fn glUnmapBuffer(target: GLenum) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_unmap_buffer(target))
}
#[no_mangle]
unsafe extern "C" fn glUnmapNamedBuffer(buffer: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_unmap_named_buffer(buffer))
}
#[no_mangle]
unsafe extern "C" fn glUseProgram(program: GLuint) {
    with_ctx(|mut state| state.oxidegl_use_program(program))
}
#[no_mangle]
unsafe extern "C" fn glUseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint) {
    with_ctx(|mut state| state.oxidegl_use_program_stages(pipeline, stages, program))
}
#[no_mangle]
unsafe extern "C" fn glValidateProgram(program: GLuint) {
    with_ctx(|mut state| state.oxidegl_validate_program(program))
}
#[no_mangle]
unsafe extern "C" fn glValidateProgramPipeline(pipeline: GLuint) {
    with_ctx(|mut state| state.oxidegl_validate_program_pipeline(pipeline))
}
#[no_mangle]
unsafe extern "C" fn glVertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_array_element_buffer(vaobj, buffer))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib1d(index: GLuint, x: GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib1d(index, x))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib1dv(index: GLuint, v: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib1dv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib1f(index: GLuint, x: GLfloat) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib1f(index, x))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib1fv(index: GLuint, v: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib1fv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib1s(index: GLuint, x: GLshort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib1s(index, x))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib1sv(index: GLuint, v: *const GLshort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib1sv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib2d(index, x, y))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib2dv(index: GLuint, v: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib2dv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib2f(index, x, y))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib2fv(index: GLuint, v: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib2fv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib2s(index, x, y))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib2sv(index: GLuint, v: *const GLshort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib2sv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib3d(index, x, y, z))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib3dv(index: GLuint, v: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib3dv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib3f(index, x, y, z))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib3fv(index: GLuint, v: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib3fv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib3s(index, x, y, z))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib3sv(index: GLuint, v: *const GLshort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib3sv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4Nbv(index: GLuint, v: *const GLbyte) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4_nbv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4Niv(index: GLuint, v: *const GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4_niv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4Nsv(index: GLuint, v: *const GLshort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4_nsv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4Nub(
    index: GLuint,
    x: GLubyte,
    y: GLubyte,
    z: GLubyte,
    w: GLubyte,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4_nub(index, x, y, z, w))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4Nubv(index: GLuint, v: *const GLubyte) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4_nubv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4Nuiv(index: GLuint, v: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4_nuiv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4Nusv(index: GLuint, v: *const GLushort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4_nusv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4bv(index: GLuint, v: *const GLbyte) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4bv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4d(
    index: GLuint,
    x: GLdouble,
    y: GLdouble,
    z: GLdouble,
    w: GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4d(index, x, y, z, w))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4dv(index: GLuint, v: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4dv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4f(
    index: GLuint,
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    w: GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4f(index, x, y, z, w))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4fv(index: GLuint, v: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4fv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4iv(index: GLuint, v: *const GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4iv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4s(
    index: GLuint,
    x: GLshort,
    y: GLshort,
    z: GLshort,
    w: GLshort,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4s(index, x, y, z, w))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4sv(index: GLuint, v: *const GLshort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4sv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4ubv(index: GLuint, v: *const GLubyte) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4ubv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4uiv(index: GLuint, v: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4uiv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttrib4usv(index: GLuint, v: *const GLushort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib4usv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI1i(index: GLuint, x: GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i1i(index, x))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI2i(index: GLuint, x: GLint, y: GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i2i(index, x, y))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i3i(index, x, y, z))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i4i(index, x, y, z, w))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI1ui(index: GLuint, x: GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i1ui(index, x))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i2ui(index, x, y))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i3ui(index, x, y, z))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i4ui(index, x, y, z, w))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI1iv(index: GLuint, v: *const GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i1iv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI2iv(index: GLuint, v: *const GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i2iv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI3iv(index: GLuint, v: *const GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i3iv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI4iv(index: GLuint, v: *const GLint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i4iv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI1uiv(index: GLuint, v: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i1uiv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI2uiv(index: GLuint, v: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i2uiv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI3uiv(index: GLuint, v: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i3uiv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI4uiv(index: GLuint, v: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i4uiv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI4bv(index: GLuint, v: *const GLbyte) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i4bv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI4sv(index: GLuint, v: *const GLshort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i4sv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI4ubv(index: GLuint, v: *const GLubyte) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i4ubv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribI4usv(index: GLuint, v: *const GLushort) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_i4usv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribP1ui(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: GLuint,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_p1ui(index, r#type, normalized, value))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribP2ui(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: GLuint,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_p2ui(index, r#type, normalized, value))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribP3ui(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: GLuint,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_p3ui(index, r#type, normalized, value))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribP4ui(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: GLuint,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_p4ui(index, r#type, normalized, value))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribL1d(index: GLuint, x: GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_l1d(index, x))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_l2d(index, x, y))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_l3d(index, x, y, z))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribL4d(
    index: GLuint,
    x: GLdouble,
    y: GLdouble,
    z: GLdouble,
    w: GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_l4d(index, x, y, z, w))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribL1dv(index: GLuint, v: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_l1dv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribL2dv(index: GLuint, v: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_l2dv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribL3dv(index: GLuint, v: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_l3dv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribL4dv(index: GLuint, v: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_l4dv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_binding(attribindex, bindingindex))
}
#[no_mangle]
unsafe extern "C" fn glVertexArrayAttribBinding(
    vaobj: GLuint,
    attribindex: GLuint,
    bindingindex: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_array_attrib_binding(vaobj, attribindex, bindingindex)
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_divisor(index, divisor))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribFormat(
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    normalized: GLboolean,
    relativeoffset: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_attrib_format(attribindex, size, r#type, normalized, relativeoffset)
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribIFormat(
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    relativeoffset: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_attrib_i_format(attribindex, size, r#type, relativeoffset)
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribLFormat(
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    relativeoffset: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_attrib_l_format(attribindex, size, r#type, relativeoffset)
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexArrayAttribFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    normalized: GLboolean,
    relativeoffset: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_array_attrib_format(
            vaobj,
            attribindex,
            size,
            r#type,
            normalized,
            relativeoffset,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexArrayAttribIFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    relativeoffset: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_array_attrib_i_format(vaobj, attribindex, size, r#type, relativeoffset)
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexArrayAttribLFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    r#type: GLenum,
    relativeoffset: GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_array_attrib_l_format(vaobj, attribindex, size, r#type, relativeoffset)
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribPointer(
    index: GLuint,
    size: GLint,
    r#type: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_attrib_pointer(index, size, r#type, normalized, stride, pointer)
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribIPointer(
    index: GLuint,
    size: GLint,
    r#type: GLenum,
    stride: GLsizei,
    pointer: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_attrib_i_pointer(index, size, r#type, stride, pointer)
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribLPointer(
    index: GLuint,
    size: GLint,
    r#type: GLenum,
    stride: GLsizei,
    pointer: *const GLvoid,
) {
    with_ctx(|mut state| {
        state.oxidegl_vertex_attrib_l_pointer(index, size, r#type, stride, pointer)
    })
}
#[no_mangle]
unsafe extern "C" fn glVertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) {
    with_ctx(|mut state| state.oxidegl_vertex_binding_divisor(bindingindex, divisor))
}
#[no_mangle]
unsafe extern "C" fn glVertexArrayBindingDivisor(
    vaobj: GLuint,
    bindingindex: GLuint,
    divisor: GLuint,
) {
    with_ctx(|mut state| state.oxidegl_vertex_array_binding_divisor(vaobj, bindingindex, divisor))
}
#[no_mangle]
unsafe extern "C" fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    with_ctx(|mut state| state.oxidegl_viewport(x, y, width, height))
}
#[no_mangle]
unsafe extern "C" fn glViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_viewport_arrayv(first, count, v))
}
#[no_mangle]
unsafe extern "C" fn glViewportIndexedf(
    index: GLuint,
    x: GLfloat,
    y: GLfloat,
    w: GLfloat,
    h: GLfloat,
) {
    with_ctx(|mut state| state.oxidegl_viewport_indexedf(index, x, y, w, h))
}
#[no_mangle]
unsafe extern "C" fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
    with_ctx(|mut state| state.oxidegl_wait_sync(sync, flags, timeout))
}
#[no_mangle]
unsafe extern "C" fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_tex_parameterfv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) {
    with_ctx(|mut state| state.oxidegl_tex_parameteriv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glDisable(cap: GLenum) {
    with_ctx(|mut state| state.oxidegl_disable(cap))
}
#[no_mangle]
unsafe extern "C" fn glPixelStorei(pname: GLenum, param: GLint) {
    with_ctx(|mut state| state.oxidegl_pixel_storei(pname, param))
}
#[no_mangle]
unsafe extern "C" fn glGetDoublev(pname: GLenum, data: *mut GLdouble) {
    with_ctx(|mut state| state.oxidegl_get_doublev(pname, data))
}
#[no_mangle]
unsafe extern "C" fn glGetFloatv(pname: GLenum, data: *mut GLfloat) {
    with_ctx(|mut state| state.oxidegl_get_floatv(pname, data))
}
#[no_mangle]
unsafe extern "C" fn glGetIntegerv(pname: GLenum, data: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_integerv(pname, data))
}
#[no_mangle]
unsafe extern "C" fn glPointParameterfv(pname: GLenum, params: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_point_parameterfv(pname, params))
}
#[no_mangle]
unsafe extern "C" fn glPointParameteri(pname: GLenum, param: GLint) {
    with_ctx(|mut state| state.oxidegl_point_parameteri(pname, param))
}
#[no_mangle]
unsafe extern "C" fn glPointParameteriv(pname: GLenum, params: *const GLint) {
    with_ctx(|mut state| state.oxidegl_point_parameteriv(pname, params))
}
#[no_mangle]
unsafe extern "C" fn glEndQuery(target: GLenum) {
    with_ctx(|mut state| state.oxidegl_end_query(target))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) {
    with_ctx(|mut state| state.oxidegl_get_query_objectuiv(id, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glColorMaski(
    index: GLuint,
    r: GLboolean,
    g: GLboolean,
    b: GLboolean,
    a: GLboolean,
) {
    with_ctx(|mut state| state.oxidegl_color_maski(index, r, g, b, a))
}
#[no_mangle]
unsafe extern "C" fn glGetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) {
    with_ctx(|mut state| state.oxidegl_get_booleani_v(target, index, data))
}
#[no_mangle]
unsafe extern "C" fn glEnablei(target: GLenum, index: GLuint) {
    with_ctx(|mut state| state.oxidegl_enablei(target, index))
}
#[no_mangle]
unsafe extern "C" fn glDisablei(target: GLenum, index: GLuint) {
    with_ctx(|mut state| state.oxidegl_disablei(target, index))
}
#[no_mangle]
unsafe extern "C" fn glIsEnabledi(target: GLenum, index: GLuint) -> GLboolean {
    with_ctx(|mut state| state.oxidegl_is_enabledi(target, index))
}
#[no_mangle]
unsafe extern "C" fn glEndTransformFeedback() {
    with_ctx(|mut state| state.oxidegl_end_transform_feedback())
}
#[no_mangle]
unsafe extern "C" fn glEndConditionalRender() {
    with_ctx(|mut state| state.oxidegl_end_conditional_render())
}
#[no_mangle]
unsafe extern "C" fn glTexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) {
    with_ctx(|mut state| state.oxidegl_tex_parameter_iiv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glTexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_tex_parameter_iuiv(target, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte {
    with_ctx(|mut state| state.oxidegl_get_stringi(name, index))
}
#[no_mangle]
unsafe extern "C" fn glGetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_integeri_v(target, index, data))
}
#[no_mangle]
unsafe extern "C" fn glGetInteger64v(pname: GLenum, data: *mut GLint64) {
    with_ctx(|mut state| state.oxidegl_get_integer64v(pname, data))
}
#[no_mangle]
unsafe extern "C" fn glGetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) {
    with_ctx(|mut state| state.oxidegl_get_integer64i_v(target, index, data))
}
#[no_mangle]
unsafe extern "C" fn glSamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) {
    with_ctx(|mut state| state.oxidegl_sampler_parameteri(sampler, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glSamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) {
    with_ctx(|mut state| state.oxidegl_sampler_parameteriv(sampler, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glSamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_sampler_parameterfv(sampler, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glSamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint) {
    with_ctx(|mut state| state.oxidegl_sampler_parameter_iiv(sampler, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glSamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_sampler_parameter_iuiv(sampler, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glGetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_sampler_parameteriv(sampler, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint) {
    with_ctx(|mut state| state.oxidegl_get_sampler_parameter_iiv(sampler, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetSamplerParameterIuiv(
    sampler: GLuint,
    pname: GLenum,
    params: *mut GLuint,
) {
    with_ctx(|mut state| state.oxidegl_get_sampler_parameter_iuiv(sampler, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) {
    with_ctx(|mut state| state.oxidegl_get_query_objecti64v(id, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) {
    with_ctx(|mut state| state.oxidegl_get_query_objectui64v(id, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribP1uiv(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_p1uiv(index, r#type, normalized, value))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribP2uiv(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_p2uiv(index, r#type, normalized, value))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribP3uiv(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_p3uiv(index, r#type, normalized, value))
}
#[no_mangle]
unsafe extern "C" fn glVertexAttribP4uiv(
    index: GLuint,
    r#type: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    with_ctx(|mut state| state.oxidegl_vertex_attrib_p4uiv(index, r#type, normalized, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform1d(location: GLint, x: GLdouble) {
    with_ctx(|mut state| state.oxidegl_uniform1d(location, x))
}
#[no_mangle]
unsafe extern "C" fn glUniform2d(location: GLint, x: GLdouble, y: GLdouble) {
    with_ctx(|mut state| state.oxidegl_uniform2d(location, x, y))
}
#[no_mangle]
unsafe extern "C" fn glUniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) {
    with_ctx(|mut state| state.oxidegl_uniform3d(location, x, y, z))
}
#[no_mangle]
unsafe extern "C" fn glUniform4d(
    location: GLint,
    x: GLdouble,
    y: GLdouble,
    z: GLdouble,
    w: GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform4d(location, x, y, z, w))
}
#[no_mangle]
unsafe extern "C" fn glUniform1dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_uniform1dv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform2dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_uniform2dv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform3dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_uniform3dv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniform4dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    with_ctx(|mut state| state.oxidegl_uniform4dv(location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix2dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix2dv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix3dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix3dv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix4dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix4dv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix2x3dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix2x3dv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix2x4dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix2x4dv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix3x2dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix3x2dv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix3x4dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix3x4dv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix4x2dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix4x2dv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glUniformMatrix4x3dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_uniform_matrix4x3dv(location, count, transpose, value))
}
#[no_mangle]
unsafe extern "C" fn glPatchParameterfv(pname: GLenum, values: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_patch_parameterfv(pname, values))
}
#[no_mangle]
unsafe extern "C" fn glEndQueryIndexed(target: GLenum, index: GLuint) {
    with_ctx(|mut state| state.oxidegl_end_query_indexed(target, index))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) {
    with_ctx(|mut state| state.oxidegl_program_uniform1d(program, location, v0))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform1dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform1dv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform2d(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform2d(program, location, v0, v1))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform2dv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform3d(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
    v2: GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform3d(program, location, v0, v1, v2))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform3dv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform4d(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
    v2: GLdouble,
    v3: GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform4d(program, location, v0, v1, v2, v3))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniform4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    with_ctx(|mut state| state.oxidegl_program_uniform4dv(program, location, count, value))
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix2dv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix3dv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix4dv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix2x3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix2x3dv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix3x2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix3x2dv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix2x4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix2x4dv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix4x2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix4x2dv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix3x4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix3x4dv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glProgramUniformMatrix4x3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    with_ctx(|mut state| {
        state.oxidegl_program_uniform_matrix4x3dv(program, location, count, transpose, value)
    })
}
#[no_mangle]
unsafe extern "C" fn glViewportIndexedfv(index: GLuint, v: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_viewport_indexedfv(index, v))
}
#[no_mangle]
unsafe extern "C" fn glGetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat) {
    with_ctx(|mut state| state.oxidegl_get_floati_v(target, index, data))
}
#[no_mangle]
unsafe extern "C" fn glGetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble) {
    with_ctx(|mut state| state.oxidegl_get_doublei_v(target, index, data))
}
#[no_mangle]
unsafe extern "C" fn glGetTransformFeedbacki_v(
    xfb: GLuint,
    pname: GLenum,
    index: GLuint,
    param: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_transform_feedbacki_v(xfb, pname, index, param))
}
#[no_mangle]
unsafe extern "C" fn glGetTransformFeedbacki64_v(
    xfb: GLuint,
    pname: GLenum,
    index: GLuint,
    param: *mut GLint64,
) {
    with_ctx(|mut state| state.oxidegl_get_transform_feedbacki64_v(xfb, pname, index, param))
}
#[no_mangle]
unsafe extern "C" fn glTextureParameterf(texture: GLuint, pname: GLenum, param: GLfloat) {
    with_ctx(|mut state| state.oxidegl_texture_parameterf(texture, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glTextureParameterfv(texture: GLuint, pname: GLenum, param: *const GLfloat) {
    with_ctx(|mut state| state.oxidegl_texture_parameterfv(texture, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glTextureParameteri(texture: GLuint, pname: GLenum, param: GLint) {
    with_ctx(|mut state| state.oxidegl_texture_parameteri(texture, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glTextureParameterIiv(texture: GLuint, pname: GLenum, params: *const GLint) {
    with_ctx(|mut state| state.oxidegl_texture_parameter_iiv(texture, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glTextureParameterIuiv(texture: GLuint, pname: GLenum, params: *const GLuint) {
    with_ctx(|mut state| state.oxidegl_texture_parameter_iuiv(texture, pname, params))
}
#[no_mangle]
unsafe extern "C" fn glTextureParameteriv(texture: GLuint, pname: GLenum, param: *const GLint) {
    with_ctx(|mut state| state.oxidegl_texture_parameteriv(texture, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glGetVertexArrayIndexediv(
    vaobj: GLuint,
    index: GLuint,
    pname: GLenum,
    param: *mut GLint,
) {
    with_ctx(|mut state| state.oxidegl_get_vertex_array_indexediv(vaobj, index, pname, param))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryBufferObjecti64v(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    with_ctx(|mut state| state.oxidegl_get_query_buffer_objecti64v(id, buffer, pname, offset))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryBufferObjectiv(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    with_ctx(|mut state| state.oxidegl_get_query_buffer_objectiv(id, buffer, pname, offset))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryBufferObjectui64v(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    with_ctx(|mut state| state.oxidegl_get_query_buffer_objectui64v(id, buffer, pname, offset))
}
#[no_mangle]
unsafe extern "C" fn glGetQueryBufferObjectuiv(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    with_ctx(|mut state| state.oxidegl_get_query_buffer_objectuiv(id, buffer, pname, offset))
}
#[no_mangle]
unsafe extern "C" fn glSpecializeShader(
    shader: GLuint,
    pEntryPoint: *const GLchar,
    numSpecializationConstants: GLuint,
    pConstantIndex: *const GLuint,
    pConstantValue: *const GLuint,
) {
    with_ctx(|mut state| {
        state.oxidegl_specialize_shader(
            shader,
            pEntryPoint,
            numSpecializationConstants,
            pConstantIndex,
            pConstantValue,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glMultiDrawArraysIndirectCount(
    mode: GLenum,
    indirect: *const GLvoid,
    drawcount: GLintptr,
    maxdrawcount: GLsizei,
    stride: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_multi_draw_arrays_indirect_count(
            mode,
            indirect,
            drawcount,
            maxdrawcount,
            stride,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glMultiDrawElementsIndirectCount(
    mode: GLenum,
    r#type: GLenum,
    indirect: *const GLvoid,
    drawcount: GLintptr,
    maxdrawcount: GLsizei,
    stride: GLsizei,
) {
    with_ctx(|mut state| {
        state.oxidegl_multi_draw_elements_indirect_count(
            mode,
            r#type,
            indirect,
            drawcount,
            maxdrawcount,
            stride,
        )
    })
}
#[no_mangle]
unsafe extern "C" fn glPolygonOffsetClamp(factor: GLfloat, units: GLfloat, clamp: GLfloat) {
    with_ctx(|mut state| state.oxidegl_polygon_offset_clamp(factor, units, clamp))
}
