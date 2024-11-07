use objc2_metal::{MTLPrimitiveType, MTLRenderCommandEncoder};

use crate::{
    context::Context,
    dispatch::gl_types::{GLint, GLsizei, GLuint, GLvoid},
    enums::{DrawElementsType, PrimitiveType},
};

impl Context {
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `first`
    ///
    /// > Specifies the starting index in the enabled arrays.
    ///
    /// `count`
    ///
    /// > Specifies the number of indices to be rendered.
    ///
    /// ### Description
    /// [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays) specifies
    /// multiple geometric primitives with very few subroutine calls. Instead of
    /// calling a GL procedure to pass each individual vertex, normal, texture
    /// coordinate, edge flag, or color, you can prespecify separate arrays of
    /// vertices, normals, and colors and use them to construct a sequence of primitives
    /// with a single call to [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays).
    ///
    /// When [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays) is
    /// called, it uses `count` sequential elements from each enabled array to
    /// construct a sequence of geometric primitives, beginning with element `first`.
    /// `mode` specifies what kind of primitives are constructed and how the array
    /// elements construct those primitives.
    ///
    /// Vertex attributes that are modified by [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays)
    /// have an unspecified value after [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays)
    /// returns. Attributes that aren't modified remain well defined.
    ///
    /// ### Notes
    /// [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY), [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY),
    /// [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// and [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) are
    /// available only if the GL version is 3.2 or greater.

    pub fn oxidegl_draw_arrays(&mut self, mode: PrimitiveType, first: GLint, count: GLsizei) {
        //FIXME hack, move to module
        let Context {
            gl_state: state,
            platform_state,
        } = self;
        platform_state.update_state(state, true);
        #[expect(
            clippy::cast_sign_loss,
            reason = "OpenGL is a state of the art 3D graphics API"
        )]
        unsafe {
            platform_state
                .current_render_encoder()
                .drawPrimitives_vertexStart_vertexCount(
                    MTLPrimitiveType::Triangle,
                    first as usize,
                    count as usize,
                );
        };
        //panic!("command oxidegl_draw_arrays not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `indirect`
    ///
    /// > Specifies the address of a structure containing the draw parameters.
    ///
    /// ### Description
    /// [**glDrawArraysIndirect**](crate::context::Context::oxidegl_draw_arrays_indirect)
    /// specifies multiple geometric primitives with very few subroutine calls.
    /// [**glDrawArraysIndirect**](crate::context::Context::oxidegl_draw_arrays_indirect)
    /// behaves similarly to [**glDrawArraysInstancedBaseInstance**](crate::context::Context::oxidegl_draw_arrays_instanced_base_instance),
    /// execept that the parameters to [**glDrawArraysInstancedBaseInstance**](crate::context::Context::oxidegl_draw_arrays_instanced_base_instance)
    /// are stored in memory at the address given by `indirect`.
    ///
    /// The parameters addressed by `indirect` are packed into a structure that
    /// takes the form (in C):
    ///
    /// If a buffer is bound to the [`GL_DRAW_INDIRECT_BUFFER`](crate::enums::GL_DRAW_INDIRECT_BUFFER)
    /// binding at the time of a call to [**glDrawArraysIndirect**](crate::context::Context::oxidegl_draw_arrays_indirect),
    /// `indirect` is interpreted as an offset, in basic machine units, into that
    /// buffer and the parameter data is read from the buffer rather than from
    /// client memory.
    ///
    /// In contrast to [**glDrawArraysInstancedBaseInstance**](crate::context::Context::oxidegl_draw_arrays_instanced_base_instance),
    /// the
    ///
    /// Vertex attributes that are modified by [**glDrawArraysIndirect**](crate::context::Context::oxidegl_draw_arrays_indirect)
    /// have an unspecified value after [**glDrawArraysIndirect**](crate::context::Context::oxidegl_draw_arrays_indirect)
    /// returns. Attributes that aren't modified remain well defined.
    ///
    /// ### Notes
    /// The `baseInstance` member of the `DrawArraysIndirectCommand` structure
    /// is defined only if the GL version is 4.2 or greater. For versions of the
    /// GL less than 4.2, this parameter is present but is reserved and should
    /// be set to zero. On earlier versions of the GL, behavior is undefined if
    /// it is non-zero.

    pub unsafe fn oxidegl_draw_arrays_indirect(
        &mut self,
        mode: PrimitiveType,
        indirect: *const GLvoid,
    ) {
        panic!("command oxidegl_draw_arrays_indirect not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES)
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// > and [`GL_PATCHES`](crate::enums::GL_PATCHES) are accepted.
    ///
    /// `first`
    ///
    /// > Specifies the starting index in the enabled arrays.
    ///
    /// `count`
    ///
    /// > Specifies the number of indices to be rendered.
    ///
    /// `instancecount`
    ///
    /// > Specifies the number of instances of the specified range of indices to
    /// > be rendered.
    ///
    /// ### Description
    /// [**glDrawArraysInstanced**](crate::context::Context::oxidegl_draw_arrays_instanced)
    /// behaves identically to [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays)
    /// except that `instancecount` instances of the range of elements are executed
    /// and the value of the internal counter `instanceID` advances for each iteration.
    /// `instanceID` is an internal 32-bit integer counter that may be read by
    /// a vertex shader as [`gl_InstanceID`](crate::enums::gl_InstanceID).
    ///
    /// [**glDrawArraysInstanced**](crate::context::Context::oxidegl_draw_arrays_instanced)
    /// has the same effect as:

    pub fn oxidegl_draw_arrays_instanced(
        &mut self,
        mode: PrimitiveType,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    ) {
        panic!("command oxidegl_draw_arrays_instanced not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES)
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// > and [`GL_PATCHES`](crate::enums::GL_PATCHES) are accepted.
    ///
    /// `first`
    ///
    /// > Specifies the starting index in the enabled arrays.
    ///
    /// `count`
    ///
    /// > Specifies the number of indices to be rendered.
    ///
    /// `instancecount`
    ///
    /// > Specifies the number of instances of the specified range of indices to
    /// > be rendered.
    ///
    /// `baseinstance`
    ///
    /// > Specifies the base instance for use in fetching instanced vertex attributes.
    ///
    /// ### Description
    /// [**glDrawArraysInstancedBaseInstance**](crate::context::Context::oxidegl_draw_arrays_instanced_base_instance)
    /// behaves identically to [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays)
    /// except that `instancecount` instances of the range of elements are executed
    /// and the value of the internal counter `instanceID` advances for each iteration.
    /// `instanceID` is an internal 32-bit integer counter that may be read by
    /// a vertex shader as [`gl_InstanceID`](crate::enums::gl_InstanceID).
    ///
    /// [**glDrawArraysInstancedBaseInstance**](crate::context::Context::oxidegl_draw_arrays_instanced_base_instance)
    /// has the same effect as:
    ///
    /// Specific vertex attributes may be classified as *instanced* through the
    /// use of [**glVertexAttribDivisor**](crate::context::Context::oxidegl_vertex_attrib_divisor).
    /// Instanced vertex attributes supply per-instance vertex data to the vertex
    /// shader. The index of the vertex fetched from the enabled instanced vertex
    /// attribute arrays is calculated as: `[inlineq]` `baseinstance` does not
    /// affect the shader-visible value of [`gl_InstanceID`](crate::enums::gl_InstanceID).

    pub fn oxidegl_draw_arrays_instanced_base_instance(
        &mut self,
        mode: PrimitiveType,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
        baseinstance: GLuint,
    ) {
        panic!("command oxidegl_draw_arrays_instanced_base_instance not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements to be rendered.
    ///
    /// `type`
    ///
    /// > Specifies the type of the values in `indices`. Must be one of [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
    /// > [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT), or [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT).
    ///
    /// `indices`
    ///
    /// > Specifies a pointer to the location where the indices are stored.
    ///
    /// ### Description
    /// [**glDrawElements**](crate::context::Context::oxidegl_draw_elements) specifies
    /// multiple geometric primitives with very few subroutine calls. Instead of
    /// calling a GL function to pass each individual vertex, normal, texture coordinate,
    /// edge flag, or color, you can prespecify separate arrays of vertices, normals,
    /// and so on, and use them to construct a sequence of primitives with a single
    /// call to [**glDrawElements**](crate::context::Context::oxidegl_draw_elements).
    ///
    /// When [**glDrawElements**](crate::context::Context::oxidegl_draw_elements)
    /// is called, it uses `count` sequential elements from an enabled array, starting
    /// at `indices` to construct a sequence of geometric primitives. `mode` specifies
    /// what kind of primitives are constructed and how the array elements construct
    /// these primitives. If more than one array is enabled, each is used.
    ///
    /// Vertex attributes that are modified by [**glDrawElements**](crate::context::Context::oxidegl_draw_elements)
    /// have an unspecified value after [**glDrawElements**](crate::context::Context::oxidegl_draw_elements)
    /// returns. Attributes that aren't modified maintain their previous values.
    ///
    /// ### Notes
    /// [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY), [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY),
    /// [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// and [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) are
    /// available only if the GL version is 3.2 or greater.

    pub unsafe fn oxidegl_draw_elements(
        &mut self,
        mode: PrimitiveType,
        count: GLsizei,
        r#type: DrawElementsType,
        indices: *const GLvoid,
    ) {
        panic!("command oxidegl_draw_elements not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// > and [`GL_PATCHES`](crate::enums::GL_PATCHES) are accepted.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements to be rendered.
    ///
    /// `type`
    ///
    /// > Specifies the type of the values in indices. Must be one of [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
    /// > [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT), or [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT).
    ///
    /// `indices`
    ///
    /// > Specifies a pointer to the location where the indices are stored.
    ///
    /// `basevertex`
    ///
    /// > Specifies a constant that should be added to each element of `indices`
    /// > when chosing elements from the enabled vertex arrays.
    ///
    /// ### Description
    /// [**glDrawElementsBaseVertex**](crate::context::Context::oxidegl_draw_elements_base_vertex)
    /// behaves identically to [**glDrawElements**](crate::context::Context::oxidegl_draw_elements)
    /// except that the *i* th element transferred by the corresponding draw call
    /// will be taken from element `indices` \[i\]+ `basevertex` of each enabled
    /// array. If the resulting value is larger than the maximum value representable
    /// by `type`, it is as if the calculation were upconverted to 32-bit unsigned
    /// integers (with wrapping on overflow conditions). The operation is undefined
    /// if the sum would be negative.
    ///
    /// ### Notes
    /// [**glDrawElementsBaseVertex**](crate::context::Context::oxidegl_draw_elements_base_vertex)
    /// is only supported if the GL version is 3.2 or greater, or if the

    pub unsafe fn oxidegl_draw_elements_base_vertex(
        &mut self,
        mode: PrimitiveType,
        count: GLsizei,
        r#type: DrawElementsType,
        indices: *const GLvoid,
        basevertex: GLint,
    ) {
        panic!("command oxidegl_draw_elements_base_vertex not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `type`
    ///
    /// > Specifies the type of data in the buffer bound to the [`GL_ELEMENT_ARRAY_BUFFER`](crate::enums::GL_ELEMENT_ARRAY_BUFFER)
    /// > binding.
    ///
    /// `indirect`
    ///
    /// > Specifies the address of a structure containing the draw parameters.
    ///
    /// ### Description
    /// [**glDrawElementsIndirect**](crate::context::Context::oxidegl_draw_elements_indirect)
    /// specifies multiple indexed geometric primitives with very few subroutine
    /// calls. [**glDrawElementsIndirect**](crate::context::Context::oxidegl_draw_elements_indirect)
    /// behaves similarly to [**glDrawElementsInstancedBaseVertexBaseInstance**](crate::context::Context::oxidegl_draw_elements_instanced_base_vertex_base_instance),
    /// execpt that the parameters to [**glDrawElementsInstancedBaseVertexBaseInstance**](crate::context::Context::oxidegl_draw_elements_instanced_base_vertex_base_instance)
    /// are stored in memory at the address given by `indirect`.
    ///
    /// The parameters addressed by `indirect` are packed into a structure that
    /// takes the form (in C):
    ///
    /// [**glDrawElementsIndirect**](crate::context::Context::oxidegl_draw_elements_indirect)
    /// is equivalent to:
    ///
    ///
    /// If a buffer is bound to the [`GL_DRAW_INDIRECT_BUFFER`](crate::enums::GL_DRAW_INDIRECT_BUFFER)
    /// binding at the time of a call to [**glDrawElementsIndirect**](crate::context::Context::oxidegl_draw_elements_indirect),
    /// `indirect` is interpreted as an offset, in basic machine units, into that
    /// buffer and the parameter data is read from the buffer rather than from
    /// client memory.
    ///
    /// Note that indices stored in client memory are not supported. If no buffer
    /// is bound to the [`GL_ELEMENT_ARRAY_BUFFER`](crate::enums::GL_ELEMENT_ARRAY_BUFFER)
    /// binding, an error will be generated.
    ///
    /// The results of the operation are undefined if the
    ///
    /// Vertex attributes that are modified by [**glDrawElementsIndirect**](crate::context::Context::oxidegl_draw_elements_indirect)
    /// have an unspecified value after [**glDrawElementsIndirect**](crate::context::Context::oxidegl_draw_elements_indirect)
    /// returns. Attributes that aren't modified remain well defined.
    ///
    /// ### Notes
    /// The `baseInstance` member of the `DrawElementsIndirectCommand` structure
    /// is defined only if the GL version is 4.2 or greater. For versions of the
    /// GL less than 4.2, this parameter is present but is reserved and should
    /// be set to zero. On earlier versions of the GL, behavior is undefined if
    /// it is non-zero.

    pub unsafe fn oxidegl_draw_elements_indirect(
        &mut self,
        mode: PrimitiveType,
        r#type: DrawElementsType,
        indirect: *const GLvoid,
    ) {
        panic!("command oxidegl_draw_elements_indirect not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements to be rendered.
    ///
    /// `type`
    ///
    /// > Specifies the type of the values in `indices`. Must be one of [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
    /// > [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT), or [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT).
    ///
    /// `indices`
    ///
    /// > Specifies a pointer to the location where the indices are stored.
    ///
    /// `instancecount`
    ///
    /// > Specifies the number of instances of the specified range of indices to
    /// > be rendered.
    ///
    /// ### Description
    /// [**glDrawElementsInstanced**](crate::context::Context::oxidegl_draw_elements_instanced)
    /// behaves identically to [**glDrawElements**](crate::context::Context::oxidegl_draw_elements)
    /// except that `instancecount` instances of the set of elements are executed
    /// and the value of the internal counter `instanceID` advances for each iteration.
    /// `instanceID` is an internal 32-bit integer counter that may be read by
    /// a vertex shader as [`gl_InstanceID`](crate::enums::gl_InstanceID).
    ///
    /// [**glDrawElementsInstanced**](crate::context::Context::oxidegl_draw_elements_instanced)
    /// has the same effect as:
    ///
    /// ### Notes
    /// [**glDrawElementsInstanced**](crate::context::Context::oxidegl_draw_elements_instanced)
    /// is available only if the GL version is 3.1 or greater.
    ///
    /// [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY), [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY),
    /// [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// and [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) are
    /// available only if the GL version is 3.2 or greater.

    pub unsafe fn oxidegl_draw_elements_instanced(
        &mut self,
        mode: PrimitiveType,
        count: GLsizei,
        r#type: DrawElementsType,
        indices: *const GLvoid,
        instancecount: GLsizei,
    ) {
        panic!("command oxidegl_draw_elements_instanced not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements to be rendered.
    ///
    /// `type`
    ///
    /// > Specifies the type of the values in `indices`. Must be one of [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
    /// > [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT), or [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT).
    ///
    /// `indices`
    ///
    /// > Specifies a pointer to the location where the indices are stored.
    ///
    /// `instancecount`
    ///
    /// > Specifies the number of instances of the specified range of indices to
    /// > be rendered.
    ///
    /// `baseinstance`
    ///
    /// > Specifies the base instance for use in fetching instanced vertex attributes.
    ///
    /// ### Description
    /// [**glDrawElementsInstancedBaseInstance**](crate::context::Context::oxidegl_draw_elements_instanced_base_instance)
    /// behaves identically to [**glDrawElements**](crate::context::Context::oxidegl_draw_elements)
    /// except that `instancecount` instances of the set of elements are executed
    /// and the value of the internal counter `instanceID` advances for each iteration.
    /// `instanceID` is an internal 32-bit integer counter that may be read by
    /// a vertex shader as [`gl_InstanceID`](crate::enums::gl_InstanceID).
    ///
    /// [**glDrawElementsInstancedBaseInstance**](crate::context::Context::oxidegl_draw_elements_instanced_base_instance)
    /// has the same effect as:
    ///
    /// Specific vertex attributes may be classified as *instanced* through the
    /// use of [**glVertexAttribDivisor**](crate::context::Context::oxidegl_vertex_attrib_divisor).
    /// Instanced vertex attributes supply per-instance vertex data to the vertex
    /// shader. The index of the vertex fetched from the enabled instanced vertex
    /// attribute arrays is calculated as `[inlineq]` `baseinstance` does not affect
    /// the shader-visible value of [`gl_InstanceID`](crate::enums::gl_InstanceID).
    ///
    /// ### Notes
    /// [**glDrawElementsInstancedBaseInstance**](crate::context::Context::oxidegl_draw_elements_instanced_base_instance)
    /// is available only if the GL version is 4.2 or greater.
    ///
    /// [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY), [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY),
    /// [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// and [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) are
    /// available only if the GL version is 3.2 or greater.

    pub unsafe fn oxidegl_draw_elements_instanced_base_instance(
        &mut self,
        mode: PrimitiveType,
        count: GLsizei,
        r#type: DrawElementsType,
        indices: *const GLvoid,
        instancecount: GLsizei,
        baseinstance: GLuint,
    ) {
        panic!("command oxidegl_draw_elements_instanced_base_instance not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// > and [`GL_PATCHES`](crate::enums::GL_PATCHES) are accepted.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements to be rendered.
    ///
    /// `type`
    ///
    /// > Specifies the type of the values in indices. Must be one of [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
    /// > [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT), or [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT).
    ///
    /// `indices`
    ///
    /// > Specifies a pointer to the location where the indices are stored.
    ///
    /// `instancecount`
    ///
    /// > Specifies the number of instances of the indexed geometry that should be
    /// > drawn.
    ///
    /// `basevertex`
    ///
    /// > Specifies a constant that should be added to each element of `indices`
    /// > when chosing elements from the enabled vertex arrays.
    ///
    /// ### Description
    /// [**glDrawElementsInstancedBaseVertex**](crate::context::Context::oxidegl_draw_elements_instanced_base_vertex)
    /// behaves identically to [**glDrawElementsInstanced**](crate::context::Context::oxidegl_draw_elements_instanced)
    /// except that the *i* th element transferred by the corresponding draw call
    /// will be taken from element `indices` \[i\]+ `basevertex` of each enabled
    /// array. If the resulting value is larger than the maximum value representable
    /// by `type`, it is as if the calculation were upconverted to 32-bit unsigned
    /// integers (with wrapping on overflow conditions). The operation is undefined
    /// if the sum would be negative.
    ///
    /// ### Notes
    /// [**glDrawElementsInstancedBaseVertex**](crate::context::Context::oxidegl_draw_elements_instanced_base_vertex)
    /// is only supported if the GL version is 3.2 or greater.

    pub unsafe fn oxidegl_draw_elements_instanced_base_vertex(
        &mut self,
        mode: PrimitiveType,
        count: GLsizei,
        r#type: DrawElementsType,
        indices: *const GLvoid,
        instancecount: GLsizei,
        basevertex: GLint,
    ) {
        panic!("command oxidegl_draw_elements_instanced_base_vertex not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// > and [`GL_PATCHES`](crate::enums::GL_PATCHES) are accepted.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements to be rendered.
    ///
    /// `type`
    ///
    /// > Specifies the type of the values in indices. Must be one of [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
    /// > [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT), or [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT).
    ///
    /// `indices`
    ///
    /// > Specifies a pointer to the location where the indices are stored.
    ///
    /// `instancecount`
    ///
    /// > Specifies the number of instances of the indexed geometry that should be
    /// > drawn.
    ///
    /// `basevertex`
    ///
    /// > Specifies a constant that should be added to each element of `indices`
    /// > when chosing elements from the enabled vertex arrays.
    ///
    /// `baseinstance`
    ///
    /// > Specifies the base instance for use in fetching instanced vertex attributes.
    ///
    /// ### Description
    /// [**glDrawElementsInstancedBaseVertexBaseInstance**](crate::context::Context::oxidegl_draw_elements_instanced_base_vertex_base_instance)
    /// behaves identically to [**glDrawElementsInstanced**](crate::context::Context::oxidegl_draw_elements_instanced)
    /// except that the *i* th element transferred by the corresponding draw call
    /// will be taken from element `indices` \[i\]+ `basevertex` of each enabled
    /// array. If the resulting value is larger than the maximum value representable
    /// by `type`, it is as if the calculation were upconverted to 32-bit unsigned
    /// integers (with wrapping on overflow conditions). The operation is undefined
    /// if the sum would be negative.
    ///
    /// Specific vertex attributes may be classified as *instanced* through the
    /// use of [**glVertexAttribDivisor**](crate::context::Context::oxidegl_vertex_attrib_divisor).
    /// Instanced vertex attributes supply per-instance vertex data to the vertex
    /// shader. The index of the vertex fetched from the enabled instanced vertex
    /// attribute arrays is calculated as `[inlineq]` `baseinstance` does not affect
    /// the shader-visible value of [`gl_InstanceID`](crate::enums::gl_InstanceID).
    ///
    /// ### Notes
    /// [**glDrawElementsInstancedBaseVertex**](crate::context::Context::oxidegl_draw_elements_instanced_base_vertex)
    /// is only supported if the GL version is 3.2 or greater.

    pub unsafe fn oxidegl_draw_elements_instanced_base_vertex_base_instance(
        &mut self,
        mode: PrimitiveType,
        count: GLsizei,
        r#type: DrawElementsType,
        indices: *const GLvoid,
        instancecount: GLsizei,
        basevertex: GLint,
        baseinstance: GLuint,
    ) {
        panic!(
            "command oxidegl_draw_elements_instanced_base_vertex_base_instance not yet implemented"
        );
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `start`
    ///
    /// > Specifies the minimum array index contained in `indices`.
    ///
    /// `end`
    ///
    /// > Specifies the maximum array index contained in `indices`.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements to be rendered.
    ///
    /// `type`
    ///
    /// > Specifies the type of the values in `indices`. Must be one of [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
    /// > [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT), or [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT).
    ///
    /// `indices`
    ///
    /// > Specifies a pointer to the location where the indices are stored.
    ///
    /// ### Description
    /// [**glDrawRangeElements**](crate::context::Context::oxidegl_draw_range_elements)
    /// is a restricted form of [**glDrawElements**](crate::context::Context::oxidegl_draw_elements).
    /// `mode`, and `count` match the corresponding arguments to [**glDrawElements**](crate::context::Context::oxidegl_draw_elements),
    /// with the additional constraint that all values in the arrays `count` must
    /// lie between `start` and `end`, inclusive.
    ///
    /// Implementations denote recommended maximum amounts of vertex and index
    /// data, which may be queried by calling [**glGet**](crate::context::Context::oxidegl_get)
    /// with argument [`GL_MAX_ELEMENTS_VERTICES`](crate::enums::GL_MAX_ELEMENTS_VERTICES)
    /// and [`GL_MAX_ELEMENTS_INDICES`](crate::enums::GL_MAX_ELEMENTS_INDICES).
    /// If `[inlineq]` [`GL_MAX_ELEMENTS_VERTICES`](crate::enums::GL_MAX_ELEMENTS_VERTICES),
    /// or if `count` is greater than the value of [`GL_MAX_ELEMENTS_INDICES`](crate::enums::GL_MAX_ELEMENTS_INDICES),
    /// then the call may operate at reduced performance. There is no requirement
    /// that all vertices in the range `[inlineq]`
    ///
    /// When [**glDrawRangeElements**](crate::context::Context::oxidegl_draw_range_elements)
    /// is called, it uses `count` sequential elements from an enabled array, starting
    /// at `start` to construct a sequence of geometric primitives. `mode` specifies
    /// what kind of primitives are constructed, and how the array elements construct
    /// these primitives. If more than one array is enabled, each is used.
    ///
    /// Vertex attributes that are modified by [**glDrawRangeElements**](crate::context::Context::oxidegl_draw_range_elements)
    /// have an unspecified value after [**glDrawRangeElements**](crate::context::Context::oxidegl_draw_range_elements)
    /// returns. Attributes that aren't modified maintain their previous values.
    ///
    /// ### Notes
    /// [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY), [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY),
    /// [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// and [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY) are
    /// available only if the GL version is 3.2 or greater.
    ///
    /// ### Associated Gets
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_ELEMENTS_VERTICES`](crate::enums::GL_MAX_ELEMENTS_VERTICES)
    ///
    /// [**glGet**](crate::context::Context::oxidegl_get) with argument [`GL_MAX_ELEMENTS_INDICES`](crate::enums::GL_MAX_ELEMENTS_INDICES)

    pub unsafe fn oxidegl_draw_range_elements(
        &mut self,
        mode: PrimitiveType,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        r#type: DrawElementsType,
        indices: *const GLvoid,
    ) {
        panic!("command oxidegl_draw_range_elements not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY)
    /// > and [`GL_PATCHES`](crate::enums::GL_PATCHES) are accepted.
    ///
    /// `start`
    ///
    /// > Specifies the minimum array index contained in `indices`.
    ///
    /// `end`
    ///
    /// > Specifies the maximum array index contained in `indices`.
    ///
    /// `count`
    ///
    /// > Specifies the number of elements to be rendered.
    ///
    /// `type`
    ///
    /// > Specifies the type of the values in indices. Must be one of [`GL_UNSIGNED_BYTE`](crate::enums::GL_UNSIGNED_BYTE),
    /// > [`GL_UNSIGNED_SHORT`](crate::enums::GL_UNSIGNED_SHORT), or [`GL_UNSIGNED_INT`](crate::enums::GL_UNSIGNED_INT).
    ///
    /// `indices`
    ///
    /// > Specifies a pointer to the location where the indices are stored.
    ///
    /// `basevertex`
    ///
    /// > Specifies a constant that should be added to each element of `indices`
    /// > when chosing elements from the enabled vertex arrays.
    ///
    /// ### Description
    /// [**glDrawRangeElementsBaseVertex**](crate::context::Context::oxidegl_draw_range_elements_base_vertex)
    /// is a restricted form of [**glDrawElementsBaseVertex**](crate::context::Context::oxidegl_draw_elements_base_vertex).
    /// `mode`, `count` and `basevertex` match the corresponding arguments to [**glDrawElementsBaseVertex**](crate::context::Context::oxidegl_draw_elements_base_vertex),
    /// with the additional constraint that all values in the array `indices`
    /// must lie between `start` and `end`, inclusive, prior to adding `basevertex`.
    /// Index values lying outside the range \[ `start`, `end` \] are treated
    /// in the same way as [**glDrawElementsBaseVertex**](crate::context::Context::oxidegl_draw_elements_base_vertex).
    /// The *i* th element transferred by the corresponding draw call will be
    /// taken from element `indices` \[i\]+ `basevertex` of each enabled array.
    /// If the resulting value is larger than the maximum value representable by
    /// `type`, it is as if the calculation were upconverted to 32-bit unsigned
    /// integers (with wrapping on overflow conditions). The operation is undefined
    /// if the sum would be negative.

    pub unsafe fn oxidegl_draw_range_elements_base_vertex(
        &mut self,
        mode: PrimitiveType,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        r#type: DrawElementsType,
        indices: *const GLvoid,
        basevertex: GLint,
    ) {
        panic!("command oxidegl_draw_range_elements_base_vertex not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `id`
    ///
    /// > Specifies the name of a transform feedback object from which to retrieve
    /// > a primitive count.
    ///
    /// ### Description
    /// [**glDrawTransformFeedback**](crate::context::Context::oxidegl_draw_transform_feedback)
    /// draws primitives of a type specified by `mode` using a count retrieved
    /// from the transform feedback specified by `id`. Calling [**glDrawTransformFeedback**](crate::context::Context::oxidegl_draw_transform_feedback)
    /// is equivalent to calling [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays)
    /// with `mode` as specified, `first` set to zero, and `count` set to the number
    /// of vertices captured on vertex stream zero the last time transform feedback
    /// was active on the transform feedback object named by `id`.

    pub fn oxidegl_draw_transform_feedback(&mut self, mode: PrimitiveType, id: GLuint) {
        panic!("command oxidegl_draw_transform_feedback not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `id`
    ///
    /// > Specifies the name of a transform feedback object from which to retrieve
    /// > a primitive count.
    ///
    /// `instancecount`
    ///
    /// > Specifies the number of instances of the geometry to render.
    ///
    /// ### Description
    /// [**glDrawTransformFeedbackInstanced**](crate::context::Context::oxidegl_draw_transform_feedback_instanced)
    /// draws multiple copies of a range of primitives of a type specified by `mode`
    /// using a count retrieved from the transform feedback stream specified by
    /// `stream` of the transform feedback object specified by `id`. Calling [**glDrawTransformFeedbackInstanced**](crate::context::Context::oxidegl_draw_transform_feedback_instanced)
    /// is equivalent to calling [**glDrawArraysInstanced**](crate::context::Context::oxidegl_draw_arrays_instanced)
    /// with `mode` and `instancecount` as specified, `first` set to zero, and
    /// `count` set to the number of vertices captured on vertex stream zero the
    /// last time transform feedback was active on the transform feedback object
    /// named by `id`.
    ///
    /// Calling [**glDrawTransformFeedbackInstanced**](crate::context::Context::oxidegl_draw_transform_feedback_instanced)
    /// is equivalent to calling [**glDrawTransformFeedbackStreamInstanced**](crate::context::Context::oxidegl_draw_transform_feedback_stream_instanced)
    /// with `stream` set to zero.

    pub fn oxidegl_draw_transform_feedback_instanced(
        &mut self,
        mode: PrimitiveType,
        id: GLuint,
        instancecount: GLsizei,
    ) {
        panic!("command oxidegl_draw_transform_feedback_instanced not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `id`
    ///
    /// > Specifies the name of a transform feedback object from which to retrieve
    /// > a primitive count.
    ///
    /// `stream`
    ///
    /// > Specifies the index of the transform feedback stream from which to retrieve
    /// > a primitive count.
    ///
    /// ### Description
    /// [**glDrawTransformFeedbackStream**](crate::context::Context::oxidegl_draw_transform_feedback_stream)
    /// draws primitives of a type specified by `mode` using a count retrieved
    /// from the transform feedback stream specified by `stream` of the transform
    /// feedback object specified by `id`. Calling [**glDrawTransformFeedbackStream**](crate::context::Context::oxidegl_draw_transform_feedback_stream)
    /// is equivalent to calling [**glDrawArrays**](crate::context::Context::oxidegl_draw_arrays)
    /// with `mode` as specified, `first` set to zero, and `count` set to the number
    /// of vertices captured on vertex stream `stream` the last time transform
    /// feedback was active on the transform feedback object named by `id`.
    ///
    /// Calling [**glDrawTransformFeedback**](crate::context::Context::oxidegl_draw_transform_feedback)
    /// is equivalent to calling [**glDrawTransformFeedbackStream**](crate::context::Context::oxidegl_draw_transform_feedback_stream)
    /// with `stream` set to zero.

    pub fn oxidegl_draw_transform_feedback_stream(
        &mut self,
        mode: PrimitiveType,
        id: GLuint,
        stream: GLuint,
    ) {
        panic!("command oxidegl_draw_transform_feedback_stream not yet implemented");
    }
    /// ### Parameters
    /// `mode`
    ///
    /// > Specifies what kind of primitives to render. Symbolic constants [`GL_POINTS`](crate::enums::GL_POINTS),
    /// > [`GL_LINE_STRIP`](crate::enums::GL_LINE_STRIP), [`GL_LINE_LOOP`](crate::enums::GL_LINE_LOOP),
    /// > [`GL_LINES`](crate::enums::GL_LINES), [`GL_LINE_STRIP_ADJACENCY`](crate::enums::GL_LINE_STRIP_ADJACENCY),
    /// > [`GL_LINES_ADJACENCY`](crate::enums::GL_LINES_ADJACENCY), [`GL_TRIANGLE_STRIP`](crate::enums::GL_TRIANGLE_STRIP),
    /// > [`GL_TRIANGLE_FAN`](crate::enums::GL_TRIANGLE_FAN), [`GL_TRIANGLES`](crate::enums::GL_TRIANGLES),
    /// > [`GL_TRIANGLE_STRIP_ADJACENCY`](crate::enums::GL_TRIANGLE_STRIP_ADJACENCY),
    /// > [`GL_TRIANGLES_ADJACENCY`](crate::enums::GL_TRIANGLES_ADJACENCY), and [`GL_PATCHES`](crate::enums::GL_PATCHES)
    /// > are accepted.
    ///
    /// `id`
    ///
    /// > Specifies the name of a transform feedback object from which to retrieve
    /// > a primitive count.
    ///
    /// `stream`
    ///
    /// > Specifies the index of the transform feedback stream from which to retrieve
    /// > a primitive count.
    ///
    /// `instancecount`
    ///
    /// > Specifies the number of instances of the geometry to render.
    ///
    /// ### Description
    /// [**glDrawTransformFeedbackStreamInstanced**](crate::context::Context::oxidegl_draw_transform_feedback_stream_instanced)
    /// draws multiple copies of a range of primitives of a type specified by `mode`
    /// using a count retrieved from the transform feedback stream specified by
    /// `stream` of the transform feedback object specified by `id`. Calling [**glDrawTransformFeedbackStreamInstanced**](crate::context::Context::oxidegl_draw_transform_feedback_stream_instanced)
    /// is equivalent to calling [**glDrawArraysInstanced**](crate::context::Context::oxidegl_draw_arrays_instanced)
    /// with `mode` and `instancecount` as specified, `first` set to zero, and
    /// `count` set to the number of vertices captured on vertex stream `stream`
    /// the last time transform feedback was active on the transform feedback object
    /// named by `id`.
    ///
    /// Calling [**glDrawTransformFeedbackInstanced**](crate::context::Context::oxidegl_draw_transform_feedback_instanced)
    /// is equivalent to calling [**glDrawTransformFeedbackStreamInstanced**](crate::context::Context::oxidegl_draw_transform_feedback_stream_instanced)
    /// with `stream` set to zero.

    pub fn oxidegl_draw_transform_feedback_stream_instanced(
        &mut self,
        mode: PrimitiveType,
        id: GLuint,
        stream: GLuint,
        instancecount: GLsizei,
    ) {
        panic!("command oxidegl_draw_transform_feedback_stream_instanced not yet implemented");
    }
}
