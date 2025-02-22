// adapted from: https://rust-unofficial.github.io/patterns/idioms/ffi/accepting-strings.html
// and from:     http://jakegoulding.com/rust-ffi-omnibus/string_return/

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Log a message at the specified level.
///
/// # Safety
///
/// It is the caller's guarantee to ensure `stmt`:
///
/// - is not a null pointer
/// - points to valid, initialized data
/// - points to memory ending in a null byte
/// - won't be mutated for the duration of this function call
#[no_mangle]
pub unsafe extern "C" fn acquire_help(c_stmt: *const c_char) -> *mut c_char {
    if c_stmt.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = CStr::from_ptr(c_stmt);
    let stmt = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let help = match stmt {
        "application" => include_str!("help/application.md"),
        "AV-Bible-S4T" => include_str!("help/AV-Bible-S4T.md"),
        "export" => include_str!("help/export.md"),
        "hashtags" => include_str!("help/hashtags.md"),
        "language" => include_str!("help/language.md"),
        "selection" => include_str!("help/selection.md"),
        "settings" => include_str!("help/settings.md"),
        "system" => include_str!("help/system.md"),
        _ => "",
    };

    let c_string = CString::new(help).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn release_help(c_lent: *mut c_char) -> bool {
    unsafe {
        if c_lent.is_null() {
            return false;
        }
        let _reclaim = CString::from_raw(c_lent);
        return true;
    };
}

#[no_mangle]
pub unsafe extern "C" fn get_library_revision() -> u32 {
    return 0x09190214;
}