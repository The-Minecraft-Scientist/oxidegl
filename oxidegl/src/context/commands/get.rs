#[allow(clippy::wildcard_imports)]
use crate::{
    context::state::item::OxideGLItem,
    dispatch::gltypes::*,
    enums::{
        GL_CULL_FACE_MODE, GL_LINE_WIDTH, GL_LINE_WIDTH_GRANULARITY, GL_LINE_WIDTH_RANGE,
        GL_POINT_SIZE, GL_POINT_SIZE_GRANULARITY, GL_POINT_SIZE_RANGE, GL_POLYGON_MODE,
        GL_RENDERER, GL_SHADING_LANGUAGE_VERSION, GL_VENDOR, GL_VERSION,
    },
};

use crate::context::Context;

impl Context {
    fn get(&self, parameter_name: GLenum, idx: Option<GLint>) -> OxideGLItem {
        let item: OxideGLItem = match parameter_name {
            GL_POINT_SIZE => self.gl_state.point_size.into(), // GL_POINT_SIZE
            GL_POINT_SIZE_RANGE => self.gl_state.characteristics.point_size_range.into(), // GL_POINT_SIZE_RANGE
            GL_POINT_SIZE_GRANULARITY => {
                self.gl_state.characteristics.point_size_granularity.into()
            } // GL_POINT_SIZE_GRANULARITY
            GL_LINE_WIDTH => self.gl_state.line_width.into(), // GL_LINE_WIDTH
            GL_LINE_WIDTH_RANGE => self.gl_state.characteristics.line_width_range.into(), // GL_LINE_WIDTH_RANGE
            GL_LINE_WIDTH_GRANULARITY => {
                self.gl_state.characteristics.line_width_granularity.into()
            } // GL_LINE_WIDTH_GRANULARITY
            GL_POLYGON_MODE => self.gl_state.polygon_mode.into(), // GL_POLYGON_MODE
            GL_CULL_FACE_MODE => self.gl_state.cull_face_mode.into(), // GL_CULL_FACE_MODE
            // 0x0B46 => self.state.front_face.into(), // GL_FRONT_FACE

            // 0x0B70 => self.attribs.depth_range, 2.into(), // GL_DEPTH_RANGE

            // 0x0B72 => self.state.depth_writemask.into(), // GL_DEPTH_WRITEMASK
            // 0x0B73 => self.state.depth_clear_value.into(), // GL_DEPTH_CLEAR_VALUE
            // 0x0B74 => self.state.depth_func.into(), // GL_DEPTH_FUNC
            // 0x0B91 => self.state.stencil_clear_value.into(), // GL_STENCIL_CLEAR_VALUE
            // 0x0B92 => self.state.stencil_func.into(), // GL_STENCIL_FUNC
            // 0x0B93 => self.state.stencil_value_mask.into(), // GL_STENCIL_VALUE_MASK
            // 0x0B94 => self.state.stencil_fail.into(), // GL_STENCIL_FAIL
            // 0x0B95 => self.state.stencil_pass_depth_fail.into(), // GL_STENCIL_PASS_DEPTH_FAIL
            // 0x0B96 => self.state.stencil_pass_depth_pass.into(), // GL_STENCIL_PASS_DEPTH_PASS
            // 0x0B97 => self.state.stencil_ref.into(), // GL_STENCIL_REF
            // 0x0B98 => self.state.stencil_writemask.into(), // GL_STENCIL_WRITEMASK

            // 0x0BA2 => RET_TYPE_COUNT(type, viewport, 4.into(), // GL_VIEWPORT

            // 0x0BE0 => self.state.blend_dst_rgb[0].into(), // GL_BLEND_DST
            // 0x0BE1 => self.state.blend_src_rgb[0].into(), // GL_BLEND_SRC

            // 0x0BF0 => self.state.logic_op_mode.into(), // GL_LOGIC_OP_MODE
            // 0x0C01 => RET_TYPE(type, draw_buffer.into(), // GL_DRAW_BUFFER
            // 0x0C02 => RET_TYPE(type, read_buffer.into(), // GL_READ_BUFFER

            // 0x0C10 => RET_TYPE_VAR_COUNT(type, scissor_box, 4.into(), // GL_SCISSOR_BOX

            // 0x0C22 => RET_TYPE_COUNT(type, color_clear_value, 4.into(), // GL_COLOR_CLEAR_VALUE

            // 0x0C23 => RET_TYPE_VAR_COUNT(type, color_writemask[0], 4.into(), // GL_COLOR_WRITEMASK

            // 0x0D33 => self.state.max_texture_size.into(), // GL_MAX_TEXTURE_SIZE
            // 0x0D3A => self.state.max_viewport_dims.into(), // GL_MAX_VIEWPORT_DIMS
            // 0x0D50 => self.state.subpixel_bits.into(), // GL_SUBPIXEL_BITS
            // 0x0B00 => self.state.current_color.into(), // GL_CURRENT_COLOR
            // 0x0B01 => self.state.current_index.into(), // GL_CURRENT_INDEX
            // 0x0B02 => self.state.current_normal.into(), // GL_CURRENT_NORMAL
            // 0x0B04 => self.state.current_raster_color.into(), // GL_CURRENT_RASTER_COLOR
            // 0x0B05 => self.state.current_raster_index.into(), // GL_CURRENT_RASTER_INDEX
            // 0x0B06 => self.state.current_raster_texture_coords.into(), // GL_CURRENT_RASTER_TEXTURE_COORDS
            // 0x0B07 => self.state.current_raster_position.into(), // GL_CURRENT_RASTER_POSITION
            // 0x0B08 => self.state.current_raster_position_valid.into(), // GL_CURRENT_RASTER_POSITION_VALID
            // 0x0B09 => self.state.current_raster_distance.into(), // GL_CURRENT_RASTER_DISTANCE
            // 0x0B25 => self.state.line_stipple_pattern.into(), // GL_LINE_STIPPLE_PATTERN
            // 0x0B26 => self.state.line_stipple_repeat.into(), // GL_LINE_STIPPLE_REPEAT
            // 0x0B30 => self.state.list_mode.into(), // GL_LIST_MODE
            // 0x0B31 => self.state.max_list_nesting.into(), // GL_MAX_LIST_NESTING
            // 0x0B32 => self.state.list_base.into(), // GL_LIST_BASE
            // 0x0B33 => self.state.list_index.into(), // GL_LIST_INDEX
            // 0x0B43 => self.state.edge_flag.into(), // GL_EDGE_FLAG
            // 0x0B54 => self.state.shade_model.into(), // GL_SHADE_MODEL
            // 0x0B55 => self.state.color_material_face.into(), // GL_COLOR_MATERIAL_FACE
            // 0x0B56 => self.state.color_material_parameter.into(), // GL_COLOR_MATERIAL_PARAMETER
            // 0x0B80 => self.state.accum_clear_value.into(), // GL_ACCUM_CLEAR_VALUE
            // 0x0BA0 => self.state.matrix_mode.into(), // GL_MATRIX_MODE
            // 0x0BA3 => self.state.modelview_stack_depth.into(), // GL_MODELVIEW_STACK_DEPTH
            // 0x0BA4 => self.state.projection_stack_depth.into(), // GL_PROJECTION_STACK_DEPTH
            // 0x0BA5 => self.state.texture_stack_depth.into(), // GL_TEXTURE_STACK_DEPTH
            // 0x0BA6 => self.state.modelview_matrix.into(), // GL_MODELVIEW_MATRIX
            // 0x0BA7 => self.state.projection_matrix.into(), // GL_PROJECTION_MATRIX
            // 0x0BB0 => self.state.attrib_stack_depth.into(), // GL_ATTRIB_STACK_DEPTH
            // 0x0BC1 => self.state.alpha_test_func.into(), // GL_ALPHA_TEST_FUNC
            // 0x0BC2 => self.state.alpha_test_ref.into(), // GL_ALPHA_TEST_REF
            // 0x0BF1 => self.state.logic_op.into(), // GL_LOGIC_OP
            // 0x0C00 => self.state.aux_buffers.into(), // GL_AUX_BUFFERS
            // 0x0C20 => self.state.index_clear_value.into(), // GL_INDEX_CLEAR_VALUE
            // 0x0C21 => self.state.index_writemask.into(), // GL_INDEX_WRITEMASK
            // 0x0C30 => self.state.index_mode.into(), // GL_INDEX_MODE
            // 0x0C31 => self.state.rgba_mode.into(), // GL_RGBA_MODE
            // 0x0C40 => self.state.render_mode.into(), // GL_RENDER_MODE
            // 0x0CB0 => self.state.pixel_map_i_to_i_size.into(), // GL_PIXEL_MAP_I_TO_I_SIZE
            // 0x0CB1 => self.state.pixel_map_s_to_s_size.into(), // GL_PIXEL_MAP_S_TO_S_SIZE
            // 0x0CB2 => self.state.pixel_map_i_to_r_size.into(), // GL_PIXEL_MAP_I_TO_R_SIZE
            // 0x0CB3 => self.state.pixel_map_i_to_g_size.into(), // GL_PIXEL_MAP_I_TO_G_SIZE
            // 0x0CB4 => self.state.pixel_map_i_to_b_size.into(), // GL_PIXEL_MAP_I_TO_B_SIZE
            // 0x0CB5 => self.state.pixel_map_i_to_a_size.into(), // GL_PIXEL_MAP_I_TO_A_SIZE
            // 0x0CB6 => self.state.pixel_map_r_to_r_size.into(), // GL_PIXEL_MAP_R_TO_R_SIZE
            // 0x0CB7 => self.state.pixel_map_g_to_g_size.into(), // GL_PIXEL_MAP_G_TO_G_SIZE
            // 0x0CB8 => self.state.pixel_map_b_to_b_size.into(), // GL_PIXEL_MAP_B_TO_B_SIZE
            // 0x0CB9 => self.state.pixel_map_a_to_a_size.into(), // GL_PIXEL_MAP_A_TO_A_SIZE
            // 0x0D16 => self.state.zoom_x.into(), // GL_ZOOM_X
            // 0x0D17 => self.state.zoom_y.into(), // GL_ZOOM_Y
            // 0x0D30 => self.state.max_eval_order.into(), // GL_MAX_EVAL_ORDER
            // 0x0D31 => self.state.max_lights.into(), // GL_MAX_LIGHTS
            // 0x0D32 => self.state.max_clip_planes.into(), // GL_MAX_CLIP_PLANES
            // 0x0D34 => self.state.max_pixel_map_table.into(), // GL_MAX_PIXEL_MAP_TABLE
            // 0x0D35 => self.state.max_attrib_stack_depth.into(), // GL_MAX_ATTRIB_STACK_DEPTH
            // 0x0D36 => self.state.max_modelview_stack_depth.into(), // GL_MAX_MODELVIEW_STACK_DEPTH
            // 0x0D37 => self.state.max_name_stack_depth.into(), // GL_MAX_NAME_STACK_DEPTH
            // 0x0D38 => self.state.max_projection_stack_depth.into(), // GL_MAX_PROJECTION_STACK_DEPTH
            // 0x0D39 => self.state.max_texture_stack_depth.into(), // GL_MAX_TEXTURE_STACK_DEPTH
            // 0x0D51 => self.state.index_bits.into(), // GL_INDEX_BITS
            // 0x0D52 => self.state.red_bits.into(), // GL_RED_BITS
            // 0x0D53 => self.state.green_bits.into(), // GL_GREEN_BITS
            // 0x0D54 => self.state.blue_bits.into(), // GL_BLUE_BITS
            // 0x0D55 => self.state.alpha_bits.into(), // GL_ALPHA_BITS
            // 0x0D56 => self.state.depth_bits.into(), // GL_DEPTH_BITS
            // 0x0D57 => self.state.stencil_bits.into(), // GL_STENCIL_BITS
            // 0x0D58 => self.state.accum_red_bits.into(), // GL_ACCUM_RED_BITS
            // 0x0D59 => self.state.accum_green_bits.into(), // GL_ACCUM_GREEN_BITS
            // 0x0D5A => self.state.accum_blue_bits.into(), // GL_ACCUM_BLUE_BITS
            // 0x0D5B => self.state.accum_alpha_bits.into(), // GL_ACCUM_ALPHA_BITS
            // 0x0D70 => self.state.name_stack_depth.into(), // GL_NAME_STACK_DEPTH
            // 0x0DD0 => self.state.map1_grid_domain.into(), // GL_MAP1_GRID_DOMAIN
            // 0x0DD1 => self.state.map1_grid_segments.into(), // GL_MAP1_GRID_SEGMENTS
            // 0x0DD2 => self.state.map2_grid_domain.into(), // GL_MAP2_GRID_DOMAIN
            // 0x0DD3 => self.state.map2_grid_segments.into(), // GL_MAP2_GRID_SEGMENTS
            // 0x2A00 => self.state.polygon_offset_units.into(), // GL_POLYGON_OFFSET_UNITS
            // 0x8038 => self.state.polygon_offset_factor.into(), // GL_POLYGON_OFFSET_FACTOR
            // 0x8068 => self.state.texture_binding_1d.into(), // GL_TEXTURE_BINDING_1D
            // 0x8069 => self.state.texture_binding_2d.into(), // GL_TEXTURE_BINDING_2D
            // 0x0BB1 => self.state.client_attrib_stack_depth.into(), // GL_CLIENT_ATTRIB_STACK_DEPTH
            // 0x0D3B => self.state.max_client_attrib_stack_depth.into(), // GL_MAX_CLIENT_ATTRIB_STACK_DEPTH
            // 0x0DF1 => self.state.feedback_buffer_size.into(), // GL_FEEDBACK_BUFFER_SIZE
            // 0x0DF2 => self.state.feedback_buffer_type.into(), // GL_FEEDBACK_BUFFER_TYPE
            // 0x0DF4 => self.state.selection_buffer_size.into(), // GL_SELECTION_BUFFER_SIZE
            // 0x807A => self.state.vertex_array_size.into(), // GL_VERTEX_ARRAY_SIZE
            // 0x807B => self.state.vertex_array_type.into(), // GL_VERTEX_ARRAY_TYPE
            // 0x807C => self.state.vertex_array_stride.into(), // GL_VERTEX_ARRAY_STRIDE
            // 0x807E => self.state.normal_array_type.into(), // GL_NORMAL_ARRAY_TYPE
            // 0x807F => self.state.normal_array_stride.into(), // GL_NORMAL_ARRAY_STRIDE
            // 0x8081 => self.state.color_array_size.into(), // GL_COLOR_ARRAY_SIZE
            // 0x8082 => self.state.color_array_type.into(), // GL_COLOR_ARRAY_TYPE
            // 0x8083 => self.state.color_array_stride.into(), // GL_COLOR_ARRAY_STRIDE
            // 0x8085 => self.state.index_array_type.into(), // GL_INDEX_ARRAY_TYPE
            // 0x8086 => self.state.index_array_stride.into(), // GL_INDEX_ARRAY_STRIDE
            // 0x8088 => self.state.texture_coord_array_size.into(), // GL_TEXTURE_COORD_ARRAY_SIZE
            // 0x8089 => self.state.texture_coord_array_type.into(), // GL_TEXTURE_COORD_ARRAY_TYPE
            // 0x808A => self.state.texture_coord_array_stride.into(), // GL_TEXTURE_COORD_ARRAY_STRIDE
            // 0x808C => self.state.edge_flag_array_stride.into(), // GL_EDGE_FLAG_ARRAY_STRIDE
            // 0x806A => self.state.texture_binding_3d.into(), // GL_TEXTURE_BINDING_3D
            // 0x8073 => self.state.max_3d_texture_size.into(), // GL_MAX_3D_TEXTURE_SIZE
            // 0x80E8 => self.state.max_elements_vertices.into(), // GL_MAX_ELEMENTS_VERTICES
            // 0x80E9 => self.state.max_elements_indices.into(), // GL_MAX_ELEMENTS_INDICES
            // 0x846E => self.state.aliased_line_width_range.into(), // GL_ALIASED_LINE_WIDTH_RANGE
            // 0x846D => self.state.aliased_point_size_range.into(), // GL_ALIASED_POINT_SIZE_RANGE
            // 0x84E0 => RET_TYPE(type, active_texture.into(), // GL_ACTIVE_TEXTURE
            // 0x80AA => self.state.sample_coverage_value.into(), // GL_SAMPLE_COVERAGE_VALUE
            // 0x80AB => self.state.sample_coverage_invert.into(), // GL_SAMPLE_COVERAGE_INVERT
            // 0x8514 => self.state.texture_binding_cube_map.into(), // GL_TEXTURE_BINDING_CUBE_MAP
            // 0x851C => self.state.max_cube_map_texture_size.into(), // GL_MAX_CUBE_MAP_TEXTURE_SIZE
            // 0x86A2 => self.state.num_compressed_texture_formats.into(), // GL_NUM_COMPRESSED_TEXTURE_FORMATS
            // 0x86A3 => self.state.compressed_texture_formats.into(), // GL_COMPRESSED_TEXTURE_FORMATS

            // 0x80C8 => self.state.blend_dst_rgb[0].into(), // GL_BLEND_DST_RGB
            // 0x80C9 => self.state.blend_src_rgb[0].into(), // GL_BLEND_SRC_RGB
            // 0x80CA => self.state.blend_dst_alpha[0].into(), // GL_BLEND_DST_ALPHA
            // 0x80CB => self.state.blend_src_alpha[0].into(), // GL_BLEND_SRC_ALPHA

            // 0x84FD => self.state.max_texture_lod_bias.into(), // GL_MAX_TEXTURE_LOD_BIAS

            // 0x8005 => RET_TYPE_VAR_COUNT(type, blend_color,4.into(), // GL_BLEND_COLOR

            // 0x8894 => self.state.array_buffer_binding.into(), // GL_ARRAY_BUFFER_BINDING
            // 0x8895 => self.state.element_array_buffer_binding.into(), // GL_ELEMENT_ARRAY_BUFFER_BINDING
            // 0x8009 => self.state.blend_equation_rgb[0].into(), // GL_BLEND_EQUATION_RGB
            // 0x8800 => self.state.stencil_back_func.into(), // GL_STENCIL_BACK_FUNC
            // 0x8801 => self.state.stencil_back_fail.into(), // GL_STENCIL_BACK_FAIL
            // 0x8802 => self.state.stencil_back_pass_depth_fail.into(), // GL_STENCIL_BACK_PASS_DEPTH_FAIL
            // 0x8803 => self.state.stencil_back_pass_depth_pass.into(), // GL_STENCIL_BACK_PASS_DEPTH_PASS
            // 0x8824 => self.state.max_draw_buffers.into(), // GL_MAX_DRAW_BUFFERS

            // 0x883D => self.state.blend_equation_alpha[0].into(), // GL_BLEND_EQUATION_ALPHA

            // 0x8869 => self.max_vertex_attribs.into(), // GL_MAX_VERTEX_ATTRIBS
            // 0x8872 => self.state.max_texture_image_units.into(), // GL_MAX_TEXTURE_IMAGE_UNITS
            // 0x8B49 => self.state.max_fragment_uniform_components.into(), // GL_MAX_FRAGMENT_UNIFORM_COMPONENTS
            // 0x8B4A => self.state.max_vertex_uniform_components.into(), // GL_MAX_VERTEX_UNIFORM_COMPONENTS
            // 0x8B4B => self.state.max_varying_floats.into(), // GL_MAX_VARYING_FLOATS
            // 0x8B4C => self.state.max_vertex_texture_image_units.into(), // GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS
            // 0x8B4D => self.state.max_combined_texture_image_units.into(), // GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS
            // 0x8B8D => self.state.current_program.into(), // GL_CURRENT_PROGRAM
            // 0x8CA3 => self.state.stencil_back_ref.into(), // GL_STENCIL_BACK_REF
            // 0x8CA4 => self.state.stencil_back_value_mask.into(), // GL_STENCIL_BACK_VALUE_MASK
            // 0x8CA5 => self.state.stencil_back_writemask.into(), // GL_STENCIL_BACK_WRITEMASK
            // 0x88ED => self.state.pixel_pack_buffer_binding.into(), // GL_PIXEL_PACK_BUFFER_BINDING
            // 0x88EF => self.state.pixel_unpack_buffer_binding.into(), // GL_PIXEL_UNPACK_BUFFER_BINDING
            // 0x821B => self.state.major_version.into(), // GL_MAJOR_VERSION
            // 0x821C => self.state.minor_version.into(), // GL_MINOR_VERSION
            0x821D => self.gl_state.characteristics.num_extensions.into(), // GL_NUM_EXTENSIONS
            0x821E => self.gl_state.characteristics.context_flags.into(),
            // GL_CONTEXT_FLAGS
            // 0x88FF => self.state.max_array_texture_layers.into(), // GL_MAX_ARRAY_TEXTURE_LAYERS
            // 0x8904 => self.state.min_program_texel_offset.into(), // GL_MIN_PROGRAM_TEXEL_OFFSET
            // 0x8905 => self.state.max_program_texel_offset.into(), // GL_MAX_PROGRAM_TEXEL_OFFSET
            // 0x8C1C => self.state.texture_binding_1d_array.into(), // GL_TEXTURE_BINDING_1D_ARRAY
            // 0x8C1D => self.state.texture_binding_2d_array.into(), // GL_TEXTURE_BINDING_2D_ARRAY
            // 0x84E8 => self.state.max_renderbuffer_size.into(), // GL_MAX_RENDERBUFFER_SIZE
            // 0x8CA6 => self.state.draw_framebuffer_binding.into(), // GL_DRAW_FRAMEBUFFER_BINDING
            // 0x8CA7 => self.state.renderbuffer_binding.into(), // GL_RENDERBUFFER_BINDING
            // 0x8CAA => self.state.read_framebuffer_binding.into(), // GL_READ_FRAMEBUFFER_BINDING
            // 0x85B5 => self.state.vertex_array_binding.into(), // GL_VERTEX_ARRAY_BINDING
            // 0x8C2B => self.state.max_texture_buffer_size.into(), // GL_MAX_TEXTURE_BUFFER_SIZE
            // 0x8C2C => self.state.texture_binding_buffer.into(), // GL_TEXTURE_BINDING_BUFFER
            // 0x84F6 => self.state.texture_binding_rectangle.into(), // GL_TEXTURE_BINDING_RECTANGLE
            // 0x84F8 => self.state.max_rectangle_texture_size.into(), // GL_MAX_RECTANGLE_TEXTURE_SIZE
            // 0x8F9E => self.state.primitive_restart_index.into(), // GL_PRIMITIVE_RESTART_INDEX
            // 0x8A28 => self.state.uniform_buffer_binding.into(), // GL_UNIFORM_BUFFER_BINDING
            // 0x8A29 => self.state.uniform_buffer_start.into(), // GL_UNIFORM_BUFFER_START
            // 0x8A2A => self.state.uniform_buffer_size.into(), // GL_UNIFORM_BUFFER_SIZE
            // 0x8A2B => self.state.max_vertex_uniform_blocks.into(), // GL_MAX_VERTEX_UNIFORM_BLOCKS
            // 0x8A2C => self.state.max_geometry_uniform_blocks.into(), // GL_MAX_GEOMETRY_UNIFORM_BLOCKS
            // 0x8A2D => self.state.max_fragment_uniform_blocks.into(), // GL_MAX_FRAGMENT_UNIFORM_BLOCKS
            // 0x8A2E => self.state.max_combined_uniform_blocks.into(), // GL_MAX_COMBINED_UNIFORM_BLOCKS
            // 0x8A2F => self.state.max_uniform_buffer_bindings.into(), // GL_MAX_UNIFORM_BUFFER_BINDINGS
            // 0x8A30 => self.state.max_uniform_block_size.into(), // GL_MAX_UNIFORM_BLOCK_SIZE
            // 0x8A31 => self.state.max_combined_vertex_uniform_components.into(), // GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS
            // 0x8A32 => self.state.max_combined_geometry_uniform_components.into(), // GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS
            // 0x8A33 => self.state.max_combined_fragment_uniform_components.into(), // GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS
            // 0x8A34 => self.state.uniform_buffer_offset_alignment.into(), // GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT
            // 0x8C29 => self.state.max_geometry_texture_image_units.into(), // GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS
            // 0x8DDF => self.state.max_geometry_uniform_components.into(), // GL_MAX_GEOMETRY_UNIFORM_COMPONENTS
            // 0x9122 => self.state.max_vertex_output_components.into(), // GL_MAX_VERTEX_OUTPUT_COMPONENTS
            // 0x9123 => self.state.max_geometry_input_components.into(), // GL_MAX_GEOMETRY_INPUT_COMPONENTS
            // 0x9124 => self.state.max_geometry_output_components.into(), // GL_MAX_GEOMETRY_OUTPUT_COMPONENTS
            // 0x9125 => self.state.max_fragment_input_components.into(), // GL_MAX_FRAGMENT_INPUT_COMPONENTS
            0x9126 => self.gl_state.characteristics.context_profile_mask.into(), // GL_CONTEXT_PROFILE_MASK
            // 0x8E4F => self.state.provoking_vertex.into(), // GL_PROVOKING_VERTEX
            // 0x9111 => self.state.max_server_wait_timeout.into(), // GL_MAX_SERVER_WAIT_TIMEOUT
            // 0x8E59 => self.state.max_sample_mask_words.into(), // GL_MAX_SAMPLE_MASK_WORDS
            // 0x9104 => self.state.texture_binding_2d_multisample.into(), // GL_TEXTURE_BINDING_2D_MULTISAMPLE
            // 0x9105 => self.state.texture_binding_2d_multisample_array.into(), // GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY
            // 0x910E => self.state.max_color_texture_samples.into(), // GL_MAX_COLOR_TEXTURE_SAMPLES
            // 0x910F => self.state.max_depth_texture_samples.into(), // GL_MAX_DEPTH_TEXTURE_SAMPLES
            // 0x9110 => self.state.max_integer_samples.into(), // GL_MAX_INTEGER_SAMPLES
            // 0x88FC => self.state.max_dual_source_draw_buffers.into(), // GL_MAX_DUAL_SOURCE_DRAW_BUFFERS
            // 0x8919 => self.state.sampler_binding.into(), // GL_SAMPLER_BINDING
            // 0x8E89 => self.state.max_tess_control_uniform_blocks.into(), // GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS
            // 0x8E8A => self.state.max_tess_evaluation_uniform_blocks.into(), // GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS
            // 0x8DFA => self.state.shader_compiler.into(), // GL_SHADER_COMPILER
            // 0x8DF8 => self.state.shader_binary_formats.into(), // GL_SHADER_BINARY_FORMATS
            // 0x8DF9 => self.state.num_shader_binary_formats.into(), // GL_NUM_SHADER_BINARY_FORMATS
            // 0x8DFB => self.state.max_vertex_uniform_vectors.into(), // GL_MAX_VERTEX_UNIFORM_VECTORS
            // 0x8DFC => self.state.max_varying_vectors.into(), // GL_MAX_VARYING_VECTORS
            // 0x8DFD => self.state.max_fragment_uniform_vectors.into(), // GL_MAX_FRAGMENT_UNIFORM_VECTORS
            // 0x87FE => self.state.num_program_binary_formats.into(), // GL_NUM_PROGRAM_BINARY_FORMATS
            // 0x87FF => self.state.program_binary_formats.into(), // GL_PROGRAM_BINARY_FORMATS
            // 0x825A => self.state.program_pipeline_binding.into(), // GL_PROGRAM_PIPELINE_BINDING
            // 0x825B => self.state.max_viewports.into(), // GL_MAX_VIEWPORTS
            // 0x825C => self.state.viewport_subpixel_bits.into(), // GL_VIEWPORT_SUBPIXEL_BITS
            // 0x825D => self.state.viewport_bounds_range.into(), // GL_VIEWPORT_BOUNDS_RANGE
            // 0x825E => self.state.layer_provoking_vertex.into(), // GL_LAYER_PROVOKING_VERTEX
            // 0x825F => self.state.viewport_index_provoking_vertex.into(), // GL_VIEWPORT_INDEX_PROVOKING_VERTEX
            // 0x90BC => self.state.min_map_buffer_alignment.into(), // GL_MIN_MAP_BUFFER_ALIGNMENT
            // 0x92D2 => self.state.max_vertex_atomic_counters.into(), // GL_MAX_VERTEX_ATOMIC_COUNTERS
            // 0x92D3 => self.state.max_tess_control_atomic_counters.into(), // GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS
            // 0x92D4 => self.state.max_tess_evaluation_atomic_counters.into(), // GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS
            // 0x92D5 => self.state.max_geometry_atomic_counters.into(), // GL_MAX_GEOMETRY_ATOMIC_COUNTERS
            // 0x92D6 => self.state.max_fragment_atomic_counters.into(), // GL_MAX_FRAGMENT_ATOMIC_COUNTERS
            // 0x92D7 => self.state.max_combined_atomic_counters.into(), // GL_MAX_COMBINED_ATOMIC_COUNTERS
            // 0x8D6B => self.state.max_element_index.into(), // GL_MAX_ELEMENT_INDEX
            // 0x91BB => self.state.max_compute_uniform_blocks.into(), // GL_MAX_COMPUTE_UNIFORM_BLOCKS
            // 0x91BC => self.state.max_compute_texture_image_units.into(), // GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS
            // 0x8263 => self.state.max_compute_uniform_components.into(), // GL_MAX_COMPUTE_UNIFORM_COMPONENTS
            // 0x8264 => self.state.max_compute_atomic_counter_buffers.into(), // GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS
            // 0x8265 => self.state.max_compute_atomic_counters.into(), // GL_MAX_COMPUTE_ATOMIC_COUNTERS
            // 0x8266 => self.state.max_combined_compute_uniform_components.into(), // GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS
            // 0x90EB => self.state.max_compute_work_group_invocations.into(), // GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS
            // 0x91BE => self.state.max_compute_work_group_count[0].into(), // GL_MAX_COMPUTE_WORK_GROUP_COUNT
            // 0x91BF => self.state.max_compute_work_group_size[0].into(), // GL_MAX_COMPUTE_WORK_GROUP_SIZE
            // 0x90EF => self.state.dispatch_indirect_buffer_binding.into(), // GL_DISPATCH_INDIRECT_BUFFER_BINDING
            // 0x826C => self.state.max_debug_group_stack_depth.into(), // GL_MAX_DEBUG_GROUP_STACK_DEPTH
            // 0x826D => self.state.debug_group_stack_depth.into(), // GL_DEBUG_GROUP_STACK_DEPTH
            // 0x82E8 => self.state.max_label_length.into(), // GL_MAX_LABEL_LENGTH
            // 0x826E => self.state.max_uniform_locations.into(), // GL_MAX_UNIFORM_LOCATIONS
            // 0x9315 => self.state.max_framebuffer_width.into(), // GL_MAX_FRAMEBUFFER_WIDTH
            // 0x9316 => self.state.max_framebuffer_height.into(), // GL_MAX_FRAMEBUFFER_HEIGHT
            // 0x9317 => self.state.max_framebuffer_layers.into(), // GL_MAX_FRAMEBUFFER_LAYERS
            // 0x9318 => self.state.max_framebuffer_samples.into(), // GL_MAX_FRAMEBUFFER_SAMPLES
            // 0x90D3 => self.state.shader_storage_buffer_binding.into(), // GL_SHADER_STORAGE_BUFFER_BINDING
            // 0x90D4 => self.state.shader_storage_buffer_start.into(), // GL_SHADER_STORAGE_BUFFER_START
            // 0x90D5 => self.state.shader_storage_buffer_size.into(), // GL_SHADER_STORAGE_BUFFER_SIZE
            // 0x90D6 => self.state.max_vertex_shader_storage_blocks.into(), // GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS
            // 0x90D7 => self.state.max_geometry_shader_storage_blocks.into(), // GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS
            // 0x90D8 => self.state.max_tess_control_shader_storage_blocks.into(), // GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS
            // 0x90D9 => self.state.max_tess_evaluation_shader_storage_blocks.into(), // GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS
            // 0x90DA => self.state.max_fragment_shader_storage_blocks.into(), // GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS
            // 0x90DB => self.state.max_compute_shader_storage_blocks.into(), // GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS
            // 0x90DC => self.state.max_combined_shader_storage_blocks.into(), // GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS
            // 0x90DD => self.state.max_shader_storage_buffer_bindings.into(), // GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS
            // 0x90DF => self.state.shader_storage_buffer_offset_alignment.into(), // GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT
            // 0x919F => self.state.texture_buffer_offset_alignment.into(), // GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT
            // 0x82D6 => self.state.vertex_binding_divisor.into(), // GL_VERTEX_BINDING_DIVISOR
            // 0x82D7 => self.state.vertex_binding_offset.into(), // GL_VERTEX_BINDING_OFFSET
            // 0x82D8 => self.state.vertex_binding_stride.into(), // GL_VERTEX_BINDING_STRIDE
            // 0x82D9 => self.state.max_vertex_attrib_relative_offset.into(), // GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET
            // 0x82DA => self.state.max_vertex_attrib_bindings.into(), // GL_MAX_VERTEX_ATTRIB_BINDINGS
            u => {
                panic!("unrecognized enum {u:x}")
            }
        };

        println!("GLget asked for parameter {parameter_name:x}, got:");
        dbg!(item)
    }

    pub unsafe fn oxidegl_get_booleanv(&mut self, pname: GLenum, mut data: *mut GLboolean) {
        for item in self.get(pname, None).arr() {
            // Safety: This is user-accessible UB. We have ZERO information about what is behind this pointer
            // Trusting the user like this kinda sucks but its just How The API Works™
            unsafe {
                *data = item.into_bool();
                data = data.add(1);
            }
        }
    }

    pub unsafe fn oxidegl_get_doublev(&mut self, pname: GLenum, mut data: *mut GLdouble) {
        for item in self.get(pname, None).arr() {
            // SAFETY: see above
            unsafe {
                *data = item.into_double();
                data = data.add(1);
            }
        }
    }

    pub unsafe fn oxidegl_get_floatv(&mut self, pname: GLenum, mut data: *mut GLfloat) {
        for item in self.get(pname, None).arr() {
            // SAFETY: see above
            unsafe {
                *data = item.into_float();
                data = data.add(1);
            }
        }
    }

    pub unsafe fn oxidegl_get_integerv(&mut self, pname: GLenum, mut data: *mut GLint) {
        for item in self.get(pname, None).arr() {
            // SAFETY: see above
            unsafe {
                *data = item.into_int();
                data = data.add(1);
            }
        }
    }

    pub unsafe fn oxidegl_get_booleani_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLboolean,
    ) {
        panic!("command oxidegl_get_booleani_v not yet implemented");
    }

    pub unsafe fn oxidegl_get_integeri_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLint,
    ) {
        panic!("command oxidegl_get_integeri_v not yet implemented");
    }

    pub unsafe fn oxidegl_get_integer64v(&mut self, pname: GLenum, data: *mut GLint64) {
        panic!("command oxidegl_get_integer64v not yet implemented");
    }
    pub unsafe fn oxidegl_get_integer64i_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLint64,
    ) {
        panic!("command oxidegl_get_integer64i_v not yet implemented");
    }

    pub unsafe fn oxidegl_get_floati_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLfloat,
    ) {
        panic!("command oxidegl_get_floati_v not yet implemented");
    }

    pub unsafe fn oxidegl_get_doublei_v(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLdouble,
    ) {
        panic!("command oxidegl_get_doublei_v not yet implemented");
    }
    fn get_string(name: GLenum) -> *const GLubyte {
        const VENDOR: &[u8] = b"Charles Liske\0";
        const RENDERER: &[u8] = b"OxideGL\0";
        const VERSION: &[u8] = b"4.6.0\0";
        match name {
            GL_VENDOR => VENDOR.as_ptr(),
            GL_RENDERER => RENDERER.as_ptr(),
            GL_VERSION | GL_SHADING_LANGUAGE_VERSION => VERSION.as_ptr(),
            _ => {
                panic!("unrecognized enum in gl get string")
            }
        }
    }
}

/// ### Parameters
/// `name`
///
/// > Specifies a symbolic constant, one of [`GL_VENDOR`](crate::enums::GL_VENDOR),
/// > [`GL_RENDERER`](crate::enums::GL_RENDERER), [`GL_VERSION`](crate::enums::GL_VERSION),
/// > or [`GL_SHADING_LANGUAGE_VERSION`](crate::enums::GL_SHADING_LANGUAGE_VERSION).
/// > Additionally, [**glGetStringi**](crate::context::OxideGLContext::oxidegl_get_stringi)
/// > accepts the [`GL_EXTENSIONS`](crate::enums::GL_EXTENSIONS) token.
///
/// `index`
///
/// > For [**glGetStringi**](crate::context::OxideGLContext::oxidegl_get_stringi),
/// > specifies the index of the string to return.
///
/// ### Description
/// [**glGetString**](crate::context::OxideGLContext::oxidegl_get_string) returns
/// a pointer to a static string describing some aspect of the current GL connection.
/// `name` can be one of the following:
///
/// [`GL_VENDOR`](crate::enums::GL_VENDOR)
///
/// > Returns the company responsible for this GL implementation. This name does
/// > not change from release to release.
///
/// [`GL_RENDERER`](crate::enums::GL_RENDERER)
///
/// > Returns the name of the renderer. This name is typically specific to a
/// > particular configuration of a hardware platform. It does not change from
/// > release to release.
///
/// [`GL_VERSION`](crate::enums::GL_VERSION)
///
/// > Returns a version or release number.
///
/// [`GL_SHADING_LANGUAGE_VERSION`](crate::enums::GL_SHADING_LANGUAGE_VERSION)
///
/// > Returns a version or release number for the shading language.
///
/// [**glGetStringi**](crate::context::OxideGLContext::oxidegl_get_stringi)
/// returns a pointer to a static string indexed by `index`. `name` can be
/// one of the following:
///
/// [`GL_EXTENSIONS`](crate::enums::GL_EXTENSIONS)
///
/// > For [**glGetStringi**](crate::context::OxideGLContext::oxidegl_get_stringi)
/// > only, returns the extension string supported by the implementation at `index`.
///
/// Strings [`GL_VENDOR`](crate::enums::GL_VENDOR) and [`GL_RENDERER`](crate::enums::GL_RENDERER)
/// together uniquely specify a platform. They do not change from release to
/// release and should be used by platform-recognition algorithms.
///
/// The [`GL_VERSION`](crate::enums::GL_VERSION) and [`GL_SHADING_LANGUAGE_VERSION`](crate::enums::GL_SHADING_LANGUAGE_VERSION)
/// strings begin with a version number. The version number uses one of these
/// forms:
///
/// *`major_number.minor_number`* *`major_number.minor_number.release_number`*
///
/// Vendor-specific information may follow the version number. Its format depends
/// on the implementation, but a space always separates the version number
/// and the vendor-specific information.
///
/// All strings are null-terminated.
///
/// ### Notes
/// If an error is generated, [**glGetString**](crate::context::OxideGLContext::oxidegl_get_string)
/// returns 0.
///
/// The client and server may support different versions. [**glGetString**](crate::context::OxideGLContext::oxidegl_get_string)
/// always returns a compatible version number. The release number always describes
/// the server.
pub mod get_string {
    use crate::context::Context;
    use crate::dispatch::gltypes::{GLenum, GLubyte, GLuint};
    impl Context {
        pub(crate) fn oxidegl_get_string(&mut self, name: GLenum) -> *const GLubyte {
            Self::get_string(name)
        }
        pub(crate) fn oxidegl_get_stringi(
            &mut self,
            name: GLenum,
            index: GLuint,
        ) -> *const GLubyte {
            //TODO: this is probably wrong
            Self::get_string(name)
        }
    }
}

/// ### Parameters
/// `target`
///
/// > Indicates the usage of the internal format. `target` must be [`GL_TEXTURE_1D`](crate::enums::GL_TEXTURE_1D),
/// > [`GL_TEXTURE_1D_ARRAY`](crate::enums::GL_TEXTURE_1D_ARRAY), [`GL_TEXTURE_2D`](crate::enums::GL_TEXTURE_2D),
/// > [`GL_TEXTURE_2D_ARRAY`](crate::enums::GL_TEXTURE_2D_ARRAY), [`GL_TEXTURE_3D`](crate::enums::GL_TEXTURE_3D),
/// > [`GL_TEXTURE_CUBE_MAP`](crate::enums::GL_TEXTURE_CUBE_MAP), [`GL_TEXTURE_CUBE_MAP_ARRAY`](crate::enums::GL_TEXTURE_CUBE_MAP_ARRAY),
/// > [`GL_TEXTURE_RECTANGLE`](crate::enums::GL_TEXTURE_RECTANGLE), [`GL_TEXTURE_BUFFER`](crate::enums::GL_TEXTURE_BUFFER),
/// > [`GL_RENDERBUFFER`](crate::enums::GL_RENDERBUFFER), [`GL_TEXTURE_2D_MULTISAMPLE`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE)
/// > or [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY).
///
/// `internalformat`
///
/// > Specifies the internal format about which to retrieve information.
///
/// `pname`
///
/// > Specifies the type of information to query.
///
/// `bufSize`
///
/// > Specifies the maximum number of integers of the specified width that may
/// > be written to `params` by the function.
///
/// `params`
///
/// > Specifies the address of a variable into which to write the retrieved information.
///
/// ### Description
/// [**glGetInternalformativ**](crate::context::OxideGLContext::oxidegl_get_internalformativ)
/// and [**glGetInternalformati64v**](crate::context::OxideGLContext::oxidegl_get_internalformati64v)
/// retrieve information about implementation-dependent support for internal
/// formats. `target` indicates the target with which the internal format will
/// be used and must be one of [`GL_RENDERBUFFER`](crate::enums::GL_RENDERBUFFER),
/// [`GL_TEXTURE_2D_MULTISAMPLE`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE),
/// or [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY),
/// corresponding to usage as a renderbuffer, two-dimensional multisample
/// texture or two-dimensional multisample array texture, respectively.
///
/// `internalformat` specifies the internal format about which to retrieve
/// information and must be a color-renderable, depth-renderable or stencil-renderable
/// format.
///
/// The information retrieved will be written to memory addressed by the pointer
/// specified in `params`. No more than `bufSize` integers will be written
/// to this memory.
///
/// If `pname` is [`GL_NUM_SAMPLE_COUNTS`](crate::enums::GL_NUM_SAMPLE_COUNTS),
/// the number of sample counts that would be returned by querying [`GL_SAMPLES`](crate::enums::GL_SAMPLES)
/// will be returned in `params`.
///
/// If `pname` is [`GL_SAMPLES`](crate::enums::GL_SAMPLES), the sample counts
/// supported for `internalformat` and `target` are written into `params` in
/// descending numeric order. Only positive values are returned. Querying [`GL_SAMPLES`](crate::enums::GL_SAMPLES)
/// with `bufSize` of one will return just the maximum supported number of
/// samples for this format. The maximum value in [`GL_SAMPLES`](crate::enums::GL_SAMPLES)
/// is guaranteed to be at least the lowest of the following: The value of
/// > [`GL_MAX_INTEGER_SAMPLES`](crate::enums::GL_MAX_INTEGER_SAMPLES) if `internalformat`
/// > is a signed or unsigned integer format.
///
/// > The value of [`GL_MAX_DEPTH_TEXTURE_SAMPLES`](crate::enums::GL_MAX_DEPTH_TEXTURE_SAMPLES)
/// > if `internalformat` is a depth- or stencil-renderable format and `target`
/// > is [`GL_TEXTURE_2D_MULTISAMPLE`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE),
/// > [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY).
///
/// > The value of [`GL_MAX_COLOR_TEXTURE_SAMPLES`](crate::enums::GL_MAX_COLOR_TEXTURE_SAMPLES)
/// > if `internalformat` is a color-renderable format and `target` is [`GL_TEXTURE_2D_MULTISAMPLE`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE)
/// > or [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`](crate::enums::GL_TEXTURE_2D_MULTISAMPLE_ARRAY).
///
/// > The value of [`GL_MAX_SAMPLES`](crate::enums::GL_MAX_SAMPLES).
///
///
/// If `pname` is [`GL_INTERNALFORMAT_SUPPORTED`](crate::enums::GL_INTERNALFORMAT_SUPPORTED),
/// `params` is set to [`GL_TRUE`](crate::enums::GL_TRUE) if `internalformat`
/// is a supported internal format for `target` and to [`GL_FALSE`](crate::enums::GL_FALSE)
/// otherwise.
///
/// If `pname` is [`GL_INTERNALFORMAT_PREFERRED`](crate::enums::GL_INTERNALFORMAT_PREFERRED),
/// `params` is set to [`GL_TRUE`](crate::enums::GL_TRUE) if `internalformat`
/// is an format for `target` that is preferred by the implementation and to
/// [`GL_FALSE`](crate::enums::GL_FALSE) otherwise.
///
/// If `pname` is [`GL_INTERNALFORMAT_RED_SIZE`](crate::enums::GL_INTERNALFORMAT_RED_SIZE),
/// [`GL_INTERNALFORMAT_GREEN_SIZE`](crate::enums::GL_INTERNALFORMAT_GREEN_SIZE),
/// [`GL_INTERNALFORMAT_BLUE_SIZE`](crate::enums::GL_INTERNALFORMAT_BLUE_SIZE),
/// [`GL_INTERNALFORMAT_ALPHA_SIZE`](crate::enums::GL_INTERNALFORMAT_ALPHA_SIZE),
/// [`GL_INTERNALFORMAT_DEPTH_SIZE`](crate::enums::GL_INTERNALFORMAT_DEPTH_SIZE),
/// [`GL_INTERNALFORMAT_STENCIL_SIZE`](crate::enums::GL_INTERNALFORMAT_STENCIL_SIZE),
/// or [`GL_INTERNALFORMAT_SHARED_SIZE`](crate::enums::GL_INTERNALFORMAT_SHARED_SIZE)
/// then `params` is set to the actual resolutions that would be used for storing
/// image array components for the resource for the red, green, blue, alpha,
/// depth, stencil and shared channels respectively. If `internalformat` is
/// a compressed internal format, then `params` is set to the component resolution
/// of an uncompressed internal format that produces an image of roughly the
/// same quality as the compressed algorithm. If the internal format is unsupported,
/// or if a particular component is not present in the format, 0 is written
/// to `params`.
///
/// If `pname` is [`GL_INTERNALFORMAT_RED_TYPE`](crate::enums::GL_INTERNALFORMAT_RED_TYPE),
/// [`GL_INTERNALFORMAT_GREEN_TYPE`](crate::enums::GL_INTERNALFORMAT_GREEN_TYPE),
/// [`GL_INTERNALFORMAT_BLUE_TYPE`](crate::enums::GL_INTERNALFORMAT_BLUE_TYPE),
/// [`GL_INTERNALFORMAT_ALPHA_TYPE`](crate::enums::GL_INTERNALFORMAT_ALPHA_TYPE),
/// [`GL_INTERNALFORMAT_DEPTH_TYPE`](crate::enums::GL_INTERNALFORMAT_DEPTH_TYPE),
/// or [`GL_INTERNALFORMAT_STENCIL_TYPE`](crate::enums::GL_INTERNALFORMAT_STENCIL_TYPE)
/// then `params` is set to a token identifying the data type used to store
/// the respective component. If the `internalformat` represents a compressed
/// internal format then the types returned specify how components are interpreted
/// after decompression.
///
/// If `pname` is [`GL_MAX_WIDTH`](crate::enums::GL_MAX_WIDTH), [`GL_MAX_HEIGHT`](crate::enums::GL_MAX_HEIGHT),
/// [`GL_MAX_DEPTH`](crate::enums::GL_MAX_DEPTH), or [`GL_MAX_LAYERS`](crate::enums::GL_MAX_LAYERS)
/// then `pname` is filled with the maximum width, height, depth or layer count
/// for textures with internal format `internalformat`, respectively. If `pname`
/// is [`GL_MAX_COMBINED_DIMENSIONS`](crate::enums::GL_MAX_COMBINED_DIMENSIONS)
/// then `pname` is filled with the maximum combined dimensions of a texture
/// of the specified internal format.
///
/// If `pname` is [`GL_COLOR_COMPONENTS`](crate::enums::GL_COLOR_COMPONENTS)
/// then `params` is set to the value [`GL_TRUE`](crate::enums::GL_TRUE) if
/// the internal format contains any color component (i.e., red, green, blue
/// or alpha) and to [`GL_FALSE`](crate::enums::GL_FALSE) otherwise. If `pname`
/// is [`GL_DEPTH_COMPONENTS`](crate::enums::GL_DEPTH_COMPONENTS) or [`GL_STENCIL_COMPONENTS`](crate::enums::GL_STENCIL_COMPONENTS)
/// then `params` is set to [`GL_TRUE`](crate::enums::GL_TRUE) if the internal
/// format contains a depth or stencil component, respectively, and to [`GL_FALSE`](crate::enums::GL_FALSE)
/// otherwise.
///
/// If `pname` is [`GL_COLOR_RENDERABLE`](crate::enums::GL_COLOR_RENDERABLE),
/// [`GL_DEPTH_RENDERABLE`](crate::enums::GL_DEPTH_RENDERABLE) or [`GL_STENCIL_RENDERABLE`](crate::enums::GL_STENCIL_RENDERABLE)
/// then `params` is set to [`GL_TRUE`](crate::enums::GL_TRUE) if the specified
/// internal format is color, depth or stencil renderable, respectively, and
/// to [`GL_FALSE`](crate::enums::GL_FALSE) otherwise.
///
/// If `pname` is [`GL_FRAMEBUFFER_RENDERABLE`](crate::enums::GL_FRAMEBUFFER_RENDERABLE)
/// or [`GL_FRAMEBUFFER_RENDERABLE_LAYERED`](crate::enums::GL_FRAMEBUFFER_RENDERABLE_LAYERED)
/// then `params` is set to one of [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT),
/// [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT) or [`GL_NONE`](crate::enums::GL_NONE)
/// to indicate that framebuffer attachments (layered attachments in the case
/// of [`GL_FRAMEBUFFER_RENDERABLE_LAYERED`](crate::enums::GL_FRAMEBUFFER_RENDERABLE_LAYERED))
/// with that internal format are either renderable with no restrictions,
/// renderable with some restrictions or not renderable at all.
///
/// If `pname` is [`GL_FRAMEBUFFER_BLEND`](crate::enums::GL_FRAMEBUFFER_BLEND),
/// `params` is set to [`GL_TRUE`](crate::enums::GL_TRUE) to indicate that
/// the internal format is supported for blending operations when attached
/// to a framebuffer, and to [`GL_FALSE`](crate::enums::GL_FALSE) otherwise.
///
/// If `pname` is [`GL_READ_PIXELS`](crate::enums::GL_READ_PIXELS) then `params`
/// is set to [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT), [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT)
/// or [`GL_NONE`](crate::enums::GL_NONE) to that either full support, limited
/// support or no support at all is supplied for reading pixels from framebuffer
/// attachments in the specified internal format.
///
/// If `pname` is [`GL_READ_PIXELS_FORMAT`](crate::enums::GL_READ_PIXELS_FORMAT)
/// or [`GL_READ_PIXELS_TYPE`](crate::enums::GL_READ_PIXELS_TYPE) then `params`
/// is filled with the format or type, respectively, most recommended to obtain
/// the highest image quality and performance. For [`GL_READ_PIXELS_FORMAT`](crate::enums::GL_READ_PIXELS_FORMAT),
/// the value returned in `params` is a token that is accepted for the `format`
/// argument to [**glReadPixels**](crate::context::OxideGLContext::oxidegl_read_pixels).
/// For [`GL_READ_PIXELS_TYPE`](crate::enums::GL_READ_PIXELS_TYPE), the value
/// returned in `params` is a token that is accepted for the `type` argument
/// to [**glReadPixels**](crate::context::OxideGLContext::oxidegl_read_pixels).
///
/// If `pname` is [`GL_TEXTURE_IMAGE_FORMAT`](crate::enums::GL_TEXTURE_IMAGE_FORMAT)
/// or [`GL_TEXTURE_IMAGE_TYPE`](crate::enums::GL_TEXTURE_IMAGE_TYPE) then
/// `params` is filled with the implementation-recommended format or type to
/// be used in calls to [**glTexImage2D**](crate::context::OxideGLContext::oxidegl_tex_image2_d)
/// and other similar functions. For [`GL_TEXTURE_IMAGE_FORMAT`](crate::enums::GL_TEXTURE_IMAGE_FORMAT),
/// `params` is filled with a token suitable for use as the `format` argument
/// to [**glTexImage2D**](crate::context::OxideGLContext::oxidegl_tex_image2_d).
/// For [`GL_TEXTURE_IMAGE_TYPE`](crate::enums::GL_TEXTURE_IMAGE_TYPE), `params`
/// is filled with a token suitable for use as the `type` argument to [**glTexImage2D**](crate::context::OxideGLContext::oxidegl_tex_image2_d).
///
/// If `pname` is [`GL_GET_TEXTURE_IMAGE_FORMAT`](crate::enums::GL_GET_TEXTURE_IMAGE_FORMAT)
/// or [`GL_GET_TEXTURE_IMAGE_TYPE`](crate::enums::GL_GET_TEXTURE_IMAGE_TYPE)
/// then `params` is filled with the implementation-recommended format or type
/// to be used in calls to [**glGetTexImage**](crate::context::OxideGLContext::oxidegl_get_tex_image)
/// and other similar functions. For [`GL_GET_TEXTURE_IMAGE_FORMAT`](crate::enums::GL_GET_TEXTURE_IMAGE_FORMAT),
/// `params` is filled with a token suitable for use as the `format` argument
/// to [**glGetTexImage**](crate::context::OxideGLContext::oxidegl_get_tex_image).
/// For [`GL_GET_TEXTURE_IMAGE_TYPE`](crate::enums::GL_GET_TEXTURE_IMAGE_TYPE),
/// `params` is filled with a token suitable for use as the `type` argument
/// to [**glGetTexImage**](crate::context::OxideGLContext::oxidegl_get_tex_image).
///
/// If `pname` is [`GL_MIPMAP`](crate::enums::GL_MIPMAP) then `pname` is set
/// to [`GL_TRUE`](crate::enums::GL_TRUE) to indicate that the specified internal
/// format supports mipmaps and to [`GL_FALSE`](crate::enums::GL_FALSE) otherwise.
///
/// If `pname` is [`GL_GENERATE_MIPMAP`](crate::enums::GL_GENERATE_MIPMAP)
/// or [`GL_AUTO_GENERATE_MIPMAP`](crate::enums::GL_AUTO_GENERATE_MIPMAP) then
/// `params` is indicates the level of support for manual or automatic mipmap
/// generation for the specified internal format, respectively. Returned values
/// may be one of [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT), [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT)
/// and [`GL_NONE`](crate::enums::GL_NONE) to indicate either full support,
/// limited support or no support at all.
///
/// If `pname` is [`GL_COLOR_ENCODING`](crate::enums::GL_COLOR_ENCODING) then
/// the color encoding for the resource is returned in `params`. Possible values
/// for color buffers are [`GL_LINEAR`](crate::enums::GL_LINEAR) or [`GL_SRGB`](crate::enums::GL_SRGB),
/// for linear or sRGB-encoded color components, respectively. For non-color
/// formats (such as depth or stencil), or for unsupported resources, the value
/// [`GL_NONE`](crate::enums::GL_NONE) is returned.
///
/// If `pname` is [`GL_SRGB_READ`](crate::enums::GL_SRGB_READ), or [`GL_SRGB_WRITE`](crate::enums::GL_SRGB_WRITE)
/// then `params` indicates the level of support for reading and writing to
/// sRGB encoded images, respectively. For [`GL_SRGB_READ`](crate::enums::GL_SRGB_READ),
/// support for converting from sRGB colorspace on read operations is returned
/// in `params` and for [`GL_SRGB_WRITE`](crate::enums::GL_SRGB_WRITE), support
/// for converting to sRGB colorspace on write operations to the resource is
/// returned in `params`. `params` may be set to [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT),
/// [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT), or [`GL_NONE`](crate::enums::GL_NONE)
/// to indicate full support, limited support or no support at all, respecitively.
///
/// If `pname` is [`GL_FILTER`](crate::enums::GL_FILTER) the `params` is set
/// to either [`GL_TRUE`](crate::enums::GL_TRUE) or [`GL_FALSE`](crate::enums::GL_FALSE)
/// to indicate support or lack thereof for filter modes other than [`GL_NEAREST`](crate::enums::GL_NEAREST)
/// or [`GL_NEAREST_MIPMAP`](crate::enums::GL_NEAREST_MIPMAP) for the specified
/// internal format.
///
/// If `pname` is [`GL_VERTEX_TEXTURE`](crate::enums::GL_VERTEX_TEXTURE), [`GL_TESS_CONTROL_TEXTURE`](crate::enums::GL_TESS_CONTROL_TEXTURE),
/// [`GL_TESS_EVALUATION_TEXTURE`](crate::enums::GL_TESS_EVALUATION_TEXTURE),
/// [`GL_GEOMETRY_TEXTURE`](crate::enums::GL_GEOMETRY_TEXTURE), [`GL_FRAGMENT_TEXTURE`](crate::enums::GL_FRAGMENT_TEXTURE),
/// or [`GL_COMPUTE_TEXTURE`](crate::enums::GL_COMPUTE_TEXTURE), then the
/// value written to `params` indicates support for use of the resource as
/// a source of texturing in the vertex, tessellation control, tessellation
/// evaluation, geometry, fragment and compute shader stages, respectively.
/// `params` may be set to [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT),
/// [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT) or [`GL_NONE`](crate::enums::GL_NONE)
/// to indicate full support, limited support or no support at all, respectively.
///
/// If `pname` is [`GL_TEXTURE_SHADOW`](crate::enums::GL_TEXTURE_SHADOW), [`GL_TEXTURE_GATHER`](crate::enums::GL_TEXTURE_GATHER)
/// or [`GL_TEXTURE_GATHER_SHADOW`](crate::enums::GL_TEXTURE_GATHER_SHADOW)
/// then the value written to `params` indicates the level of support for using
/// the resource with a shadow sampler, in gather operations or as a shadow
/// sampler in gather operations, respectively. Returned values may be [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT),
/// [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT) or [`GL_NONE`](crate::enums::GL_NONE)
/// to indicate full support, limited support or no support at all, respectively.
///
/// If `pname` is [`GL_SHADER_IMAGE_LOAD`](crate::enums::GL_SHADER_IMAGE_LOAD),
/// [`GL_SHADER_IMAGE_STORE`](crate::enums::GL_SHADER_IMAGE_STORE) or [`GL_SHADER_IMAGE_ATOMIC`](crate::enums::GL_SHADER_IMAGE_ATOMIC)
/// then the value returned in `params` indicates the level of support for
/// image loads, stores and atomics for resources of the specified internal
/// format. Returned values may be [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT),
/// [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT) or [`GL_NONE`](crate::enums::GL_NONE)
/// to indicate full support, limited support or no support at all, respectively.
///
/// If `pname` is [`GL_IMAGE_TEXEL_SIZE`](crate::enums::GL_IMAGE_TEXEL_SIZE)
/// then the size of a texel when the resource when used as an image texture
/// is returned in `params`. If the resource is not supported for image textures
/// zero is returned.
///
/// If `pname` is [`GL_IMAGE_COMPATIBILITY_CLASS`](crate::enums::GL_IMAGE_COMPATIBILITY_CLASS)
/// then the compatibility class of the resource when used as an image texture
/// is returned in `params`. The possible values returned are [`GL_IMAGE_CLASS_4_X_32`](crate::enums::GL_IMAGE_CLASS_4_X_32),
/// [`GL_IMAGE_CLASS_2_X_32`](crate::enums::GL_IMAGE_CLASS_2_X_32), [`GL_IMAGE_CLASS_1_X_32`](crate::enums::GL_IMAGE_CLASS_1_X_32),
/// [`GL_IMAGE_CLASS_4_X_16`](crate::enums::GL_IMAGE_CLASS_4_X_16), [`GL_IMAGE_CLASS_2_X_16`](crate::enums::GL_IMAGE_CLASS_2_X_16),
/// [`GL_IMAGE_CLASS_1_X_16`](crate::enums::GL_IMAGE_CLASS_1_X_16), [`GL_IMAGE_CLASS_4_X_8`](crate::enums::GL_IMAGE_CLASS_4_X_8),
/// [`GL_IMAGE_CLASS_2_X_8`](crate::enums::GL_IMAGE_CLASS_2_X_8), [`GL_IMAGE_CLASS_1_X_8`](crate::enums::GL_IMAGE_CLASS_1_X_8),
/// [`GL_IMAGE_CLASS_11_11_10`](crate::enums::GL_IMAGE_CLASS_11_11_10), and
/// [`GL_IMAGE_CLASS_10_10_10_2`](crate::enums::GL_IMAGE_CLASS_10_10_10_2),
/// which correspond to the 4x32, 2x32, 1x32, 4x16, 2x16, 1x16, 4x8, 2x8,
/// 1x8, the class (a) 11/11/10 packed floating-point format, and the class
/// (b) 10/10/10/2 packed formats, respectively. If the resource is not supported
/// for image textures, [`GL_NONE`](crate::enums::GL_NONE) is returned.
///
/// If `pname` is [`GL_IMAGE_PIXEL_FORMAT`](crate::enums::GL_IMAGE_PIXEL_FORMAT)
/// or [`GL_IMAGE_PIXEL_TYPE`](crate::enums::GL_IMAGE_PIXEL_TYPE) then the
/// pixel format or type of the resource when used as an image texture is returned
/// in `params`, respectively. In either case, the resource is not supported
/// for image textures [`GL_NONE`](crate::enums::GL_NONE) is returned.
///
/// If `pname` is [`GL_IMAGE_FORMAT_COMPATIBILITY_TYPE`](crate::enums::GL_IMAGE_FORMAT_COMPATIBILITY_TYPE),
/// the matching criteria use for the resource when used as an image textures
/// is returned in `params`. Possible values returned in `params` are [`GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE`](crate::enums::GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE)
/// or [`GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS`](crate::enums::GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS).
/// If the resource is not supported for image textures, [`GL_NONE`](crate::enums::GL_NONE)
/// is returned.
///
/// If `pname` is [`GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST`](crate::enums::GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST)
/// or [`GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST`](crate::enums::GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST),
/// support for using the resource both as a source for texture sampling while
/// it is bound as a buffer for depth or stencil test, respectively, is written
/// to `params`. Possible values returned are [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT),
/// [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT), or [`GL_NONE`](crate::enums::GL_NONE)
/// to indicate full support, limited support or no support at all. If the
/// resource or operation is not supported, [`GL_NONE`](crate::enums::GL_NONE)
/// is returned.
///
/// If `pname` is [`GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE`](crate::enums::GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE)
/// or [`GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE`](crate::enums::GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE),
/// support for using the resource both as a source for texture sampling while
/// performing depth or stencil writes to the resources, respectively, is written
/// to `params`. Possible values returned are [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT),
/// [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT), or [`GL_NONE`](crate::enums::GL_NONE)
/// to indicate full support, limited support or no support at all. If the
/// resource or operation is not supported, [`GL_NONE`](crate::enums::GL_NONE)
/// is returned.
///
/// If `pname` is [`GL_TEXTURE_COMPRESSED`](crate::enums::GL_TEXTURE_COMPRESSED)
/// then [`GL_TRUE`](crate::enums::GL_TRUE) is returned in `params` if `internalformat`
/// is a compressed internal format. [`GL_FALSE`](crate::enums::GL_FALSE) is
/// returned in `params` otherwise.
///
/// If `pname` is [`GL_TEXTURE_COMPRESSED_BLOCK_WIDTH`](crate::enums::GL_TEXTURE_COMPRESSED_BLOCK_WIDTH),
/// [`GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT`](crate::enums::GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT)
/// or [`GL_TEXTURE_COMPRESSED_BLOCK_SIZE`](crate::enums::GL_TEXTURE_COMPRESSED_BLOCK_SIZE)
/// then the width, height or total size, respectively of a block (in basic
/// machine units) is returned in `params`. If the internal format is not compressed,
/// or the resource is not supported, 0 is returned.
///
/// If `pname` is [`GL_CLEAR_BUFFER`](crate::enums::GL_CLEAR_BUFFER), the level
/// of support for using the resource with [**glClearBufferData**](crate::context::OxideGLContext::oxidegl_clear_buffer_data)
/// and [**glClearBufferSubData**](crate::context::OxideGLContext::oxidegl_clear_buffer_sub_data)
/// is returned in `params`. Possible values returned are [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT),
/// [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT), or [`GL_NONE`](crate::enums::GL_NONE)
/// to indicate full support, limited support or no support at all, respectively.
/// If the resource or operation is not supported, [`GL_NONE`](crate::enums::GL_NONE)
/// is returned.
///
/// If `pname` is [`GL_TEXTURE_VIEW`](crate::enums::GL_TEXTURE_VIEW), the level
/// of support for using the resource with the [**glTextureView**](crate::context::OxideGLContext::oxidegl_texture_view)
/// command is returned in `params`. Possible values returned are [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT),
/// [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT), or [`GL_NONE`](crate::enums::GL_NONE)
/// to indicate full support, limited support or no support at all, respectively.
/// If the resource or operation is not supported, [`GL_NONE`](crate::enums::GL_NONE)
/// is returned.
///
/// If `pname` is [`GL_VIEW_COMPATIBILITY_CLASS`](crate::enums::GL_VIEW_COMPATIBILITY_CLASS)
/// then the compatibility class of the resource when used as a texture view
/// is returned in `params`. The possible values returned are [`GL_VIEW_CLASS_128_BITS`](crate::enums::GL_VIEW_CLASS_128_BITS),
/// [`GL_VIEW_CLASS_96_BITS`](crate::enums::GL_VIEW_CLASS_96_BITS), [`GL_VIEW_CLASS_64_BITS`](crate::enums::GL_VIEW_CLASS_64_BITS),
/// [`GL_VIEW_CLASS_48_BITS`](crate::enums::GL_VIEW_CLASS_48_BITS), [`GL_VIEW_CLASS_32_BITS`](crate::enums::GL_VIEW_CLASS_32_BITS),
/// [`GL_VIEW_CLASS_24_BITS`](crate::enums::GL_VIEW_CLASS_24_BITS), [`GL_VIEW_CLASS_16_BITS`](crate::enums::GL_VIEW_CLASS_16_BITS),
/// [`GL_VIEW_CLASS_8_BITS`](crate::enums::GL_VIEW_CLASS_8_BITS), [`GL_VIEW_CLASS_S3TC_DXT1_RGB`](crate::enums::GL_VIEW_CLASS_S3TC_DXT1_RGB),
/// [`GL_VIEW_CLASS_S3TC_DXT1_RGBA`](crate::enums::GL_VIEW_CLASS_S3TC_DXT1_RGBA),
/// [`GL_VIEW_CLASS_S3TC_DXT3_RGBA`](crate::enums::GL_VIEW_CLASS_S3TC_DXT3_RGBA),
/// [`GL_VIEW_CLASS_S3TC_DXT5_RGBA`](crate::enums::GL_VIEW_CLASS_S3TC_DXT5_RGBA),
/// [`GL_VIEW_CLASS_RGTC1_RED`](crate::enums::GL_VIEW_CLASS_RGTC1_RED), [`GL_VIEW_CLASS_RGTC2_RG`](crate::enums::GL_VIEW_CLASS_RGTC2_RG),
/// [`GL_VIEW_CLASS_BPTC_UNORM`](crate::enums::GL_VIEW_CLASS_BPTC_UNORM), and
/// [`GL_VIEW_CLASS_BPTC_FLOAT`](crate::enums::GL_VIEW_CLASS_BPTC_FLOAT).
///
/// If `pname` is [`GL_CLEAR_TEXTURE`](crate::enums::GL_CLEAR_TEXTURE) then
/// the presence of support for using the [**glClearTexImage**](crate::context::OxideGLContext::oxidegl_clear_tex_image)
/// and [**glClearTexSubImage**](crate::context::OxideGLContext::oxidegl_clear_tex_sub_image)
/// commands with the resource is written to `params`. Possible values written
/// are [`GL_FULL_SUPPORT`](crate::enums::GL_FULL_SUPPORT), [`GL_CAVEAT_SUPPORT`](crate::enums::GL_CAVEAT_SUPPORT),
/// or [`GL_NONE`](crate::enums::GL_NONE) to indicate full support, limited
/// support or no support at all, respectively. If the resource or operation
/// is not supported, [`GL_NONE`](crate::enums::GL_NONE) is returned.
///
/// ### Notes
/// [**glGetInternalformativ**](crate::context::OxideGLContext::oxidegl_get_internalformativ)
/// is available only if the GL version is 4.2 or higher.
///
/// The tokens [`GL_INTERNALFORMAT_SUPPORTED`](crate::enums::GL_INTERNALFORMAT_SUPPORTED),
/// [`GL_INTERNALFORMAT_PREFERRED`](crate::enums::GL_INTERNALFORMAT_PREFERRED),
/// [`GL_INTERNALFORMAT_RED_SIZE`](crate::enums::GL_INTERNALFORMAT_RED_SIZE),
/// [`GL_INTERNALFORMAT_GREEN_SIZE`](crate::enums::GL_INTERNALFORMAT_GREEN_SIZE),
/// [`GL_INTERNALFORMAT_BLUE_SIZE`](crate::enums::GL_INTERNALFORMAT_BLUE_SIZE),
/// [`GL_INTERNALFORMAT_ALPHA_SIZE`](crate::enums::GL_INTERNALFORMAT_ALPHA_SIZE),
/// [`GL_INTERNALFORMAT_DEPTH_SIZE`](crate::enums::GL_INTERNALFORMAT_DEPTH_SIZE),
/// [`GL_INTERNALFORMAT_STENCIL_SIZE`](crate::enums::GL_INTERNALFORMAT_STENCIL_SIZE),
/// [`GL_INTERNALFORMAT_SHARED_SIZE`](crate::enums::GL_INTERNALFORMAT_SHARED_SIZE),
/// [`GL_INTERNALFORMAT_RED_TYPE`](crate::enums::GL_INTERNALFORMAT_RED_TYPE),
/// [`GL_INTERNALFORMAT_GREEN_TYPE`](crate::enums::GL_INTERNALFORMAT_GREEN_TYPE),
/// [`GL_INTERNALFORMAT_BLUE_TYPE`](crate::enums::GL_INTERNALFORMAT_BLUE_TYPE),
/// [`GL_INTERNALFORMAT_ALPHA_TYPE`](crate::enums::GL_INTERNALFORMAT_ALPHA_TYPE),
/// [`GL_INTERNALFORMAT_DEPTH_TYPE`](crate::enums::GL_INTERNALFORMAT_DEPTH_TYPE),
/// [`GL_INTERNALFORMAT_STENCIL_TYPE`](crate::enums::GL_INTERNALFORMAT_STENCIL_TYPE),
/// [`GL_MAX_WIDTH`](crate::enums::GL_MAX_WIDTH), [`GL_MAX_HEIGHT`](crate::enums::GL_MAX_HEIGHT),
/// [`GL_MAX_DEPTH`](crate::enums::GL_MAX_DEPTH), [`GL_MAX_LAYERS`](crate::enums::GL_MAX_LAYERS),
/// [`GL_MAX_COMBINED_DIMENSIONS`](crate::enums::GL_MAX_COMBINED_DIMENSIONS),
/// [`GL_COLOR_COMPONENTS`](crate::enums::GL_COLOR_COMPONENTS), [`GL_DEPTH_COMPONENTS`](crate::enums::GL_DEPTH_COMPONENTS),
/// [`GL_STENCIL_COMPONENTS`](crate::enums::GL_STENCIL_COMPONENTS), [`GL_COLOR_RENDERABLE`](crate::enums::GL_COLOR_RENDERABLE),
/// [`GL_DEPTH_RENDERABLE`](crate::enums::GL_DEPTH_RENDERABLE), [`GL_STENCIL_RENDERABLE`](crate::enums::GL_STENCIL_RENDERABLE),
/// [`GL_FRAMEBUFFER_RENDERABLE`](crate::enums::GL_FRAMEBUFFER_RENDERABLE),
/// [`GL_FRAMEBUFFER_RENDERABLE_LAYERED`](crate::enums::GL_FRAMEBUFFER_RENDERABLE_LAYERED),
/// [`GL_FRAMEBUFFER_BLEND`](crate::enums::GL_FRAMEBUFFER_BLEND), [`GL_READ_PIXELS`](crate::enums::GL_READ_PIXELS),
/// [`GL_READ_PIXELS_FORMAT`](crate::enums::GL_READ_PIXELS_FORMAT), [`GL_READ_PIXELS_TYPE`](crate::enums::GL_READ_PIXELS_TYPE),
/// [`GL_TEXTURE_IMAGE_FORMAT`](crate::enums::GL_TEXTURE_IMAGE_FORMAT), [`GL_TEXTURE_IMAGE_TYPE`](crate::enums::GL_TEXTURE_IMAGE_TYPE),
/// [`GL_GET_TEXTURE_IMAGE_FORMAT`](crate::enums::GL_GET_TEXTURE_IMAGE_FORMAT),
/// [`GL_GET_TEXTURE_IMAGE_TYPE`](crate::enums::GL_GET_TEXTURE_IMAGE_TYPE),
/// [`GL_MIPMAP`](crate::enums::GL_MIPMAP), [`GL_GENERATE_MIPMAP`](crate::enums::GL_GENERATE_MIPMAP),
/// [`GL_AUTO_GENERATE_MIPMAP`](crate::enums::GL_AUTO_GENERATE_MIPMAP), [`GL_COLOR_ENCODING`](crate::enums::GL_COLOR_ENCODING),
/// [`GL_SRGB_READ`](crate::enums::GL_SRGB_READ), [`GL_SRGB_WRITE`](crate::enums::GL_SRGB_WRITE),
/// [`GL_SRGB_DECODE_ARB`](crate::enums::GL_SRGB_DECODE_ARB), [`GL_FILTER`](crate::enums::GL_FILTER),
/// [`GL_VERTEX_TEXTURE`](crate::enums::GL_VERTEX_TEXTURE), [`GL_TESS_CONTROL_TEXTURE`](crate::enums::GL_TESS_CONTROL_TEXTURE),
/// [`GL_TESS_EVALUATION_TEXTURE`](crate::enums::GL_TESS_EVALUATION_TEXTURE),
/// [`GL_GEOMETRY_TEXTURE`](crate::enums::GL_GEOMETRY_TEXTURE), [`GL_FRAGMENT_TEXTURE`](crate::enums::GL_FRAGMENT_TEXTURE),
/// [`GL_COMPUTE_TEXTURE`](crate::enums::GL_COMPUTE_TEXTURE), [`GL_TEXTURE_SHADOW`](crate::enums::GL_TEXTURE_SHADOW),
/// [`GL_TEXTURE_GATHER`](crate::enums::GL_TEXTURE_GATHER), [`GL_TEXTURE_GATHER_SHADOW`](crate::enums::GL_TEXTURE_GATHER_SHADOW),
/// [`GL_SHADER_IMAGE_LOAD`](crate::enums::GL_SHADER_IMAGE_LOAD), [`GL_SHADER_IMAGE_STORE`](crate::enums::GL_SHADER_IMAGE_STORE),
/// [`GL_SHADER_IMAGE_ATOMIC`](crate::enums::GL_SHADER_IMAGE_ATOMIC), [`GL_IMAGE_TEXEL_SIZE`](crate::enums::GL_IMAGE_TEXEL_SIZE),
/// [`GL_IMAGE_COMPATIBILITY_CLASS`](crate::enums::GL_IMAGE_COMPATIBILITY_CLASS),
/// [`GL_IMAGE_PIXEL_FORMAT`](crate::enums::GL_IMAGE_PIXEL_FORMAT), [`GL_IMAGE_PIXEL_TYPE`](crate::enums::GL_IMAGE_PIXEL_TYPE),
/// [`GL_IMAGE_FORMAT_COMPATIBILITY_TYPE`](crate::enums::GL_IMAGE_FORMAT_COMPATIBILITY_TYPE),
/// [`GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST`](crate::enums::GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST),
/// [`GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST`](crate::enums::GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST),
/// [`GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE`](crate::enums::GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE),
/// [`GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE`](crate::enums::GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE),
/// [`GL_TEXTURE_COMPRESSED`](crate::enums::GL_TEXTURE_COMPRESSED), [`GL_TEXTURE_COMPRESSED_BLOCK_WIDTH`](crate::enums::GL_TEXTURE_COMPRESSED_BLOCK_WIDTH),
/// [`GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT`](crate::enums::GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT),
/// [`GL_TEXTURE_COMPRESSED_BLOCK_SIZE`](crate::enums::GL_TEXTURE_COMPRESSED_BLOCK_SIZE),
/// [`GL_CLEAR_BUFFER`](crate::enums::GL_CLEAR_BUFFER), [`GL_TEXTURE_VIEW`](crate::enums::GL_TEXTURE_VIEW),
/// and [`GL_VIEW_COMPATIBILITY_CLASS`](crate::enums::GL_VIEW_COMPATIBILITY_CLASS)
/// are supported only if the GL version is 4.3 or higher.
///
/// The [`GL_CLEAR_TEXTURE`](crate::enums::GL_CLEAR_TEXTURE) token is accepted
/// for `pname` only if the GL version is 4.4 or higher.
pub mod get_internalformat {
    use crate::context::Context;
    use crate::dispatch::gltypes::{GLenum, GLint, GLint64, GLsizei};
    impl Context {
        pub(crate) unsafe fn oxidegl_get_internalformativ(
            &mut self,
            target: GLenum,
            internalformat: GLenum,
            pname: GLenum,
            count: GLsizei,
            params: *mut GLint,
        ) {
            panic!("command oxidegl_get_internalformativ not yet implemented");
        }
        pub(crate) unsafe fn oxidegl_get_internalformati64v(
            &mut self,
            target: GLenum,
            internalformat: GLenum,
            pname: GLenum,
            count: GLsizei,
            params: *mut GLint64,
        ) {
            panic!("command oxidegl_get_internalformati64v not yet implemented");
        }
    }
}
