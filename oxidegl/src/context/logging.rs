use std::{
    cell::Cell,
    ffi::{c_void, CStr, CString},
    ptr,
    rc::Rc,
    sync::Arc,
};

use flexi_logger::{writers::LogWriter, Logger};
use log::{trace, Level, Log, Record};

use crate::{
    dispatch::gl_types::GLDEBUGPROC,
    enums::{DebugSeverity, DebugSource, DebugType},
    generated,
};

use super::Context;

// We store the debug logging infrastructure in a separate thread local to avoid passing it in by-reference to every log call (which cannot be avoided/worked around with macros)
thread_local! {
    static LOGGING_STATE: Cell<Option<DebugCallbackContainer>> = const {Cell::new(None)};
}

impl Context {
    pub(crate) fn install_debug_state(&mut self) {
        LOGGING_STATE.replace(self.gl_state.debug_log_callback.take());
    }
    pub(crate) fn uninstall_debug_state(&mut self) {
        self.gl_state.debug_log_callback = LOGGING_STATE.take();
    }
}

const LOG_SRC_SHADER_COMPILER: &str = "shc";
const LOG_SRC_API: &str = "api";
const LOG_SRC_APP: &str = "app";
const LOG_SRC_OTHER: &str = "oth";

const LOG_TYPE_ERROR: &str = "err";
const LOG_TYPE_DEPRECATED_BEHAVIOR: &str = "dep";
const LOG_TYPE_UB: &str = "ubb";
const LOG_TYPE_PORTABILITY: &str = "por";
const LOG_TYPE_PERF: &str = "prf";
const LOG_TYPE_MARKER: &str = "mrk";
const LOG_TYPE_PUSH_GROUP: &str = "psh";
const LOG_TYPE_POP_GROUP: &str = "pop";
const LOG_TYPE_OTHER: &str = "oth";

const OXIDEGL_TARGET_PREFIX: &str = "ogl";

impl From<&str> for DebugSource {
    #[inline]
    fn from(value: &str) -> Self {
        if value.len() != 3 {
            return Self::DebugSourceOther;
        }
        match value {
            LOG_SRC_SHADER_COMPILER => Self::DebugSourceShaderCompiler,
            LOG_SRC_API => Self::DebugSourceApi,
            LOG_SRC_APP => Self::DebugSourceApplication,
            _ => Self::DebugSourceOther,
        }
    }
}
impl From<DebugSource> for &'static str {
    fn from(value: DebugSource) -> Self {
        match value {
            DebugSource::DebugSourceApi => &LOG_SRC_API,
            DebugSource::DebugSourceApplication => &LOG_SRC_APP,
            DebugSource::DebugSourceShaderCompiler => &LOG_SRC_SHADER_COMPILER,
            _ => &LOG_SRC_OTHER,
        }
    }
}
impl From<&str> for DebugType {
    fn from(value: &str) -> Self {
        if value.len() != 3 {
            return DebugType::DebugTypeOther;
        }
        match value {
            LOG_TYPE_ERROR => Self::DebugTypeError,
            LOG_TYPE_DEPRECATED_BEHAVIOR => Self::DebugTypeDeprecatedBehavior,
            LOG_TYPE_UB => Self::DebugTypeUndefinedBehavior,
            LOG_TYPE_PORTABILITY => Self::DebugTypePortability,
            LOG_TYPE_PERF => Self::DebugTypePerformance,
            LOG_TYPE_PUSH_GROUP => Self::DebugTypePushGroup,
            LOG_TYPE_POP_GROUP => Self::DebugTypePopGroup,
            _ => Self::DebugTypeOther,
        }
    }
}
impl From<DebugType> for &'static str {
    fn from(value: DebugType) -> Self {
        match value {
            DebugType::DontCare => &LOG_TYPE_OTHER,
            DebugType::DebugTypeError => &LOG_TYPE_ERROR,
            DebugType::DebugTypeDeprecatedBehavior => &LOG_TYPE_DEPRECATED_BEHAVIOR,
            DebugType::DebugTypeUndefinedBehavior => &LOG_TYPE_UB,
            DebugType::DebugTypePortability => &LOG_TYPE_PORTABILITY,
            DebugType::DebugTypePerformance => &LOG_TYPE_PERF,
            DebugType::DebugTypeOther => &LOG_TYPE_OTHER,
            DebugType::DebugTypeMarker => &LOG_TYPE_MARKER,
            DebugType::DebugTypePushGroup => &LOG_TYPE_PUSH_GROUP,
            DebugType::DebugTypePopGroup => &LOG_TYPE_POP_GROUP,
        }
    }
}
impl From<Level> for DebugSeverity {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => Self::DebugSeverityHigh,
            Level::Warn => Self::DebugSeverityMedium,
            Level::Info => Self::DebugSeverityLow,
            Level::Debug | Level::Trace => Self::DebugSeverityNotification,
        }
    }
}
pub(crate) fn get_gl_metadata(target: &str, sev: Level) -> DebugMessageMeta {
    let sev: DebugSeverity = sev.into();
    if target.len() == 9 && &target[0..3] == OXIDEGL_TARGET_PREFIX {
        let src: DebugSource = target[3..6].into();
        let ty: DebugType = target[6..9].into();
        DebugMessageMeta {
            src,
            ty,
            sev,
            id: 0,
        }
    } else {
        DebugMessageMeta {
            src: DebugSource::DebugSourceOther,
            ty: DebugType::DebugTypeOther,
            sev,
            id: 0,
        }
    }
}
#[derive(Debug)]
pub(crate) struct DebugMessageMeta {
    pub(crate) src: DebugSource,
    pub(crate) ty: DebugType,
    pub(crate) sev: DebugSeverity,
    pub(crate) id: u32,
}

#[derive(Debug)]
pub(crate) struct DebugCallbackContainer {
    messages: Vec<Option<DebugLogMessage>>,
    callback: Option<GLDEBUGPROC>,
    user_param_ptr: *mut c_void,
}
impl DebugCallbackContainer {
    pub(crate) fn new_default() -> Self {
        Self {
            messages: Vec::new(),
            callback: None,
            user_param_ptr: ptr::null_mut(),
        }
    }
}
pub(crate) fn init_logger() {
    Logger::try_with_env_or_str("none, oxidegl=trace")
        .unwrap()
        .log_to_writer(Box::new(OxideGLDebugLogWriter))
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .start()
        .unwrap();
    trace!("OxideGL Logger initialized");
}
#[derive(Debug)]
pub(crate) struct DebugLogMessage {
    msg: Box<CStr>,
    meta: DebugMessageMeta,
}
pub struct OxideGLDebugLogWriter;

const FID: Option<u16> = crate::generated::FNAME_LOOKUP.get(file!());
impl LogWriter for OxideGLDebugLogWriter {
    fn write(
        &self,
        _now: &mut flexi_logger::DeferredNow,
        record: &log::Record,
    ) -> std::io::Result<()> {
        let Some(mut container) = LOGGING_STATE.take() else {
            return Ok(());
        };
        let gl_msg: DebugLogMessage = record.into();
        if let Some(callback) = container.callback {
            #[expect(clippy::cast_possible_truncation, reason = "blame the API")]
            //Safety: GL client ensures this callback doesn't do crimes
            unsafe {
                callback(
                    gl_msg.meta.src as u32,
                    gl_msg.meta.ty as u32,
                    gl_msg.meta.id,
                    gl_msg.meta.sev as u32,
                    gl_msg.msg.count_bytes() as u32,
                    gl_msg.msg.as_ptr().cast(),
                    container.user_param_ptr,
                );
            };
        }
        container.messages.push(Some(gl_msg));
        LOGGING_STATE.set(Some(container));
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }
}
