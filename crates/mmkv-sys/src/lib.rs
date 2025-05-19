pub mod bindings {
    include!(concat!(std::env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
// Rust enums for C++ enums
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    None = 4,
}

impl From<LogLevel> for bindings::MMKVLogLevel {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Debug => bindings::MMKVLogLevel_MMKVLogDebug,
            LogLevel::Info => bindings::MMKVLogLevel_MMKVLogInfo,
            LogLevel::Warning => bindings::MMKVLogLevel_MMKVLogWarning,
            LogLevel::Error => bindings::MMKVLogLevel_MMKVLogError,
            LogLevel::None => bindings::MMKVLogLevel_MMKVLogNone,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoverStrategic {
    OnErrorDiscard = 0,
    OnErrorRecover = 1,
}

impl From<RecoverStrategic> for bindings::MMKVRecoverStrategic {
    fn from(strategy: RecoverStrategic) -> Self {
        match strategy {
            RecoverStrategic::OnErrorDiscard => bindings::MMKVRecoverStrategic_OnErrorDiscard,
            RecoverStrategic::OnErrorRecover => bindings::MMKVRecoverStrategic_OnErrorRecover,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    CrcCheckFail = 0,
    FileLength = 1,
}

impl From<ErrorType> for bindings::MMKVErrorType {
    fn from(err: ErrorType) -> Self {
        match err {
            ErrorType::CrcCheckFail => bindings::MMKVErrorType_MMKVCRCCheckFail,
            ErrorType::FileLength => bindings::MMKVErrorType_MMKVFileLength,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncFlag {
    Sync = 1,
    Async = 0,
}

impl From<SyncFlag> for bindings::SyncFlag {
    fn from(flag: SyncFlag) -> Self {
        match flag {
            SyncFlag::Sync => bindings::SyncFlag_MMKV_SYNC,
            SyncFlag::Async => bindings::SyncFlag_MMKV_ASYNC,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MMKVMode {
    SingleProcess = 1,
    MultiProcess = 2,
    ReadOnly = 32,
}

impl From<MMKVMode> for bindings::MMKVMode {
    fn from(mode: MMKVMode) -> Self {
        match mode {
            MMKVMode::SingleProcess => bindings::MMKVMode_MMKV_SINGLE_PROCESS,
            MMKVMode::MultiProcess => bindings::MMKVMode_MMKV_MULTI_PROCESS,
            MMKVMode::ReadOnly => bindings::MMKVMode_MMKV_READ_ONLY,
        }
    }
}

// Main MMKV wrapper
pub struct MMKV {
    ptr: *mut bindings::MMKV,
}

impl MMKV {
    /// Initialize MMKV with root directory and log level
    pub fn initialize(root_dir: &str, log_level: LogLevel) {
        let c_root_dir = CString::new(root_dir).unwrap();
        unsafe {
            bindings::MMKV_initializeMMKV(
                c_root_dir.as_ptr() as *const bindings::MMKVPath_t,
                log_level.into(),
                None,
            );
        }
    }

    /// Get default MMKV instance
    pub fn default(mode: MMKVMode, crypt_key: Option<&str>) -> Option<Self> {
        let crypt_key_ptr = crypt_key
            .map(|k| CString::new(k).unwrap())
            .map_or(ptr::null(), |k| k.as_ptr() as *const bindings::std_string);

        let ptr = unsafe { bindings::MMKV_defaultMMKV(mode.into(), crypt_key_ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Get MMKV instance with ID
    pub fn with_id(
        id: &str,
        mode: MMKVMode,
        crypt_key: Option<&str>,
        root_path: Option<&str>,
        expected_capacity: usize,
    ) -> Option<Self> {
        let c_id = CString::new(id).unwrap();
        let crypt_key_ptr = crypt_key
            .map(|k| CString::new(k).unwrap())
            .map_or(ptr::null(), |k| k.as_ptr() as *const bindings::std_string);
        let root_path_ptr = root_path
            .map(|p| CString::new(p).unwrap())
            .map_or(ptr::null(), |p| p.as_ptr() as *const bindings::MMKVPath_t);

        let ptr = unsafe {
            bindings::MMKV_mmkvWithID(
                c_id.as_ptr() as *const bindings::std_string,
                mode.into(),
                crypt_key_ptr,
                root_path_ptr,
                expected_capacity,
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    // Value setters with proper Rust types
    // Helper to create string_view from Rust string
    fn make_string_view(&self, s: &str) -> bindings::std_string_view {
        let c_str = CString::new(s).unwrap();
        let len = s.len();
        unsafe {
            std::mem::transmute::<(*const c_char, usize), bindings::std_string_view>((
                c_str.as_ptr(),
                len,
            ))
        }
    }

    pub fn set_bool(&mut self, key: &str, value: bool) -> bool {
        let key_view = self.make_string_view(key);
        unsafe { bindings::MMKV_set(self.ptr, value, key_view) }
    }

    pub fn set_i32(&mut self, key: &str, value: i32) -> bool {
        let key_view = self.make_string_view(key);
        unsafe { bindings::MMKV_set2(self.ptr, value, key_view) }
    }

    pub fn set_u32(&mut self, key: &str, value: u32) -> bool {
        let key_view = self.make_string_view(key);
        unsafe { bindings::MMKV_set4(self.ptr, value, key_view) }
    }

    pub fn set_i64(&mut self, key: &str, value: i64) -> bool {
        let key_view = self.make_string_view(key);
        unsafe { bindings::MMKV_set6(self.ptr, value, key_view) }
    }

    pub fn set_u64(&mut self, key: &str, value: u64) -> bool {
        let key_view = self.make_string_view(key);
        unsafe { bindings::MMKV_set8(self.ptr, value, key_view) }
    }

    pub fn set_f32(&mut self, key: &str, value: f32) -> bool {
        let key_view = self.make_string_view(key);
        unsafe { bindings::MMKV_set10(self.ptr, value, key_view) }
    }

    pub fn set_f64(&mut self, key: &str, value: f64) -> bool {
        let key_view = self.make_string_view(key);
        unsafe { bindings::MMKV_set12(self.ptr, value, key_view) }
    }

    pub fn set_str(&mut self, key: &str, value: &str) -> bool {
        let key_view = self.make_string_view(key);
        let c_value = CString::new(value).unwrap();
        unsafe { bindings::MMKV_set14(self.ptr, c_value.as_ptr(), key_view) }
    }

    // Value getters
    pub fn get_bool(&mut self, key: &str, default: bool) -> (bool, bool) {
        let key_view = self.make_string_view(key);
        let mut has_value = false;
        let value = unsafe { bindings::MMKV_getBool(self.ptr, key_view, default, &mut has_value) };
        (value, has_value)
    }

    pub fn get_i32(&mut self, key: &str, default: i32) -> (i32, bool) {
        let key_view = self.make_string_view(key);
        let mut has_value = false;
        let value = unsafe { bindings::MMKV_getInt32(self.ptr, key_view, default, &mut has_value) };
        (value, has_value)
    }

    // Other methods...
    pub fn contains_key(&mut self, key: &str) -> bool {
        let key_view = self.make_string_view(key);
        unsafe { bindings::MMKV_containsKey(self.ptr, key_view) }
    }

    pub fn remove_key(&mut self, key: &str) -> bool {
        let key_view = self.make_string_view(key);
        unsafe { bindings::MMKV_removeValueForKey(self.ptr, key_view) }
    }

    pub fn clear_all(&mut self, keep_space: bool) {
        unsafe {
            bindings::MMKV_clearAll(self.ptr, keep_space);
        }
    }
}

impl Drop for MMKV {
    fn drop(&mut self) {
        unsafe {
            bindings::MMKV_close(self.ptr);
        }
    }
}

// Log handler setup
pub fn set_log_level(level: LogLevel) {
    unsafe {
        bindings::MMKV_setLogLevel(level.into());
    }
}
//TODO register log handler
// pub fn register_log_handler<F>(handler: F)
// where
//     F: Fn(LogLevel, &str, i32, &str, &str) + 'static,
// {
//     use std::sync::Arc;
//     let handler = Arc::new(handler);

//     extern "C" fn log_handler_wrapper(
//         level: bindings::MMKVLogLevel,
//         file: *const c_char,
//         line: c_int,
//         function: *const c_char,
//         message: bindings::MMKVLog_t,
//     ) {
//         let level = match level {
//             bindings::MMKVLogLevel_MMKVLogDebug => LogLevel::Debug,
//             bindings::MMKVLogLevel_MMKVLogInfo => LogLevel::Info,
//             bindings::MMKVLogLevel_MMKVLogWarning => LogLevel::Warning,
//             bindings::MMKVLogLevel_MMKVLogError => LogLevel::Error,
//             bindings::MMKVLogLevel_MMKVLogNone => LogLevel::None,
//             _ => LogLevel::None,
//         };

//         let file = unsafe { CStr::from_ptr(file) }.to_str().unwrap_or("");
//         let function = unsafe { CStr::from_ptr(function) }.to_str().unwrap_or("");
//         let message = unsafe { CStr::from_ptr(*message as *const c_char) }
//             .to_str()
//             .unwrap_or("");

//         // Get handler from global storage
//         if let Some(handler) = LOG_HANDLER.lock().unwrap().as_ref() {
//             handler(level, file, line, function, message);
//         }
//     }

//     // Store handler in global storage
//     *LOG_HANDLER.lock().unwrap() = Some(handler);

//     unsafe {
//         bindings::MMKV_registerLogHandler(Some(log_handler_wrapper));
//     }
// }

// // Global storage for log handler
// lazy_static! {
//     static ref LOG_HANDLER: std::sync::Mutex<Option<Arc<dyn Fn(LogLevel, &str, i32, &str, &str) + Send + Sync>>> =
//         std::sync::Mutex::new(None);
// }

// pub fn unregister_log_handler() {
//     *LOG_HANDLER.lock().unwrap() = None;
//     unsafe {
//         bindings::MMKV_unRegisterLogHandler();
//     }
// }
