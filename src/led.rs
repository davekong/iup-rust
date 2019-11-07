//! LED *(Dialog-specification language)* functionalities.
//!
//! LED is a dialog-specification language whose purpose is not to be a complete programming language,
//! but rather to make dialog specification simpler than in C. Additionally it allows users to easily
//! edit your application layout from external files without touching any source code.
//!
//! In LED, attributes and expressions follow this form:
//!
//! `elem = element[attribute1=value1,attribute2=value2,...](...expression...)`
//!
//! The names of the elements must not contain the “iup” prefix.
//! Attribute values are always interpreted as strings, but they need to be in quotes (“…”) only
//! when they include spaces. The “IUP_” prefix must not be added to the names of the attributes
//! and predefined values. Expressions contain parameters for creating the element.
//!
//! In LED there is no distinction between upper and lower case, except for attribute names.
//! 
//! Also there is no optional parameters, in arrays at least one parameter must exist.
//!
//! To simply view a LED file objects use the LED Viewer application called [IupView][1], in the
//! applications included in the distribution. Pre-compiled binaries are available at the
//! [Download][2].
//!
//! You need to check out the [IUP documentation][0] for each control to see their
//! respective function signatures in LED.
//!
//! **Note:** Using LED may allow you to create controls not yet implemented in iup-rust and
//! that's *fine*. Use a `Handle` to have access to controls created from LED.
//!
//! [0]: http://webserver2.tecgraf.puc-rio.br/iup/
//! [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/led.html
//! [2]: http://webserver2.tecgraf.puc-rio.br/iup/en/download.html

use iup_sys;
use std::path::Path;
use std::result::Result;
use std::ffi::CString;


/// Compiles a LED specification from a file.
///
/// Each time the function loads a LED file, the elements contained in it are created.
/// Therefore, the same LED file cannot be loaded several times, otherwise the elements will also
/// be created several times.
///
/// In case of failure returns the compilation error message.
pub fn load<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref();

    let str = path.to_str().ok_or_else(|| "Failed to convert Path to string".to_string())?;
    let cstr = CString::new(str).unwrap();

    match unsafe { iup_sys::IupLoad(cstr.as_ptr()) } {
        err if err.is_null() => Ok(()),
        err => Err(string_from_cstr!(err)),
    }
}

/// Compiles a LED specification from a string.
///
/// See the `load` function for additional semantic details.
pub fn load_buffer<S: Into<String>>(buf: S) -> Result<(), String> {
    let cstr = CString::new(buf.into()).unwrap();
    match unsafe { iup_sys::IupLoadBuffer(cstr.as_ptr()) } {
        err if err.is_null() => Ok(()),
        err => Err(string_from_cstr!(err)),
    }
}

