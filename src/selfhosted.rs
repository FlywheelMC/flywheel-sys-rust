use crate::prelude::*;


#[allow(dead_code)]
unsafe extern "C" {
    pub unsafe fn flywheel_system_set_motd(in_motd : u32, motd_len : u32);
    pub unsafe fn flywheel_trace(in_msg : u32, msg_len : u32);
    pub unsafe fn flywheel_debug(in_msg : u32, msg_len : u32);
    pub unsafe fn flywheel_info(in_msg : u32, msg_len : u32);
    pub unsafe fn flywheel_pass(in_msg : u32, msg_len : u32);
    pub unsafe fn flywheel_warn(in_msg : u32, msg_len : u32);
    pub unsafe fn flywheel_error(in_msg : u32, msg_len : u32);
    pub unsafe fn flywheel_fatal(in_msg : u32, msg_len : u32);
}


impl Server {

    /// Changes the server list MOTD text.
    #[doc(cfg(feature = "selfhosted"))]
    pub fn set_motd(motd : &str) {
        unsafe { flywheel_system_set_motd(motd.as_ptr() as u32, motd.len() as u32); }
    }

}

/// Logs a message to the console at the `TRACE` level.
///
/// Only available on selfhosted Flywheel servers.
#[doc(cfg(feature = "selfhosted"))]
pub macro trace( $( $fmt:tt )* ) { {
    let msg = format!( $( $fmt )* );
    let msg = format!("[{}:{}:{}] {msg}", module_path!(), line!(), column!());
    unsafe { flywheel_trace(msg.as_ptr() as u32, msg.len() as u32); }
} }

/// Logs a message to the console at the `DEBUG` level.
///
/// Only available on selfhosted Flywheel servers.
#[doc(cfg(feature = "selfhosted"))]
pub macro debug( $( $fmt:tt )* ) { {
    let msg = format!( $( $fmt )* );
    let msg = format!("[{}:{}:{}] {msg}", module_path!(), line!(), column!());
    unsafe { flywheel_debug(msg.as_ptr() as u32, msg.len() as u32); }
} }

/// Logs a message to the console at the `INFO` level.
///
/// Only available on selfhosted Flywheel servers.
#[doc(cfg(feature = "selfhosted"))]
pub macro info( $( $fmt:tt )* ) { {
    let msg = format!( $( $fmt )* );
    let msg = format!("[{}:{}:{}] {msg}", module_path!(), line!(), column!());
    unsafe { flywheel_info(msg.as_ptr() as u32, msg.len() as u32); }
} }

/// Logs a message to the console at the `PASS` level.
///
/// Only available on selfhosted Flywheel servers.
#[doc(cfg(feature = "selfhosted"))]
pub macro pass( $( $fmt:tt )* ) { {
    let msg = format!( $( $fmt )* );
    let msg = format!("[{}:{}:{}] {msg}", module_path!(), line!(), column!());
    unsafe { flywheel_pass(msg.as_ptr() as u32, msg.len() as u32); }
} }

/// Logs a message to the console at the `WARN` level.
///
/// Only available on selfhosted Flywheel servers.
#[doc(cfg(feature = "selfhosted"))]
pub macro warn( $( $fmt:tt )* ) { {
    let msg = format!( $( $fmt )* );
    let msg = format!("[{}:{}:{}] {msg}", module_path!(), line!(), column!());
    unsafe { flywheel_warn(msg.as_ptr() as u32, msg.len() as u32); }
} }

/// Logs a message to the console at the `ERROR` level.
///
/// Only available on selfhosted Flywheel servers.
#[doc(cfg(feature = "selfhosted"))]
pub macro error( $( $fmt:tt )* ) { {
    let msg = format!( $( $fmt )* );
    let msg = format!("[{}:{}:{}] {msg}", module_path!(), line!(), column!());
    unsafe { flywheel_error(msg.as_ptr() as u32, msg.len() as u32); }
} }

/// Logs a message to the console at the `FATAL` level.
///
/// Only available on selfhosted Flywheel servers.
#[doc(cfg(feature = "selfhosted"))]
pub macro fatal( $( $fmt:tt )* ) { {
    let msg = format!( $( $fmt )* );
    let msg = format!("[{}:{}:{}] {msg}", module_path!(), line!(), column!());
    unsafe { flywheel_fatal(msg.as_ptr() as u32, msg.len() as u32); }
} }
