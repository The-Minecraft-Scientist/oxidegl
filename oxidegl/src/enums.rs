use crate::dispatch::conversions::{GLenumExt, GlDstType, SrcType, UnsafeFromGLenum};
use crate::dispatch::gl_types::*;
use bitflags::bitflags;
pub const GL_DEPTH_BUFFER_BIT: GLenum = 0x100;
pub const GL_STENCIL_BUFFER_BIT: GLenum = 0x400;
pub const GL_COLOR_BUFFER_BIT: GLenum = 0x4000;
pub const GL_FALSE: GLenum = 0;
pub const GL_TRUE: GLenum = 1;
pub const GL_POINTS: GLenum = 0;
pub const GL_LINES: GLenum = 1;
pub const GL_LINE_LOOP: GLenum = 2;
pub const GL_LINE_STRIP: GLenum = 3;
pub const GL_TRIANGLES: GLenum = 4;
pub const GL_TRIANGLE_STRIP: GLenum = 5;
pub const GL_TRIANGLE_FAN: GLenum = 6;
pub const GL_NEVER: GLenum = 512;
pub const GL_LESS: GLenum = 513;
pub const GL_EQUAL: GLenum = 514;
pub const GL_LEQUAL: GLenum = 515;
pub const GL_GREATER: GLenum = 516;
pub const GL_NOTEQUAL: GLenum = 517;
pub const GL_GEQUAL: GLenum = 518;
pub const GL_ALWAYS: GLenum = 519;
pub const GL_ZERO: GLenum = 0;
pub const GL_ONE: GLenum = 1;
pub const GL_SRC_COLOR: GLenum = 768;
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 769;
pub const GL_SRC_ALPHA: GLenum = 770;
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 771;
pub const GL_DST_ALPHA: GLenum = 772;
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 773;
pub const GL_DST_COLOR: GLenum = 774;
pub const GL_ONE_MINUS_DST_COLOR: GLenum = 775;
pub const GL_SRC_ALPHA_SATURATE: GLenum = 776;
pub const GL_FRONT_LEFT: GLenum = 1024;
pub const GL_FRONT_RIGHT: GLenum = 1025;
pub const GL_BACK_LEFT: GLenum = 1026;
pub const GL_BACK_RIGHT: GLenum = 1027;
pub const GL_FRONT: GLenum = 1028;
pub const GL_LEFT: GLenum = 1030;
pub const GL_RIGHT: GLenum = 1031;
pub const GL_FRONT_AND_BACK: GLenum = 1032;
pub const GL_INVALID_ENUM: GLenum = 1280;
pub const GL_INVALID_VALUE: GLenum = 1281;
pub const GL_INVALID_OPERATION: GLenum = 1282;
pub const GL_OUT_OF_MEMORY: GLenum = 1285;
pub const GL_CW: GLenum = 2304;
pub const GL_CCW: GLenum = 2305;
pub const GL_POINT_SIZE: GLenum = 2833;
pub const GL_POINT_SIZE_RANGE: GLenum = 2834;
pub const GL_POINT_SIZE_GRANULARITY: GLenum = 2835;
pub const GL_LINE_SMOOTH: GLenum = 2848;
pub const GL_LINE_WIDTH: GLenum = 2849;
pub const GL_LINE_WIDTH_RANGE: GLenum = 2850;
pub const GL_LINE_WIDTH_GRANULARITY: GLenum = 2851;
pub const GL_POLYGON_MODE: GLenum = 2880;
pub const GL_POLYGON_SMOOTH: GLenum = 2881;
pub const GL_CULL_FACE: GLenum = 2884;
pub const GL_CULL_FACE_MODE: GLenum = 2885;
pub const GL_FRONT_FACE: GLenum = 2886;
pub const GL_DEPTH_RANGE: GLenum = 2928;
pub const GL_DEPTH_TEST: GLenum = 2929;
pub const GL_DEPTH_WRITEMASK: GLenum = 2930;
pub const GL_DEPTH_CLEAR_VALUE: GLenum = 2931;
pub const GL_DEPTH_FUNC: GLenum = 2932;
pub const GL_STENCIL_TEST: GLenum = 2960;
pub const GL_STENCIL_CLEAR_VALUE: GLenum = 2961;
pub const GL_STENCIL_FUNC: GLenum = 2962;
pub const GL_STENCIL_VALUE_MASK: GLenum = 2963;
pub const GL_STENCIL_FAIL: GLenum = 2964;
pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = 2965;
pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = 2966;
pub const GL_STENCIL_REF: GLenum = 2967;
pub const GL_STENCIL_WRITEMASK: GLenum = 2968;
pub const GL_VIEWPORT: GLenum = 2978;
pub const GL_DITHER: GLenum = 3024;
pub const GL_BLEND_DST: GLenum = 3040;
pub const GL_BLEND_SRC: GLenum = 3041;
pub const GL_BLEND: GLenum = 3042;
pub const GL_LOGIC_OP_MODE: GLenum = 3056;
pub const GL_DRAW_BUFFER: GLenum = 3073;
pub const GL_READ_BUFFER: GLenum = 3074;
pub const GL_SCISSOR_BOX: GLenum = 3088;
pub const GL_SCISSOR_TEST: GLenum = 3089;
pub const GL_COLOR_CLEAR_VALUE: GLenum = 3106;
pub const GL_COLOR_WRITEMASK: GLenum = 3107;
pub const GL_DOUBLEBUFFER: GLenum = 3122;
pub const GL_STEREO: GLenum = 3123;
pub const GL_LINE_SMOOTH_HINT: GLenum = 3154;
pub const GL_POLYGON_SMOOTH_HINT: GLenum = 3155;
pub const GL_UNPACK_SWAP_BYTES: GLenum = 3312;
pub const GL_UNPACK_LSB_FIRST: GLenum = 3313;
pub const GL_UNPACK_ROW_LENGTH: GLenum = 3314;
pub const GL_UNPACK_SKIP_ROWS: GLenum = 3315;
pub const GL_UNPACK_SKIP_PIXELS: GLenum = 3316;
pub const GL_UNPACK_ALIGNMENT: GLenum = 3317;
pub const GL_PACK_SWAP_BYTES: GLenum = 3328;
pub const GL_PACK_LSB_FIRST: GLenum = 3329;
pub const GL_PACK_ROW_LENGTH: GLenum = 3330;
pub const GL_PACK_SKIP_ROWS: GLenum = 3331;
pub const GL_PACK_SKIP_PIXELS: GLenum = 3332;
pub const GL_PACK_ALIGNMENT: GLenum = 3333;
pub const GL_MAX_TEXTURE_SIZE: GLenum = 3379;
pub const GL_MAX_VIEWPORT_DIMS: GLenum = 3386;
pub const GL_SUBPIXEL_BITS: GLenum = 0xD50;
pub const GL_TEXTURE_1D: GLenum = 3552;
pub const GL_TEXTURE_2D: GLenum = 3553;
pub const GL_TEXTURE_WIDTH: GLenum = 4096;
pub const GL_TEXTURE_HEIGHT: GLenum = 4097;
pub const GL_TEXTURE_BORDER_COLOR: GLenum = 4100;
pub const GL_DONT_CARE: GLenum = 4352;
pub const GL_FASTEST: GLenum = 4353;
pub const GL_NICEST: GLenum = 4354;
pub const GL_BYTE: GLenum = 5120;
pub const GL_UNSIGNED_BYTE: GLenum = 5121;
pub const GL_SHORT: GLenum = 5122;
pub const GL_UNSIGNED_SHORT: GLenum = 5123;
pub const GL_INT: GLenum = 5124;
pub const GL_UNSIGNED_INT: GLenum = 5125;
pub const GL_FLOAT: GLenum = 5126;
pub const GL_CLEAR: GLenum = 5376;
pub const GL_AND: GLenum = 5377;
pub const GL_AND_REVERSE: GLenum = 5378;
pub const GL_COPY: GLenum = 5379;
pub const GL_AND_INVERTED: GLenum = 5380;
pub const GL_NOOP: GLenum = 5381;
pub const GL_XOR: GLenum = 5382;
pub const GL_OR: GLenum = 5383;
pub const GL_NOR: GLenum = 5384;
pub const GL_EQUIV: GLenum = 5385;
pub const GL_INVERT: GLenum = 5386;
pub const GL_OR_REVERSE: GLenum = 5387;
pub const GL_COPY_INVERTED: GLenum = 5388;
pub const GL_OR_INVERTED: GLenum = 5389;
pub const GL_NAND: GLenum = 5390;
pub const GL_SET: GLenum = 5391;
pub const GL_TEXTURE: GLenum = 5890;
pub const GL_COLOR: GLenum = 6144;
pub const GL_DEPTH: GLenum = 6145;
pub const GL_STENCIL: GLenum = 6146;
pub const GL_DEPTH_COMPONENT: GLenum = 6402;
pub const GL_RED: GLenum = 6403;
pub const GL_GREEN: GLenum = 6404;
pub const GL_BLUE: GLenum = 6405;
pub const GL_ALPHA: GLenum = 6406;
pub const GL_RGB: GLenum = 6407;
pub const GL_RGBA: GLenum = 6408;
pub const GL_POINT: GLenum = 6912;
pub const GL_LINE: GLenum = 6913;
pub const GL_FILL: GLenum = 6914;
pub const GL_KEEP: GLenum = 7680;
pub const GL_REPLACE: GLenum = 7681;
pub const GL_INCR: GLenum = 7682;
pub const GL_DECR: GLenum = 7683;
pub const GL_VENDOR: GLenum = 7936;
pub const GL_RENDERER: GLenum = 7937;
pub const GL_VERSION: GLenum = 7938;
pub const GL_EXTENSIONS: GLenum = 7939;
pub const GL_NEAREST: GLenum = 9728;
pub const GL_LINEAR: GLenum = 9729;
pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = 9984;
pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = 9985;
pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = 9986;
pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = 9987;
pub const GL_TEXTURE_MAG_FILTER: GLenum = 10240;
pub const GL_TEXTURE_MIN_FILTER: GLenum = 10241;
pub const GL_TEXTURE_WRAP_S: GLenum = 10242;
pub const GL_TEXTURE_WRAP_T: GLenum = 10243;
pub const GL_REPEAT: GLenum = 10497;
pub const GL_COLOR_LOGIC_OP: GLenum = 3058;
pub const GL_POLYGON_OFFSET_UNITS: GLenum = 10752;
pub const GL_POLYGON_OFFSET_POINT: GLenum = 10753;
pub const GL_POLYGON_OFFSET_LINE: GLenum = 10754;
pub const GL_POLYGON_OFFSET_FILL: GLenum = 32823;
pub const GL_POLYGON_OFFSET_FACTOR: GLenum = 32824;
pub const GL_TEXTURE_INTERNAL_FORMAT: GLenum = 4099;
pub const GL_TEXTURE_RED_SIZE: GLenum = 32860;
pub const GL_TEXTURE_GREEN_SIZE: GLenum = 32861;
pub const GL_TEXTURE_BLUE_SIZE: GLenum = 32862;
pub const GL_TEXTURE_ALPHA_SIZE: GLenum = 32863;
pub const GL_DOUBLE: GLenum = 5130;
pub const GL_PROXY_TEXTURE_1D: GLenum = 32867;
pub const GL_PROXY_TEXTURE_2D: GLenum = 32868;
pub const GL_R3_G3_B2: GLenum = 10768;
pub const GL_RGB4: GLenum = 32847;
pub const GL_RGB5: GLenum = 32848;
pub const GL_RGB8: GLenum = 32849;
pub const GL_RGB10: GLenum = 32850;
pub const GL_RGB12: GLenum = 32851;
pub const GL_RGB16: GLenum = 32852;
pub const GL_RGBA2: GLenum = 32853;
pub const GL_RGBA4: GLenum = 32854;
pub const GL_RGB5_A1: GLenum = 32855;
pub const GL_RGBA8: GLenum = 32856;
pub const GL_RGB10_A2: GLenum = 32857;
pub const GL_RGBA12: GLenum = 32858;
pub const GL_RGBA16: GLenum = 32859;
pub const GL_UNSIGNED_BYTE_3_3_2: GLenum = 32818;
pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = 32819;
pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = 32820;
pub const GL_UNSIGNED_INT_8_8_8_8: GLenum = 32821;
pub const GL_UNSIGNED_INT_10_10_10_2: GLenum = 32822;
pub const GL_PACK_SKIP_IMAGES: GLenum = 32875;
pub const GL_PACK_IMAGE_HEIGHT: GLenum = 32876;
pub const GL_UNPACK_SKIP_IMAGES: GLenum = 32877;
pub const GL_UNPACK_IMAGE_HEIGHT: GLenum = 32878;
pub const GL_TEXTURE_3D: GLenum = 32879;
pub const GL_PROXY_TEXTURE_3D: GLenum = 32880;
pub const GL_TEXTURE_DEPTH: GLenum = 32881;
pub const GL_TEXTURE_WRAP_R: GLenum = 32882;
pub const GL_MAX_3D_TEXTURE_SIZE: GLenum = 32883;
pub const GL_UNSIGNED_BYTE_2_3_3_REV: GLenum = 33634;
pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = 33635;
pub const GL_UNSIGNED_SHORT_5_6_5_REV: GLenum = 33636;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 33637;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 33638;
pub const GL_UNSIGNED_INT_8_8_8_8_REV: GLenum = 33639;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = 33640;
pub const GL_BGR: GLenum = 32992;
pub const GL_BGRA: GLenum = 32993;
pub const GL_MAX_ELEMENTS_VERTICES: GLenum = 33000;
pub const GL_MAX_ELEMENTS_INDICES: GLenum = 33001;
pub const GL_CLAMP_TO_EDGE: GLenum = 33071;
pub const GL_TEXTURE_MIN_LOD: GLenum = 33082;
pub const GL_TEXTURE_MAX_LOD: GLenum = 33083;
pub const GL_TEXTURE_BASE_LEVEL: GLenum = 33084;
pub const GL_TEXTURE_MAX_LEVEL: GLenum = 33085;
pub const GL_SMOOTH_POINT_SIZE_RANGE: GLenum = 2834;
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 2835;
pub const GL_SMOOTH_LINE_WIDTH_RANGE: GLenum = 2850;
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 2851;
pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = 33902;
pub const GL_TEXTURE0: GLenum = 33984;
pub const GL_TEXTURE1: GLenum = 33985;
pub const GL_TEXTURE2: GLenum = 33986;
pub const GL_TEXTURE3: GLenum = 33987;
pub const GL_TEXTURE4: GLenum = 33988;
pub const GL_TEXTURE5: GLenum = 33989;
pub const GL_TEXTURE6: GLenum = 33990;
pub const GL_TEXTURE7: GLenum = 33991;
pub const GL_TEXTURE8: GLenum = 33992;
pub const GL_TEXTURE9: GLenum = 33993;
pub const GL_TEXTURE10: GLenum = 33994;
pub const GL_TEXTURE11: GLenum = 33995;
pub const GL_TEXTURE12: GLenum = 33996;
pub const GL_TEXTURE13: GLenum = 33997;
pub const GL_TEXTURE14: GLenum = 33998;
pub const GL_TEXTURE15: GLenum = 33999;
pub const GL_TEXTURE16: GLenum = 34000;
pub const GL_TEXTURE17: GLenum = 34001;
pub const GL_TEXTURE18: GLenum = 34002;
pub const GL_TEXTURE19: GLenum = 34003;
pub const GL_TEXTURE20: GLenum = 34004;
pub const GL_TEXTURE21: GLenum = 34005;
pub const GL_TEXTURE22: GLenum = 34006;
pub const GL_TEXTURE23: GLenum = 34007;
pub const GL_TEXTURE24: GLenum = 34008;
pub const GL_TEXTURE25: GLenum = 34009;
pub const GL_TEXTURE26: GLenum = 34010;
pub const GL_TEXTURE27: GLenum = 34011;
pub const GL_TEXTURE28: GLenum = 34012;
pub const GL_TEXTURE29: GLenum = 34013;
pub const GL_TEXTURE30: GLenum = 34014;
pub const GL_TEXTURE31: GLenum = 34015;
pub const GL_ACTIVE_TEXTURE: GLenum = 34016;
pub const GL_MULTISAMPLE: GLenum = 32925;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 32926;
pub const GL_SAMPLE_ALPHA_TO_ONE: GLenum = 32927;
pub const GL_SAMPLE_COVERAGE: GLenum = 32928;
pub const GL_SAMPLE_BUFFERS: GLenum = 32936;
pub const GL_SAMPLES: GLenum = 32937;
pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = 32938;
pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = 32939;
pub const GL_TEXTURE_CUBE_MAP: GLenum = 34067;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 34069;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 34070;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 34071;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 34072;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 34073;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 34074;
pub const GL_PROXY_TEXTURE_CUBE_MAP: GLenum = 34075;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 34076;
pub const GL_COMPRESSED_RGB: GLenum = 34029;
pub const GL_COMPRESSED_RGBA: GLenum = 34030;
pub const GL_TEXTURE_COMPRESSION_HINT: GLenum = 34031;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 34464;
pub const GL_TEXTURE_COMPRESSED: GLenum = 34465;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 34466;
pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = 34467;
pub const GL_CLAMP_TO_BORDER: GLenum = 33069;
pub const GL_BLEND_DST_RGB: GLenum = 32968;
pub const GL_BLEND_SRC_RGB: GLenum = 32969;
pub const GL_BLEND_DST_ALPHA: GLenum = 32970;
pub const GL_BLEND_SRC_ALPHA: GLenum = 32971;
pub const GL_POINT_FADE_THRESHOLD_SIZE: GLenum = 33064;
pub const GL_DEPTH_COMPONENT16: GLenum = 33189;
pub const GL_DEPTH_COMPONENT24: GLenum = 33190;
pub const GL_DEPTH_COMPONENT32: GLenum = 33191;
pub const GL_MIRRORED_REPEAT: GLenum = 33648;
pub const GL_MAX_TEXTURE_LOD_BIAS: GLenum = 34045;
pub const GL_TEXTURE_LOD_BIAS: GLenum = 34049;
pub const GL_INCR_WRAP: GLenum = 34055;
pub const GL_DECR_WRAP: GLenum = 34056;
pub const GL_TEXTURE_DEPTH_SIZE: GLenum = 34890;
pub const GL_TEXTURE_COMPARE_MODE: GLenum = 34892;
pub const GL_TEXTURE_COMPARE_FUNC: GLenum = 34893;
pub const GL_BLEND_COLOR: GLenum = 32773;
pub const GL_BLEND_EQUATION: GLenum = 32777;
pub const GL_CONSTANT_COLOR: GLenum = 32769;
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = 32770;
pub const GL_CONSTANT_ALPHA: GLenum = 32771;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = 32772;
pub const GL_FUNC_ADD: GLenum = 32774;
pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = 32779;
pub const GL_FUNC_SUBTRACT: GLenum = 32778;
pub const GL_MIN: GLenum = 32775;
pub const GL_MAX: GLenum = 32776;
pub const GL_BUFFER_SIZE: GLenum = 34660;
pub const GL_BUFFER_USAGE: GLenum = 34661;
pub const GL_QUERY_COUNTER_BITS: GLenum = 0x8864;
pub const GL_CURRENT_QUERY: GLenum = 34917;
pub const GL_QUERY_RESULT: GLenum = 34918;
pub const GL_QUERY_RESULT_AVAILABLE: GLenum = 34919;
pub const GL_ARRAY_BUFFER: GLenum = 34962;
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 34963;
pub const GL_ARRAY_BUFFER_BINDING: GLenum = 34964;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 34965;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 34975;
pub const GL_READ_ONLY: GLenum = 35000;
pub const GL_WRITE_ONLY: GLenum = 35001;
pub const GL_READ_WRITE: GLenum = 35002;
pub const GL_BUFFER_ACCESS: GLenum = 35003;
pub const GL_BUFFER_MAPPED: GLenum = 35004;
pub const GL_BUFFER_MAP_POINTER: GLenum = 35005;
pub const GL_STREAM_DRAW: GLenum = 35040;
pub const GL_STREAM_READ: GLenum = 35041;
pub const GL_STREAM_COPY: GLenum = 35042;
pub const GL_STATIC_DRAW: GLenum = 35044;
pub const GL_STATIC_READ: GLenum = 35045;
pub const GL_STATIC_COPY: GLenum = 35046;
pub const GL_DYNAMIC_DRAW: GLenum = 35048;
pub const GL_DYNAMIC_READ: GLenum = 35049;
pub const GL_DYNAMIC_COPY: GLenum = 35050;
pub const GL_SAMPLES_PASSED: GLenum = 35092;
pub const GL_SRC1_ALPHA: GLenum = 34185;
pub const GL_BLEND_EQUATION_RGB: GLenum = 32777;
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 34338;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 34339;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 34340;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 34341;
pub const GL_CURRENT_VERTEX_ATTRIB: GLenum = 34342;
pub const GL_VERTEX_PROGRAM_POINT_SIZE: GLenum = 34370;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 34373;
pub const GL_STENCIL_BACK_FUNC: GLenum = 34816;
pub const GL_STENCIL_BACK_FAIL: GLenum = 34817;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 34818;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 34819;
pub const GL_MAX_DRAW_BUFFERS: GLenum = 34852;
pub const GL_DRAW_BUFFER0: GLenum = 34853;
pub const GL_DRAW_BUFFER1: GLenum = 34854;
pub const GL_DRAW_BUFFER2: GLenum = 34855;
pub const GL_DRAW_BUFFER3: GLenum = 34856;
pub const GL_DRAW_BUFFER4: GLenum = 34857;
pub const GL_DRAW_BUFFER5: GLenum = 34858;
pub const GL_DRAW_BUFFER6: GLenum = 34859;
pub const GL_DRAW_BUFFER7: GLenum = 34860;
pub const GL_DRAW_BUFFER8: GLenum = 34861;
pub const GL_DRAW_BUFFER9: GLenum = 34862;
pub const GL_DRAW_BUFFER10: GLenum = 34863;
pub const GL_DRAW_BUFFER11: GLenum = 34864;
pub const GL_DRAW_BUFFER12: GLenum = 34865;
pub const GL_DRAW_BUFFER13: GLenum = 34866;
pub const GL_DRAW_BUFFER14: GLenum = 34867;
pub const GL_DRAW_BUFFER15: GLenum = 34868;
pub const GL_BLEND_EQUATION_ALPHA: GLenum = 34877;
pub const GL_MAX_VERTEX_ATTRIBS: GLenum = 34921;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 34922;
pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = 34930;
pub const GL_FRAGMENT_SHADER: GLenum = 35632;
pub const GL_VERTEX_SHADER: GLenum = 35633;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35657;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 35658;
pub const GL_MAX_VARYING_FLOATS: GLenum = 35659;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 35660;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 35661;
pub const GL_SHADER_TYPE: GLenum = 35663;
pub const GL_FLOAT_VEC2: GLenum = 35664;
pub const GL_FLOAT_VEC3: GLenum = 35665;
pub const GL_FLOAT_VEC4: GLenum = 35666;
pub const GL_INT_VEC2: GLenum = 35667;
pub const GL_INT_VEC3: GLenum = 35668;
pub const GL_INT_VEC4: GLenum = 35669;
pub const GL_BOOL: GLenum = 35670;
pub const GL_BOOL_VEC2: GLenum = 35671;
pub const GL_BOOL_VEC3: GLenum = 35672;
pub const GL_BOOL_VEC4: GLenum = 35673;
pub const GL_FLOAT_MAT2: GLenum = 35674;
pub const GL_FLOAT_MAT3: GLenum = 35675;
pub const GL_FLOAT_MAT4: GLenum = 35676;
pub const GL_SAMPLER_1D: GLenum = 35677;
pub const GL_SAMPLER_2D: GLenum = 35678;
pub const GL_SAMPLER_3D: GLenum = 35679;
pub const GL_SAMPLER_CUBE: GLenum = 35680;
pub const GL_SAMPLER_1D_SHADOW: GLenum = 35681;
pub const GL_SAMPLER_2D_SHADOW: GLenum = 35682;
pub const GL_DELETE_STATUS: GLenum = 35712;
pub const GL_COMPILE_STATUS: GLenum = 35713;
pub const GL_LINK_STATUS: GLenum = 35714;
pub const GL_VALIDATE_STATUS: GLenum = 35715;
pub const GL_INFO_LOG_LENGTH: GLenum = 35716;
pub const GL_ATTACHED_SHADERS: GLenum = 35717;
pub const GL_ACTIVE_UNIFORMS: GLenum = 35718;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 35719;
pub const GL_SHADER_SOURCE_LENGTH: GLenum = 35720;
pub const GL_ACTIVE_ATTRIBUTES: GLenum = 35721;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 35722;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 35723;
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 35724;
pub const GL_CURRENT_PROGRAM: GLenum = 35725;
pub const GL_POINT_SPRITE_COORD_ORIGIN: GLenum = 36000;
pub const GL_STENCIL_BACK_REF: GLenum = 36003;
pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = 36004;
pub const GL_STENCIL_BACK_WRITEMASK: GLenum = 36005;
pub const GL_PIXEL_PACK_BUFFER: GLenum = 35051;
pub const GL_PIXEL_UNPACK_BUFFER: GLenum = 35052;
pub const GL_PIXEL_PACK_BUFFER_BINDING: GLenum = 35053;
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = 35055;
pub const GL_FLOAT_MAT2x3: GLenum = 35685;
pub const GL_FLOAT_MAT2x4: GLenum = 35686;
pub const GL_FLOAT_MAT3x2: GLenum = 35687;
pub const GL_FLOAT_MAT3x4: GLenum = 35688;
pub const GL_FLOAT_MAT4x2: GLenum = 35689;
pub const GL_FLOAT_MAT4x3: GLenum = 35690;
pub const GL_SRGB: GLenum = 35904;
pub const GL_SRGB8: GLenum = 35905;
pub const GL_SRGB_ALPHA: GLenum = 35906;
pub const GL_SRGB8_ALPHA8: GLenum = 35907;
pub const GL_COMPRESSED_SRGB: GLenum = 35912;
pub const GL_COMPRESSED_SRGB_ALPHA: GLenum = 35913;
pub const GL_COMPARE_REF_TO_TEXTURE: GLenum = 34894;
pub const GL_CLIP_DISTANCE0: GLenum = 12288;
pub const GL_CLIP_DISTANCE1: GLenum = 12289;
pub const GL_CLIP_DISTANCE2: GLenum = 12290;
pub const GL_CLIP_DISTANCE3: GLenum = 12291;
pub const GL_CLIP_DISTANCE4: GLenum = 12292;
pub const GL_CLIP_DISTANCE5: GLenum = 12293;
pub const GL_CLIP_DISTANCE6: GLenum = 12294;
pub const GL_CLIP_DISTANCE7: GLenum = 12295;
pub const GL_MAX_CLIP_DISTANCES: GLenum = 3378;
pub const GL_MAJOR_VERSION: GLenum = 33307;
pub const GL_MINOR_VERSION: GLenum = 33308;
pub const GL_NUM_EXTENSIONS: GLenum = 33309;
pub const GL_CONTEXT_FLAGS: GLenum = 33310;
pub const GL_COMPRESSED_RED: GLenum = 33317;
pub const GL_COMPRESSED_RG: GLenum = 33318;
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x1;
pub const GL_RGBA32F: GLenum = 34836;
pub const GL_RGB32F: GLenum = 34837;
pub const GL_RGBA16F: GLenum = 34842;
pub const GL_RGB16F: GLenum = 34843;
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 35069;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = 35071;
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: GLenum = 35076;
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: GLenum = 35077;
pub const GL_CLAMP_READ_COLOR: GLenum = 35100;
pub const GL_FIXED_ONLY: GLenum = 35101;
pub const GL_MAX_VARYING_COMPONENTS: GLenum = 35659;
pub const GL_TEXTURE_1D_ARRAY: GLenum = 35864;
pub const GL_PROXY_TEXTURE_1D_ARRAY: GLenum = 35865;
pub const GL_TEXTURE_2D_ARRAY: GLenum = 35866;
pub const GL_PROXY_TEXTURE_2D_ARRAY: GLenum = 35867;
pub const GL_R11F_G11F_B10F: GLenum = 35898;
pub const GL_RGB9_E5: GLenum = 35901;
pub const GL_UNSIGNED_INT_5_9_9_9_REV: GLenum = 35902;
pub const GL_TEXTURE_SHARED_SIZE: GLenum = 35903;
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 35958;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 35967;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 35968;
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = 35971;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 35972;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 35973;
pub const GL_PRIMITIVES_GENERATED: GLenum = 35975;
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 35976;
pub const GL_RASTERIZER_DISCARD: GLenum = 35977;
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 35978;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 35979;
pub const GL_INTERLEAVED_ATTRIBS: GLenum = 35980;
pub const GL_SEPARATE_ATTRIBS: GLenum = 35981;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 35983;
pub const GL_RGBA32UI: GLenum = 36208;
pub const GL_RGB32UI: GLenum = 36209;
pub const GL_RGBA16UI: GLenum = 36214;
pub const GL_RGB16UI: GLenum = 36215;
pub const GL_RGBA8UI: GLenum = 36220;
pub const GL_RGB8UI: GLenum = 36221;
pub const GL_RGBA32I: GLenum = 36226;
pub const GL_RGB32I: GLenum = 36227;
pub const GL_RGBA16I: GLenum = 36232;
pub const GL_RGB16I: GLenum = 36233;
pub const GL_RGBA8I: GLenum = 36238;
pub const GL_RGB8I: GLenum = 36239;
pub const GL_RED_INTEGER: GLenum = 36244;
pub const GL_GREEN_INTEGER: GLenum = 36245;
pub const GL_BLUE_INTEGER: GLenum = 36246;
pub const GL_RGB_INTEGER: GLenum = 36248;
pub const GL_RGBA_INTEGER: GLenum = 36249;
pub const GL_BGR_INTEGER: GLenum = 36250;
pub const GL_BGRA_INTEGER: GLenum = 36251;
pub const GL_SAMPLER_1D_ARRAY: GLenum = 36288;
pub const GL_SAMPLER_2D_ARRAY: GLenum = 36289;
pub const GL_SAMPLER_1D_ARRAY_SHADOW: GLenum = 36291;
pub const GL_SAMPLER_2D_ARRAY_SHADOW: GLenum = 36292;
pub const GL_SAMPLER_CUBE_SHADOW: GLenum = 36293;
pub const GL_UNSIGNED_INT_VEC2: GLenum = 36294;
pub const GL_UNSIGNED_INT_VEC3: GLenum = 36295;
pub const GL_UNSIGNED_INT_VEC4: GLenum = 36296;
pub const GL_INT_SAMPLER_1D: GLenum = 36297;
pub const GL_INT_SAMPLER_2D: GLenum = 36298;
pub const GL_INT_SAMPLER_3D: GLenum = 36299;
pub const GL_INT_SAMPLER_CUBE: GLenum = 36300;
pub const GL_INT_SAMPLER_1D_ARRAY: GLenum = 36302;
pub const GL_INT_SAMPLER_2D_ARRAY: GLenum = 36303;
pub const GL_UNSIGNED_INT_SAMPLER_1D: GLenum = 36305;
pub const GL_UNSIGNED_INT_SAMPLER_2D: GLenum = 36306;
pub const GL_UNSIGNED_INT_SAMPLER_3D: GLenum = 36307;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: GLenum = 36308;
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 36310;
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 36311;
pub const GL_QUERY_WAIT: GLenum = 36371;
pub const GL_QUERY_NO_WAIT: GLenum = 36372;
pub const GL_QUERY_BY_REGION_WAIT: GLenum = 36373;
pub const GL_QUERY_BY_REGION_NO_WAIT: GLenum = 36374;
pub const GL_BUFFER_ACCESS_FLAGS: GLenum = 37151;
pub const GL_BUFFER_MAP_LENGTH: GLenum = 37152;
pub const GL_BUFFER_MAP_OFFSET: GLenum = 37153;
pub const GL_DEPTH_COMPONENT32F: GLenum = 36012;
pub const GL_DEPTH32F_STENCIL8: GLenum = 36013;
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 36269;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 1286;
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 33296;
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 33297;
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 33298;
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 33299;
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 33300;
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 33301;
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 33302;
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 33303;
pub const GL_FRAMEBUFFER_DEFAULT: GLenum = 33304;
pub const GL_FRAMEBUFFER_UNDEFINED: GLenum = 33305;
pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = 33306;
pub const GL_MAX_RENDERBUFFER_SIZE: GLenum = 34024;
pub const GL_DEPTH_STENCIL: GLenum = 34041;
pub const GL_UNSIGNED_INT_24_8: GLenum = 34042;
pub const GL_DEPTH24_STENCIL8: GLenum = 35056;
pub const GL_TEXTURE_STENCIL_SIZE: GLenum = 35057;
pub const GL_TEXTURE_RED_TYPE: GLenum = 35856;
pub const GL_TEXTURE_GREEN_TYPE: GLenum = 35857;
pub const GL_TEXTURE_BLUE_TYPE: GLenum = 35858;
pub const GL_TEXTURE_ALPHA_TYPE: GLenum = 35859;
pub const GL_TEXTURE_DEPTH_TYPE: GLenum = 35862;
pub const GL_UNSIGNED_NORMALIZED: GLenum = 35863;
pub const GL_FRAMEBUFFER_BINDING: GLenum = 36006;
pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = 36006;
pub const GL_RENDERBUFFER_BINDING: GLenum = 36007;
pub const GL_READ_FRAMEBUFFER: GLenum = 36008;
pub const GL_DRAW_FRAMEBUFFER: GLenum = 36009;
pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = 36010;
pub const GL_RENDERBUFFER_SAMPLES: GLenum = 36011;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 36048;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 36049;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 36050;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 36051;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 36052;
pub const GL_FRAMEBUFFER_COMPLETE: GLenum = 36053;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 36054;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 36055;
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 36059;
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 36060;
pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = 36061;
pub const GL_MAX_COLOR_ATTACHMENTS: GLenum = 36063;
pub const GL_COLOR_ATTACHMENT0: GLenum = 36064;
pub const GL_COLOR_ATTACHMENT1: GLenum = 36065;
pub const GL_COLOR_ATTACHMENT2: GLenum = 36066;
pub const GL_COLOR_ATTACHMENT3: GLenum = 36067;
pub const GL_COLOR_ATTACHMENT4: GLenum = 36068;
pub const GL_COLOR_ATTACHMENT5: GLenum = 36069;
pub const GL_COLOR_ATTACHMENT6: GLenum = 36070;
pub const GL_COLOR_ATTACHMENT7: GLenum = 36071;
pub const GL_COLOR_ATTACHMENT8: GLenum = 36072;
pub const GL_COLOR_ATTACHMENT9: GLenum = 36073;
pub const GL_COLOR_ATTACHMENT10: GLenum = 36074;
pub const GL_COLOR_ATTACHMENT11: GLenum = 36075;
pub const GL_COLOR_ATTACHMENT12: GLenum = 36076;
pub const GL_COLOR_ATTACHMENT13: GLenum = 36077;
pub const GL_COLOR_ATTACHMENT14: GLenum = 36078;
pub const GL_COLOR_ATTACHMENT15: GLenum = 36079;
pub const GL_COLOR_ATTACHMENT16: GLenum = 36080;
pub const GL_COLOR_ATTACHMENT17: GLenum = 36081;
pub const GL_COLOR_ATTACHMENT18: GLenum = 36082;
pub const GL_COLOR_ATTACHMENT19: GLenum = 36083;
pub const GL_COLOR_ATTACHMENT20: GLenum = 36084;
pub const GL_COLOR_ATTACHMENT21: GLenum = 36085;
pub const GL_COLOR_ATTACHMENT22: GLenum = 36086;
pub const GL_COLOR_ATTACHMENT23: GLenum = 36087;
pub const GL_COLOR_ATTACHMENT24: GLenum = 36088;
pub const GL_COLOR_ATTACHMENT25: GLenum = 36089;
pub const GL_COLOR_ATTACHMENT26: GLenum = 36090;
pub const GL_COLOR_ATTACHMENT27: GLenum = 36091;
pub const GL_COLOR_ATTACHMENT28: GLenum = 36092;
pub const GL_COLOR_ATTACHMENT29: GLenum = 36093;
pub const GL_COLOR_ATTACHMENT30: GLenum = 36094;
pub const GL_COLOR_ATTACHMENT31: GLenum = 36095;
pub const GL_DEPTH_ATTACHMENT: GLenum = 36096;
pub const GL_STENCIL_ATTACHMENT: GLenum = 36128;
pub const GL_FRAMEBUFFER: GLenum = 36160;
pub const GL_RENDERBUFFER: GLenum = 36161;
pub const GL_RENDERBUFFER_WIDTH: GLenum = 36162;
pub const GL_RENDERBUFFER_HEIGHT: GLenum = 36163;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = 36164;
pub const GL_STENCIL_INDEX1: GLenum = 36166;
pub const GL_STENCIL_INDEX4: GLenum = 36167;
pub const GL_STENCIL_INDEX16: GLenum = 36169;
pub const GL_RENDERBUFFER_RED_SIZE: GLenum = 36176;
pub const GL_RENDERBUFFER_GREEN_SIZE: GLenum = 36177;
pub const GL_RENDERBUFFER_BLUE_SIZE: GLenum = 36178;
pub const GL_RENDERBUFFER_ALPHA_SIZE: GLenum = 36179;
pub const GL_RENDERBUFFER_DEPTH_SIZE: GLenum = 36180;
pub const GL_RENDERBUFFER_STENCIL_SIZE: GLenum = 36181;
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 36182;
pub const GL_MAX_SAMPLES: GLenum = 36183;
pub const GL_FRAMEBUFFER_SRGB: GLenum = 36281;
pub const GL_HALF_FLOAT: GLenum = 5131;
pub const GL_MAP_INVALIDATE_RANGE_BIT: GLenum = 0x4;
pub const GL_MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x8;
pub const GL_MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x10;
pub const GL_MAP_UNSYNCHRONIZED_BIT: GLenum = 0x20;
pub const GL_COMPRESSED_RED_RGTC1: GLenum = 36283;
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: GLenum = 36284;
pub const GL_COMPRESSED_RG_RGTC2: GLenum = 36285;
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: GLenum = 36286;
pub const GL_RG: GLenum = 33319;
pub const GL_RG_INTEGER: GLenum = 33320;
pub const GL_R8: GLenum = 33321;
pub const GL_R16: GLenum = 33322;
pub const GL_RG8: GLenum = 33323;
pub const GL_RG16: GLenum = 33324;
pub const GL_R16F: GLenum = 33325;
pub const GL_R32F: GLenum = 33326;
pub const GL_RG16F: GLenum = 33327;
pub const GL_RG32F: GLenum = 33328;
pub const GL_R8I: GLenum = 33329;
pub const GL_R8UI: GLenum = 33330;
pub const GL_R16I: GLenum = 33331;
pub const GL_R16UI: GLenum = 33332;
pub const GL_R32I: GLenum = 33333;
pub const GL_R32UI: GLenum = 33334;
pub const GL_RG8I: GLenum = 33335;
pub const GL_RG8UI: GLenum = 33336;
pub const GL_RG16I: GLenum = 33337;
pub const GL_RG16UI: GLenum = 33338;
pub const GL_RG32I: GLenum = 33339;
pub const GL_RG32UI: GLenum = 33340;
pub const GL_VERTEX_ARRAY_BINDING: GLenum = 34229;
pub const GL_SAMPLER_2D_RECT: GLenum = 35683;
pub const GL_SAMPLER_2D_RECT_SHADOW: GLenum = 35684;
pub const GL_SAMPLER_BUFFER: GLenum = 36290;
pub const GL_INT_SAMPLER_2D_RECT: GLenum = 36301;
pub const GL_INT_SAMPLER_BUFFER: GLenum = 36304;
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 36309;
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 36312;
pub const GL_TEXTURE_BUFFER: GLenum = 35882;
pub const GL_MAX_TEXTURE_BUFFER_SIZE: GLenum = 35883;
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 35885;
pub const GL_TEXTURE_RECTANGLE: GLenum = 34037;
pub const GL_PROXY_TEXTURE_RECTANGLE: GLenum = 34039;
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 34040;
pub const GL_R8_SNORM: GLenum = 36756;
pub const GL_RG8_SNORM: GLenum = 36757;
pub const GL_RGB8_SNORM: GLenum = 36758;
pub const GL_RGBA8_SNORM: GLenum = 36759;
pub const GL_R16_SNORM: GLenum = 36760;
pub const GL_RG16_SNORM: GLenum = 36761;
pub const GL_RGB16_SNORM: GLenum = 36762;
pub const GL_RGBA16_SNORM: GLenum = 36763;
pub const GL_SIGNED_NORMALIZED: GLenum = 36764;
pub const GL_PRIMITIVE_RESTART: GLenum = 36765;
pub const GL_PRIMITIVE_RESTART_INDEX: GLenum = 36766;
pub const GL_COPY_READ_BUFFER: GLenum = 36662;
pub const GL_COPY_WRITE_BUFFER: GLenum = 36663;
pub const GL_UNIFORM_BUFFER: GLenum = 35345;
pub const GL_UNIFORM_BUFFER_BINDING: GLenum = 35368;
pub const GL_UNIFORM_BUFFER_START: GLenum = 35369;
pub const GL_UNIFORM_BUFFER_SIZE: GLenum = 35370;
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 35371;
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 35372;
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 35373;
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 35374;
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 35375;
pub const GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = 35376;
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 35377;
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 35378;
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35379;
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 35380;
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 35381;
pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = 35382;
pub const GL_UNIFORM_TYPE: GLenum = 35383;
pub const GL_UNIFORM_SIZE: GLenum = 35384;
pub const GL_UNIFORM_NAME_LENGTH: GLenum = 35385;
pub const GL_UNIFORM_BLOCK_INDEX: GLenum = 35386;
pub const GL_UNIFORM_OFFSET: GLenum = 35387;
pub const GL_UNIFORM_ARRAY_STRIDE: GLenum = 35388;
pub const GL_UNIFORM_MATRIX_STRIDE: GLenum = 35389;
pub const GL_UNIFORM_IS_ROW_MAJOR: GLenum = 35390;
pub const GL_UNIFORM_BLOCK_BINDING: GLenum = 35391;
pub const GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = 35392;
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = 35393;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 35394;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 35395;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 35396;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 35397;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 35398;
pub const GL_INVALID_INDEX: GLenum = 4294967295;
pub const GL_CONTEXT_CORE_PROFILE_BIT: GLenum = 0x1;
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x2;
pub const GL_LINES_ADJACENCY: GLenum = 10;
pub const GL_LINE_STRIP_ADJACENCY: GLenum = 11;
pub const GL_TRIANGLES_ADJACENCY: GLenum = 12;
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = 13;
pub const GL_PROGRAM_POINT_SIZE: GLenum = 34370;
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 35881;
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 36263;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 36264;
pub const GL_GEOMETRY_SHADER: GLenum = 36313;
pub const GL_GEOMETRY_VERTICES_OUT: GLenum = 35094;
pub const GL_GEOMETRY_INPUT_TYPE: GLenum = 35095;
pub const GL_GEOMETRY_OUTPUT_TYPE: GLenum = 35096;
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 36319;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 36320;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 36321;
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 37154;
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 37155;
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 37156;
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 37157;
pub const GL_CONTEXT_PROFILE_MASK: GLenum = 37158;
pub const GL_DEPTH_CLAMP: GLenum = 34383;
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 36428;
pub const GL_FIRST_VERTEX_CONVENTION: GLenum = 36429;
pub const GL_LAST_VERTEX_CONVENTION: GLenum = 36430;
pub const GL_PROVOKING_VERTEX: GLenum = 36431;
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 34895;
pub const GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = 37137;
pub const GL_OBJECT_TYPE: GLenum = 37138;
pub const GL_SYNC_CONDITION: GLenum = 37139;
pub const GL_SYNC_STATUS: GLenum = 37140;
pub const GL_SYNC_FLAGS: GLenum = 37141;
pub const GL_SYNC_FENCE: GLenum = 37142;
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = 37143;
pub const GL_UNSIGNALED: GLenum = 37144;
pub const GL_SIGNALED: GLenum = 37145;
pub const GL_ALREADY_SIGNALED: GLenum = 37146;
pub const GL_TIMEOUT_EXPIRED: GLenum = 37147;
pub const GL_CONDITION_SATISFIED: GLenum = 37148;
pub const GL_WAIT_FAILED: GLenum = 37149;
pub const GL_SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x1;
pub const GL_SAMPLE_POSITION: GLenum = 36432;
pub const GL_SAMPLE_MASK: GLenum = 36433;
pub const GL_SAMPLE_MASK_VALUE: GLenum = 36434;
pub const GL_MAX_SAMPLE_MASK_WORDS: GLenum = 36441;
pub const GL_TEXTURE_2D_MULTISAMPLE: GLenum = 37120;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 37121;
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 37122;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 37123;
pub const GL_TEXTURE_SAMPLES: GLenum = 37126;
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 37127;
pub const GL_SAMPLER_2D_MULTISAMPLE: GLenum = 37128;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 37129;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 37130;
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 37131;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 37132;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 37133;
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GLenum = 37134;
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 37135;
pub const GL_MAX_INTEGER_SAMPLES: GLenum = 37136;
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 35070;
pub const GL_SRC1_COLOR: GLenum = 35065;
pub const GL_ONE_MINUS_SRC1_COLOR: GLenum = 35066;
pub const GL_ONE_MINUS_SRC1_ALPHA: GLenum = 35067;
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 35068;
pub const GL_ANY_SAMPLES_PASSED: GLenum = 35887;
pub const GL_SAMPLER_BINDING: GLenum = 35097;
pub const GL_RGB10_A2UI: GLenum = 36975;
pub const GL_TEXTURE_SWIZZLE_R: GLenum = 36418;
pub const GL_TEXTURE_SWIZZLE_G: GLenum = 36419;
pub const GL_TEXTURE_SWIZZLE_B: GLenum = 36420;
pub const GL_TEXTURE_SWIZZLE_A: GLenum = 36421;
pub const GL_TEXTURE_SWIZZLE_RGBA: GLenum = 36422;
pub const GL_TIME_ELAPSED: GLenum = 35007;
pub const GL_TIMESTAMP: GLenum = 36392;
pub const GL_INT_2_10_10_10_REV: GLenum = 36255;
pub const GL_SAMPLE_SHADING: GLenum = 35894;
pub const GL_MIN_SAMPLE_SHADING_VALUE: GLenum = 35895;
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 36446;
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 36447;
pub const GL_TEXTURE_CUBE_MAP_ARRAY: GLenum = 36873;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 36875;
pub const GL_SAMPLER_CUBE_MAP_ARRAY: GLenum = 36876;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 36877;
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 36878;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 36879;
pub const GL_DRAW_INDIRECT_BUFFER: GLenum = 36671;
pub const GL_DRAW_INDIRECT_BUFFER_BINDING: GLenum = 36675;
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 36442;
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 36443;
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 36444;
pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
pub const GL_DOUBLE_VEC2: GLenum = 36860;
pub const GL_DOUBLE_VEC3: GLenum = 36861;
pub const GL_DOUBLE_VEC4: GLenum = 36862;
pub const GL_DOUBLE_MAT2: GLenum = 36678;
pub const GL_DOUBLE_MAT3: GLenum = 36679;
pub const GL_DOUBLE_MAT4: GLenum = 36680;
pub const GL_DOUBLE_MAT2x3: GLenum = 36681;
pub const GL_DOUBLE_MAT2x4: GLenum = 36682;
pub const GL_DOUBLE_MAT3x2: GLenum = 36683;
pub const GL_DOUBLE_MAT3x4: GLenum = 36684;
pub const GL_DOUBLE_MAT4x2: GLenum = 36685;
pub const GL_DOUBLE_MAT4x3: GLenum = 36686;
pub const GL_ACTIVE_SUBROUTINES: GLenum = 36325;
pub const GL_ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 36326;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 36423;
pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 36424;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 36425;
pub const GL_MAX_SUBROUTINES: GLenum = 36327;
pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 36328;
pub const GL_NUM_COMPATIBLE_SUBROUTINES: GLenum = 36426;
pub const GL_COMPATIBLE_SUBROUTINES: GLenum = 36427;
pub const GL_PATCHES: GLenum = 14;
pub const GL_PATCH_VERTICES: GLenum = 36466;
pub const GL_PATCH_DEFAULT_INNER_LEVEL: GLenum = 36467;
pub const GL_PATCH_DEFAULT_OUTER_LEVEL: GLenum = 36468;
pub const GL_TESS_CONTROL_OUTPUT_VERTICES: GLenum = 36469;
pub const GL_TESS_GEN_MODE: GLenum = 36470;
pub const GL_TESS_GEN_SPACING: GLenum = 36471;
pub const GL_TESS_GEN_VERTEX_ORDER: GLenum = 36472;
pub const GL_TESS_GEN_POINT_MODE: GLenum = 36473;
pub const GL_ISOLINES: GLenum = 36474;
pub const GL_QUADS: GLenum = 7;
pub const GL_FRACTIONAL_ODD: GLenum = 36475;
pub const GL_FRACTIONAL_EVEN: GLenum = 36476;
pub const GL_MAX_PATCH_VERTICES: GLenum = 36477;
pub const GL_MAX_TESS_GEN_LEVEL: GLenum = 36478;
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 36479;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 36480;
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 36481;
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 36482;
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 36483;
pub const GL_MAX_TESS_PATCH_COMPONENTS: GLenum = 36484;
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 36485;
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 36486;
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 36489;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 36490;
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 34924;
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 34925;
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 36382;
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 36383;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 34032;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 34033;
pub const GL_TESS_EVALUATION_SHADER: GLenum = 36487;
pub const GL_TESS_CONTROL_SHADER: GLenum = 36488;
pub const GL_TRANSFORM_FEEDBACK: GLenum = 36386;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 36387;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 36388;
pub const GL_TRANSFORM_FEEDBACK_BINDING: GLenum = 36389;
pub const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 36464;
pub const GL_MAX_VERTEX_STREAMS: GLenum = 36465;
pub const GL_FIXED: GLenum = 5132;
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 35738;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 35739;
pub const GL_LOW_FLOAT: GLenum = 36336;
pub const GL_MEDIUM_FLOAT: GLenum = 36337;
pub const GL_HIGH_FLOAT: GLenum = 36338;
pub const GL_LOW_INT: GLenum = 36339;
pub const GL_MEDIUM_INT: GLenum = 36340;
pub const GL_HIGH_INT: GLenum = 36341;
pub const GL_SHADER_COMPILER: GLenum = 36346;
pub const GL_SHADER_BINARY_FORMATS: GLenum = 36344;
pub const GL_NUM_SHADER_BINARY_FORMATS: GLenum = 36345;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = 36347;
pub const GL_MAX_VARYING_VECTORS: GLenum = 36348;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 36349;
pub const GL_RGB565: GLenum = 36194;
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 33367;
pub const GL_PROGRAM_BINARY_LENGTH: GLenum = 34625;
pub const GL_NUM_PROGRAM_BINARY_FORMATS: GLenum = 34814;
pub const GL_PROGRAM_BINARY_FORMATS: GLenum = 34815;
pub const GL_VERTEX_SHADER_BIT: GLenum = 0x1;
pub const GL_FRAGMENT_SHADER_BIT: GLenum = 0x2;
pub const GL_GEOMETRY_SHADER_BIT: GLenum = 0x4;
pub const GL_TESS_CONTROL_SHADER_BIT: GLenum = 0x8;
pub const GL_TESS_EVALUATION_SHADER_BIT: GLenum = 0x10;
pub const GL_ALL_SHADER_BITS: GLenum = 0xFFFFFFFF;
pub const GL_PROGRAM_SEPARABLE: GLenum = 33368;
pub const GL_ACTIVE_PROGRAM: GLenum = 33369;
pub const GL_PROGRAM_PIPELINE_BINDING: GLenum = 33370;
pub const GL_MAX_VIEWPORTS: GLenum = 33371;
pub const GL_VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
pub const GL_VIEWPORT_BOUNDS_RANGE: GLenum = 33373;
pub const GL_LAYER_PROVOKING_VERTEX: GLenum = 33374;
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 33375;
pub const GL_UNDEFINED_VERTEX: GLenum = 33376;
pub const GL_COPY_READ_BUFFER_BINDING: GLenum = 36662;
pub const GL_COPY_WRITE_BUFFER_BINDING: GLenum = 36663;
pub const GL_TRANSFORM_FEEDBACK_ACTIVE: GLenum = 36388;
pub const GL_TRANSFORM_FEEDBACK_PAUSED: GLenum = 36387;
pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 37159;
pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 37160;
pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 37161;
pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 37162;
pub const GL_PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 37163;
pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 37164;
pub const GL_PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 37165;
pub const GL_PACK_COMPRESSED_BLOCK_SIZE: GLenum = 37166;
pub const GL_NUM_SAMPLE_COUNTS: GLenum = 37760;
pub const GL_MIN_MAP_BUFFER_ALIGNMENT: GLenum = 37052;
pub const GL_ATOMIC_COUNTER_BUFFER: GLenum = 37568;
pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 37569;
pub const GL_ATOMIC_COUNTER_BUFFER_START: GLenum = 37570;
pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 37571;
pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 37572;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 37573;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 37574;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 37575;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 37576;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 37577;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 37578;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 37579;
pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 37580;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 37581;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 37582;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 37583;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 37584;
pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 37585;
pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 37586;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 37587;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 37588;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 37589;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 37590;
pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 37591;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 37592;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 37596;
pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 37593;
pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 37594;
pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 37595;
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLenum = 0x1;
pub const GL_ELEMENT_ARRAY_BARRIER_BIT: GLenum = 0x2;
pub const GL_UNIFORM_BARRIER_BIT: GLenum = 0x4;
pub const GL_TEXTURE_FETCH_BARRIER_BIT: GLenum = 0x8;
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: GLenum = 0x20;
pub const GL_COMMAND_BARRIER_BIT: GLenum = 0x40;
pub const GL_PIXEL_BUFFER_BARRIER_BIT: GLenum = 0x80;
pub const GL_TEXTURE_UPDATE_BARRIER_BIT: GLenum = 0x100;
pub const GL_BUFFER_UPDATE_BARRIER_BIT: GLenum = 0x200;
pub const GL_FRAMEBUFFER_BARRIER_BIT: GLenum = 0x400;
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: GLenum = 0x800;
pub const GL_ATOMIC_COUNTER_BARRIER_BIT: GLenum = 0x1000;
pub const GL_ALL_BARRIER_BITS: GLenum = 0xFFFFFFFF;
pub const GL_MAX_IMAGE_UNITS: GLenum = 36664;
pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 36665;
pub const GL_IMAGE_BINDING_NAME: GLenum = 36666;
pub const GL_IMAGE_BINDING_LEVEL: GLenum = 36667;
pub const GL_IMAGE_BINDING_LAYERED: GLenum = 36668;
pub const GL_IMAGE_BINDING_LAYER: GLenum = 36669;
pub const GL_IMAGE_BINDING_ACCESS: GLenum = 36670;
pub const GL_IMAGE_1D: GLenum = 36940;
pub const GL_IMAGE_2D: GLenum = 36941;
pub const GL_IMAGE_3D: GLenum = 36942;
pub const GL_IMAGE_2D_RECT: GLenum = 36943;
pub const GL_IMAGE_CUBE: GLenum = 36944;
pub const GL_IMAGE_BUFFER: GLenum = 36945;
pub const GL_IMAGE_1D_ARRAY: GLenum = 36946;
pub const GL_IMAGE_2D_ARRAY: GLenum = 36947;
pub const GL_IMAGE_CUBE_MAP_ARRAY: GLenum = 36948;
pub const GL_IMAGE_2D_MULTISAMPLE: GLenum = 36949;
pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 36950;
pub const GL_INT_IMAGE_1D: GLenum = 36951;
pub const GL_INT_IMAGE_2D: GLenum = 36952;
pub const GL_INT_IMAGE_3D: GLenum = 36953;
pub const GL_INT_IMAGE_2D_RECT: GLenum = 36954;
pub const GL_INT_IMAGE_CUBE: GLenum = 36955;
pub const GL_INT_IMAGE_BUFFER: GLenum = 36956;
pub const GL_INT_IMAGE_1D_ARRAY: GLenum = 36957;
pub const GL_INT_IMAGE_2D_ARRAY: GLenum = 36958;
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 36959;
pub const GL_INT_IMAGE_2D_MULTISAMPLE: GLenum = 36960;
pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 36961;
pub const GL_UNSIGNED_INT_IMAGE_1D: GLenum = 36962;
pub const GL_UNSIGNED_INT_IMAGE_2D: GLenum = 36963;
pub const GL_UNSIGNED_INT_IMAGE_3D: GLenum = 36964;
pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 36965;
pub const GL_UNSIGNED_INT_IMAGE_CUBE: GLenum = 36966;
pub const GL_UNSIGNED_INT_IMAGE_BUFFER: GLenum = 36967;
pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 36968;
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 36969;
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 36970;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 36971;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 36972;
pub const GL_MAX_IMAGE_SAMPLES: GLenum = 36973;
pub const GL_IMAGE_BINDING_FORMAT: GLenum = 36974;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 37063;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 37064;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 37065;
pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 37066;
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 37067;
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 37068;
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 37069;
pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 37070;
pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 37071;
pub const GL_COMPRESSED_RGBA_BPTC_UNORM: GLenum = 36492;
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = 36493;
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = 36494;
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = 36495;
pub const GL_TEXTURE_IMMUTABLE_FORMAT: GLenum = 37167;
pub const GL_NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 33513;
pub const GL_VERTEX_ATTRIB_ARRAY_LONG: GLenum = 34638;
pub const GL_COMPRESSED_RGB8_ETC2: GLenum = 37492;
pub const GL_COMPRESSED_SRGB8_ETC2: GLenum = 37493;
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 37494;
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 37495;
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: GLenum = 37496;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 37497;
pub const GL_COMPRESSED_R11_EAC: GLenum = 37488;
pub const GL_COMPRESSED_SIGNED_R11_EAC: GLenum = 37489;
pub const GL_COMPRESSED_RG11_EAC: GLenum = 37490;
pub const GL_COMPRESSED_SIGNED_RG11_EAC: GLenum = 37491;
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 36201;
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 36202;
pub const GL_MAX_ELEMENT_INDEX: GLenum = 36203;
pub const GL_COMPUTE_SHADER: GLenum = 37305;
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 37307;
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 37308;
pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 37309;
pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 33378;
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 33379;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 33380;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 33381;
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 33382;
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 37099;
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 37310;
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 37311;
pub const GL_COMPUTE_WORK_GROUP_SIZE: GLenum = 33383;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 37100;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 37101;
pub const GL_DISPATCH_INDIRECT_BUFFER: GLenum = 37102;
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 37103;
pub const GL_COMPUTE_SHADER_BIT: GLenum = 0x20;
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 33346;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 33347;
pub const GL_DEBUG_CALLBACK_FUNCTION: GLenum = 33348;
pub const GL_DEBUG_CALLBACK_USER_PARAM: GLenum = 33349;
pub const GL_DEBUG_SOURCE_API: GLenum = 33350;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 33351;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = 33352;
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = 33353;
pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = 33354;
pub const GL_DEBUG_SOURCE_OTHER: GLenum = 33355;
pub const GL_DEBUG_TYPE_ERROR: GLenum = 33356;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 33357;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 33358;
pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = 33359;
pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = 33360;
pub const GL_DEBUG_TYPE_OTHER: GLenum = 33361;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GLenum = 37187;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GLenum = 37188;
pub const GL_DEBUG_LOGGED_MESSAGES: GLenum = 37189;
pub const GL_DEBUG_SEVERITY_HIGH: GLenum = 37190;
pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = 37191;
pub const GL_DEBUG_SEVERITY_LOW: GLenum = 37192;
pub const GL_DEBUG_TYPE_MARKER: GLenum = 33384;
pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = 33385;
pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = 33386;
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = 33387;
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 33388;
pub const GL_DEBUG_GROUP_STACK_DEPTH: GLenum = 33389;
pub const GL_BUFFER: GLenum = 33504;
pub const GL_SHADER: GLenum = 33505;
pub const GL_PROGRAM: GLenum = 33506;
pub const GL_VERTEX_ARRAY: GLenum = 32884;
pub const GL_QUERY: GLenum = 33507;
pub const GL_PROGRAM_PIPELINE: GLenum = 33508;
pub const GL_SAMPLER: GLenum = 33510;
pub const GL_MAX_LABEL_LENGTH: GLenum = 33512;
pub const GL_DEBUG_OUTPUT: GLenum = 37600;
pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLenum = 0x2;
pub const GL_MAX_UNIFORM_LOCATIONS: GLenum = 33390;
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 37648;
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 37649;
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 37650;
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 37651;
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 37652;
pub const GL_MAX_FRAMEBUFFER_WIDTH: GLenum = 37653;
pub const GL_MAX_FRAMEBUFFER_HEIGHT: GLenum = 37654;
pub const GL_MAX_FRAMEBUFFER_LAYERS: GLenum = 37655;
pub const GL_MAX_FRAMEBUFFER_SAMPLES: GLenum = 37656;
pub const GL_INTERNALFORMAT_SUPPORTED: GLenum = 33391;
pub const GL_INTERNALFORMAT_PREFERRED: GLenum = 33392;
pub const GL_INTERNALFORMAT_RED_SIZE: GLenum = 33393;
pub const GL_INTERNALFORMAT_GREEN_SIZE: GLenum = 33394;
pub const GL_INTERNALFORMAT_BLUE_SIZE: GLenum = 33395;
pub const GL_INTERNALFORMAT_ALPHA_SIZE: GLenum = 33396;
pub const GL_INTERNALFORMAT_DEPTH_SIZE: GLenum = 33397;
pub const GL_INTERNALFORMAT_STENCIL_SIZE: GLenum = 33398;
pub const GL_INTERNALFORMAT_SHARED_SIZE: GLenum = 33399;
pub const GL_INTERNALFORMAT_RED_TYPE: GLenum = 33400;
pub const GL_INTERNALFORMAT_GREEN_TYPE: GLenum = 33401;
pub const GL_INTERNALFORMAT_BLUE_TYPE: GLenum = 33402;
pub const GL_INTERNALFORMAT_ALPHA_TYPE: GLenum = 33403;
pub const GL_INTERNALFORMAT_DEPTH_TYPE: GLenum = 33404;
pub const GL_INTERNALFORMAT_STENCIL_TYPE: GLenum = 33405;
pub const GL_MAX_WIDTH: GLenum = 33406;
pub const GL_MAX_HEIGHT: GLenum = 33407;
pub const GL_MAX_DEPTH: GLenum = 33408;
pub const GL_MAX_LAYERS: GLenum = 33409;
pub const GL_MAX_COMBINED_DIMENSIONS: GLenum = 33410;
pub const GL_COLOR_COMPONENTS: GLenum = 33411;
pub const GL_DEPTH_COMPONENTS: GLenum = 33412;
pub const GL_STENCIL_COMPONENTS: GLenum = 33413;
pub const GL_COLOR_RENDERABLE: GLenum = 33414;
pub const GL_DEPTH_RENDERABLE: GLenum = 33415;
pub const GL_STENCIL_RENDERABLE: GLenum = 33416;
pub const GL_FRAMEBUFFER_RENDERABLE: GLenum = 33417;
pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 33418;
pub const GL_FRAMEBUFFER_BLEND: GLenum = 33419;
pub const GL_READ_PIXELS: GLenum = 33420;
pub const GL_READ_PIXELS_FORMAT: GLenum = 33421;
pub const GL_READ_PIXELS_TYPE: GLenum = 33422;
pub const GL_TEXTURE_IMAGE_FORMAT: GLenum = 33423;
pub const GL_TEXTURE_IMAGE_TYPE: GLenum = 33424;
pub const GL_GET_TEXTURE_IMAGE_FORMAT: GLenum = 33425;
pub const GL_GET_TEXTURE_IMAGE_TYPE: GLenum = 33426;
pub const GL_MIPMAP: GLenum = 33427;
pub const GL_MANUAL_GENERATE_MIPMAP: GLenum = 33428;
pub const GL_AUTO_GENERATE_MIPMAP: GLenum = 33429;
pub const GL_COLOR_ENCODING: GLenum = 33430;
pub const GL_SRGB_READ: GLenum = 33431;
pub const GL_SRGB_WRITE: GLenum = 33432;
pub const GL_FILTER: GLenum = 33434;
pub const GL_VERTEX_TEXTURE: GLenum = 33435;
pub const GL_TESS_CONTROL_TEXTURE: GLenum = 33436;
pub const GL_TESS_EVALUATION_TEXTURE: GLenum = 33437;
pub const GL_GEOMETRY_TEXTURE: GLenum = 33438;
pub const GL_FRAGMENT_TEXTURE: GLenum = 33439;
pub const GL_COMPUTE_TEXTURE: GLenum = 33440;
pub const GL_TEXTURE_SHADOW: GLenum = 33441;
pub const GL_TEXTURE_GATHER: GLenum = 33442;
pub const GL_TEXTURE_GATHER_SHADOW: GLenum = 33443;
pub const GL_SHADER_IMAGE_LOAD: GLenum = 33444;
pub const GL_SHADER_IMAGE_STORE: GLenum = 33445;
pub const GL_SHADER_IMAGE_ATOMIC: GLenum = 33446;
pub const GL_IMAGE_TEXEL_SIZE: GLenum = 33447;
pub const GL_IMAGE_COMPATIBILITY_CLASS: GLenum = 33448;
pub const GL_IMAGE_PIXEL_FORMAT: GLenum = 33449;
pub const GL_IMAGE_PIXEL_TYPE: GLenum = 33450;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 33452;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 33453;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 33454;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 33455;
pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 33457;
pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 33458;
pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 33459;
pub const GL_CLEAR_BUFFER: GLenum = 33460;
pub const GL_TEXTURE_VIEW: GLenum = 33461;
pub const GL_VIEW_COMPATIBILITY_CLASS: GLenum = 33462;
pub const GL_FULL_SUPPORT: GLenum = 33463;
pub const GL_CAVEAT_SUPPORT: GLenum = 33464;
pub const GL_IMAGE_CLASS_4_X_32: GLenum = 33465;
pub const GL_IMAGE_CLASS_2_X_32: GLenum = 33466;
pub const GL_IMAGE_CLASS_1_X_32: GLenum = 33467;
pub const GL_IMAGE_CLASS_4_X_16: GLenum = 33468;
pub const GL_IMAGE_CLASS_2_X_16: GLenum = 33469;
pub const GL_IMAGE_CLASS_1_X_16: GLenum = 33470;
pub const GL_IMAGE_CLASS_4_X_8: GLenum = 33471;
pub const GL_IMAGE_CLASS_2_X_8: GLenum = 33472;
pub const GL_IMAGE_CLASS_1_X_8: GLenum = 33473;
pub const GL_IMAGE_CLASS_11_11_10: GLenum = 33474;
pub const GL_IMAGE_CLASS_10_10_10_2: GLenum = 33475;
pub const GL_VIEW_CLASS_128_BITS: GLenum = 0x82C4;
pub const GL_VIEW_CLASS_96_BITS: GLenum = 0x82C5;
pub const GL_VIEW_CLASS_64_BITS: GLenum = 0x82C6;
pub const GL_VIEW_CLASS_48_BITS: GLenum = 0x82C7;
pub const GL_VIEW_CLASS_32_BITS: GLenum = 0x82C8;
pub const GL_VIEW_CLASS_24_BITS: GLenum = 0x82C9;
pub const GL_VIEW_CLASS_16_BITS: GLenum = 0x82CA;
pub const GL_VIEW_CLASS_8_BITS: GLenum = 0x82CB;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 33484;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 33485;
pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 33486;
pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 33487;
pub const GL_VIEW_CLASS_RGTC1_RED: GLenum = 33488;
pub const GL_VIEW_CLASS_RGTC2_RG: GLenum = 33489;
pub const GL_VIEW_CLASS_BPTC_UNORM: GLenum = 33490;
pub const GL_VIEW_CLASS_BPTC_FLOAT: GLenum = 33491;
pub const GL_UNIFORM: GLenum = 37601;
pub const GL_UNIFORM_BLOCK: GLenum = 37602;
pub const GL_PROGRAM_INPUT: GLenum = 37603;
pub const GL_PROGRAM_OUTPUT: GLenum = 37604;
pub const GL_BUFFER_VARIABLE: GLenum = 37605;
pub const GL_SHADER_STORAGE_BLOCK: GLenum = 37606;
pub const GL_VERTEX_SUBROUTINE: GLenum = 37608;
pub const GL_TESS_CONTROL_SUBROUTINE: GLenum = 37609;
pub const GL_TESS_EVALUATION_SUBROUTINE: GLenum = 37610;
pub const GL_GEOMETRY_SUBROUTINE: GLenum = 37611;
pub const GL_FRAGMENT_SUBROUTINE: GLenum = 37612;
pub const GL_COMPUTE_SUBROUTINE: GLenum = 37613;
pub const GL_VERTEX_SUBROUTINE_UNIFORM: GLenum = 37614;
pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 37615;
pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 37616;
pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 37617;
pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 37618;
pub const GL_COMPUTE_SUBROUTINE_UNIFORM: GLenum = 37619;
pub const GL_TRANSFORM_FEEDBACK_VARYING: GLenum = 37620;
pub const GL_ACTIVE_RESOURCES: GLenum = 37621;
pub const GL_MAX_NAME_LENGTH: GLenum = 37622;
pub const GL_MAX_NUM_ACTIVE_VARIABLES: GLenum = 37623;
pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 37624;
pub const GL_NAME_LENGTH: GLenum = 37625;
pub const GL_TYPE: GLenum = 37626;
pub const GL_ARRAY_SIZE: GLenum = 37627;
pub const GL_OFFSET: GLenum = 37628;
pub const GL_BLOCK_INDEX: GLenum = 37629;
pub const GL_ARRAY_STRIDE: GLenum = 37630;
pub const GL_MATRIX_STRIDE: GLenum = 37631;
pub const GL_IS_ROW_MAJOR: GLenum = 37632;
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 37633;
pub const GL_BUFFER_BINDING: GLenum = 37634;
pub const GL_BUFFER_DATA_SIZE: GLenum = 37635;
pub const GL_NUM_ACTIVE_VARIABLES: GLenum = 37636;
pub const GL_ACTIVE_VARIABLES: GLenum = 37637;
pub const GL_REFERENCED_BY_VERTEX_SHADER: GLenum = 37638;
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 37639;
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 37640;
pub const GL_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 37641;
pub const GL_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 37642;
pub const GL_REFERENCED_BY_COMPUTE_SHADER: GLenum = 37643;
pub const GL_TOP_LEVEL_ARRAY_SIZE: GLenum = 37644;
pub const GL_TOP_LEVEL_ARRAY_STRIDE: GLenum = 37645;
pub const GL_LOCATION: GLenum = 37646;
pub const GL_LOCATION_INDEX: GLenum = 37647;
pub const GL_IS_PER_PATCH: GLenum = 37607;
pub const GL_SHADER_STORAGE_BUFFER: GLenum = 37074;
pub const GL_SHADER_STORAGE_BUFFER_BINDING: GLenum = 37075;
pub const GL_SHADER_STORAGE_BUFFER_START: GLenum = 37076;
pub const GL_SHADER_STORAGE_BUFFER_SIZE: GLenum = 37077;
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 37078;
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 37079;
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 37080;
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 37081;
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 37082;
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 37083;
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 37084;
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 37085;
pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 37086;
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 37087;
pub const GL_SHADER_STORAGE_BARRIER_BIT: GLenum = 0x2000;
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 36665;
pub const GL_DEPTH_STENCIL_TEXTURE_MODE: GLenum = 37098;
pub const GL_TEXTURE_BUFFER_OFFSET: GLenum = 37277;
pub const GL_TEXTURE_BUFFER_SIZE: GLenum = 37278;
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 37279;
pub const GL_TEXTURE_VIEW_MIN_LEVEL: GLenum = 33499;
pub const GL_TEXTURE_VIEW_NUM_LEVELS: GLenum = 33500;
pub const GL_TEXTURE_VIEW_MIN_LAYER: GLenum = 33501;
pub const GL_TEXTURE_VIEW_NUM_LAYERS: GLenum = 33502;
pub const GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = 33503;
pub const GL_VERTEX_ATTRIB_BINDING: GLenum = 33492;
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 33493;
pub const GL_VERTEX_BINDING_DIVISOR: GLenum = 33494;
pub const GL_VERTEX_BINDING_OFFSET: GLenum = 33495;
pub const GL_VERTEX_BINDING_STRIDE: GLenum = 33496;
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 33497;
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 33498;
pub const GL_VERTEX_BINDING_BUFFER: GLenum = 36687;
pub const GL_STACK_UNDERFLOW: GLenum = 1284;
pub const GL_STACK_OVERFLOW: GLenum = 1283;
pub const GL_MAX_VERTEX_ATTRIB_STRIDE: GLenum = 33509;
pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 33313;
pub const GL_TEXTURE_BUFFER_BINDING: GLenum = 35882;
pub const GL_MAP_READ_BIT: GLenum = 0x1;
pub const GL_MAP_WRITE_BIT: GLenum = 0x2;
pub const GL_MAP_PERSISTENT_BIT: GLenum = 0x40;
pub const GL_MAP_COHERENT_BIT: GLenum = 0x80;
pub const GL_DYNAMIC_STORAGE_BIT: GLenum = 0x100;
pub const GL_CLIENT_STORAGE_BIT: GLenum = 0x200;
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLenum = 0x4000;
pub const GL_BUFFER_IMMUTABLE_STORAGE: GLenum = 33311;
pub const GL_BUFFER_STORAGE_FLAGS: GLenum = 33312;
pub const GL_CLEAR_TEXTURE: GLenum = 37733;
pub const GL_LOCATION_COMPONENT: GLenum = 37706;
pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = 35982;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = 37707;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = 37708;
pub const GL_QUERY_BUFFER: GLenum = 37266;
pub const GL_QUERY_BUFFER_BARRIER_BIT: GLenum = 0x8000;
pub const GL_QUERY_BUFFER_BINDING: GLenum = 37267;
pub const GL_QUERY_RESULT_NO_WAIT: GLenum = 37268;
pub const GL_MIRROR_CLAMP_TO_EDGE: GLenum = 34627;
pub const GL_STENCIL_INDEX: GLenum = 6401;
pub const GL_STENCIL_INDEX8: GLenum = 36168;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: GLenum = 35899;
pub const GL_LOWER_LEFT: GLenum = 36001;
pub const GL_UPPER_LEFT: GLenum = 36002;
pub const GL_NEGATIVE_ONE_TO_ONE: GLenum = 37726;
pub const GL_ZERO_TO_ONE: GLenum = 37727;
pub const GL_CLIP_ORIGIN: GLenum = 37724;
pub const GL_CLIP_DEPTH_MODE: GLenum = 37725;
pub const GL_QUERY_WAIT_INVERTED: GLenum = 36375;
pub const GL_QUERY_NO_WAIT_INVERTED: GLenum = 36376;
pub const GL_QUERY_BY_REGION_WAIT_INVERTED: GLenum = 36377;
pub const GL_QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = 36378;
pub const GL_MAX_CULL_DISTANCES: GLenum = 33529;
pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = 33530;
pub const GL_TEXTURE_TARGET: GLenum = 4102;
pub const GL_QUERY_TARGET: GLenum = 33514;
pub const GL_TEXTURE_BINDING_1D: GLenum = 32872;
pub const GL_TEXTURE_BINDING_1D_ARRAY: GLenum = 35868;
pub const GL_TEXTURE_BINDING_2D: GLenum = 32873;
pub const GL_TEXTURE_BINDING_2D_ARRAY: GLenum = 35869;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 37124;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 37125;
pub const GL_TEXTURE_BINDING_3D: GLenum = 32874;
pub const GL_TEXTURE_BINDING_BUFFER: GLenum = 35884;
pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = 34068;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 36874;
pub const GL_TEXTURE_BINDING_RECTANGLE: GLenum = 34038;
pub const GL_BACK: GLenum = 1029;
pub const GL_NO_ERROR: GLenum = 0;
pub const GL_GUILTY_CONTEXT_RESET: GLenum = 33363;
pub const GL_INNOCENT_CONTEXT_RESET: GLenum = 33364;
pub const GL_UNKNOWN_CONTEXT_RESET: GLenum = 33365;
pub const GL_RESET_NOTIFICATION_STRATEGY: GLenum = 33366;
pub const GL_LOSE_CONTEXT_ON_RESET: GLenum = 33362;
pub const GL_NO_RESET_NOTIFICATION: GLenum = 33377;
pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLenum = 0x4;
pub const GL_CONTEXT_LOST: GLenum = 1287;
pub const GL_CONTEXT_RELEASE_BEHAVIOR: GLenum = 33531;
pub const GL_NONE: GLenum = 0;
pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = 33532;
pub const GL_SHADER_BINARY_FORMAT_SPIR_V: GLenum = 38225;
pub const GL_SPIR_V_BINARY: GLenum = 38226;
pub const GL_PARAMETER_BUFFER: GLenum = 33006;
pub const GL_PARAMETER_BUFFER_BINDING: GLenum = 33007;
pub const GL_CONTEXT_FLAG_NO_ERROR_BIT: GLenum = 0x8;
pub const GL_VERTICES_SUBMITTED: GLenum = 33518;
pub const GL_PRIMITIVES_SUBMITTED: GLenum = 33519;
pub const GL_VERTEX_SHADER_INVOCATIONS: GLenum = 33520;
pub const GL_TESS_CONTROL_SHADER_PATCHES: GLenum = 33521;
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS: GLenum = 33522;
pub const GL_GEOMETRY_SHADER_INVOCATIONS: GLenum = 34943;
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED: GLenum = 33523;
pub const GL_FRAGMENT_SHADER_INVOCATIONS: GLenum = 33524;
pub const GL_COMPUTE_SHADER_INVOCATIONS: GLenum = 33525;
pub const GL_CLIPPING_INPUT_PRIMITIVES: GLenum = 33526;
pub const GL_CLIPPING_OUTPUT_PRIMITIVES: GLenum = 33527;
pub const GL_POLYGON_OFFSET_CLAMP: GLenum = 36379;
pub const GL_SPIR_V_EXTENSIONS: GLenum = 38227;
pub const GL_NUM_SPIR_V_EXTENSIONS: GLenum = 38228;
pub const GL_TEXTURE_MAX_ANISOTROPY: GLenum = 34046;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: GLenum = 34047;
pub const GL_TRANSFORM_FEEDBACK_OVERFLOW: GLenum = 33516;
pub const GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW: GLenum = 33517;
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum TextureTarget {
    Texture1D = GL_TEXTURE_1D,
    Texture2D = GL_TEXTURE_2D,
    ProxyTexture1D = GL_PROXY_TEXTURE_1D,
    ProxyTexture2D = GL_PROXY_TEXTURE_2D,
    Texture3D = GL_TEXTURE_3D,
    ProxyTexture3D = GL_PROXY_TEXTURE_3D,
    TextureCubeMap = GL_TEXTURE_CUBE_MAP,
    TextureCubeMapPositiveX = GL_TEXTURE_CUBE_MAP_POSITIVE_X,
    TextureCubeMapNegativeX = GL_TEXTURE_CUBE_MAP_NEGATIVE_X,
    TextureCubeMapPositiveY = GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
    TextureCubeMapNegativeY = GL_TEXTURE_CUBE_MAP_NEGATIVE_Y,
    TextureCubeMapPositiveZ = GL_TEXTURE_CUBE_MAP_POSITIVE_Z,
    TextureCubeMapNegativeZ = GL_TEXTURE_CUBE_MAP_NEGATIVE_Z,
    ProxyTextureCubeMap = GL_PROXY_TEXTURE_CUBE_MAP,
    Texture1DArray = GL_TEXTURE_1D_ARRAY,
    ProxyTexture1DArray = GL_PROXY_TEXTURE_1D_ARRAY,
    Texture2DArray = GL_TEXTURE_2D_ARRAY,
    ProxyTexture2DArray = GL_PROXY_TEXTURE_2D_ARRAY,
    Renderbuffer = GL_RENDERBUFFER,
    TextureBuffer = GL_TEXTURE_BUFFER,
    TextureRectangle = GL_TEXTURE_RECTANGLE,
    ProxyTextureRectangle = GL_PROXY_TEXTURE_RECTANGLE,
    Texture2DMultisample = GL_TEXTURE_2D_MULTISAMPLE,
    ProxyTexture2DMultisample = GL_PROXY_TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = GL_TEXTURE_2D_MULTISAMPLE_ARRAY,
    ProxyTexture2DMultisampleArray = GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY,
    TextureCubeMapArray = GL_TEXTURE_CUBE_MAP_ARRAY,
    ProxyTextureCubeMapArray = GL_PROXY_TEXTURE_CUBE_MAP_ARRAY,
}
impl UnsafeFromGLenum for TextureTarget {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = TextureTarget::from_repr(val) else {
            println!("Attempt to create a TextureTarget from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<TextureTarget> for u32 {
    fn from(value: TextureTarget) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for TextureTarget {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum FrontFaceDirection {
    Cw = GL_CW,
    Ccw = GL_CCW,
}
impl UnsafeFromGLenum for FrontFaceDirection {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = FrontFaceDirection::from_repr(val) else {
            println!(
                "Attempt to create a FrontFaceDirection from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<FrontFaceDirection> for u32 {
    fn from(value: FrontFaceDirection) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for FrontFaceDirection {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum UniformType {
    Int = GL_INT,
    UnsignedInt = GL_UNSIGNED_INT,
    Float = GL_FLOAT,
    Double = GL_DOUBLE,
    FloatVec2 = GL_FLOAT_VEC2,
    FloatVec3 = GL_FLOAT_VEC3,
    FloatVec4 = GL_FLOAT_VEC4,
    IntVec2 = GL_INT_VEC2,
    IntVec3 = GL_INT_VEC3,
    IntVec4 = GL_INT_VEC4,
    Bool = GL_BOOL,
    BoolVec2 = GL_BOOL_VEC2,
    BoolVec3 = GL_BOOL_VEC3,
    BoolVec4 = GL_BOOL_VEC4,
    FloatMat2 = GL_FLOAT_MAT2,
    FloatMat3 = GL_FLOAT_MAT3,
    FloatMat4 = GL_FLOAT_MAT4,
    Sampler1D = GL_SAMPLER_1D,
    Sampler2D = GL_SAMPLER_2D,
    Sampler3D = GL_SAMPLER_3D,
    SamplerCube = GL_SAMPLER_CUBE,
    Sampler1DShadow = GL_SAMPLER_1D_SHADOW,
    Sampler2DShadow = GL_SAMPLER_2D_SHADOW,
    FloatMat2x3 = GL_FLOAT_MAT2x3,
    FloatMat2x4 = GL_FLOAT_MAT2x4,
    FloatMat3x2 = GL_FLOAT_MAT3x2,
    FloatMat3x4 = GL_FLOAT_MAT3x4,
    FloatMat4x2 = GL_FLOAT_MAT4x2,
    FloatMat4x3 = GL_FLOAT_MAT4x3,
    Sampler1DArray = GL_SAMPLER_1D_ARRAY,
    Sampler2DArray = GL_SAMPLER_2D_ARRAY,
    Sampler1DArrayShadow = GL_SAMPLER_1D_ARRAY_SHADOW,
    Sampler2DArrayShadow = GL_SAMPLER_2D_ARRAY_SHADOW,
    SamplerCubeShadow = GL_SAMPLER_CUBE_SHADOW,
    UnsignedIntVec2 = GL_UNSIGNED_INT_VEC2,
    UnsignedIntVec3 = GL_UNSIGNED_INT_VEC3,
    UnsignedIntVec4 = GL_UNSIGNED_INT_VEC4,
    IntSampler1D = GL_INT_SAMPLER_1D,
    IntSampler2D = GL_INT_SAMPLER_2D,
    IntSampler3D = GL_INT_SAMPLER_3D,
    IntSamplerCube = GL_INT_SAMPLER_CUBE,
    IntSampler1DArray = GL_INT_SAMPLER_1D_ARRAY,
    IntSampler2DArray = GL_INT_SAMPLER_2D_ARRAY,
    UnsignedIntSampler1D = GL_UNSIGNED_INT_SAMPLER_1D,
    UnsignedIntSampler2D = GL_UNSIGNED_INT_SAMPLER_2D,
    UnsignedIntSampler3D = GL_UNSIGNED_INT_SAMPLER_3D,
    UnsignedIntSamplerCube = GL_UNSIGNED_INT_SAMPLER_CUBE,
    UnsignedIntSampler1DArray = GL_UNSIGNED_INT_SAMPLER_1D_ARRAY,
    UnsignedIntSampler2DArray = GL_UNSIGNED_INT_SAMPLER_2D_ARRAY,
    Sampler2DRect = GL_SAMPLER_2D_RECT,
    Sampler2DRectShadow = GL_SAMPLER_2D_RECT_SHADOW,
    SamplerBuffer = GL_SAMPLER_BUFFER,
    IntSampler2DRect = GL_INT_SAMPLER_2D_RECT,
    IntSamplerBuffer = GL_INT_SAMPLER_BUFFER,
    UnsignedIntSampler2DRect = GL_UNSIGNED_INT_SAMPLER_2D_RECT,
    UnsignedIntSamplerBuffer = GL_UNSIGNED_INT_SAMPLER_BUFFER,
    Sampler2DMultisample = GL_SAMPLER_2D_MULTISAMPLE,
    IntSampler2DMultisample = GL_INT_SAMPLER_2D_MULTISAMPLE,
    UnsignedIntSampler2DMultisample = GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE,
    Sampler2DMultisampleArray = GL_SAMPLER_2D_MULTISAMPLE_ARRAY,
    IntSampler2DMultisampleArray = GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY,
    UnsignedIntSampler2DMultisampleArray = GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY,
    SamplerCubeMapArray = GL_SAMPLER_CUBE_MAP_ARRAY,
    SamplerCubeMapArrayShadow = GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW,
    IntSamplerCubeMapArray = GL_INT_SAMPLER_CUBE_MAP_ARRAY,
    UnsignedIntSamplerCubeMapArray = GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY,
    DoubleVec2 = GL_DOUBLE_VEC2,
    DoubleVec3 = GL_DOUBLE_VEC3,
    DoubleVec4 = GL_DOUBLE_VEC4,
    DoubleMat2 = GL_DOUBLE_MAT2,
    DoubleMat3 = GL_DOUBLE_MAT3,
    DoubleMat4 = GL_DOUBLE_MAT4,
    DoubleMat2x3 = GL_DOUBLE_MAT2x3,
    DoubleMat2x4 = GL_DOUBLE_MAT2x4,
    DoubleMat3x2 = GL_DOUBLE_MAT3x2,
    DoubleMat3x4 = GL_DOUBLE_MAT3x4,
    DoubleMat4x2 = GL_DOUBLE_MAT4x2,
    DoubleMat4x3 = GL_DOUBLE_MAT4x3,
}
impl UnsafeFromGLenum for UniformType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = UniformType::from_repr(val) else {
            println!("Attempt to create a UniformType from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<UniformType> for u32 {
    fn from(value: UniformType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for UniformType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum PrimitiveType {
    Points = GL_POINTS,
    Lines = GL_LINES,
    LineLoop = GL_LINE_LOOP,
    LineStrip = GL_LINE_STRIP,
    Triangles = GL_TRIANGLES,
    TriangleStrip = GL_TRIANGLE_STRIP,
    TriangleFan = GL_TRIANGLE_FAN,
    LinesAdjacency = GL_LINES_ADJACENCY,
    LineStripAdjacency = GL_LINE_STRIP_ADJACENCY,
    TrianglesAdjacency = GL_TRIANGLES_ADJACENCY,
    TriangleStripAdjacency = GL_TRIANGLE_STRIP_ADJACENCY,
    Patches = GL_PATCHES,
    Quads = GL_QUADS,
}
impl UnsafeFromGLenum for PrimitiveType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = PrimitiveType::from_repr(val) else {
            println!("Attempt to create a PrimitiveType from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<PrimitiveType> for u32 {
    fn from(value: PrimitiveType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for PrimitiveType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ProgramParameterPName {
    ProgramBinaryRetrievableHint = GL_PROGRAM_BINARY_RETRIEVABLE_HINT,
    ProgramSeparable = GL_PROGRAM_SEPARABLE,
}
impl UnsafeFromGLenum for ProgramParameterPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ProgramParameterPName::from_repr(val) else {
            println!("Attempt to create a ProgramParameterPName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ProgramParameterPName> for u32 {
    fn from(value: ProgramParameterPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ProgramParameterPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum FramebufferAttachment {
    DepthStencilAttachment = GL_DEPTH_STENCIL_ATTACHMENT,
    ColorAttachment0 = GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = GL_COLOR_ATTACHMENT1,
    ColorAttachment2 = GL_COLOR_ATTACHMENT2,
    ColorAttachment3 = GL_COLOR_ATTACHMENT3,
    ColorAttachment4 = GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = GL_COLOR_ATTACHMENT9,
    ColorAttachment10 = GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = GL_COLOR_ATTACHMENT15,
    ColorAttachment16 = GL_COLOR_ATTACHMENT16,
    ColorAttachment17 = GL_COLOR_ATTACHMENT17,
    ColorAttachment18 = GL_COLOR_ATTACHMENT18,
    ColorAttachment19 = GL_COLOR_ATTACHMENT19,
    ColorAttachment20 = GL_COLOR_ATTACHMENT20,
    ColorAttachment21 = GL_COLOR_ATTACHMENT21,
    ColorAttachment22 = GL_COLOR_ATTACHMENT22,
    ColorAttachment23 = GL_COLOR_ATTACHMENT23,
    ColorAttachment24 = GL_COLOR_ATTACHMENT24,
    ColorAttachment25 = GL_COLOR_ATTACHMENT25,
    ColorAttachment26 = GL_COLOR_ATTACHMENT26,
    ColorAttachment27 = GL_COLOR_ATTACHMENT27,
    ColorAttachment28 = GL_COLOR_ATTACHMENT28,
    ColorAttachment29 = GL_COLOR_ATTACHMENT29,
    ColorAttachment30 = GL_COLOR_ATTACHMENT30,
    ColorAttachment31 = GL_COLOR_ATTACHMENT31,
    DepthAttachment = GL_DEPTH_ATTACHMENT,
    StencilAttachment = GL_STENCIL_ATTACHMENT,
}
impl UnsafeFromGLenum for FramebufferAttachment {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = FramebufferAttachment::from_repr(val) else {
            println!("Attempt to create a FramebufferAttachment from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<FramebufferAttachment> for u32 {
    fn from(value: FramebufferAttachment) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for FramebufferAttachment {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum BufferPName {
    BufferSize = GL_BUFFER_SIZE,
    BufferUsage = GL_BUFFER_USAGE,
    BufferAccess = GL_BUFFER_ACCESS,
    BufferMapped = GL_BUFFER_MAPPED,
    BufferAccessFlags = GL_BUFFER_ACCESS_FLAGS,
    BufferMapLength = GL_BUFFER_MAP_LENGTH,
    BufferMapOffset = GL_BUFFER_MAP_OFFSET,
    BufferImmutableStorage = GL_BUFFER_IMMUTABLE_STORAGE,
    BufferStorageFlags = GL_BUFFER_STORAGE_FLAGS,
}
impl UnsafeFromGLenum for BufferPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = BufferPName::from_repr(val) else {
            println!("Attempt to create a BufferPName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<BufferPName> for u32 {
    fn from(value: BufferPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for BufferPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
bitflags! {
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct MapBufferAccessMask: u32 {
const MAP_INVALIDATE_RANGE_BIT = GL_MAP_INVALIDATE_RANGE_BIT;
const MAP_INVALIDATE_BUFFER_BIT = GL_MAP_INVALIDATE_BUFFER_BIT;
const MAP_FLUSH_EXPLICIT_BIT = GL_MAP_FLUSH_EXPLICIT_BIT;
const MAP_UNSYNCHRONIZED_BIT = GL_MAP_UNSYNCHRONIZED_BIT;
const MAP_READ_BIT = GL_MAP_READ_BIT;
const MAP_WRITE_BIT = GL_MAP_WRITE_BIT;
const MAP_PERSISTENT_BIT = GL_MAP_PERSISTENT_BIT;
const MAP_COHERENT_BIT = GL_MAP_COHERENT_BIT;
}
}
impl UnsafeFromGLenum for MapBufferAccessMask {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = MapBufferAccessMask::from_bits(val) else {
            println!("Attempt to create a MapBufferAccessMask from a GLenum with an invalid bit set! {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<MapBufferAccessMask> for u32 {
    fn from(value: MapBufferAccessMask) -> u32 {
        value.bits()
    }
}
impl<Dst: GlDstType> SrcType<Dst> for MapBufferAccessMask {
    fn cast(self) -> Dst {
        Dst::from_uint(self.bits())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum VertexProvokingMode {
    FirstVertexConvention = GL_FIRST_VERTEX_CONVENTION,
    LastVertexConvention = GL_LAST_VERTEX_CONVENTION,
}
impl UnsafeFromGLenum for VertexProvokingMode {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = VertexProvokingMode::from_repr(val) else {
            println!(
                "Attempt to create a VertexProvokingMode from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<VertexProvokingMode> for u32 {
    fn from(value: VertexProvokingMode) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for VertexProvokingMode {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum DrawBufferMode {
    FrontLeft = GL_FRONT_LEFT,
    FrontRight = GL_FRONT_RIGHT,
    BackLeft = GL_BACK_LEFT,
    BackRight = GL_BACK_RIGHT,
    Front = GL_FRONT,
    Left = GL_LEFT,
    Right = GL_RIGHT,
    FrontAndBack = GL_FRONT_AND_BACK,
    ColorAttachment0 = GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = GL_COLOR_ATTACHMENT1,
    ColorAttachment2 = GL_COLOR_ATTACHMENT2,
    ColorAttachment3 = GL_COLOR_ATTACHMENT3,
    ColorAttachment4 = GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = GL_COLOR_ATTACHMENT9,
    ColorAttachment10 = GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = GL_COLOR_ATTACHMENT15,
    ColorAttachment16 = GL_COLOR_ATTACHMENT16,
    ColorAttachment17 = GL_COLOR_ATTACHMENT17,
    ColorAttachment18 = GL_COLOR_ATTACHMENT18,
    ColorAttachment19 = GL_COLOR_ATTACHMENT19,
    ColorAttachment20 = GL_COLOR_ATTACHMENT20,
    ColorAttachment21 = GL_COLOR_ATTACHMENT21,
    ColorAttachment22 = GL_COLOR_ATTACHMENT22,
    ColorAttachment23 = GL_COLOR_ATTACHMENT23,
    ColorAttachment24 = GL_COLOR_ATTACHMENT24,
    ColorAttachment25 = GL_COLOR_ATTACHMENT25,
    ColorAttachment26 = GL_COLOR_ATTACHMENT26,
    ColorAttachment27 = GL_COLOR_ATTACHMENT27,
    ColorAttachment28 = GL_COLOR_ATTACHMENT28,
    ColorAttachment29 = GL_COLOR_ATTACHMENT29,
    ColorAttachment30 = GL_COLOR_ATTACHMENT30,
    ColorAttachment31 = GL_COLOR_ATTACHMENT31,
    Back = GL_BACK,
    None = GL_NONE,
}
impl UnsafeFromGLenum for DrawBufferMode {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = DrawBufferMode::from_repr(val) else {
            println!(
                "Attempt to create a DrawBufferMode from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<DrawBufferMode> for u32 {
    fn from(value: DrawBufferMode) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for DrawBufferMode {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum AtomicCounterBufferPName {
    AtomicCounterBufferBinding = GL_ATOMIC_COUNTER_BUFFER_BINDING,
    AtomicCounterBufferDataSize = GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE,
    AtomicCounterBufferActiveAtomicCounters = GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS,
    AtomicCounterBufferActiveAtomicCounterIndices =
        GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES,
    AtomicCounterBufferReferencedByVertexShader =
        GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER,
    AtomicCounterBufferReferencedByTessControlShader =
        GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER,
    AtomicCounterBufferReferencedByTessEvaluationShader =
        GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER,
    AtomicCounterBufferReferencedByGeometryShader =
        GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER,
    AtomicCounterBufferReferencedByFragmentShader =
        GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER,
    AtomicCounterBufferReferencedByComputeShader =
        GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER,
}
impl UnsafeFromGLenum for AtomicCounterBufferPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = AtomicCounterBufferPName::from_repr(val) else {
            println!("Attempt to create a AtomicCounterBufferPName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<AtomicCounterBufferPName> for u32 {
    fn from(value: AtomicCounterBufferPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for AtomicCounterBufferPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum Buffer {
    Color = GL_COLOR,
    Depth = GL_DEPTH,
    Stencil = GL_STENCIL,
}
impl UnsafeFromGLenum for Buffer {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = Buffer::from_repr(val) else {
            println!("Attempt to create a Buffer from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<Buffer> for u32 {
    fn from(value: Buffer) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for Buffer {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum GetTextureParameter {
    TextureWidth = GL_TEXTURE_WIDTH,
    TextureHeight = GL_TEXTURE_HEIGHT,
    TextureBorderColor = GL_TEXTURE_BORDER_COLOR,
    TextureMagFilter = GL_TEXTURE_MAG_FILTER,
    TextureMinFilter = GL_TEXTURE_MIN_FILTER,
    TextureWrapS = GL_TEXTURE_WRAP_S,
    TextureWrapT = GL_TEXTURE_WRAP_T,
    TextureInternalFormat = GL_TEXTURE_INTERNAL_FORMAT,
    TextureRedSize = GL_TEXTURE_RED_SIZE,
    TextureGreenSize = GL_TEXTURE_GREEN_SIZE,
    TextureBlueSize = GL_TEXTURE_BLUE_SIZE,
    TextureAlphaSize = GL_TEXTURE_ALPHA_SIZE,
}
impl UnsafeFromGLenum for GetTextureParameter {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = GetTextureParameter::from_repr(val) else {
            println!(
                "Attempt to create a GetTextureParameter from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<GetTextureParameter> for u32 {
    fn from(value: GetTextureParameter) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for GetTextureParameter {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum FramebufferParameterName {
    FramebufferDefaultWidth = GL_FRAMEBUFFER_DEFAULT_WIDTH,
    FramebufferDefaultHeight = GL_FRAMEBUFFER_DEFAULT_HEIGHT,
    FramebufferDefaultLayers = GL_FRAMEBUFFER_DEFAULT_LAYERS,
    FramebufferDefaultSamples = GL_FRAMEBUFFER_DEFAULT_SAMPLES,
    FramebufferDefaultFixedSampleLocations = GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS,
}
impl UnsafeFromGLenum for FramebufferParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = FramebufferParameterName::from_repr(val) else {
            println!("Attempt to create a FramebufferParameterName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<FramebufferParameterName> for u32 {
    fn from(value: FramebufferParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for FramebufferParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum DrawElementsType {
    UnsignedByte = GL_UNSIGNED_BYTE,
    UnsignedShort = GL_UNSIGNED_SHORT,
    UnsignedInt = GL_UNSIGNED_INT,
}
impl UnsafeFromGLenum for DrawElementsType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = DrawElementsType::from_repr(val) else {
            println!(
                "Attempt to create a DrawElementsType from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<DrawElementsType> for u32 {
    fn from(value: DrawElementsType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for DrawElementsType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum BufferAccess {
    ReadOnly = GL_READ_ONLY,
    WriteOnly = GL_WRITE_ONLY,
    ReadWrite = GL_READ_WRITE,
}
impl UnsafeFromGLenum for BufferAccess {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = BufferAccess::from_repr(val) else {
            println!("Attempt to create a BufferAccess from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<BufferAccess> for u32 {
    fn from(value: BufferAccess) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for BufferAccess {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum PatchParameterName {
    PatchVertices = GL_PATCH_VERTICES,
    PatchDefaultInnerLevel = GL_PATCH_DEFAULT_INNER_LEVEL,
    PatchDefaultOuterLevel = GL_PATCH_DEFAULT_OUTER_LEVEL,
}
impl UnsafeFromGLenum for PatchParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = PatchParameterName::from_repr(val) else {
            println!(
                "Attempt to create a PatchParameterName from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<PatchParameterName> for u32 {
    fn from(value: PatchParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for PatchParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ClipControlDepth {
    NegativeOneToOne = GL_NEGATIVE_ONE_TO_ONE,
    ZeroToOne = GL_ZERO_TO_ONE,
}
impl UnsafeFromGLenum for ClipControlDepth {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ClipControlDepth::from_repr(val) else {
            println!(
                "Attempt to create a ClipControlDepth from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ClipControlDepth> for u32 {
    fn from(value: ClipControlDepth) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ClipControlDepth {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
bitflags! {
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct UseProgramStageMask: u32 {
const VERTEX_SHADER_BIT = GL_VERTEX_SHADER_BIT;
const FRAGMENT_SHADER_BIT = GL_FRAGMENT_SHADER_BIT;
const GEOMETRY_SHADER_BIT = GL_GEOMETRY_SHADER_BIT;
const TESS_CONTROL_SHADER_BIT = GL_TESS_CONTROL_SHADER_BIT;
const TESS_EVALUATION_SHADER_BIT = GL_TESS_EVALUATION_SHADER_BIT;
const ALL_SHADER_BITS = GL_ALL_SHADER_BITS;
const COMPUTE_SHADER_BIT = GL_COMPUTE_SHADER_BIT;
}
}
impl UnsafeFromGLenum for UseProgramStageMask {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = UseProgramStageMask::from_bits(val) else {
            println!("Attempt to create a UseProgramStageMask from a GLenum with an invalid bit set! {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<UseProgramStageMask> for u32 {
    fn from(value: UseProgramStageMask) -> u32 {
        value.bits()
    }
}
impl<Dst: GlDstType> SrcType<Dst> for UseProgramStageMask {
    fn cast(self) -> Dst {
        Dst::from_uint(self.bits())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum GetPointervPName {
    DebugCallbackFunction = GL_DEBUG_CALLBACK_FUNCTION,
    DebugCallbackUserParam = GL_DEBUG_CALLBACK_USER_PARAM,
}
impl UnsafeFromGLenum for GetPointervPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = GetPointervPName::from_repr(val) else {
            println!(
                "Attempt to create a GetPointervPName from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<GetPointervPName> for u32 {
    fn from(value: GetPointervPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for GetPointervPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum GetPName {
    PointSize = GL_POINT_SIZE,
    PointSizeRange = GL_POINT_SIZE_RANGE,
    PointSizeGranularity = GL_POINT_SIZE_GRANULARITY,
    LineSmooth = GL_LINE_SMOOTH,
    LineWidth = GL_LINE_WIDTH,
    LineWidthRange = GL_LINE_WIDTH_RANGE,
    LineWidthGranularity = GL_LINE_WIDTH_GRANULARITY,
    PolygonMode = GL_POLYGON_MODE,
    PolygonSmooth = GL_POLYGON_SMOOTH,
    CullFace = GL_CULL_FACE,
    CullFaceMode = GL_CULL_FACE_MODE,
    FrontFace = GL_FRONT_FACE,
    DepthRange = GL_DEPTH_RANGE,
    DepthTest = GL_DEPTH_TEST,
    DepthWritemask = GL_DEPTH_WRITEMASK,
    DepthClearValue = GL_DEPTH_CLEAR_VALUE,
    DepthFunc = GL_DEPTH_FUNC,
    StencilTest = GL_STENCIL_TEST,
    StencilClearValue = GL_STENCIL_CLEAR_VALUE,
    StencilFunc = GL_STENCIL_FUNC,
    StencilValueMask = GL_STENCIL_VALUE_MASK,
    StencilFail = GL_STENCIL_FAIL,
    StencilPassDepthFail = GL_STENCIL_PASS_DEPTH_FAIL,
    StencilPassDepthPass = GL_STENCIL_PASS_DEPTH_PASS,
    StencilRef = GL_STENCIL_REF,
    StencilWritemask = GL_STENCIL_WRITEMASK,
    Viewport = GL_VIEWPORT,
    Dither = GL_DITHER,
    BlendDst = GL_BLEND_DST,
    BlendSrc = GL_BLEND_SRC,
    Blend = GL_BLEND,
    LogicOpMode = GL_LOGIC_OP_MODE,
    DrawBuffer = GL_DRAW_BUFFER,
    ReadBuffer = GL_READ_BUFFER,
    ScissorBox = GL_SCISSOR_BOX,
    ScissorTest = GL_SCISSOR_TEST,
    ColorClearValue = GL_COLOR_CLEAR_VALUE,
    ColorWritemask = GL_COLOR_WRITEMASK,
    Doublebuffer = GL_DOUBLEBUFFER,
    Stereo = GL_STEREO,
    LineSmoothHint = GL_LINE_SMOOTH_HINT,
    PolygonSmoothHint = GL_POLYGON_SMOOTH_HINT,
    UnpackSwapBytes = GL_UNPACK_SWAP_BYTES,
    UnpackLsbFirst = GL_UNPACK_LSB_FIRST,
    UnpackRowLength = GL_UNPACK_ROW_LENGTH,
    UnpackSkipRows = GL_UNPACK_SKIP_ROWS,
    UnpackSkipPixels = GL_UNPACK_SKIP_PIXELS,
    UnpackAlignment = GL_UNPACK_ALIGNMENT,
    PackSwapBytes = GL_PACK_SWAP_BYTES,
    PackLsbFirst = GL_PACK_LSB_FIRST,
    PackRowLength = GL_PACK_ROW_LENGTH,
    PackSkipRows = GL_PACK_SKIP_ROWS,
    PackSkipPixels = GL_PACK_SKIP_PIXELS,
    PackAlignment = GL_PACK_ALIGNMENT,
    MaxTextureSize = GL_MAX_TEXTURE_SIZE,
    MaxViewportDims = GL_MAX_VIEWPORT_DIMS,
    SubpixelBits = GL_SUBPIXEL_BITS,
    Texture1D = GL_TEXTURE_1D,
    Texture2D = GL_TEXTURE_2D,
    ColorLogicOp = GL_COLOR_LOGIC_OP,
    PolygonOffsetUnits = GL_POLYGON_OFFSET_UNITS,
    PolygonOffsetPoint = GL_POLYGON_OFFSET_POINT,
    PolygonOffsetLine = GL_POLYGON_OFFSET_LINE,
    PolygonOffsetFill = GL_POLYGON_OFFSET_FILL,
    PolygonOffsetFactor = GL_POLYGON_OFFSET_FACTOR,
    PackSkipImages = GL_PACK_SKIP_IMAGES,
    PackImageHeight = GL_PACK_IMAGE_HEIGHT,
    UnpackSkipImages = GL_UNPACK_SKIP_IMAGES,
    UnpackImageHeight = GL_UNPACK_IMAGE_HEIGHT,
    Max3DTextureSize = GL_MAX_3D_TEXTURE_SIZE,
    MaxElementsVertices = GL_MAX_ELEMENTS_VERTICES,
    MaxElementsIndices = GL_MAX_ELEMENTS_INDICES,
    AliasedLineWidthRange = GL_ALIASED_LINE_WIDTH_RANGE,
    ActiveTexture = GL_ACTIVE_TEXTURE,
    SampleBuffers = GL_SAMPLE_BUFFERS,
    Samples = GL_SAMPLES,
    SampleCoverageValue = GL_SAMPLE_COVERAGE_VALUE,
    SampleCoverageInvert = GL_SAMPLE_COVERAGE_INVERT,
    MaxCubeMapTextureSize = GL_MAX_CUBE_MAP_TEXTURE_SIZE,
    TextureCompressionHint = GL_TEXTURE_COMPRESSION_HINT,
    NumCompressedTextureFormats = GL_NUM_COMPRESSED_TEXTURE_FORMATS,
    CompressedTextureFormats = GL_COMPRESSED_TEXTURE_FORMATS,
    BlendDstRgb = GL_BLEND_DST_RGB,
    BlendSrcRgb = GL_BLEND_SRC_RGB,
    BlendDstAlpha = GL_BLEND_DST_ALPHA,
    BlendSrcAlpha = GL_BLEND_SRC_ALPHA,
    PointFadeThresholdSize = GL_POINT_FADE_THRESHOLD_SIZE,
    MaxTextureLodBias = GL_MAX_TEXTURE_LOD_BIAS,
    BlendColor = GL_BLEND_COLOR,
    BlendEquation = GL_BLEND_EQUATION,
    ArrayBufferBinding = GL_ARRAY_BUFFER_BINDING,
    ElementArrayBufferBinding = GL_ELEMENT_ARRAY_BUFFER_BINDING,
    StencilBackFunc = GL_STENCIL_BACK_FUNC,
    StencilBackFail = GL_STENCIL_BACK_FAIL,
    StencilBackPassDepthFail = GL_STENCIL_BACK_PASS_DEPTH_FAIL,
    StencilBackPassDepthPass = GL_STENCIL_BACK_PASS_DEPTH_PASS,
    MaxDrawBuffers = GL_MAX_DRAW_BUFFERS,
    BlendEquationAlpha = GL_BLEND_EQUATION_ALPHA,
    MaxVertexAttribs = GL_MAX_VERTEX_ATTRIBS,
    MaxTextureImageUnits = GL_MAX_TEXTURE_IMAGE_UNITS,
    MaxFragmentUniformComponents = GL_MAX_FRAGMENT_UNIFORM_COMPONENTS,
    MaxVertexUniformComponents = GL_MAX_VERTEX_UNIFORM_COMPONENTS,
    MaxVaryingFloats = GL_MAX_VARYING_FLOATS,
    MaxVertexTextureImageUnits = GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS,
    MaxCombinedTextureImageUnits = GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS,
    FragmentShaderDerivativeHint = GL_FRAGMENT_SHADER_DERIVATIVE_HINT,
    CurrentProgram = GL_CURRENT_PROGRAM,
    StencilBackRef = GL_STENCIL_BACK_REF,
    StencilBackValueMask = GL_STENCIL_BACK_VALUE_MASK,
    StencilBackWritemask = GL_STENCIL_BACK_WRITEMASK,
    PixelPackBufferBinding = GL_PIXEL_PACK_BUFFER_BINDING,
    PixelUnpackBufferBinding = GL_PIXEL_UNPACK_BUFFER_BINDING,
    MaxClipDistances = GL_MAX_CLIP_DISTANCES,
    MajorVersion = GL_MAJOR_VERSION,
    MinorVersion = GL_MINOR_VERSION,
    NumExtensions = GL_NUM_EXTENSIONS,
    ContextFlags = GL_CONTEXT_FLAGS,
    MaxArrayTextureLayers = GL_MAX_ARRAY_TEXTURE_LAYERS,
    MinProgramTexelOffset = GL_MIN_PROGRAM_TEXEL_OFFSET,
    MaxProgramTexelOffset = GL_MAX_PROGRAM_TEXEL_OFFSET,
    TransformFeedbackBufferStart = GL_TRANSFORM_FEEDBACK_BUFFER_START,
    TransformFeedbackBufferSize = GL_TRANSFORM_FEEDBACK_BUFFER_SIZE,
    TransformFeedbackBufferBinding = GL_TRANSFORM_FEEDBACK_BUFFER_BINDING,
    MaxRenderbufferSize = GL_MAX_RENDERBUFFER_SIZE,
    DrawFramebufferBinding = GL_DRAW_FRAMEBUFFER_BINDING,
    RenderbufferBinding = GL_RENDERBUFFER_BINDING,
    ReadFramebufferBinding = GL_READ_FRAMEBUFFER_BINDING,
    MaxColorAttachments = GL_MAX_COLOR_ATTACHMENTS,
    VertexArrayBinding = GL_VERTEX_ARRAY_BINDING,
    MaxTextureBufferSize = GL_MAX_TEXTURE_BUFFER_SIZE,
    MaxRectangleTextureSize = GL_MAX_RECTANGLE_TEXTURE_SIZE,
    PrimitiveRestartIndex = GL_PRIMITIVE_RESTART_INDEX,
    UniformBufferBinding = GL_UNIFORM_BUFFER_BINDING,
    UniformBufferStart = GL_UNIFORM_BUFFER_START,
    UniformBufferSize = GL_UNIFORM_BUFFER_SIZE,
    MaxVertexUniformBlocks = GL_MAX_VERTEX_UNIFORM_BLOCKS,
    MaxGeometryUniformBlocks = GL_MAX_GEOMETRY_UNIFORM_BLOCKS,
    MaxFragmentUniformBlocks = GL_MAX_FRAGMENT_UNIFORM_BLOCKS,
    MaxCombinedUniformBlocks = GL_MAX_COMBINED_UNIFORM_BLOCKS,
    MaxUniformBufferBindings = GL_MAX_UNIFORM_BUFFER_BINDINGS,
    MaxUniformBlockSize = GL_MAX_UNIFORM_BLOCK_SIZE,
    MaxCombinedVertexUniformComponents = GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS,
    MaxCombinedGeometryUniformComponents = GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS,
    MaxCombinedFragmentUniformComponents = GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS,
    UniformBufferOffsetAlignment = GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT,
    ProgramPointSize = GL_PROGRAM_POINT_SIZE,
    MaxGeometryTextureImageUnits = GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS,
    MaxGeometryUniformComponents = GL_MAX_GEOMETRY_UNIFORM_COMPONENTS,
    MaxVertexOutputComponents = GL_MAX_VERTEX_OUTPUT_COMPONENTS,
    MaxGeometryInputComponents = GL_MAX_GEOMETRY_INPUT_COMPONENTS,
    MaxGeometryOutputComponents = GL_MAX_GEOMETRY_OUTPUT_COMPONENTS,
    MaxFragmentInputComponents = GL_MAX_FRAGMENT_INPUT_COMPONENTS,
    ContextProfileMask = GL_CONTEXT_PROFILE_MASK,
    ProvokingVertex = GL_PROVOKING_VERTEX,
    MaxServerWaitTimeout = GL_MAX_SERVER_WAIT_TIMEOUT,
    MaxSampleMaskWords = GL_MAX_SAMPLE_MASK_WORDS,
    MaxColorTextureSamples = GL_MAX_COLOR_TEXTURE_SAMPLES,
    MaxDepthTextureSamples = GL_MAX_DEPTH_TEXTURE_SAMPLES,
    MaxIntegerSamples = GL_MAX_INTEGER_SAMPLES,
    MaxDualSourceDrawBuffers = GL_MAX_DUAL_SOURCE_DRAW_BUFFERS,
    SamplerBinding = GL_SAMPLER_BINDING,
    Timestamp = GL_TIMESTAMP,
    DrawIndirectBufferBinding = GL_DRAW_INDIRECT_BUFFER_BINDING,
    MaxTessControlUniformBlocks = GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS,
    MaxTessEvaluationUniformBlocks = GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS,
    MaxTransformFeedbackBuffers = GL_MAX_TRANSFORM_FEEDBACK_BUFFERS,
    MaxVertexStreams = GL_MAX_VERTEX_STREAMS,
    ImplementationColorReadType = GL_IMPLEMENTATION_COLOR_READ_TYPE,
    ImplementationColorReadFormat = GL_IMPLEMENTATION_COLOR_READ_FORMAT,
    ShaderCompiler = GL_SHADER_COMPILER,
    ShaderBinaryFormats = GL_SHADER_BINARY_FORMATS,
    NumShaderBinaryFormats = GL_NUM_SHADER_BINARY_FORMATS,
    MaxVertexUniformVectors = GL_MAX_VERTEX_UNIFORM_VECTORS,
    MaxVaryingVectors = GL_MAX_VARYING_VECTORS,
    MaxFragmentUniformVectors = GL_MAX_FRAGMENT_UNIFORM_VECTORS,
    NumProgramBinaryFormats = GL_NUM_PROGRAM_BINARY_FORMATS,
    ProgramBinaryFormats = GL_PROGRAM_BINARY_FORMATS,
    ProgramPipelineBinding = GL_PROGRAM_PIPELINE_BINDING,
    MaxViewports = GL_MAX_VIEWPORTS,
    ViewportSubpixelBits = GL_VIEWPORT_SUBPIXEL_BITS,
    ViewportBoundsRange = GL_VIEWPORT_BOUNDS_RANGE,
    LayerProvokingVertex = GL_LAYER_PROVOKING_VERTEX,
    ViewportIndexProvokingVertex = GL_VIEWPORT_INDEX_PROVOKING_VERTEX,
    CopyReadBufferBinding = GL_COPY_READ_BUFFER_BINDING,
    CopyWriteBufferBinding = GL_COPY_WRITE_BUFFER_BINDING,
    MinMapBufferAlignment = GL_MIN_MAP_BUFFER_ALIGNMENT,
    AtomicCounterBufferBinding = GL_ATOMIC_COUNTER_BUFFER_BINDING,
    MaxVertexAtomicCounterBuffers = GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS,
    MaxTessControlAtomicCounterBuffers = GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS,
    MaxTessEvaluationAtomicCounterBuffers = GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS,
    MaxGeometryAtomicCounterBuffers = GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS,
    MaxFragmentAtomicCounterBuffers = GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS,
    MaxCombinedAtomicCounterBuffers = GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS,
    MaxVertexAtomicCounters = GL_MAX_VERTEX_ATOMIC_COUNTERS,
    MaxTessControlAtomicCounters = GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS,
    MaxTessEvaluationAtomicCounters = GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS,
    MaxGeometryAtomicCounters = GL_MAX_GEOMETRY_ATOMIC_COUNTERS,
    MaxFragmentAtomicCounters = GL_MAX_FRAGMENT_ATOMIC_COUNTERS,
    MaxCombinedAtomicCounters = GL_MAX_COMBINED_ATOMIC_COUNTERS,
    MaxElementIndex = GL_MAX_ELEMENT_INDEX,
    MaxComputeUniformBlocks = GL_MAX_COMPUTE_UNIFORM_BLOCKS,
    MaxComputeTextureImageUnits = GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS,
    MaxComputeUniformComponents = GL_MAX_COMPUTE_UNIFORM_COMPONENTS,
    MaxComputeAtomicCounterBuffers = GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS,
    MaxComputeAtomicCounters = GL_MAX_COMPUTE_ATOMIC_COUNTERS,
    MaxCombinedComputeUniformComponents = GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS,
    MaxComputeWorkGroupInvocations = GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS,
    MaxComputeWorkGroupCount = GL_MAX_COMPUTE_WORK_GROUP_COUNT,
    MaxComputeWorkGroupSize = GL_MAX_COMPUTE_WORK_GROUP_SIZE,
    DispatchIndirectBufferBinding = GL_DISPATCH_INDIRECT_BUFFER_BINDING,
    MaxDebugGroupStackDepth = GL_MAX_DEBUG_GROUP_STACK_DEPTH,
    DebugGroupStackDepth = GL_DEBUG_GROUP_STACK_DEPTH,
    VertexArray = GL_VERTEX_ARRAY,
    MaxLabelLength = GL_MAX_LABEL_LENGTH,
    MaxUniformLocations = GL_MAX_UNIFORM_LOCATIONS,
    MaxFramebufferWidth = GL_MAX_FRAMEBUFFER_WIDTH,
    MaxFramebufferHeight = GL_MAX_FRAMEBUFFER_HEIGHT,
    MaxFramebufferLayers = GL_MAX_FRAMEBUFFER_LAYERS,
    MaxFramebufferSamples = GL_MAX_FRAMEBUFFER_SAMPLES,
    ShaderStorageBufferBinding = GL_SHADER_STORAGE_BUFFER_BINDING,
    ShaderStorageBufferStart = GL_SHADER_STORAGE_BUFFER_START,
    ShaderStorageBufferSize = GL_SHADER_STORAGE_BUFFER_SIZE,
    MaxVertexShaderStorageBlocks = GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS,
    MaxGeometryShaderStorageBlocks = GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS,
    MaxTessControlShaderStorageBlocks = GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS,
    MaxTessEvaluationShaderStorageBlocks = GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS,
    MaxFragmentShaderStorageBlocks = GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS,
    MaxComputeShaderStorageBlocks = GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS,
    MaxCombinedShaderStorageBlocks = GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS,
    MaxShaderStorageBufferBindings = GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS,
    ShaderStorageBufferOffsetAlignment = GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT,
    TextureBufferOffsetAlignment = GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT,
    VertexBindingDivisor = GL_VERTEX_BINDING_DIVISOR,
    VertexBindingOffset = GL_VERTEX_BINDING_OFFSET,
    VertexBindingStride = GL_VERTEX_BINDING_STRIDE,
    MaxVertexAttribRelativeOffset = GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET,
    MaxVertexAttribBindings = GL_MAX_VERTEX_ATTRIB_BINDINGS,
    TextureBufferBinding = GL_TEXTURE_BUFFER_BINDING,
    QueryBufferBinding = GL_QUERY_BUFFER_BINDING,
    TextureBinding1D = GL_TEXTURE_BINDING_1D,
    TextureBinding1DArray = GL_TEXTURE_BINDING_1D_ARRAY,
    TextureBinding2D = GL_TEXTURE_BINDING_2D,
    TextureBinding2DArray = GL_TEXTURE_BINDING_2D_ARRAY,
    TextureBinding2DMultisample = GL_TEXTURE_BINDING_2D_MULTISAMPLE,
    TextureBinding2DMultisampleArray = GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY,
    TextureBinding3D = GL_TEXTURE_BINDING_3D,
    TextureBindingBuffer = GL_TEXTURE_BINDING_BUFFER,
    TextureBindingCubeMap = GL_TEXTURE_BINDING_CUBE_MAP,
    TextureBindingRectangle = GL_TEXTURE_BINDING_RECTANGLE,
    ParameterBufferBinding = GL_PARAMETER_BUFFER_BINDING,
}
impl UnsafeFromGLenum for GetPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = GetPName::from_repr(val) else {
            println!("Attempt to create a GetPName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<GetPName> for u32 {
    fn from(value: GetPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for GetPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum LogicOp {
    Clear = GL_CLEAR,
    And = GL_AND,
    AndReverse = GL_AND_REVERSE,
    Copy = GL_COPY,
    AndInverted = GL_AND_INVERTED,
    Noop = GL_NOOP,
    Xor = GL_XOR,
    Or = GL_OR,
    Nor = GL_NOR,
    Equiv = GL_EQUIV,
    Invert = GL_INVERT,
    OrReverse = GL_OR_REVERSE,
    CopyInverted = GL_COPY_INVERTED,
    OrInverted = GL_OR_INVERTED,
    Nand = GL_NAND,
    Set = GL_SET,
}
impl UnsafeFromGLenum for LogicOp {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = LogicOp::from_repr(val) else {
            println!("Attempt to create a LogicOp from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<LogicOp> for u32 {
    fn from(value: LogicOp) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for LogicOp {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum StencilOp {
    Zero = GL_ZERO,
    Invert = GL_INVERT,
    Keep = GL_KEEP,
    Replace = GL_REPLACE,
    Incr = GL_INCR,
    Decr = GL_DECR,
    IncrWrap = GL_INCR_WRAP,
    DecrWrap = GL_DECR_WRAP,
}
impl UnsafeFromGLenum for StencilOp {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = StencilOp::from_repr(val) else {
            println!("Attempt to create a StencilOp from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<StencilOp> for u32 {
    fn from(value: StencilOp) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for StencilOp {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
bitflags! {
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct BufferStorageMask: u32 {
const MAP_READ_BIT = GL_MAP_READ_BIT;
const MAP_WRITE_BIT = GL_MAP_WRITE_BIT;
const MAP_PERSISTENT_BIT = GL_MAP_PERSISTENT_BIT;
const MAP_COHERENT_BIT = GL_MAP_COHERENT_BIT;
const DYNAMIC_STORAGE_BIT = GL_DYNAMIC_STORAGE_BIT;
const CLIENT_STORAGE_BIT = GL_CLIENT_STORAGE_BIT;
}
}
impl UnsafeFromGLenum for BufferStorageMask {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = BufferStorageMask::from_bits(val) else {
            println!("Attempt to create a BufferStorageMask from a GLenum with an invalid bit set! {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<BufferStorageMask> for u32 {
    fn from(value: BufferStorageMask) -> u32 {
        value.bits()
    }
}
impl<Dst: GlDstType> SrcType<Dst> for BufferStorageMask {
    fn cast(self) -> Dst {
        Dst::from_uint(self.bits())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum StringName {
    Vendor = GL_VENDOR,
    Renderer = GL_RENDERER,
    Version = GL_VERSION,
    Extensions = GL_EXTENSIONS,
    ShadingLanguageVersion = GL_SHADING_LANGUAGE_VERSION,
}
impl UnsafeFromGLenum for StringName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = StringName::from_repr(val) else {
            println!("Attempt to create a StringName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<StringName> for u32 {
    fn from(value: StringName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for StringName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
bitflags! {
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ClearBufferMask: u32 {
const DEPTH_BUFFER_BIT = GL_DEPTH_BUFFER_BIT;
const STENCIL_BUFFER_BIT = GL_STENCIL_BUFFER_BIT;
const COLOR_BUFFER_BIT = GL_COLOR_BUFFER_BIT;
}
}
impl UnsafeFromGLenum for ClearBufferMask {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ClearBufferMask::from_bits(val) else {
            println!("Attempt to create a ClearBufferMask from a GLenum with an invalid bit set! {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ClearBufferMask> for u32 {
    fn from(value: ClearBufferMask) -> u32 {
        value.bits()
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ClearBufferMask {
    fn cast(self) -> Dst {
        Dst::from_uint(self.bits())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum TransformFeedbackBufferMode {
    InterleavedAttribs = GL_INTERLEAVED_ATTRIBS,
    SeparateAttribs = GL_SEPARATE_ATTRIBS,
}
impl UnsafeFromGLenum for TransformFeedbackBufferMode {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = TransformFeedbackBufferMode::from_repr(val) else {
            println!("Attempt to create a TransformFeedbackBufferMode from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<TransformFeedbackBufferMode> for u32 {
    fn from(value: TransformFeedbackBufferMode) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for TransformFeedbackBufferMode {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum QueryObjectParameterName {
    QueryResult = GL_QUERY_RESULT,
    QueryResultAvailable = GL_QUERY_RESULT_AVAILABLE,
    QueryResultNoWait = GL_QUERY_RESULT_NO_WAIT,
    QueryTarget = GL_QUERY_TARGET,
}
impl UnsafeFromGLenum for QueryObjectParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = QueryObjectParameterName::from_repr(val) else {
            println!("Attempt to create a QueryObjectParameterName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<QueryObjectParameterName> for u32 {
    fn from(value: QueryObjectParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for QueryObjectParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum SamplerParameterI {
    TextureMagFilter = GL_TEXTURE_MAG_FILTER,
    TextureMinFilter = GL_TEXTURE_MIN_FILTER,
    TextureWrapS = GL_TEXTURE_WRAP_S,
    TextureWrapT = GL_TEXTURE_WRAP_T,
    TextureWrapR = GL_TEXTURE_WRAP_R,
    TextureCompareMode = GL_TEXTURE_COMPARE_MODE,
    TextureCompareFunc = GL_TEXTURE_COMPARE_FUNC,
}
impl UnsafeFromGLenum for SamplerParameterI {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = SamplerParameterI::from_repr(val) else {
            println!(
                "Attempt to create a SamplerParameterI from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<SamplerParameterI> for u32 {
    fn from(value: SamplerParameterI) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for SamplerParameterI {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum SizedInternalFormat {
    R3G3B2 = GL_R3_G3_B2,
    Rgb4 = GL_RGB4,
    Rgb5 = GL_RGB5,
    Rgb8 = GL_RGB8,
    Rgb10 = GL_RGB10,
    Rgb12 = GL_RGB12,
    Rgb16 = GL_RGB16,
    Rgba2 = GL_RGBA2,
    Rgba4 = GL_RGBA4,
    Rgb5A1 = GL_RGB5_A1,
    Rgba8 = GL_RGBA8,
    Rgb10A2 = GL_RGB10_A2,
    Rgba12 = GL_RGBA12,
    Rgba16 = GL_RGBA16,
    DepthComponent16 = GL_DEPTH_COMPONENT16,
    DepthComponent24 = GL_DEPTH_COMPONENT24,
    DepthComponent32 = GL_DEPTH_COMPONENT32,
    Srgb8 = GL_SRGB8,
    Srgb8Alpha8 = GL_SRGB8_ALPHA8,
    Rgba32f = GL_RGBA32F,
    Rgb32f = GL_RGB32F,
    Rgba16f = GL_RGBA16F,
    Rgb16f = GL_RGB16F,
    R11fG11fB10f = GL_R11F_G11F_B10F,
    Rgb9E5 = GL_RGB9_E5,
    Rgba32ui = GL_RGBA32UI,
    Rgb32ui = GL_RGB32UI,
    Rgba16ui = GL_RGBA16UI,
    Rgb16ui = GL_RGB16UI,
    Rgba8ui = GL_RGBA8UI,
    Rgb8ui = GL_RGB8UI,
    Rgba32i = GL_RGBA32I,
    Rgb32i = GL_RGB32I,
    Rgba16i = GL_RGBA16I,
    Rgb16i = GL_RGB16I,
    Rgba8i = GL_RGBA8I,
    Rgb8i = GL_RGB8I,
    DepthComponent32f = GL_DEPTH_COMPONENT32F,
    Depth32fStencil8 = GL_DEPTH32F_STENCIL8,
    Depth24Stencil8 = GL_DEPTH24_STENCIL8,
    StencilIndex1 = GL_STENCIL_INDEX1,
    StencilIndex4 = GL_STENCIL_INDEX4,
    StencilIndex16 = GL_STENCIL_INDEX16,
    CompressedRedRgtc1 = GL_COMPRESSED_RED_RGTC1,
    CompressedSignedRedRgtc1 = GL_COMPRESSED_SIGNED_RED_RGTC1,
    CompressedRgRgtc2 = GL_COMPRESSED_RG_RGTC2,
    CompressedSignedRgRgtc2 = GL_COMPRESSED_SIGNED_RG_RGTC2,
    R8 = GL_R8,
    R16 = GL_R16,
    Rg8 = GL_RG8,
    Rg16 = GL_RG16,
    R16f = GL_R16F,
    R32f = GL_R32F,
    Rg16f = GL_RG16F,
    Rg32f = GL_RG32F,
    R8i = GL_R8I,
    R8ui = GL_R8UI,
    R16i = GL_R16I,
    R16ui = GL_R16UI,
    R32i = GL_R32I,
    R32ui = GL_R32UI,
    Rg8i = GL_RG8I,
    Rg8ui = GL_RG8UI,
    Rg16i = GL_RG16I,
    Rg16ui = GL_RG16UI,
    Rg32i = GL_RG32I,
    Rg32ui = GL_RG32UI,
    R8Snorm = GL_R8_SNORM,
    Rg8Snorm = GL_RG8_SNORM,
    Rgb8Snorm = GL_RGB8_SNORM,
    Rgba8Snorm = GL_RGBA8_SNORM,
    R16Snorm = GL_R16_SNORM,
    Rg16Snorm = GL_RG16_SNORM,
    Rgb16Snorm = GL_RGB16_SNORM,
    Rgba16Snorm = GL_RGBA16_SNORM,
    Rgb10A2ui = GL_RGB10_A2UI,
    Rgb565 = GL_RGB565,
    CompressedRgbaBptcUnorm = GL_COMPRESSED_RGBA_BPTC_UNORM,
    CompressedSrgbAlphaBptcUnorm = GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM,
    CompressedRgbBptcSignedFloat = GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT,
    CompressedRgbBptcUnsignedFloat = GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT,
    CompressedRgb8Etc2 = GL_COMPRESSED_RGB8_ETC2,
    CompressedSrgb8Etc2 = GL_COMPRESSED_SRGB8_ETC2,
    CompressedRgb8PunchthroughAlpha1Etc2 = GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2,
    CompressedSrgb8PunchthroughAlpha1Etc2 = GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2,
    CompressedRgba8Etc2Eac = GL_COMPRESSED_RGBA8_ETC2_EAC,
    CompressedSrgb8Alpha8Etc2Eac = GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC,
    CompressedR11Eac = GL_COMPRESSED_R11_EAC,
    CompressedSignedR11Eac = GL_COMPRESSED_SIGNED_R11_EAC,
    CompressedRg11Eac = GL_COMPRESSED_RG11_EAC,
    CompressedSignedRg11Eac = GL_COMPRESSED_SIGNED_RG11_EAC,
    StencilIndex8 = GL_STENCIL_INDEX8,
}
impl UnsafeFromGLenum for SizedInternalFormat {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = SizedInternalFormat::from_repr(val) else {
            println!(
                "Attempt to create a SizedInternalFormat from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<SizedInternalFormat> for u32 {
    fn from(value: SizedInternalFormat) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for SizedInternalFormat {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum VertexAttribPointerType {
    Byte = GL_BYTE,
    UnsignedByte = GL_UNSIGNED_BYTE,
    Short = GL_SHORT,
    UnsignedShort = GL_UNSIGNED_SHORT,
    Int = GL_INT,
    UnsignedInt = GL_UNSIGNED_INT,
    Float = GL_FLOAT,
    Double = GL_DOUBLE,
    UnsignedInt2101010Rev = GL_UNSIGNED_INT_2_10_10_10_REV,
    HalfFloat = GL_HALF_FLOAT,
    Int2101010Rev = GL_INT_2_10_10_10_REV,
    Fixed = GL_FIXED,
    UnsignedInt10F11F11FRev = GL_UNSIGNED_INT_10F_11F_11F_REV,
}
impl UnsafeFromGLenum for VertexAttribPointerType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = VertexAttribPointerType::from_repr(val) else {
            println!("Attempt to create a VertexAttribPointerType from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<VertexAttribPointerType> for u32 {
    fn from(value: VertexAttribPointerType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for VertexAttribPointerType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum GetFramebufferParameter {
    Doublebuffer = GL_DOUBLEBUFFER,
    Stereo = GL_STEREO,
    SampleBuffers = GL_SAMPLE_BUFFERS,
    Samples = GL_SAMPLES,
    ImplementationColorReadType = GL_IMPLEMENTATION_COLOR_READ_TYPE,
    ImplementationColorReadFormat = GL_IMPLEMENTATION_COLOR_READ_FORMAT,
    FramebufferDefaultWidth = GL_FRAMEBUFFER_DEFAULT_WIDTH,
    FramebufferDefaultHeight = GL_FRAMEBUFFER_DEFAULT_HEIGHT,
    FramebufferDefaultLayers = GL_FRAMEBUFFER_DEFAULT_LAYERS,
    FramebufferDefaultSamples = GL_FRAMEBUFFER_DEFAULT_SAMPLES,
    FramebufferDefaultFixedSampleLocations = GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS,
}
impl UnsafeFromGLenum for GetFramebufferParameter {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = GetFramebufferParameter::from_repr(val) else {
            println!("Attempt to create a GetFramebufferParameter from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<GetFramebufferParameter> for u32 {
    fn from(value: GetFramebufferParameter) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for GetFramebufferParameter {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum CopyImageSubDataTarget {
    Texture1D = GL_TEXTURE_1D,
    Texture2D = GL_TEXTURE_2D,
    Texture3D = GL_TEXTURE_3D,
    TextureCubeMap = GL_TEXTURE_CUBE_MAP,
    Texture1DArray = GL_TEXTURE_1D_ARRAY,
    Texture2DArray = GL_TEXTURE_2D_ARRAY,
    Renderbuffer = GL_RENDERBUFFER,
    TextureRectangle = GL_TEXTURE_RECTANGLE,
    Texture2DMultisample = GL_TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = GL_TEXTURE_2D_MULTISAMPLE_ARRAY,
    TextureCubeMapArray = GL_TEXTURE_CUBE_MAP_ARRAY,
}
impl UnsafeFromGLenum for CopyImageSubDataTarget {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = CopyImageSubDataTarget::from_repr(val) else {
            println!("Attempt to create a CopyImageSubDataTarget from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<CopyImageSubDataTarget> for u32 {
    fn from(value: CopyImageSubDataTarget) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for CopyImageSubDataTarget {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ColorBuffer {
    FrontLeft = GL_FRONT_LEFT,
    FrontRight = GL_FRONT_RIGHT,
    BackLeft = GL_BACK_LEFT,
    BackRight = GL_BACK_RIGHT,
    Front = GL_FRONT,
    Left = GL_LEFT,
    Right = GL_RIGHT,
    FrontAndBack = GL_FRONT_AND_BACK,
    ColorAttachment0 = GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = GL_COLOR_ATTACHMENT1,
    ColorAttachment2 = GL_COLOR_ATTACHMENT2,
    ColorAttachment3 = GL_COLOR_ATTACHMENT3,
    ColorAttachment4 = GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = GL_COLOR_ATTACHMENT9,
    ColorAttachment10 = GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = GL_COLOR_ATTACHMENT15,
    ColorAttachment16 = GL_COLOR_ATTACHMENT16,
    ColorAttachment17 = GL_COLOR_ATTACHMENT17,
    ColorAttachment18 = GL_COLOR_ATTACHMENT18,
    ColorAttachment19 = GL_COLOR_ATTACHMENT19,
    ColorAttachment20 = GL_COLOR_ATTACHMENT20,
    ColorAttachment21 = GL_COLOR_ATTACHMENT21,
    ColorAttachment22 = GL_COLOR_ATTACHMENT22,
    ColorAttachment23 = GL_COLOR_ATTACHMENT23,
    ColorAttachment24 = GL_COLOR_ATTACHMENT24,
    ColorAttachment25 = GL_COLOR_ATTACHMENT25,
    ColorAttachment26 = GL_COLOR_ATTACHMENT26,
    ColorAttachment27 = GL_COLOR_ATTACHMENT27,
    ColorAttachment28 = GL_COLOR_ATTACHMENT28,
    ColorAttachment29 = GL_COLOR_ATTACHMENT29,
    ColorAttachment30 = GL_COLOR_ATTACHMENT30,
    ColorAttachment31 = GL_COLOR_ATTACHMENT31,
    Back = GL_BACK,
    None = GL_NONE,
}
impl UnsafeFromGLenum for ColorBuffer {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ColorBuffer::from_repr(val) else {
            println!("Attempt to create a ColorBuffer from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ColorBuffer> for u32 {
    fn from(value: ColorBuffer) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ColorBuffer {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum CopyBufferSubDataTarget {
    ArrayBuffer = GL_ARRAY_BUFFER,
    ElementArrayBuffer = GL_ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = GL_PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = GL_PIXEL_UNPACK_BUFFER,
    TextureBuffer = GL_TEXTURE_BUFFER,
    CopyReadBuffer = GL_COPY_READ_BUFFER,
    CopyWriteBuffer = GL_COPY_WRITE_BUFFER,
    UniformBuffer = GL_UNIFORM_BUFFER,
    DrawIndirectBuffer = GL_DRAW_INDIRECT_BUFFER,
    AtomicCounterBuffer = GL_ATOMIC_COUNTER_BUFFER,
    DispatchIndirectBuffer = GL_DISPATCH_INDIRECT_BUFFER,
    ShaderStorageBuffer = GL_SHADER_STORAGE_BUFFER,
    TransformFeedbackBuffer = GL_TRANSFORM_FEEDBACK_BUFFER,
    QueryBuffer = GL_QUERY_BUFFER,
}
impl UnsafeFromGLenum for CopyBufferSubDataTarget {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = CopyBufferSubDataTarget::from_repr(val) else {
            println!("Attempt to create a CopyBufferSubDataTarget from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<CopyBufferSubDataTarget> for u32 {
    fn from(value: CopyBufferSubDataTarget) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for CopyBufferSubDataTarget {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum DepthFunction {
    Never = GL_NEVER,
    Less = GL_LESS,
    Equal = GL_EQUAL,
    Lequal = GL_LEQUAL,
    Greater = GL_GREATER,
    Notequal = GL_NOTEQUAL,
    Gequal = GL_GEQUAL,
    Always = GL_ALWAYS,
}
impl UnsafeFromGLenum for DepthFunction {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = DepthFunction::from_repr(val) else {
            println!("Attempt to create a DepthFunction from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<DepthFunction> for u32 {
    fn from(value: DepthFunction) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for DepthFunction {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ReadBufferMode {
    FrontLeft = GL_FRONT_LEFT,
    FrontRight = GL_FRONT_RIGHT,
    BackLeft = GL_BACK_LEFT,
    BackRight = GL_BACK_RIGHT,
    Front = GL_FRONT,
    Left = GL_LEFT,
    Right = GL_RIGHT,
    ColorAttachment0 = GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = GL_COLOR_ATTACHMENT1,
    ColorAttachment2 = GL_COLOR_ATTACHMENT2,
    ColorAttachment3 = GL_COLOR_ATTACHMENT3,
    ColorAttachment4 = GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = GL_COLOR_ATTACHMENT9,
    ColorAttachment10 = GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = GL_COLOR_ATTACHMENT15,
    Back = GL_BACK,
    None = GL_NONE,
}
impl UnsafeFromGLenum for ReadBufferMode {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ReadBufferMode::from_repr(val) else {
            println!(
                "Attempt to create a ReadBufferMode from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ReadBufferMode> for u32 {
    fn from(value: ReadBufferMode) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ReadBufferMode {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum HintTarget {
    LineSmoothHint = GL_LINE_SMOOTH_HINT,
    PolygonSmoothHint = GL_POLYGON_SMOOTH_HINT,
    TextureCompressionHint = GL_TEXTURE_COMPRESSION_HINT,
    FragmentShaderDerivativeHint = GL_FRAGMENT_SHADER_DERIVATIVE_HINT,
    ProgramBinaryRetrievableHint = GL_PROGRAM_BINARY_RETRIEVABLE_HINT,
}
impl UnsafeFromGLenum for HintTarget {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = HintTarget::from_repr(val) else {
            println!("Attempt to create a HintTarget from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<HintTarget> for u32 {
    fn from(value: HintTarget) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for HintTarget {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum SubroutineParameterName {
    UniformSize = GL_UNIFORM_SIZE,
    UniformNameLength = GL_UNIFORM_NAME_LENGTH,
    NumCompatibleSubroutines = GL_NUM_COMPATIBLE_SUBROUTINES,
    CompatibleSubroutines = GL_COMPATIBLE_SUBROUTINES,
}
impl UnsafeFromGLenum for SubroutineParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = SubroutineParameterName::from_repr(val) else {
            println!("Attempt to create a SubroutineParameterName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<SubroutineParameterName> for u32 {
    fn from(value: SubroutineParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for SubroutineParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum BlitFramebufferFilter {
    Nearest = GL_NEAREST,
    Linear = GL_LINEAR,
}
impl UnsafeFromGLenum for BlitFramebufferFilter {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = BlitFramebufferFilter::from_repr(val) else {
            println!("Attempt to create a BlitFramebufferFilter from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<BlitFramebufferFilter> for u32 {
    fn from(value: BlitFramebufferFilter) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for BlitFramebufferFilter {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum TriangleFace {
    Front = GL_FRONT,
    FrontAndBack = GL_FRONT_AND_BACK,
    Back = GL_BACK,
}
impl UnsafeFromGLenum for TriangleFace {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = TriangleFace::from_repr(val) else {
            println!("Attempt to create a TriangleFace from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<TriangleFace> for u32 {
    fn from(value: TriangleFace) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for TriangleFace {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum VertexArrayPName {
    VertexAttribArrayEnabled = GL_VERTEX_ATTRIB_ARRAY_ENABLED,
    VertexAttribArraySize = GL_VERTEX_ATTRIB_ARRAY_SIZE,
    VertexAttribArrayStride = GL_VERTEX_ATTRIB_ARRAY_STRIDE,
    VertexAttribArrayType = GL_VERTEX_ATTRIB_ARRAY_TYPE,
    VertexAttribArrayNormalized = GL_VERTEX_ATTRIB_ARRAY_NORMALIZED,
    VertexAttribArrayInteger = GL_VERTEX_ATTRIB_ARRAY_INTEGER,
    VertexAttribArrayDivisor = GL_VERTEX_ATTRIB_ARRAY_DIVISOR,
    VertexAttribArrayLong = GL_VERTEX_ATTRIB_ARRAY_LONG,
    VertexAttribRelativeOffset = GL_VERTEX_ATTRIB_RELATIVE_OFFSET,
}
impl UnsafeFromGLenum for VertexArrayPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = VertexArrayPName::from_repr(val) else {
            println!(
                "Attempt to create a VertexArrayPName from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<VertexArrayPName> for u32 {
    fn from(value: VertexArrayPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for VertexArrayPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum PixelFormat {
    UnsignedShort = GL_UNSIGNED_SHORT,
    UnsignedInt = GL_UNSIGNED_INT,
    DepthComponent = GL_DEPTH_COMPONENT,
    Red = GL_RED,
    Green = GL_GREEN,
    Blue = GL_BLUE,
    Alpha = GL_ALPHA,
    Rgb = GL_RGB,
    Rgba = GL_RGBA,
    Bgr = GL_BGR,
    Bgra = GL_BGRA,
    RedInteger = GL_RED_INTEGER,
    GreenInteger = GL_GREEN_INTEGER,
    BlueInteger = GL_BLUE_INTEGER,
    RgbInteger = GL_RGB_INTEGER,
    RgbaInteger = GL_RGBA_INTEGER,
    BgrInteger = GL_BGR_INTEGER,
    BgraInteger = GL_BGRA_INTEGER,
    DepthStencil = GL_DEPTH_STENCIL,
    Rg = GL_RG,
    RgInteger = GL_RG_INTEGER,
    StencilIndex = GL_STENCIL_INDEX,
}
impl UnsafeFromGLenum for PixelFormat {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = PixelFormat::from_repr(val) else {
            println!("Attempt to create a PixelFormat from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<PixelFormat> for u32 {
    fn from(value: PixelFormat) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for PixelFormat {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum VertexAttribIType {
    Byte = GL_BYTE,
    UnsignedByte = GL_UNSIGNED_BYTE,
    Short = GL_SHORT,
    UnsignedShort = GL_UNSIGNED_SHORT,
    Int = GL_INT,
    UnsignedInt = GL_UNSIGNED_INT,
}
impl UnsafeFromGLenum for VertexAttribIType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = VertexAttribIType::from_repr(val) else {
            println!(
                "Attempt to create a VertexAttribIType from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<VertexAttribIType> for u32 {
    fn from(value: VertexAttribIType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for VertexAttribIType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum BufferStorageTarget {
    ArrayBuffer = GL_ARRAY_BUFFER,
    ElementArrayBuffer = GL_ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = GL_PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = GL_PIXEL_UNPACK_BUFFER,
    TextureBuffer = GL_TEXTURE_BUFFER,
    CopyReadBuffer = GL_COPY_READ_BUFFER,
    CopyWriteBuffer = GL_COPY_WRITE_BUFFER,
    UniformBuffer = GL_UNIFORM_BUFFER,
    DrawIndirectBuffer = GL_DRAW_INDIRECT_BUFFER,
    AtomicCounterBuffer = GL_ATOMIC_COUNTER_BUFFER,
    DispatchIndirectBuffer = GL_DISPATCH_INDIRECT_BUFFER,
    ShaderStorageBuffer = GL_SHADER_STORAGE_BUFFER,
    TransformFeedbackBuffer = GL_TRANSFORM_FEEDBACK_BUFFER,
    QueryBuffer = GL_QUERY_BUFFER,
}
impl UnsafeFromGLenum for BufferStorageTarget {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = BufferStorageTarget::from_repr(val) else {
            println!(
                "Attempt to create a BufferStorageTarget from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<BufferStorageTarget> for u32 {
    fn from(value: BufferStorageTarget) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for BufferStorageTarget {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum BlendEquationModeEXT {
    FuncAdd = GL_FUNC_ADD,
    FuncReverseSubtract = GL_FUNC_REVERSE_SUBTRACT,
    FuncSubtract = GL_FUNC_SUBTRACT,
    Min = GL_MIN,
    Max = GL_MAX,
}
impl UnsafeFromGLenum for BlendEquationModeEXT {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = BlendEquationModeEXT::from_repr(val) else {
            println!("Attempt to create a BlendEquationModeEXT from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<BlendEquationModeEXT> for u32 {
    fn from(value: BlendEquationModeEXT) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for BlendEquationModeEXT {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum RenderbufferParameterName {
    RenderbufferSamples = GL_RENDERBUFFER_SAMPLES,
    RenderbufferWidth = GL_RENDERBUFFER_WIDTH,
    RenderbufferHeight = GL_RENDERBUFFER_HEIGHT,
    RenderbufferInternalFormat = GL_RENDERBUFFER_INTERNAL_FORMAT,
    RenderbufferRedSize = GL_RENDERBUFFER_RED_SIZE,
    RenderbufferGreenSize = GL_RENDERBUFFER_GREEN_SIZE,
    RenderbufferBlueSize = GL_RENDERBUFFER_BLUE_SIZE,
    RenderbufferAlphaSize = GL_RENDERBUFFER_ALPHA_SIZE,
    RenderbufferDepthSize = GL_RENDERBUFFER_DEPTH_SIZE,
    RenderbufferStencilSize = GL_RENDERBUFFER_STENCIL_SIZE,
}
impl UnsafeFromGLenum for RenderbufferParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = RenderbufferParameterName::from_repr(val) else {
            println!("Attempt to create a RenderbufferParameterName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<RenderbufferParameterName> for u32 {
    fn from(value: RenderbufferParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for RenderbufferParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum BufferTarget {
    ArrayBuffer = GL_ARRAY_BUFFER,
    ElementArrayBuffer = GL_ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = GL_PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = GL_PIXEL_UNPACK_BUFFER,
    TextureBuffer = GL_TEXTURE_BUFFER,
    CopyReadBuffer = GL_COPY_READ_BUFFER,
    CopyWriteBuffer = GL_COPY_WRITE_BUFFER,
    UniformBuffer = GL_UNIFORM_BUFFER,
    DrawIndirectBuffer = GL_DRAW_INDIRECT_BUFFER,
    AtomicCounterBuffer = GL_ATOMIC_COUNTER_BUFFER,
    DispatchIndirectBuffer = GL_DISPATCH_INDIRECT_BUFFER,
    ShaderStorageBuffer = GL_SHADER_STORAGE_BUFFER,
    TransformFeedbackBuffer = GL_TRANSFORM_FEEDBACK_BUFFER,
    QueryBuffer = GL_QUERY_BUFFER,
    ParameterBuffer = GL_PARAMETER_BUFFER,
}
impl UnsafeFromGLenum for BufferTarget {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = BufferTarget::from_repr(val) else {
            println!("Attempt to create a BufferTarget from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<BufferTarget> for u32 {
    fn from(value: BufferTarget) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for BufferTarget {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ProgramStagePName {
    ActiveSubroutines = GL_ACTIVE_SUBROUTINES,
    ActiveSubroutineUniforms = GL_ACTIVE_SUBROUTINE_UNIFORMS,
    ActiveSubroutineUniformLocations = GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS,
    ActiveSubroutineMaxLength = GL_ACTIVE_SUBROUTINE_MAX_LENGTH,
    ActiveSubroutineUniformMaxLength = GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH,
}
impl UnsafeFromGLenum for ProgramStagePName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ProgramStagePName::from_repr(val) else {
            println!(
                "Attempt to create a ProgramStagePName from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ProgramStagePName> for u32 {
    fn from(value: ProgramStagePName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ProgramStagePName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ProgramInterfacePName {
    ActiveResources = GL_ACTIVE_RESOURCES,
    MaxNameLength = GL_MAX_NAME_LENGTH,
    MaxNumActiveVariables = GL_MAX_NUM_ACTIVE_VARIABLES,
    MaxNumCompatibleSubroutines = GL_MAX_NUM_COMPATIBLE_SUBROUTINES,
}
impl UnsafeFromGLenum for ProgramInterfacePName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ProgramInterfacePName::from_repr(val) else {
            println!("Attempt to create a ProgramInterfacePName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ProgramInterfacePName> for u32 {
    fn from(value: ProgramInterfacePName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ProgramInterfacePName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum VertexAttribEnum {
    VertexAttribArrayBufferBinding = GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING,
    VertexAttribArrayEnabled = GL_VERTEX_ATTRIB_ARRAY_ENABLED,
    VertexAttribArraySize = GL_VERTEX_ATTRIB_ARRAY_SIZE,
    VertexAttribArrayStride = GL_VERTEX_ATTRIB_ARRAY_STRIDE,
    VertexAttribArrayType = GL_VERTEX_ATTRIB_ARRAY_TYPE,
    CurrentVertexAttrib = GL_CURRENT_VERTEX_ATTRIB,
    VertexAttribArrayNormalized = GL_VERTEX_ATTRIB_ARRAY_NORMALIZED,
    VertexAttribArrayInteger = GL_VERTEX_ATTRIB_ARRAY_INTEGER,
    VertexAttribArrayDivisor = GL_VERTEX_ATTRIB_ARRAY_DIVISOR,
}
impl UnsafeFromGLenum for VertexAttribEnum {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = VertexAttribEnum::from_repr(val) else {
            println!(
                "Attempt to create a VertexAttribEnum from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<VertexAttribEnum> for u32 {
    fn from(value: VertexAttribEnum) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for VertexAttribEnum {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum VertexAttribProperty {
    VertexAttribArrayBufferBinding = GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING,
    VertexAttribArrayEnabled = GL_VERTEX_ATTRIB_ARRAY_ENABLED,
    VertexAttribArraySize = GL_VERTEX_ATTRIB_ARRAY_SIZE,
    VertexAttribArrayStride = GL_VERTEX_ATTRIB_ARRAY_STRIDE,
    VertexAttribArrayType = GL_VERTEX_ATTRIB_ARRAY_TYPE,
    CurrentVertexAttrib = GL_CURRENT_VERTEX_ATTRIB,
    VertexAttribArrayNormalized = GL_VERTEX_ATTRIB_ARRAY_NORMALIZED,
    VertexAttribArrayInteger = GL_VERTEX_ATTRIB_ARRAY_INTEGER,
    VertexAttribArrayDivisor = GL_VERTEX_ATTRIB_ARRAY_DIVISOR,
    VertexAttribArrayLong = GL_VERTEX_ATTRIB_ARRAY_LONG,
    VertexAttribBinding = GL_VERTEX_ATTRIB_BINDING,
    VertexAttribRelativeOffset = GL_VERTEX_ATTRIB_RELATIVE_OFFSET,
}
impl UnsafeFromGLenum for VertexAttribProperty {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = VertexAttribProperty::from_repr(val) else {
            println!("Attempt to create a VertexAttribProperty from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<VertexAttribProperty> for u32 {
    fn from(value: VertexAttribProperty) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for VertexAttribProperty {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum InternalFormatPName {
    Samples = GL_SAMPLES,
    TextureCompressed = GL_TEXTURE_COMPRESSED,
    NumSampleCounts = GL_NUM_SAMPLE_COUNTS,
    ImageFormatCompatibilityType = GL_IMAGE_FORMAT_COMPATIBILITY_TYPE,
    InternalformatSupported = GL_INTERNALFORMAT_SUPPORTED,
    InternalformatPreferred = GL_INTERNALFORMAT_PREFERRED,
    InternalformatRedSize = GL_INTERNALFORMAT_RED_SIZE,
    InternalformatGreenSize = GL_INTERNALFORMAT_GREEN_SIZE,
    InternalformatBlueSize = GL_INTERNALFORMAT_BLUE_SIZE,
    InternalformatAlphaSize = GL_INTERNALFORMAT_ALPHA_SIZE,
    InternalformatDepthSize = GL_INTERNALFORMAT_DEPTH_SIZE,
    InternalformatStencilSize = GL_INTERNALFORMAT_STENCIL_SIZE,
    InternalformatSharedSize = GL_INTERNALFORMAT_SHARED_SIZE,
    InternalformatRedType = GL_INTERNALFORMAT_RED_TYPE,
    InternalformatGreenType = GL_INTERNALFORMAT_GREEN_TYPE,
    InternalformatBlueType = GL_INTERNALFORMAT_BLUE_TYPE,
    InternalformatAlphaType = GL_INTERNALFORMAT_ALPHA_TYPE,
    InternalformatDepthType = GL_INTERNALFORMAT_DEPTH_TYPE,
    InternalformatStencilType = GL_INTERNALFORMAT_STENCIL_TYPE,
    MaxWidth = GL_MAX_WIDTH,
    MaxHeight = GL_MAX_HEIGHT,
    MaxDepth = GL_MAX_DEPTH,
    MaxLayers = GL_MAX_LAYERS,
    ColorComponents = GL_COLOR_COMPONENTS,
    ColorRenderable = GL_COLOR_RENDERABLE,
    DepthRenderable = GL_DEPTH_RENDERABLE,
    StencilRenderable = GL_STENCIL_RENDERABLE,
    FramebufferRenderable = GL_FRAMEBUFFER_RENDERABLE,
    FramebufferRenderableLayered = GL_FRAMEBUFFER_RENDERABLE_LAYERED,
    FramebufferBlend = GL_FRAMEBUFFER_BLEND,
    ReadPixels = GL_READ_PIXELS,
    ReadPixelsFormat = GL_READ_PIXELS_FORMAT,
    ReadPixelsType = GL_READ_PIXELS_TYPE,
    TextureImageFormat = GL_TEXTURE_IMAGE_FORMAT,
    TextureImageType = GL_TEXTURE_IMAGE_TYPE,
    GetTextureImageFormat = GL_GET_TEXTURE_IMAGE_FORMAT,
    GetTextureImageType = GL_GET_TEXTURE_IMAGE_TYPE,
    Mipmap = GL_MIPMAP,
    AutoGenerateMipmap = GL_AUTO_GENERATE_MIPMAP,
    ColorEncoding = GL_COLOR_ENCODING,
    SrgbRead = GL_SRGB_READ,
    SrgbWrite = GL_SRGB_WRITE,
    Filter = GL_FILTER,
    VertexTexture = GL_VERTEX_TEXTURE,
    TessControlTexture = GL_TESS_CONTROL_TEXTURE,
    TessEvaluationTexture = GL_TESS_EVALUATION_TEXTURE,
    GeometryTexture = GL_GEOMETRY_TEXTURE,
    FragmentTexture = GL_FRAGMENT_TEXTURE,
    ComputeTexture = GL_COMPUTE_TEXTURE,
    TextureShadow = GL_TEXTURE_SHADOW,
    TextureGather = GL_TEXTURE_GATHER,
    TextureGatherShadow = GL_TEXTURE_GATHER_SHADOW,
    ShaderImageLoad = GL_SHADER_IMAGE_LOAD,
    ShaderImageStore = GL_SHADER_IMAGE_STORE,
    ShaderImageAtomic = GL_SHADER_IMAGE_ATOMIC,
    ImageTexelSize = GL_IMAGE_TEXEL_SIZE,
    ImageCompatibilityClass = GL_IMAGE_COMPATIBILITY_CLASS,
    ImagePixelFormat = GL_IMAGE_PIXEL_FORMAT,
    ImagePixelType = GL_IMAGE_PIXEL_TYPE,
    SimultaneousTextureAndDepthTest = GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST,
    SimultaneousTextureAndStencilTest = GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST,
    SimultaneousTextureAndDepthWrite = GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE,
    SimultaneousTextureAndStencilWrite = GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE,
    TextureCompressedBlockWidth = GL_TEXTURE_COMPRESSED_BLOCK_WIDTH,
    TextureCompressedBlockHeight = GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT,
    TextureCompressedBlockSize = GL_TEXTURE_COMPRESSED_BLOCK_SIZE,
    ClearBuffer = GL_CLEAR_BUFFER,
    TextureView = GL_TEXTURE_VIEW,
    ViewCompatibilityClass = GL_VIEW_COMPATIBILITY_CLASS,
    ClearTexture = GL_CLEAR_TEXTURE,
}
impl UnsafeFromGLenum for InternalFormatPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = InternalFormatPName::from_repr(val) else {
            println!(
                "Attempt to create a InternalFormatPName from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<InternalFormatPName> for u32 {
    fn from(value: InternalFormatPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for InternalFormatPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum PolygonMode {
    Point = GL_POINT,
    Line = GL_LINE,
    Fill = GL_FILL,
}
impl UnsafeFromGLenum for PolygonMode {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = PolygonMode::from_repr(val) else {
            println!("Attempt to create a PolygonMode from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<PolygonMode> for u32 {
    fn from(value: PolygonMode) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for PolygonMode {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ProgramInterface {
    Uniform = GL_UNIFORM,
    UniformBlock = GL_UNIFORM_BLOCK,
    ProgramInput = GL_PROGRAM_INPUT,
    ProgramOutput = GL_PROGRAM_OUTPUT,
    BufferVariable = GL_BUFFER_VARIABLE,
    ShaderStorageBlock = GL_SHADER_STORAGE_BLOCK,
    VertexSubroutine = GL_VERTEX_SUBROUTINE,
    TessControlSubroutine = GL_TESS_CONTROL_SUBROUTINE,
    TessEvaluationSubroutine = GL_TESS_EVALUATION_SUBROUTINE,
    GeometrySubroutine = GL_GEOMETRY_SUBROUTINE,
    FragmentSubroutine = GL_FRAGMENT_SUBROUTINE,
    ComputeSubroutine = GL_COMPUTE_SUBROUTINE,
    VertexSubroutineUniform = GL_VERTEX_SUBROUTINE_UNIFORM,
    TessControlSubroutineUniform = GL_TESS_CONTROL_SUBROUTINE_UNIFORM,
    TessEvaluationSubroutineUniform = GL_TESS_EVALUATION_SUBROUTINE_UNIFORM,
    GeometrySubroutineUniform = GL_GEOMETRY_SUBROUTINE_UNIFORM,
    FragmentSubroutineUniform = GL_FRAGMENT_SUBROUTINE_UNIFORM,
    ComputeSubroutineUniform = GL_COMPUTE_SUBROUTINE_UNIFORM,
    TransformFeedbackVarying = GL_TRANSFORM_FEEDBACK_VARYING,
    TransformFeedbackBuffer = GL_TRANSFORM_FEEDBACK_BUFFER,
}
impl UnsafeFromGLenum for ProgramInterface {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ProgramInterface::from_repr(val) else {
            println!(
                "Attempt to create a ProgramInterface from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ProgramInterface> for u32 {
    fn from(value: ProgramInterface) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ProgramInterface {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum TextureParameterName {
    TextureWidth = GL_TEXTURE_WIDTH,
    TextureHeight = GL_TEXTURE_HEIGHT,
    TextureBorderColor = GL_TEXTURE_BORDER_COLOR,
    TextureMagFilter = GL_TEXTURE_MAG_FILTER,
    TextureMinFilter = GL_TEXTURE_MIN_FILTER,
    TextureWrapS = GL_TEXTURE_WRAP_S,
    TextureWrapT = GL_TEXTURE_WRAP_T,
    TextureInternalFormat = GL_TEXTURE_INTERNAL_FORMAT,
    TextureRedSize = GL_TEXTURE_RED_SIZE,
    TextureGreenSize = GL_TEXTURE_GREEN_SIZE,
    TextureBlueSize = GL_TEXTURE_BLUE_SIZE,
    TextureAlphaSize = GL_TEXTURE_ALPHA_SIZE,
    TextureWrapR = GL_TEXTURE_WRAP_R,
    TextureMinLod = GL_TEXTURE_MIN_LOD,
    TextureMaxLod = GL_TEXTURE_MAX_LOD,
    TextureBaseLevel = GL_TEXTURE_BASE_LEVEL,
    TextureMaxLevel = GL_TEXTURE_MAX_LEVEL,
    TextureLodBias = GL_TEXTURE_LOD_BIAS,
    TextureCompareMode = GL_TEXTURE_COMPARE_MODE,
    TextureCompareFunc = GL_TEXTURE_COMPARE_FUNC,
    TextureSwizzleR = GL_TEXTURE_SWIZZLE_R,
    TextureSwizzleG = GL_TEXTURE_SWIZZLE_G,
    TextureSwizzleB = GL_TEXTURE_SWIZZLE_B,
    TextureSwizzleA = GL_TEXTURE_SWIZZLE_A,
    TextureSwizzleRgba = GL_TEXTURE_SWIZZLE_RGBA,
    DepthStencilTextureMode = GL_DEPTH_STENCIL_TEXTURE_MODE,
    TextureMaxAnisotropy = GL_TEXTURE_MAX_ANISOTROPY,
}
impl UnsafeFromGLenum for TextureParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = TextureParameterName::from_repr(val) else {
            println!("Attempt to create a TextureParameterName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<TextureParameterName> for u32 {
    fn from(value: TextureParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for TextureParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ProgramProperty {
    DeleteStatus = GL_DELETE_STATUS,
    LinkStatus = GL_LINK_STATUS,
    ValidateStatus = GL_VALIDATE_STATUS,
    InfoLogLength = GL_INFO_LOG_LENGTH,
    AttachedShaders = GL_ATTACHED_SHADERS,
    ActiveUniforms = GL_ACTIVE_UNIFORMS,
    ActiveUniformMaxLength = GL_ACTIVE_UNIFORM_MAX_LENGTH,
    ActiveAttributes = GL_ACTIVE_ATTRIBUTES,
    ActiveAttributeMaxLength = GL_ACTIVE_ATTRIBUTE_MAX_LENGTH,
    TransformFeedbackVaryingMaxLength = GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
    TransformFeedbackBufferMode = GL_TRANSFORM_FEEDBACK_BUFFER_MODE,
    TransformFeedbackVaryings = GL_TRANSFORM_FEEDBACK_VARYINGS,
    ActiveUniformBlockMaxNameLength = GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
    ActiveUniformBlocks = GL_ACTIVE_UNIFORM_BLOCKS,
    GeometryVerticesOut = GL_GEOMETRY_VERTICES_OUT,
    GeometryInputType = GL_GEOMETRY_INPUT_TYPE,
    GeometryOutputType = GL_GEOMETRY_OUTPUT_TYPE,
    ProgramBinaryLength = GL_PROGRAM_BINARY_LENGTH,
    ActiveAtomicCounterBuffers = GL_ACTIVE_ATOMIC_COUNTER_BUFFERS,
    ComputeWorkGroupSize = GL_COMPUTE_WORK_GROUP_SIZE,
}
impl UnsafeFromGLenum for ProgramProperty {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ProgramProperty::from_repr(val) else {
            println!(
                "Attempt to create a ProgramProperty from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ProgramProperty> for u32 {
    fn from(value: ProgramProperty) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ProgramProperty {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum EnableCap {
    LineSmooth = GL_LINE_SMOOTH,
    PolygonSmooth = GL_POLYGON_SMOOTH,
    CullFace = GL_CULL_FACE,
    DepthTest = GL_DEPTH_TEST,
    StencilTest = GL_STENCIL_TEST,
    Dither = GL_DITHER,
    Blend = GL_BLEND,
    ScissorTest = GL_SCISSOR_TEST,
    Texture1D = GL_TEXTURE_1D,
    Texture2D = GL_TEXTURE_2D,
    ColorLogicOp = GL_COLOR_LOGIC_OP,
    PolygonOffsetPoint = GL_POLYGON_OFFSET_POINT,
    PolygonOffsetLine = GL_POLYGON_OFFSET_LINE,
    PolygonOffsetFill = GL_POLYGON_OFFSET_FILL,
    Multisample = GL_MULTISAMPLE,
    SampleAlphaToCoverage = GL_SAMPLE_ALPHA_TO_COVERAGE,
    SampleAlphaToOne = GL_SAMPLE_ALPHA_TO_ONE,
    SampleCoverage = GL_SAMPLE_COVERAGE,
    TextureCubeMap = GL_TEXTURE_CUBE_MAP,
    ClipDistance0 = GL_CLIP_DISTANCE0,
    ClipDistance1 = GL_CLIP_DISTANCE1,
    ClipDistance2 = GL_CLIP_DISTANCE2,
    ClipDistance3 = GL_CLIP_DISTANCE3,
    ClipDistance4 = GL_CLIP_DISTANCE4,
    ClipDistance5 = GL_CLIP_DISTANCE5,
    ClipDistance6 = GL_CLIP_DISTANCE6,
    ClipDistance7 = GL_CLIP_DISTANCE7,
    RasterizerDiscard = GL_RASTERIZER_DISCARD,
    FramebufferSrgb = GL_FRAMEBUFFER_SRGB,
    TextureRectangle = GL_TEXTURE_RECTANGLE,
    PrimitiveRestart = GL_PRIMITIVE_RESTART,
    ProgramPointSize = GL_PROGRAM_POINT_SIZE,
    DepthClamp = GL_DEPTH_CLAMP,
    TextureCubeMapSeamless = GL_TEXTURE_CUBE_MAP_SEAMLESS,
    SampleMask = GL_SAMPLE_MASK,
    SampleShading = GL_SAMPLE_SHADING,
    PrimitiveRestartFixedIndex = GL_PRIMITIVE_RESTART_FIXED_INDEX,
    DebugOutputSynchronous = GL_DEBUG_OUTPUT_SYNCHRONOUS,
    VertexArray = GL_VERTEX_ARRAY,
    DebugOutput = GL_DEBUG_OUTPUT,
}
impl UnsafeFromGLenum for EnableCap {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = EnableCap::from_repr(val) else {
            println!("Attempt to create a EnableCap from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<EnableCap> for u32 {
    fn from(value: EnableCap) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for EnableCap {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum InternalFormat {
    DepthComponent = GL_DEPTH_COMPONENT,
    Red = GL_RED,
    Rgb = GL_RGB,
    Rgba = GL_RGBA,
    R3G3B2 = GL_R3_G3_B2,
    Rgb4 = GL_RGB4,
    Rgb5 = GL_RGB5,
    Rgb8 = GL_RGB8,
    Rgb10 = GL_RGB10,
    Rgb12 = GL_RGB12,
    Rgb16 = GL_RGB16,
    Rgba2 = GL_RGBA2,
    Rgba4 = GL_RGBA4,
    Rgb5A1 = GL_RGB5_A1,
    Rgba8 = GL_RGBA8,
    Rgb10A2 = GL_RGB10_A2,
    Rgba12 = GL_RGBA12,
    Rgba16 = GL_RGBA16,
    CompressedRgb = GL_COMPRESSED_RGB,
    CompressedRgba = GL_COMPRESSED_RGBA,
    DepthComponent16 = GL_DEPTH_COMPONENT16,
    DepthComponent24 = GL_DEPTH_COMPONENT24,
    DepthComponent32 = GL_DEPTH_COMPONENT32,
    Srgb = GL_SRGB,
    Srgb8 = GL_SRGB8,
    SrgbAlpha = GL_SRGB_ALPHA,
    Srgb8Alpha8 = GL_SRGB8_ALPHA8,
    CompressedSrgb = GL_COMPRESSED_SRGB,
    CompressedSrgbAlpha = GL_COMPRESSED_SRGB_ALPHA,
    CompressedRed = GL_COMPRESSED_RED,
    CompressedRg = GL_COMPRESSED_RG,
    Rgba32f = GL_RGBA32F,
    Rgb32f = GL_RGB32F,
    Rgba16f = GL_RGBA16F,
    Rgb16f = GL_RGB16F,
    R11fG11fB10f = GL_R11F_G11F_B10F,
    Rgb9E5 = GL_RGB9_E5,
    Rgba32ui = GL_RGBA32UI,
    Rgb32ui = GL_RGB32UI,
    Rgba16ui = GL_RGBA16UI,
    Rgb16ui = GL_RGB16UI,
    Rgba8ui = GL_RGBA8UI,
    Rgb8ui = GL_RGB8UI,
    Rgba32i = GL_RGBA32I,
    Rgb32i = GL_RGB32I,
    Rgba16i = GL_RGBA16I,
    Rgb16i = GL_RGB16I,
    Rgba8i = GL_RGBA8I,
    Rgb8i = GL_RGB8I,
    DepthComponent32f = GL_DEPTH_COMPONENT32F,
    Depth32fStencil8 = GL_DEPTH32F_STENCIL8,
    DepthStencil = GL_DEPTH_STENCIL,
    Depth24Stencil8 = GL_DEPTH24_STENCIL8,
    StencilIndex1 = GL_STENCIL_INDEX1,
    StencilIndex4 = GL_STENCIL_INDEX4,
    StencilIndex16 = GL_STENCIL_INDEX16,
    CompressedRedRgtc1 = GL_COMPRESSED_RED_RGTC1,
    CompressedSignedRedRgtc1 = GL_COMPRESSED_SIGNED_RED_RGTC1,
    CompressedRgRgtc2 = GL_COMPRESSED_RG_RGTC2,
    CompressedSignedRgRgtc2 = GL_COMPRESSED_SIGNED_RG_RGTC2,
    Rg = GL_RG,
    R8 = GL_R8,
    R16 = GL_R16,
    Rg8 = GL_RG8,
    Rg16 = GL_RG16,
    R16f = GL_R16F,
    R32f = GL_R32F,
    Rg16f = GL_RG16F,
    Rg32f = GL_RG32F,
    R8i = GL_R8I,
    R8ui = GL_R8UI,
    R16i = GL_R16I,
    R16ui = GL_R16UI,
    R32i = GL_R32I,
    R32ui = GL_R32UI,
    Rg8i = GL_RG8I,
    Rg8ui = GL_RG8UI,
    Rg16i = GL_RG16I,
    Rg16ui = GL_RG16UI,
    Rg32i = GL_RG32I,
    Rg32ui = GL_RG32UI,
    R8Snorm = GL_R8_SNORM,
    Rg8Snorm = GL_RG8_SNORM,
    Rgb8Snorm = GL_RGB8_SNORM,
    Rgba8Snorm = GL_RGBA8_SNORM,
    R16Snorm = GL_R16_SNORM,
    Rg16Snorm = GL_RG16_SNORM,
    Rgb16Snorm = GL_RGB16_SNORM,
    Rgba16Snorm = GL_RGBA16_SNORM,
    Rgb10A2ui = GL_RGB10_A2UI,
    Rgb565 = GL_RGB565,
    CompressedRgbaBptcUnorm = GL_COMPRESSED_RGBA_BPTC_UNORM,
    CompressedSrgbAlphaBptcUnorm = GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM,
    CompressedRgbBptcSignedFloat = GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT,
    CompressedRgbBptcUnsignedFloat = GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT,
    CompressedRgb8Etc2 = GL_COMPRESSED_RGB8_ETC2,
    CompressedSrgb8Etc2 = GL_COMPRESSED_SRGB8_ETC2,
    CompressedRgb8PunchthroughAlpha1Etc2 = GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2,
    CompressedSrgb8PunchthroughAlpha1Etc2 = GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2,
    CompressedRgba8Etc2Eac = GL_COMPRESSED_RGBA8_ETC2_EAC,
    CompressedSrgb8Alpha8Etc2Eac = GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC,
    CompressedR11Eac = GL_COMPRESSED_R11_EAC,
    CompressedSignedR11Eac = GL_COMPRESSED_SIGNED_R11_EAC,
    CompressedRg11Eac = GL_COMPRESSED_RG11_EAC,
    CompressedSignedRg11Eac = GL_COMPRESSED_SIGNED_RG11_EAC,
    StencilIndex = GL_STENCIL_INDEX,
    StencilIndex8 = GL_STENCIL_INDEX8,
}
impl UnsafeFromGLenum for InternalFormat {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = InternalFormat::from_repr(val) else {
            println!(
                "Attempt to create a InternalFormat from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<InternalFormat> for u32 {
    fn from(value: InternalFormat) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for InternalFormat {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum VertexAttribType {
    Byte = GL_BYTE,
    UnsignedByte = GL_UNSIGNED_BYTE,
    Short = GL_SHORT,
    UnsignedShort = GL_UNSIGNED_SHORT,
    Int = GL_INT,
    UnsignedInt = GL_UNSIGNED_INT,
    Float = GL_FLOAT,
    Double = GL_DOUBLE,
    UnsignedInt2101010Rev = GL_UNSIGNED_INT_2_10_10_10_REV,
    HalfFloat = GL_HALF_FLOAT,
    Int2101010Rev = GL_INT_2_10_10_10_REV,
    Fixed = GL_FIXED,
    UnsignedInt10F11F11FRev = GL_UNSIGNED_INT_10F_11F_11F_REV,
}
impl UnsafeFromGLenum for VertexAttribType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = VertexAttribType::from_repr(val) else {
            println!(
                "Attempt to create a VertexAttribType from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<VertexAttribType> for u32 {
    fn from(value: VertexAttribType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for VertexAttribType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ObjectIdentifier {
    Texture = GL_TEXTURE,
    Framebuffer = GL_FRAMEBUFFER,
    Renderbuffer = GL_RENDERBUFFER,
    TransformFeedback = GL_TRANSFORM_FEEDBACK,
    Buffer = GL_BUFFER,
    Shader = GL_SHADER,
    Program = GL_PROGRAM,
    VertexArray = GL_VERTEX_ARRAY,
    Query = GL_QUERY,
    ProgramPipeline = GL_PROGRAM_PIPELINE,
    Sampler = GL_SAMPLER,
}
impl UnsafeFromGLenum for ObjectIdentifier {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ObjectIdentifier::from_repr(val) else {
            println!(
                "Attempt to create a ObjectIdentifier from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ObjectIdentifier> for u32 {
    fn from(value: ObjectIdentifier) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ObjectIdentifier {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum InvalidateFramebufferAttachment {
    Color = GL_COLOR,
    Depth = GL_DEPTH,
    Stencil = GL_STENCIL,
    DepthStencilAttachment = GL_DEPTH_STENCIL_ATTACHMENT,
    ColorAttachment0 = GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = GL_COLOR_ATTACHMENT1,
    ColorAttachment2 = GL_COLOR_ATTACHMENT2,
    ColorAttachment3 = GL_COLOR_ATTACHMENT3,
    ColorAttachment4 = GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = GL_COLOR_ATTACHMENT9,
    ColorAttachment10 = GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = GL_COLOR_ATTACHMENT15,
    ColorAttachment16 = GL_COLOR_ATTACHMENT16,
    ColorAttachment17 = GL_COLOR_ATTACHMENT17,
    ColorAttachment18 = GL_COLOR_ATTACHMENT18,
    ColorAttachment19 = GL_COLOR_ATTACHMENT19,
    ColorAttachment20 = GL_COLOR_ATTACHMENT20,
    ColorAttachment21 = GL_COLOR_ATTACHMENT21,
    ColorAttachment22 = GL_COLOR_ATTACHMENT22,
    ColorAttachment23 = GL_COLOR_ATTACHMENT23,
    ColorAttachment24 = GL_COLOR_ATTACHMENT24,
    ColorAttachment25 = GL_COLOR_ATTACHMENT25,
    ColorAttachment26 = GL_COLOR_ATTACHMENT26,
    ColorAttachment27 = GL_COLOR_ATTACHMENT27,
    ColorAttachment28 = GL_COLOR_ATTACHMENT28,
    ColorAttachment29 = GL_COLOR_ATTACHMENT29,
    ColorAttachment30 = GL_COLOR_ATTACHMENT30,
    ColorAttachment31 = GL_COLOR_ATTACHMENT31,
    DepthAttachment = GL_DEPTH_ATTACHMENT,
}
impl UnsafeFromGLenum for InvalidateFramebufferAttachment {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = InvalidateFramebufferAttachment::from_repr(val) else {
            println!("Attempt to create a InvalidateFramebufferAttachment from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<InvalidateFramebufferAttachment> for u32 {
    fn from(value: InvalidateFramebufferAttachment) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for InvalidateFramebufferAttachment {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum HintMode {
    DontCare = GL_DONT_CARE,
    Fastest = GL_FASTEST,
    Nicest = GL_NICEST,
}
impl UnsafeFromGLenum for HintMode {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = HintMode::from_repr(val) else {
            println!("Attempt to create a HintMode from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<HintMode> for u32 {
    fn from(value: HintMode) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for HintMode {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum BlendingFactor {
    Zero = GL_ZERO,
    One = GL_ONE,
    SrcColor = GL_SRC_COLOR,
    OneMinusSrcColor = GL_ONE_MINUS_SRC_COLOR,
    SrcAlpha = GL_SRC_ALPHA,
    OneMinusSrcAlpha = GL_ONE_MINUS_SRC_ALPHA,
    DstAlpha = GL_DST_ALPHA,
    OneMinusDstAlpha = GL_ONE_MINUS_DST_ALPHA,
    DstColor = GL_DST_COLOR,
    OneMinusDstColor = GL_ONE_MINUS_DST_COLOR,
    SrcAlphaSaturate = GL_SRC_ALPHA_SATURATE,
    ConstantColor = GL_CONSTANT_COLOR,
    OneMinusConstantColor = GL_ONE_MINUS_CONSTANT_COLOR,
    ConstantAlpha = GL_CONSTANT_ALPHA,
    OneMinusConstantAlpha = GL_ONE_MINUS_CONSTANT_ALPHA,
    Src1Alpha = GL_SRC1_ALPHA,
    Src1Color = GL_SRC1_COLOR,
    OneMinusSrc1Color = GL_ONE_MINUS_SRC1_COLOR,
    OneMinusSrc1Alpha = GL_ONE_MINUS_SRC1_ALPHA,
}
impl UnsafeFromGLenum for BlendingFactor {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = BlendingFactor::from_repr(val) else {
            println!(
                "Attempt to create a BlendingFactor from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<BlendingFactor> for u32 {
    fn from(value: BlendingFactor) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for BlendingFactor {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ClipControlOrigin {
    LowerLeft = GL_LOWER_LEFT,
    UpperLeft = GL_UPPER_LEFT,
}
impl UnsafeFromGLenum for ClipControlOrigin {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ClipControlOrigin::from_repr(val) else {
            println!(
                "Attempt to create a ClipControlOrigin from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ClipControlOrigin> for u32 {
    fn from(value: ClipControlOrigin) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ClipControlOrigin {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ConditionalRenderMode {
    QueryWait = GL_QUERY_WAIT,
    QueryNoWait = GL_QUERY_NO_WAIT,
    QueryByRegionWait = GL_QUERY_BY_REGION_WAIT,
    QueryByRegionNoWait = GL_QUERY_BY_REGION_NO_WAIT,
    QueryWaitInverted = GL_QUERY_WAIT_INVERTED,
    QueryNoWaitInverted = GL_QUERY_NO_WAIT_INVERTED,
    QueryByRegionWaitInverted = GL_QUERY_BY_REGION_WAIT_INVERTED,
    QueryByRegionNoWaitInverted = GL_QUERY_BY_REGION_NO_WAIT_INVERTED,
}
impl UnsafeFromGLenum for ConditionalRenderMode {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ConditionalRenderMode::from_repr(val) else {
            println!("Attempt to create a ConditionalRenderMode from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ConditionalRenderMode> for u32 {
    fn from(value: ConditionalRenderMode) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ConditionalRenderMode {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum FramebufferTarget {
    ReadFramebuffer = GL_READ_FRAMEBUFFER,
    DrawFramebuffer = GL_DRAW_FRAMEBUFFER,
    Framebuffer = GL_FRAMEBUFFER,
}
impl UnsafeFromGLenum for FramebufferTarget {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = FramebufferTarget::from_repr(val) else {
            println!(
                "Attempt to create a FramebufferTarget from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<FramebufferTarget> for u32 {
    fn from(value: FramebufferTarget) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for FramebufferTarget {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum VertexBufferObjectUsage {
    StreamDraw = GL_STREAM_DRAW,
    StreamRead = GL_STREAM_READ,
    StreamCopy = GL_STREAM_COPY,
    StaticDraw = GL_STATIC_DRAW,
    StaticRead = GL_STATIC_READ,
    StaticCopy = GL_STATIC_COPY,
    DynamicDraw = GL_DYNAMIC_DRAW,
    DynamicRead = GL_DYNAMIC_READ,
    DynamicCopy = GL_DYNAMIC_COPY,
}
impl UnsafeFromGLenum for VertexBufferObjectUsage {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = VertexBufferObjectUsage::from_repr(val) else {
            println!("Attempt to create a VertexBufferObjectUsage from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<VertexBufferObjectUsage> for u32 {
    fn from(value: VertexBufferObjectUsage) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for VertexBufferObjectUsage {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum PipelineParameterName {
    FragmentShader = GL_FRAGMENT_SHADER,
    VertexShader = GL_VERTEX_SHADER,
    InfoLogLength = GL_INFO_LOG_LENGTH,
    GeometryShader = GL_GEOMETRY_SHADER,
    TessEvaluationShader = GL_TESS_EVALUATION_SHADER,
    TessControlShader = GL_TESS_CONTROL_SHADER,
    ActiveProgram = GL_ACTIVE_PROGRAM,
}
impl UnsafeFromGLenum for PipelineParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = PipelineParameterName::from_repr(val) else {
            println!("Attempt to create a PipelineParameterName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<PipelineParameterName> for u32 {
    fn from(value: PipelineParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for PipelineParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum FramebufferAttachmentParameterName {
    FramebufferAttachmentColorEncoding = GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING,
    FramebufferAttachmentComponentType = GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE,
    FramebufferAttachmentRedSize = GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE,
    FramebufferAttachmentGreenSize = GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE,
    FramebufferAttachmentBlueSize = GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE,
    FramebufferAttachmentAlphaSize = GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE,
    FramebufferAttachmentDepthSize = GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE,
    FramebufferAttachmentStencilSize = GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE,
    FramebufferAttachmentObjectType = GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE,
    FramebufferAttachmentObjectName = GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME,
    FramebufferAttachmentTextureLevel = GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL,
    FramebufferAttachmentTextureCubeMapFace = GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE,
    FramebufferAttachmentTextureLayer = GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER,
    FramebufferAttachmentLayered = GL_FRAMEBUFFER_ATTACHMENT_LAYERED,
}
impl UnsafeFromGLenum for FramebufferAttachmentParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = FramebufferAttachmentParameterName::from_repr(val) else {
            println!("Attempt to create a FramebufferAttachmentParameterName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<FramebufferAttachmentParameterName> for u32 {
    fn from(value: FramebufferAttachmentParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for FramebufferAttachmentParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum SamplerParameterF {
    TextureBorderColor = GL_TEXTURE_BORDER_COLOR,
    TextureMinLod = GL_TEXTURE_MIN_LOD,
    TextureMaxLod = GL_TEXTURE_MAX_LOD,
    TextureLodBias = GL_TEXTURE_LOD_BIAS,
    TextureMaxAnisotropy = GL_TEXTURE_MAX_ANISOTROPY,
}
impl UnsafeFromGLenum for SamplerParameterF {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = SamplerParameterF::from_repr(val) else {
            println!(
                "Attempt to create a SamplerParameterF from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<SamplerParameterF> for u32 {
    fn from(value: SamplerParameterF) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for SamplerParameterF {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum UniformBlockPName {
    UniformBlockBinding = GL_UNIFORM_BLOCK_BINDING,
    UniformBlockDataSize = GL_UNIFORM_BLOCK_DATA_SIZE,
    UniformBlockNameLength = GL_UNIFORM_BLOCK_NAME_LENGTH,
    UniformBlockActiveUniforms = GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS,
    UniformBlockActiveUniformIndices = GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES,
    UniformBlockReferencedByVertexShader = GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER,
    UniformBlockReferencedByGeometryShader = GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER,
    UniformBlockReferencedByFragmentShader = GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER,
    UniformBlockReferencedByTessControlShader = GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER,
    UniformBlockReferencedByTessEvaluationShader =
        GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER,
    UniformBlockReferencedByComputeShader = GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER,
}
impl UnsafeFromGLenum for UniformBlockPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = UniformBlockPName::from_repr(val) else {
            println!(
                "Attempt to create a UniformBlockPName from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<UniformBlockPName> for u32 {
    fn from(value: UniformBlockPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for UniformBlockPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum SyncParameterName {
    ObjectType = GL_OBJECT_TYPE,
    SyncCondition = GL_SYNC_CONDITION,
    SyncStatus = GL_SYNC_STATUS,
    SyncFlags = GL_SYNC_FLAGS,
}
impl UnsafeFromGLenum for SyncParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = SyncParameterName::from_repr(val) else {
            println!(
                "Attempt to create a SyncParameterName from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<SyncParameterName> for u32 {
    fn from(value: SyncParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for SyncParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum StencilFunction {
    Never = GL_NEVER,
    Less = GL_LESS,
    Equal = GL_EQUAL,
    Lequal = GL_LEQUAL,
    Greater = GL_GREATER,
    Notequal = GL_NOTEQUAL,
    Gequal = GL_GEQUAL,
    Always = GL_ALWAYS,
}
impl UnsafeFromGLenum for StencilFunction {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = StencilFunction::from_repr(val) else {
            println!(
                "Attempt to create a StencilFunction from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<StencilFunction> for u32 {
    fn from(value: StencilFunction) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for StencilFunction {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum DebugSeverity {
    DontCare = GL_DONT_CARE,
    DebugSeverityHigh = GL_DEBUG_SEVERITY_HIGH,
    DebugSeverityMedium = GL_DEBUG_SEVERITY_MEDIUM,
    DebugSeverityLow = GL_DEBUG_SEVERITY_LOW,
    DebugSeverityNotification = GL_DEBUG_SEVERITY_NOTIFICATION,
}
impl UnsafeFromGLenum for DebugSeverity {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = DebugSeverity::from_repr(val) else {
            println!("Attempt to create a DebugSeverity from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<DebugSeverity> for u32 {
    fn from(value: DebugSeverity) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for DebugSeverity {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ProgramResourceProperty {
    NumCompatibleSubroutines = GL_NUM_COMPATIBLE_SUBROUTINES,
    CompatibleSubroutines = GL_COMPATIBLE_SUBROUTINES,
    Uniform = GL_UNIFORM,
    NameLength = GL_NAME_LENGTH,
    Type = GL_TYPE,
    ArraySize = GL_ARRAY_SIZE,
    Offset = GL_OFFSET,
    BlockIndex = GL_BLOCK_INDEX,
    ArrayStride = GL_ARRAY_STRIDE,
    MatrixStride = GL_MATRIX_STRIDE,
    IsRowMajor = GL_IS_ROW_MAJOR,
    AtomicCounterBufferIndex = GL_ATOMIC_COUNTER_BUFFER_INDEX,
    BufferBinding = GL_BUFFER_BINDING,
    BufferDataSize = GL_BUFFER_DATA_SIZE,
    NumActiveVariables = GL_NUM_ACTIVE_VARIABLES,
    ActiveVariables = GL_ACTIVE_VARIABLES,
    ReferencedByVertexShader = GL_REFERENCED_BY_VERTEX_SHADER,
    ReferencedByTessControlShader = GL_REFERENCED_BY_TESS_CONTROL_SHADER,
    ReferencedByTessEvaluationShader = GL_REFERENCED_BY_TESS_EVALUATION_SHADER,
    ReferencedByGeometryShader = GL_REFERENCED_BY_GEOMETRY_SHADER,
    ReferencedByFragmentShader = GL_REFERENCED_BY_FRAGMENT_SHADER,
    ReferencedByComputeShader = GL_REFERENCED_BY_COMPUTE_SHADER,
    TopLevelArraySize = GL_TOP_LEVEL_ARRAY_SIZE,
    TopLevelArrayStride = GL_TOP_LEVEL_ARRAY_STRIDE,
    Location = GL_LOCATION,
    LocationIndex = GL_LOCATION_INDEX,
    IsPerPatch = GL_IS_PER_PATCH,
    LocationComponent = GL_LOCATION_COMPONENT,
    TransformFeedbackBufferIndex = GL_TRANSFORM_FEEDBACK_BUFFER_INDEX,
    TransformFeedbackBufferStride = GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE,
}
impl UnsafeFromGLenum for ProgramResourceProperty {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ProgramResourceProperty::from_repr(val) else {
            println!("Attempt to create a ProgramResourceProperty from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ProgramResourceProperty> for u32 {
    fn from(value: ProgramResourceProperty) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ProgramResourceProperty {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum PixelStoreParameter {
    UnpackSwapBytes = GL_UNPACK_SWAP_BYTES,
    UnpackLsbFirst = GL_UNPACK_LSB_FIRST,
    UnpackRowLength = GL_UNPACK_ROW_LENGTH,
    UnpackSkipRows = GL_UNPACK_SKIP_ROWS,
    UnpackSkipPixels = GL_UNPACK_SKIP_PIXELS,
    UnpackAlignment = GL_UNPACK_ALIGNMENT,
    PackSwapBytes = GL_PACK_SWAP_BYTES,
    PackLsbFirst = GL_PACK_LSB_FIRST,
    PackRowLength = GL_PACK_ROW_LENGTH,
    PackSkipRows = GL_PACK_SKIP_ROWS,
    PackSkipPixels = GL_PACK_SKIP_PIXELS,
    PackAlignment = GL_PACK_ALIGNMENT,
    PackSkipImages = GL_PACK_SKIP_IMAGES,
    PackImageHeight = GL_PACK_IMAGE_HEIGHT,
    UnpackSkipImages = GL_UNPACK_SKIP_IMAGES,
    UnpackImageHeight = GL_UNPACK_IMAGE_HEIGHT,
}
impl UnsafeFromGLenum for PixelStoreParameter {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = PixelStoreParameter::from_repr(val) else {
            println!(
                "Attempt to create a PixelStoreParameter from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<PixelStoreParameter> for u32 {
    fn from(value: PixelStoreParameter) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for PixelStoreParameter {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum AttributeType {
    Int = GL_INT,
    UnsignedInt = GL_UNSIGNED_INT,
    Float = GL_FLOAT,
    Double = GL_DOUBLE,
    FloatVec2 = GL_FLOAT_VEC2,
    FloatVec3 = GL_FLOAT_VEC3,
    FloatVec4 = GL_FLOAT_VEC4,
    IntVec2 = GL_INT_VEC2,
    IntVec3 = GL_INT_VEC3,
    IntVec4 = GL_INT_VEC4,
    Bool = GL_BOOL,
    BoolVec2 = GL_BOOL_VEC2,
    BoolVec3 = GL_BOOL_VEC3,
    BoolVec4 = GL_BOOL_VEC4,
    FloatMat2 = GL_FLOAT_MAT2,
    FloatMat3 = GL_FLOAT_MAT3,
    FloatMat4 = GL_FLOAT_MAT4,
    Sampler1D = GL_SAMPLER_1D,
    Sampler2D = GL_SAMPLER_2D,
    Sampler3D = GL_SAMPLER_3D,
    SamplerCube = GL_SAMPLER_CUBE,
    Sampler1DShadow = GL_SAMPLER_1D_SHADOW,
    Sampler2DShadow = GL_SAMPLER_2D_SHADOW,
    FloatMat2x3 = GL_FLOAT_MAT2x3,
    FloatMat2x4 = GL_FLOAT_MAT2x4,
    FloatMat3x2 = GL_FLOAT_MAT3x2,
    FloatMat3x4 = GL_FLOAT_MAT3x4,
    FloatMat4x2 = GL_FLOAT_MAT4x2,
    FloatMat4x3 = GL_FLOAT_MAT4x3,
    Sampler1DArrayShadow = GL_SAMPLER_1D_ARRAY_SHADOW,
    Sampler2DArrayShadow = GL_SAMPLER_2D_ARRAY_SHADOW,
    SamplerCubeShadow = GL_SAMPLER_CUBE_SHADOW,
    UnsignedIntVec2 = GL_UNSIGNED_INT_VEC2,
    UnsignedIntVec3 = GL_UNSIGNED_INT_VEC3,
    UnsignedIntVec4 = GL_UNSIGNED_INT_VEC4,
    IntSampler1D = GL_INT_SAMPLER_1D,
    IntSampler2D = GL_INT_SAMPLER_2D,
    IntSampler3D = GL_INT_SAMPLER_3D,
    IntSamplerCube = GL_INT_SAMPLER_CUBE,
    IntSampler1DArray = GL_INT_SAMPLER_1D_ARRAY,
    IntSampler2DArray = GL_INT_SAMPLER_2D_ARRAY,
    UnsignedIntSampler1D = GL_UNSIGNED_INT_SAMPLER_1D,
    UnsignedIntSampler2D = GL_UNSIGNED_INT_SAMPLER_2D,
    UnsignedIntSampler3D = GL_UNSIGNED_INT_SAMPLER_3D,
    UnsignedIntSamplerCube = GL_UNSIGNED_INT_SAMPLER_CUBE,
    UnsignedIntSampler1DArray = GL_UNSIGNED_INT_SAMPLER_1D_ARRAY,
    UnsignedIntSampler2DArray = GL_UNSIGNED_INT_SAMPLER_2D_ARRAY,
    Sampler2DRect = GL_SAMPLER_2D_RECT,
    Sampler2DRectShadow = GL_SAMPLER_2D_RECT_SHADOW,
    SamplerBuffer = GL_SAMPLER_BUFFER,
    IntSampler2DRect = GL_INT_SAMPLER_2D_RECT,
    IntSamplerBuffer = GL_INT_SAMPLER_BUFFER,
    UnsignedIntSampler2DRect = GL_UNSIGNED_INT_SAMPLER_2D_RECT,
    UnsignedIntSamplerBuffer = GL_UNSIGNED_INT_SAMPLER_BUFFER,
    Sampler2DMultisample = GL_SAMPLER_2D_MULTISAMPLE,
    IntSampler2DMultisample = GL_INT_SAMPLER_2D_MULTISAMPLE,
    UnsignedIntSampler2DMultisample = GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE,
    Sampler2DMultisampleArray = GL_SAMPLER_2D_MULTISAMPLE_ARRAY,
    IntSampler2DMultisampleArray = GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY,
    UnsignedIntSampler2DMultisampleArray = GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY,
    SamplerCubeMapArray = GL_SAMPLER_CUBE_MAP_ARRAY,
    SamplerCubeMapArrayShadow = GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW,
    IntSamplerCubeMapArray = GL_INT_SAMPLER_CUBE_MAP_ARRAY,
    UnsignedIntSamplerCubeMapArray = GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY,
    DoubleVec2 = GL_DOUBLE_VEC2,
    DoubleVec3 = GL_DOUBLE_VEC3,
    DoubleVec4 = GL_DOUBLE_VEC4,
    DoubleMat2 = GL_DOUBLE_MAT2,
    DoubleMat3 = GL_DOUBLE_MAT3,
    DoubleMat4 = GL_DOUBLE_MAT4,
    DoubleMat2x3 = GL_DOUBLE_MAT2x3,
    DoubleMat2x4 = GL_DOUBLE_MAT2x4,
    DoubleMat3x2 = GL_DOUBLE_MAT3x2,
    DoubleMat3x4 = GL_DOUBLE_MAT3x4,
    DoubleMat4x2 = GL_DOUBLE_MAT4x2,
    DoubleMat4x3 = GL_DOUBLE_MAT4x3,
    Image1D = GL_IMAGE_1D,
    Image2D = GL_IMAGE_2D,
    Image3D = GL_IMAGE_3D,
    Image2DRect = GL_IMAGE_2D_RECT,
    ImageCube = GL_IMAGE_CUBE,
    ImageBuffer = GL_IMAGE_BUFFER,
    Image1DArray = GL_IMAGE_1D_ARRAY,
    Image2DArray = GL_IMAGE_2D_ARRAY,
    ImageCubeMapArray = GL_IMAGE_CUBE_MAP_ARRAY,
    Image2DMultisample = GL_IMAGE_2D_MULTISAMPLE,
    Image2DMultisampleArray = GL_IMAGE_2D_MULTISAMPLE_ARRAY,
    IntImage1D = GL_INT_IMAGE_1D,
    IntImage2D = GL_INT_IMAGE_2D,
    IntImage3D = GL_INT_IMAGE_3D,
    IntImage2DRect = GL_INT_IMAGE_2D_RECT,
    IntImageCube = GL_INT_IMAGE_CUBE,
    IntImageBuffer = GL_INT_IMAGE_BUFFER,
    IntImage1DArray = GL_INT_IMAGE_1D_ARRAY,
    IntImage2DArray = GL_INT_IMAGE_2D_ARRAY,
    IntImageCubeMapArray = GL_INT_IMAGE_CUBE_MAP_ARRAY,
    IntImage2DMultisample = GL_INT_IMAGE_2D_MULTISAMPLE,
    IntImage2DMultisampleArray = GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY,
    UnsignedIntImage1D = GL_UNSIGNED_INT_IMAGE_1D,
    UnsignedIntImage2D = GL_UNSIGNED_INT_IMAGE_2D,
    UnsignedIntImage3D = GL_UNSIGNED_INT_IMAGE_3D,
    UnsignedIntImage2DRect = GL_UNSIGNED_INT_IMAGE_2D_RECT,
    UnsignedIntImageCube = GL_UNSIGNED_INT_IMAGE_CUBE,
    UnsignedIntImageBuffer = GL_UNSIGNED_INT_IMAGE_BUFFER,
    UnsignedIntImage1DArray = GL_UNSIGNED_INT_IMAGE_1D_ARRAY,
    UnsignedIntImage2DArray = GL_UNSIGNED_INT_IMAGE_2D_ARRAY,
    UnsignedIntImageCubeMapArray = GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY,
    UnsignedIntImage2DMultisample = GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE,
    UnsignedIntImage2DMultisampleArray = GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY,
}
impl UnsafeFromGLenum for AttributeType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = AttributeType::from_repr(val) else {
            println!("Attempt to create a AttributeType from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<AttributeType> for u32 {
    fn from(value: AttributeType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for AttributeType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum UniformPName {
    UniformType = GL_UNIFORM_TYPE,
    UniformSize = GL_UNIFORM_SIZE,
    UniformNameLength = GL_UNIFORM_NAME_LENGTH,
    UniformBlockIndex = GL_UNIFORM_BLOCK_INDEX,
    UniformOffset = GL_UNIFORM_OFFSET,
    UniformArrayStride = GL_UNIFORM_ARRAY_STRIDE,
    UniformMatrixStride = GL_UNIFORM_MATRIX_STRIDE,
    UniformIsRowMajor = GL_UNIFORM_IS_ROW_MAJOR,
    UniformAtomicCounterBufferIndex = GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX,
}
impl UnsafeFromGLenum for UniformPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = UniformPName::from_repr(val) else {
            println!("Attempt to create a UniformPName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<UniformPName> for u32 {
    fn from(value: UniformPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for UniformPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ClampColorMode {
    False = GL_FALSE,
    True = GL_TRUE,
    FixedOnly = GL_FIXED_ONLY,
}
impl UnsafeFromGLenum for ClampColorMode {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ClampColorMode::from_repr(val) else {
            println!(
                "Attempt to create a ClampColorMode from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ClampColorMode> for u32 {
    fn from(value: ClampColorMode) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ClampColorMode {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ShaderParameterName {
    ShaderType = GL_SHADER_TYPE,
    DeleteStatus = GL_DELETE_STATUS,
    CompileStatus = GL_COMPILE_STATUS,
    InfoLogLength = GL_INFO_LOG_LENGTH,
    ShaderSourceLength = GL_SHADER_SOURCE_LENGTH,
}
impl UnsafeFromGLenum for ShaderParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ShaderParameterName::from_repr(val) else {
            println!(
                "Attempt to create a ShaderParameterName from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ShaderParameterName> for u32 {
    fn from(value: ShaderParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ShaderParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum QueryParameterName {
    QueryCounterBits = GL_QUERY_COUNTER_BITS,
    CurrentQuery = GL_CURRENT_QUERY,
}
impl UnsafeFromGLenum for QueryParameterName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = QueryParameterName::from_repr(val) else {
            println!(
                "Attempt to create a QueryParameterName from a GLenum with invalid value {val:#X}"
            );
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<QueryParameterName> for u32 {
    fn from(value: QueryParameterName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for QueryParameterName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum TextureUnit {
    Texture0 = GL_TEXTURE0,
    Texture1 = GL_TEXTURE1,
    Texture2 = GL_TEXTURE2,
    Texture3 = GL_TEXTURE3,
    Texture4 = GL_TEXTURE4,
    Texture5 = GL_TEXTURE5,
    Texture6 = GL_TEXTURE6,
    Texture7 = GL_TEXTURE7,
    Texture8 = GL_TEXTURE8,
    Texture9 = GL_TEXTURE9,
    Texture10 = GL_TEXTURE10,
    Texture11 = GL_TEXTURE11,
    Texture12 = GL_TEXTURE12,
    Texture13 = GL_TEXTURE13,
    Texture14 = GL_TEXTURE14,
    Texture15 = GL_TEXTURE15,
    Texture16 = GL_TEXTURE16,
    Texture17 = GL_TEXTURE17,
    Texture18 = GL_TEXTURE18,
    Texture19 = GL_TEXTURE19,
    Texture20 = GL_TEXTURE20,
    Texture21 = GL_TEXTURE21,
    Texture22 = GL_TEXTURE22,
    Texture23 = GL_TEXTURE23,
    Texture24 = GL_TEXTURE24,
    Texture25 = GL_TEXTURE25,
    Texture26 = GL_TEXTURE26,
    Texture27 = GL_TEXTURE27,
    Texture28 = GL_TEXTURE28,
    Texture29 = GL_TEXTURE29,
    Texture30 = GL_TEXTURE30,
    Texture31 = GL_TEXTURE31,
}
impl UnsafeFromGLenum for TextureUnit {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = TextureUnit::from_repr(val) else {
            println!("Attempt to create a TextureUnit from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<TextureUnit> for u32 {
    fn from(value: TextureUnit) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for TextureUnit {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum ShaderType {
    FragmentShader = GL_FRAGMENT_SHADER,
    VertexShader = GL_VERTEX_SHADER,
    GeometryShader = GL_GEOMETRY_SHADER,
    TessEvaluationShader = GL_TESS_EVALUATION_SHADER,
    TessControlShader = GL_TESS_CONTROL_SHADER,
    ComputeShader = GL_COMPUTE_SHADER,
}
impl UnsafeFromGLenum for ShaderType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = ShaderType::from_repr(val) else {
            println!("Attempt to create a ShaderType from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<ShaderType> for u32 {
    fn from(value: ShaderType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for ShaderType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum TransformFeedbackPName {
    TransformFeedbackBufferStart = GL_TRANSFORM_FEEDBACK_BUFFER_START,
    TransformFeedbackBufferSize = GL_TRANSFORM_FEEDBACK_BUFFER_SIZE,
    TransformFeedbackBufferBinding = GL_TRANSFORM_FEEDBACK_BUFFER_BINDING,
    TransformFeedbackActive = GL_TRANSFORM_FEEDBACK_ACTIVE,
    TransformFeedbackPaused = GL_TRANSFORM_FEEDBACK_PAUSED,
}
impl UnsafeFromGLenum for TransformFeedbackPName {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = TransformFeedbackPName::from_repr(val) else {
            println!("Attempt to create a TransformFeedbackPName from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<TransformFeedbackPName> for u32 {
    fn from(value: TransformFeedbackPName) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for TransformFeedbackPName {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum BufferUsage {
    StreamDraw = GL_STREAM_DRAW,
    StreamRead = GL_STREAM_READ,
    StreamCopy = GL_STREAM_COPY,
    StaticDraw = GL_STATIC_DRAW,
    StaticRead = GL_STATIC_READ,
    StaticCopy = GL_STATIC_COPY,
    DynamicDraw = GL_DYNAMIC_DRAW,
    DynamicRead = GL_DYNAMIC_READ,
    DynamicCopy = GL_DYNAMIC_COPY,
}
impl UnsafeFromGLenum for BufferUsage {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = BufferUsage::from_repr(val) else {
            println!("Attempt to create a BufferUsage from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<BufferUsage> for u32 {
    fn from(value: BufferUsage) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for BufferUsage {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum DebugType {
    DontCare = GL_DONT_CARE,
    DebugTypeError = GL_DEBUG_TYPE_ERROR,
    DebugTypeDeprecatedBehavior = GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR,
    DebugTypeUndefinedBehavior = GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR,
    DebugTypePortability = GL_DEBUG_TYPE_PORTABILITY,
    DebugTypePerformance = GL_DEBUG_TYPE_PERFORMANCE,
    DebugTypeOther = GL_DEBUG_TYPE_OTHER,
    DebugTypeMarker = GL_DEBUG_TYPE_MARKER,
    DebugTypePushGroup = GL_DEBUG_TYPE_PUSH_GROUP,
    DebugTypePopGroup = GL_DEBUG_TYPE_POP_GROUP,
}
impl UnsafeFromGLenum for DebugType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = DebugType::from_repr(val) else {
            println!("Attempt to create a DebugType from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<DebugType> for u32 {
    fn from(value: DebugType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for DebugType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum QueryTarget {
    SamplesPassed = GL_SAMPLES_PASSED,
    PrimitivesGenerated = GL_PRIMITIVES_GENERATED,
    TransformFeedbackPrimitivesWritten = GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN,
    AnySamplesPassed = GL_ANY_SAMPLES_PASSED,
    TimeElapsed = GL_TIME_ELAPSED,
    AnySamplesPassedConservative = GL_ANY_SAMPLES_PASSED_CONSERVATIVE,
    VerticesSubmitted = GL_VERTICES_SUBMITTED,
    PrimitivesSubmitted = GL_PRIMITIVES_SUBMITTED,
    VertexShaderInvocations = GL_VERTEX_SHADER_INVOCATIONS,
    TransformFeedbackOverflow = GL_TRANSFORM_FEEDBACK_OVERFLOW,
}
impl UnsafeFromGLenum for QueryTarget {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = QueryTarget::from_repr(val) else {
            println!("Attempt to create a QueryTarget from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<QueryTarget> for u32 {
    fn from(value: QueryTarget) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for QueryTarget {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum DebugSource {
    DontCare = GL_DONT_CARE,
    DebugSourceApi = GL_DEBUG_SOURCE_API,
    DebugSourceWindowSystem = GL_DEBUG_SOURCE_WINDOW_SYSTEM,
    DebugSourceShaderCompiler = GL_DEBUG_SOURCE_SHADER_COMPILER,
    DebugSourceThirdParty = GL_DEBUG_SOURCE_THIRD_PARTY,
    DebugSourceApplication = GL_DEBUG_SOURCE_APPLICATION,
    DebugSourceOther = GL_DEBUG_SOURCE_OTHER,
}
impl UnsafeFromGLenum for DebugSource {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = DebugSource::from_repr(val) else {
            println!("Attempt to create a DebugSource from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<DebugSource> for u32 {
    fn from(value: DebugSource) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for DebugSource {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum PrecisionType {
    LowFloat = GL_LOW_FLOAT,
    MediumFloat = GL_MEDIUM_FLOAT,
    HighFloat = GL_HIGH_FLOAT,
    LowInt = GL_LOW_INT,
    MediumInt = GL_MEDIUM_INT,
    HighInt = GL_HIGH_INT,
}
impl UnsafeFromGLenum for PrecisionType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = PrecisionType::from_repr(val) else {
            println!("Attempt to create a PrecisionType from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<PrecisionType> for u32 {
    fn from(value: PrecisionType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for PrecisionType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
bitflags! {
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct MemoryBarrierMask: u32 {
const VERTEX_ATTRIB_ARRAY_BARRIER_BIT = GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT;
const ELEMENT_ARRAY_BARRIER_BIT = GL_ELEMENT_ARRAY_BARRIER_BIT;
const UNIFORM_BARRIER_BIT = GL_UNIFORM_BARRIER_BIT;
const TEXTURE_FETCH_BARRIER_BIT = GL_TEXTURE_FETCH_BARRIER_BIT;
const SHADER_IMAGE_ACCESS_BARRIER_BIT = GL_SHADER_IMAGE_ACCESS_BARRIER_BIT;
const COMMAND_BARRIER_BIT = GL_COMMAND_BARRIER_BIT;
const PIXEL_BUFFER_BARRIER_BIT = GL_PIXEL_BUFFER_BARRIER_BIT;
const TEXTURE_UPDATE_BARRIER_BIT = GL_TEXTURE_UPDATE_BARRIER_BIT;
const BUFFER_UPDATE_BARRIER_BIT = GL_BUFFER_UPDATE_BARRIER_BIT;
const FRAMEBUFFER_BARRIER_BIT = GL_FRAMEBUFFER_BARRIER_BIT;
const TRANSFORM_FEEDBACK_BARRIER_BIT = GL_TRANSFORM_FEEDBACK_BARRIER_BIT;
const ATOMIC_COUNTER_BARRIER_BIT = GL_ATOMIC_COUNTER_BARRIER_BIT;
const ALL_BARRIER_BITS = GL_ALL_BARRIER_BITS;
const SHADER_STORAGE_BARRIER_BIT = GL_SHADER_STORAGE_BARRIER_BIT;
const CLIENT_MAPPED_BUFFER_BARRIER_BIT = GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT;
const QUERY_BUFFER_BARRIER_BIT = GL_QUERY_BUFFER_BARRIER_BIT;
}
}
impl UnsafeFromGLenum for MemoryBarrierMask {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = MemoryBarrierMask::from_bits(val) else {
            println!("Attempt to create a MemoryBarrierMask from a GLenum with an invalid bit set! {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<MemoryBarrierMask> for u32 {
    fn from(value: MemoryBarrierMask) -> u32 {
        value.bits()
    }
}
impl<Dst: GlDstType> SrcType<Dst> for MemoryBarrierMask {
    fn cast(self) -> Dst {
        Dst::from_uint(self.bits())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]
#[repr(u32)]
pub enum PixelType {
    Byte = GL_BYTE,
    UnsignedByte = GL_UNSIGNED_BYTE,
    Short = GL_SHORT,
    UnsignedShort = GL_UNSIGNED_SHORT,
    Int = GL_INT,
    UnsignedInt = GL_UNSIGNED_INT,
    Float = GL_FLOAT,
    UnsignedByte332 = GL_UNSIGNED_BYTE_3_3_2,
    UnsignedShort4444 = GL_UNSIGNED_SHORT_4_4_4_4,
    UnsignedShort5551 = GL_UNSIGNED_SHORT_5_5_5_1,
    UnsignedInt8888 = GL_UNSIGNED_INT_8_8_8_8,
    UnsignedInt1010102 = GL_UNSIGNED_INT_10_10_10_2,
    UnsignedByte233Rev = GL_UNSIGNED_BYTE_2_3_3_REV,
    UnsignedShort565 = GL_UNSIGNED_SHORT_5_6_5,
    UnsignedShort565Rev = GL_UNSIGNED_SHORT_5_6_5_REV,
    UnsignedShort4444Rev = GL_UNSIGNED_SHORT_4_4_4_4_REV,
    UnsignedShort1555Rev = GL_UNSIGNED_SHORT_1_5_5_5_REV,
    UnsignedInt8888Rev = GL_UNSIGNED_INT_8_8_8_8_REV,
    UnsignedInt2101010Rev = GL_UNSIGNED_INT_2_10_10_10_REV,
    UnsignedInt5999Rev = GL_UNSIGNED_INT_5_9_9_9_REV,
    Float32UnsignedInt248Rev = GL_FLOAT_32_UNSIGNED_INT_24_8_REV,
    UnsignedInt248 = GL_UNSIGNED_INT_24_8,
    HalfFloat = GL_HALF_FLOAT,
    UnsignedInt10F11F11FRev = GL_UNSIGNED_INT_10F_11F_11F_REV,
}
impl UnsafeFromGLenum for PixelType {
    unsafe fn unsafe_from_gl_enum(val: u32) -> Self {
        #[cfg(debug_assertions)]
        let Some(ret) = PixelType::from_repr(val) else {
            println!("Attempt to create a PixelType from a GLenum with invalid value {val:#X}");
            panic!();
        };
        #[cfg(not(debug_assertions))]
        let ret = unsafe { std::mem::transmute(val) };
        ret
    }
}
impl From<PixelType> for u32 {
    fn from(value: PixelType) -> u32 {
        value as u32
    }
}
impl<Dst: GlDstType> SrcType<Dst> for PixelType {
    fn cast(self) -> Dst {
        Dst::from_uint(self as u32)
    }
}
