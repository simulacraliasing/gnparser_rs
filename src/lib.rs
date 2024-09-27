use libc::{c_char, c_int};
use std::ffi::{CStr, CString};

#[link(name = "gnparser")]
extern "C" {
    fn ParseToString(name: *const c_char, f: *const c_char, details: c_int, cultivars: c_int, diaereses: c_int) -> *mut c_char;
    fn FreeMemory(p: *mut c_char);
    fn ParseAryToString(in_: *const *const c_char, length: c_int, f: *const c_char, details: c_int, cultivars: c_int, diaereses: c_int) -> *mut c_char;
}

pub fn parse_to_string(name: &str, format: &str, details: bool, cultivars: bool, diaereses: bool) -> Result<String, String> {
    let name_cstr = CString::new(name).map_err(|e| e.to_string())?;
    let format_cstr = CString::new(format).map_err(|e| e.to_string())?;
    let details = if details { 1 } else { 0 };
    let cultivars = if cultivars { 1 } else { 0 };
    let diaereses = if diaereses { 1 } else { 0 };

    unsafe {
        let result_ptr = ParseToString(name_cstr.as_ptr(), format_cstr.as_ptr(), details, cultivars, diaereses);
        if result_ptr.is_null() {
            return Err("Failed to parse name".into());
        }
        let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
        FreeMemory(result_ptr);
        Ok(result)
    }
}

pub fn parse_ary_to_string(names: &[&str], format: &str, details: bool, cultivars: bool, diaereses: bool) -> Result<String, String> {
    let format_cstr = CString::new(format).map_err(|e| e.to_string())?;
    let details = if details { 1 } else { 0 };
    let cultivars = if cultivars { 1 } else { 0 };
    let diaereses = if diaereses { 1 } else { 0 };

    let cstrings: Vec<CString> = names.iter().map(|&s| CString::new(s).unwrap()).collect();
    let ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();

    unsafe {
        let result_ptr = ParseAryToString(ptrs.as_ptr(), names.len() as c_int, format_cstr.as_ptr(), details, cultivars, diaereses);
        if result_ptr.is_null() {
            return Err("Failed to parse names array".into());
        }
        let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
        FreeMemory(result_ptr);
        Ok(result)
    }
}