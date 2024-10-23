#![allow(unused)] /* FIXME delete when implemented */
use core::str;
use std::{
    any,
    cell::Cell,
    collections::VecDeque,
    env,
    ffi::{c_char, c_void, CStr, CString},
    fmt::Arguments,
    mem::{self, MaybeUninit},
    pin::Pin,
    ptr, slice,
};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use flexi_logger::{writers::LogWriter, Logger};
use log::{logger, Level, Log, Record, RecordBuilder};

use crate::{
    dispatch::gl_types::{GLchar, GLsizei, GLDEBUGPROC},
    enums::{DebugSeverity, DebugSource, DebugType},
};

use super::{state::ObjectName, Context};

thread_local! {
    // We store the debug logging infrastructure in a separate thread local to avoid passing it in by-reference every log call (which cannot be avoided/worked around with macros)
    static DEBUG_STATE: Cell<Option<DebugState>> = const {Cell::new(None)};
    static STDOUT_LOGGER: bool = env::var("OXIDEGL_LOG_TO_STDOUT").is_ok()
}

#[inline]
pub(crate) fn with_debug_state_mut<F, Ret>(func: F) -> Option<Ret>
where
    F: FnOnce(Pin<&mut DebugState>) -> Ret,
{
    if let Some(mut s) = DEBUG_STATE.take() {
        let p = Pin::new(&mut s);
        let ret = Some(func(p));
        DEBUG_STATE.replace(Some(s));
        ret
    } else {
        None
    }
}

#[inline]
pub(crate) fn with_debug_state<F, Ret>(func: F) -> Option<Ret>
where
    F: FnOnce(&DebugState) -> Ret,
{
    if let Some(s) = DEBUG_STATE.take() {
        let ret = Some(func(&s));
        DEBUG_STATE.replace(Some(s));
        ret
    } else {
        None
    }
}

impl Context {
    pub(crate) fn install_debug_state(&mut self) {
        DEBUG_STATE.replace(self.gl_state.debug_log_callback.take());
    }
    pub(crate) fn uninstall_debug_state(&mut self) {
        self.gl_state.debug_log_callback = DEBUG_STATE.take();
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
#[derive(Debug)]
pub(crate) struct DebugMessageMeta {
    pub(crate) src: DebugSource,
    pub(crate) ty: DebugType,
    pub(crate) sev: DebugSeverity,
    pub(crate) id: u32,
}
#[derive(Debug)]
pub(crate) struct DebugLogMessage {
    text: Box<CStr>,
    meta: DebugMessageMeta,
}

#[derive(Debug)]
pub(crate) struct DebugState {
    messages: VecDeque<DebugLogMessage>,
    debug_groups: Vec<DebugGroup>,
    pub(crate) callback: Option<GLDEBUGPROC>,
    pub(crate) debug_labels: HashMap<any::TypeId, HashMap<u32, Box<str>>>,
    pub(crate) user_param_ptr: *mut c_void,
}
#[derive(Debug, Clone)]
pub struct DebugGroup {
    message: Box<CStr>,
    id: u32,
    src: DebugSource,
    filter: HashMap<u32, HashMap<u32, DisabledMessages>>,
}
#[derive(Debug, Clone)]
pub struct DisabledMessages {
    disabled_ids: HashSet<u32>,
    disabled_severities: HashSet<u32>,
}
impl DisabledMessages {
    fn is_enabled(&self, severity: DebugSeverity, id: u32) -> bool {
        !self.disabled_ids.contains(&id) && !self.disabled_severities.contains(&(severity as u32))
    }
    fn empty() -> Self {
        Self {
            disabled_ids: HashSet::new(),
            disabled_severities: HashSet::new(),
        }
    }
}
impl DebugState {
    #[inline]
    fn should_log(&self, meta: &DebugMessageMeta) -> bool {
        self.debug_groups
            .last()
            .unwrap()
            .filter
            .get(&(meta.src as u32))
            .is_some_and(|m| {
                m.get(&(meta.ty as u32))
                    .is_some_and(|d| d.is_enabled(meta.sev, meta.id))
            })
    }
    /// glDebugMessageControl interface
    pub(crate) unsafe fn message_control(
        &mut self,
        src: DebugSource,
        ty: DebugType,
        sev: DebugSeverity,
        count: u32,
        ids: *const u32,
        enable: bool,
    ) {
        //FIXME: this is messy af
        if count > 0 {
            let map_over_types = match self
                .debug_groups
                .last_mut()
                .unwrap()
                .filter
                .entry(src as u32)
            {
                std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                    occupied_entry.into_mut()
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(HashMap::new())
                }
            };
            let disable = match map_over_types.entry(ty as u32) {
                std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                    occupied_entry.into_mut()
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(DisabledMessages::empty())
                }
            };
            // Safety: caller ensures count and ids are valid to construct a slice of uint from
            for i in unsafe { slice::from_raw_parts(ids, count as usize) } {
                if enable {
                    disable.disabled_ids.remove(i);
                } else {
                    disable.disabled_ids.insert(*i);
                }
            }
        } else {
            let s = src as u32;
            let t = ty as u32;
            let sources = if src == DebugSource::DontCare {
                &DEBUG_SOURCES
            } else {
                slice::from_ref(&s)
            };
            let types = if ty == DebugType::DontCare {
                &DEBUG_TYPES
            } else {
                slice::from_ref(&t)
            };
            for src in sources {
                let map_over_types = match self.debug_groups.last_mut().unwrap().filter.entry(*src)
                {
                    std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                        occupied_entry.into_mut()
                    }
                    std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(HashMap::new())
                    }
                };
                for ty in types {
                    let disable = match map_over_types.entry(*ty) {
                        std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                            occupied_entry.into_mut()
                        }
                        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(DisabledMessages::empty())
                        }
                    };
                    if enable {
                        disable.disabled_severities.remove(&(sev as u32));
                    } else {
                        disable.disabled_severities.insert(sev as u32);
                    }
                }
            }
        }
    }
    /// glPushDebugGroup impl
    pub(crate) fn push_debug_group(
        &mut self,
        source: DebugSource,
        id: u32,
        length: i32,
        message: *const c_char,
    ) {
        let msg: Box<CStr> = match length {
            // Safety: caller ensures that if length is negative, message points to a valid nul-terminated C-string
            ..0 => unsafe { CStr::from_ptr(message) }.into(),
            0 => c"".into(),
            1.. => {
                #[expect(clippy::cast_sign_loss, reason = "sign loss is avoided via match arm")]
                let mut bytes =
                // Safety: caller ensures that if length is positive, it represents the length of message
                    unsafe { slice::from_raw_parts(message.cast::<u8>(), length as usize) }
                        .to_vec();
                // message is not guaranteed to be nul-terminated, rust's CStr (and the GL client) expects this
                if *bytes.last().unwrap() != 0 {
                    bytes.push(0);
                };
                CString::from_vec_with_nul(bytes)
                    .unwrap()
                    .into_boxed_c_str()
            }
        };
        let mut new_group = self.debug_groups.last().unwrap().clone();
        new_group.id = id;
        new_group.message = msg;
        new_group.src = source;
        self.debug_groups.push(new_group);

        // Cursed closure hack to work around an odd behavior in format_args
        let mut do_log = {
            #[inline]
            |args: Arguments| {
                let rec = RecordBuilder::new()
                    .args(args)
                    .level(Level::Debug)
                    .target("debug_groups")
                    .build();
                self.log_internal(
                    &rec,
                    DebugMessageMeta {
                        src: source,
                        ty: DebugType::DebugTypePushGroup,
                        sev: DebugSeverity::DebugSeverityNotification,
                        id,
                    },
                );
            }
        };
        do_log(format_args!("Pushing GL Debug group {id}"));
    }
    /// glPopDebugGroup impl
    pub(crate) fn pop_debug_group(&mut self) {
        if self.debug_groups.len() == 1 {
            log::warn!("Tried to pop the root GL debug group");
            return;
        }
        let v = self.debug_groups.pop().unwrap();
        let mut do_log = {
            #[inline]
            |args: Arguments| {
                let rec = RecordBuilder::new()
                    .args(args)
                    .level(Level::Debug)
                    .target("debug_groups")
                    .build();
                self.log_internal(
                    &rec,
                    DebugMessageMeta {
                        src: v.src,
                        ty: DebugType::DebugTypePopGroup,
                        sev: DebugSeverity::DebugSeverityNotification,
                        id: v.id,
                    },
                );
            }
        };
        do_log(format_args!(
            "Popped GL debug group {}: {}",
            v.id,
            v.message.to_str().expect("message wasn't valid UTF-8!")
        ));
    }
    pub(crate) fn log_impl(
        id: u32,
        gl_src: impl Into<Option<DebugSource>>,
        gl_ty: impl Into<Option<DebugType>>,
        rec: &Record,
    ) {
        let meta = DebugMessageMeta {
            src: gl_src.into().unwrap_or(DebugSource::DebugSourceOther),
            ty: gl_ty.into().unwrap_or(DebugType::DebugTypeOther),
            sev: rec.level().into(),
            id,
        };
        let Some(s) = DEBUG_STATE.take() else {
            log::warn!("OxideGL tried to log but logging context wasn't set");
            return;
        };
        let mut state = Some(s);
        if rec.args().as_str().is_some() {
            // skip storeback if we know no Debug impls will be invoked
            state.as_mut().unwrap().log_internal(rec, meta);
        } else {
            Self::log_internal_with_storeback(&mut state, rec, meta);
        }
        DEBUG_STATE.set(state);
    }
    fn log_internal(&mut self, rec: &Record, meta: DebugMessageMeta) {
        if !self.should_log(&meta) {
            return;
        }
        if STDOUT_LOGGER.with(|v| *v) {
            logger().log(rec);
        }
        let mut bytes = rec.args().to_string().into_bytes();
        bytes.push(0);
        let msg = CString::from_vec_with_nul(bytes)
            .expect("failed to convert log message to C string!")
            .into_boxed_c_str();
        self.log_to_gl_client(msg, meta);
    }
    /// Alternative implementation of `log_internal` which writes the `DebugState` back into
    /// the thread local storage for potential access by the format args' lazy eval
    fn log_internal_with_storeback(
        this_opt: &mut Option<Self>,
        rec: &Record,
        meta: DebugMessageMeta,
    ) {
        let this = this_opt.as_mut().unwrap();
        if !this.should_log(&meta) {
            return;
        }
        if STDOUT_LOGGER.with(|v| *v) {
            logger().log(rec);
        }

        // put the state back so the formatting code being invoked by to_string can access
        let prev = DEBUG_STATE.replace(mem::take(this_opt));

        // The `to_string` call might try to use the debug state
        let mut bytes = rec.args().to_string().into_bytes();

        // get it back to use afterwards
        *this_opt = DEBUG_STATE.replace(prev);

        let this = this_opt.as_mut().unwrap();
        bytes.push(0);
        let msg = CString::from_vec_with_nul(bytes)
            .expect("failed to convert log message to C string!")
            .into_boxed_c_str();
        this.log_to_gl_client(msg, meta);
    }
    fn log_to_gl_client(&mut self, text: Box<CStr>, meta: DebugMessageMeta) {
        if let Some(func) = self.callback {
            #[expect(
                clippy::cast_possible_truncation,
                reason = "no >4mil char log messages"
            )]
            // Safety: string pointer points to a valid, nul-terminated C string,
            // str_len is the length of the string's allocation minus one for the nul byte
            unsafe {
                func(
                    meta.src as u32,
                    meta.ty as u32,
                    meta.id,
                    meta.sev as u32,
                    text.count_bytes() as u32,
                    text.as_ptr().cast(),
                    self.user_param_ptr,
                );
            };
        } else {
            self.messages.push_back(DebugLogMessage { text, meta });
        }
    }
    #[expect(clippy::too_many_arguments, reason = "blame the GL spec for this one")]
    #[allow(clippy::undocumented_unsafe_blocks)]
    pub(crate) unsafe fn get_log_message(
        &mut self,
        count: u32,
        buf_size: GLsizei,
        sources: *mut DebugSource,
        types: *mut DebugType,
        ids: *mut u32,
        severities: *mut DebugSeverity,
        lengths: *mut GLsizei,
        buf: *mut GLchar,
    ) -> u32 {
        #[inline]
        unsafe fn nullable_slice_mut<T: Sized>(
            // borrow ptr to infer a sane lifetime on the returned slice
            ptr: &*mut T,
            count: u32,
        ) -> Option<&mut [MaybeUninit<T>]> {
            if ptr.is_null() {
                None
            } else {
                // Safety: caller ensures slices are allocated (they are not required to be initialized, hence the MaybeUninit)
                Some(unsafe {
                    slice::from_raw_parts_mut(ptr.cast::<MaybeUninit<T>>(), count as usize)
                })
            }
        }
        let mut sources = unsafe { nullable_slice_mut(&sources, count) };
        let mut types = unsafe { nullable_slice_mut(&types, count) };
        let mut ids = unsafe { nullable_slice_mut(&ids, count) };
        let mut severities = unsafe { nullable_slice_mut(&severities, count) };
        let mut lengths = unsafe { nullable_slice_mut(&lengths, count) };
        let mut buf = unsafe { nullable_slice_mut(&buf, count) };
        let mut remaining = buf_size as usize;
        let mut count_written = 0;
        loop {
            let Some(msg) = self.messages.front() else {
                break;
            };
            let msg_len = msg.text.count_bytes() + 1;
            if remaining < msg_len || count_written == count {
                break;
            }
            //unwrap: checked to be some
            let msg = self.messages.pop_front().unwrap();
            if let Some(s) = sources.as_mut() {
                s[count_written as usize].write(msg.meta.src);
            }
            if let Some(s) = types.as_mut() {
                s[count_written as usize].write(msg.meta.ty);
            };
            if let Some(s) = ids.as_mut() {
                s[count_written as usize].write(msg.meta.id);
            };
            if let Some(s) = severities.as_mut() {
                s[count_written as usize].write(msg.meta.sev);
            };
            if let Some(s) = lengths.as_mut() {
                #[expect(clippy::cast_possible_truncation, reason = "no >4gb log message")]
                s[count_written as usize].write(msg_len as u32);
            };
            if let Some(r) = mem::take(&mut buf) {
                let (write, remaining) = r.split_at_mut(msg_len);
                buf = Some(remaining);
                write.copy_from_slice(
                    // Safety: cast from [i8] to [MaybeUninit<i8>] is valid as MaybeUninit<T> has a looser or equal validity invariant and same representation as T
                    unsafe { slice::from_raw_parts(msg.text.as_ptr().cast(), msg_len) },
                );
            }
            remaining -= msg_len;
            count_written += 1;
        }
        count_written
    }
    pub(crate) fn get_label<T: ?Sized + 'static>(&self, name: ObjectName<T>) -> Option<Box<str>> {
        self.debug_labels
            .get(&any::TypeId::of::<T>())
            .and_then(|v| v.get(&name.to_raw()))
            .cloned()
    }
    pub(crate) fn set_label<T: ?Sized + 'static>(&mut self, name: ObjectName<T>, label: Box<str>) {
        let map_for_type = match self.debug_labels.entry(any::TypeId::of::<T>()) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                occupied_entry.into_mut()
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(HashMap::new())
            }
        };
        map_for_type.insert(name.to_raw(), label);
    }
    pub(crate) fn new_default() -> Self {
        let debug_groups = vec![DebugGroup {
            message: c"default debug group".into(),
            id: 0,
            src: DebugSource::DebugSourceApi,
            filter: HashMap::new(),
        }];
        Self {
            messages: VecDeque::new(),
            debug_labels: HashMap::new(),
            debug_groups,
            callback: None,
            user_param_ptr: ptr::null_mut(),
        }
    }
}
// Safety: DebugType is repr(u32)
const DEBUG_TYPES: [u32; 9] = unsafe {
    mem::transmute([
        DebugType::DebugTypeDeprecatedBehavior,
        DebugType::DebugTypeError,
        DebugType::DebugTypeMarker,
        DebugType::DebugTypeOther,
        DebugType::DebugTypePerformance,
        DebugType::DebugTypePopGroup,
        DebugType::DebugTypePushGroup,
        DebugType::DebugTypePortability,
        DebugType::DebugTypeUndefinedBehavior,
    ])
};
// Safety: DebugSource is repr(u32)
const DEBUG_SOURCES: [u32; 6] = unsafe {
    mem::transmute([
        DebugSource::DebugSourceApi,
        DebugSource::DebugSourceApplication,
        DebugSource::DebugSourceOther,
        DebugSource::DebugSourceShaderCompiler,
        DebugSource::DebugSourceThirdParty,
        DebugSource::DebugSourceWindowSystem,
    ])
};

pub(crate) fn init_logger() {
    Logger::try_with_env_or_str("none, oxidegl=trace")
        .unwrap()
        .start()
        .unwrap();
    log::trace!("OxideGL stdout logger initialized");
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
pub(crate) struct ConstStrToU16Map<const N: usize> {
    keys: [&'static str; N],
    vals: [u16; N],
}
impl<const N: usize> ConstStrToU16Map<N> {
    pub(crate) const fn get(&self, key: &'static str) -> Option<u16> {
        let mut low = 0;
        let mut high = self.keys.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if compare_strings(self.keys[mid], key) == 0 {
                return Some(self.vals[mid]);
            } else if compare_strings(key, self.keys[mid]) == -1 {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        None
    }
}
const fn min_usize(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}
/// Returns: 1 if a > b, 0 if a = b, -1 if a < b
const fn compare_strings(a: &str, b: &str) -> i32 {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let max_idx = min_usize(a.len(), b.len());
    let mut i = 0;
    while i < max_idx {
        if a[i] > b[i] {
            return 1;
        }
        if a[i] < b[i] {
            return -1;
        }
        i += 1;
    }
    if a.len() > b.len() {
        return 1;
    }
    if a.len() < b.len() {
        return -1;
    }
    0
}
