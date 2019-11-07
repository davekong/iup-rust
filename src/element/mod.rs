//! Common operations between objects.
//!
//! Every IUP object is so called an element and can be encapsulated in a `Handle`.
use iup_sys;
use libc::{c_void, c_char, c_int};
use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::result::Result;
use std::iter::repeat;
use std::str::FromStr;

pub mod guard;
pub use self::guard::Guard;

pub mod hierarchy;
pub use self::hierarchy::{Container, Node};

#[macro_use]
pub mod widget;
pub use self::widget::Widget;

/// Makes a Vec of `Element` trait objects.
///
/// This actually uses the `Handle` wrapper instead of `Element` due to the Sized requirement.
/// 
/// This should be passed to functions that expect a list of elements in the constructor.
#[macro_export]
macro_rules! elements {
    () => { vec! [] };
    ($($elem:expr),+,) => { elements! [ $($elem),+ ] };
    ($($elem:expr),*) => { vec! [ $($crate::element::Handle::from($elem)),* ] };
}

/// This macro should be used for every type binding IUP handles.
///
/// See applicable `$classname`s [here][1]. Some classes aren't on the list and should be
/// picked up manually by looking at the IUP source code or by looking at the result
/// of `Element::classname`.
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/func/iupgetclassname.html
macro_rules! impl_element {
    ($ty_path:path, $classname:expr) => {
        impl_element_nofrom!($ty_path, $classname);

        impl From<$ty_path> for $crate::element::Handle {
            fn from(elem: $ty_path) -> $crate::element::Handle {
                unsafe { $crate::element::Handle::from_raw_unchecked(elem.raw()) }
            }
        }
    };
}

/// This is called from impl_element! to do all the work.
///
/// This is a necessary thing because if we implemented `From<$ty_path> for Handle` here it'd cause
/// a compilation error during `From<Handle> for Handle`.
macro_rules! impl_element_nofrom {
    ($ty_path:path, $classname:expr) => {

        impl $crate::Element for $ty_path {
            #[inline(always)]
            fn raw(&self) -> *mut iup_sys::Ihandle {
                self.0
            }
            #[inline(always)]
            unsafe fn from_raw_unchecked(ih: *mut iup_sys::Ihandle) -> Self {
                $ty_path(ih)
            }
            #[inline]
            unsafe fn target_classname() -> &'static str {
                $classname
            }
        }

        impl ::std::fmt::Debug for $ty_path {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                fmt.write_fmt(format_args!("{}({:p})", stringify!($ty_path), self.raw()))
            }
        }

        impl Copy for $ty_path {}
        
        impl Clone for $ty_path {
            fn clone(&self) -> $ty_path {
                *self
            }
        }

        impl $crate::callback::DestroyCb for $ty_path {}
    };
}

/// An object that can wrap **any** IUP element.
///
/// The handle also provides implementation for traits that mayn't be implemented for the
/// contained element, so be careful when using it.
pub struct Handle(*mut iup_sys::Ihandle);

impl Handle {

    /// Constructs from a name associated with a element handle (with `Element::add_handle_name` or LED).
    pub fn from_named<S: Into<String>>(name: S) -> Option<Handle> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupGetHandle(cname.as_ptr()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Converts this handle object into a element object if they are compatible.
    pub fn try_downcast<E: Element>(self) -> Result<E, Handle> {
        if self.can_downcast::<E>() {
            // Since a Handle must be obtained also by using `from_raw` we can assume the handle
            // has reached the Rust binding thought it and thus using `from_raw_unchecked` here.
            Ok(unsafe { E::from_raw_unchecked(self.raw()) })
        } else {
            Err(self)
        }
    }

    /// Checks if this Element type can be downcasted to the type E.
    fn can_downcast<E: Element>(&self) -> bool {
        let lhs = unsafe { self.classname().to_bytes() };
        let rhs = unsafe { E::target_classname().as_bytes() };
        lhs == rhs || rhs == b"__iuprusthandle"
        // In case self/lhs (a Handle) is trying to cast to a target object of Handle, let it go.
    }
}

impl_element_nofrom!(Handle, "__iuprusthandle");

/// Note: The wrapped element may not support `Container`.
impl Container for Handle {}
/// Note: The wrapped element may not support `Node`.
impl Node for Handle {}
/// Note: The wrapped element may not support `Widget`.
impl Widget for Handle {}
/// Note: The wrapped element may not support `MapCb`.
impl ::callback::MapCb for Handle {}
/// Note: The wrapped element may not support `UnmapCb`.
impl ::callback::UnmapCb for Handle {}
/// Note: The wrapped element may not support `GetFocusCb`.
impl ::callback::GetFocusCb for Handle {}
/// Note: The wrapped element may not support `KillFocusCb`.
impl ::callback::KillFocusCb for Handle {}
/// Note: The wrapped element may not support `EnterWindowCb`.
impl ::callback::EnterWindowCb for Handle {}
/// Note: The wrapped element may not support `LeaveWindowCb`.
impl ::callback::LeaveWindowCb for Handle {}
/// Note: The wrapped element may not support `HelpCb`.
impl ::callback::HelpCb for Handle {}
// TODO impl K_ callbacks when it's implemented.


/// Every IUP object is an `Element`.
pub trait Element : Sized + Copy + Clone {

    /// Constructs a specialized Element object from a general Handle if they are compatible.
    fn from_handle(handle: Handle) -> Result<Self, Handle> {
        handle.try_downcast::<Self>()
    }

    /// Constructs from a name associated with a element handle (with `Element::add_handle_name` or LED).
    fn from_name<S: Into<String>>(name: S) -> Option<Handle> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupGetHandle(cname.as_ptr()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Constructs an Element from a raw IUP handle.
    ///
    /// # Safety
    /// The `from_raw_unchecked` method is faster than `from_raw` but must be used with care.
    ///
    /// The Rust binding performs important operations and checks when a raw IUP handle reaches
    /// the bounds of safe Rust binding, that only happens when `from_raw` is used. Be sure the
    /// raw handle has reached safe Rust bounds at least once before using this method.
    ///
    /// It's undefined behaviour if the raw handle is incompatible with `Self` bindings.
    /// Instead use the `Element::from_handle` to perform safe downcasting.
    unsafe fn from_raw_unchecked(ih: *mut iup_sys::Ihandle) -> Self;

    /// Constructs an Element from a raw IUP handle.
    ///
    /// It's undefined behaviour if the raw handle is incompatible with `Self` bindings.
    /// Instead use the `Element::from_handle` to perform safe downcasting.
    ///
    /// # Panics
    /// Panics if the raw handle is a null pointer.
    fn from_raw(ih: *mut iup_sys::Ihandle) -> Self {
        if ih.is_null() {
            panic!("Failed to create IUP element from raw handle because the handle is null.")
        } else {
            unsafe {
                // Note: DESTROY_CB is used here instead of LDESTROY_CB because the DESTROY_CB 
                // is called later. LDESTROY_CB is used in callbacks.rs, see it for more details.
                iup_sys::IupSetCallback(ih, cstr!("DESTROY_CB"), on_element_destroy);
                Element::from_raw_unchecked(ih)
            }
        }
    }

    /// Gets the raw IUP handle associated with this element.
    fn raw(&self) -> *mut iup_sys::Ihandle;

    /// Gets the [class name][1] of this element.
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/func/iupgetclassname.html
    unsafe fn classname(&self) -> &CStr {
        CStr::from_ptr(iup_sys::IupGetClassName(self.raw()))
    }

    /// Gets the [class name][1] the derived object should be targeting.
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/func/iupgetclassname.html
    unsafe fn target_classname() -> &'static str;

    /// Destroys an interface element and all its children.
    ///
    /// Only dialogs, timers, popup menus and images should be normally destroyed, but **detached**
    /// controls can also be destroyed.
    ///
    /// Menu bars associated with dialogs are automatically destroyed when the dialog is destroyed. 
    ///
    /// Images associated with controls are **NOT** automatically destroyed. The application must
    /// destroy them when they are not used anymore.
    fn destroy(self) {
        unsafe { iup_sys::IupDestroy(self.raw()) };
    }

    /// Checks if a specific attribute exists in the element.
    fn does_attrib_exist(&self, cname: &CString) -> bool {
        let attrib = unsafe { iup_sys::IupGetAttribute(self.raw(), cname.as_ptr()) };
        !attrib.is_null()
    }

    /// Returns the names of all attributes of an element that are set in its internal
    /// hash table only.
    fn attribs(&self) -> Vec<String> {
        let max = unsafe { iup_sys::IupGetAllAttributes(self.raw(), ptr::null_mut(), 0) };
        let mut vec = repeat(ptr::null_mut()).take(max as usize).collect::<Vec<*mut c_char>>();
        let len = unsafe { iup_sys::IupGetAllAttributes(self.raw(), vec.as_mut_ptr(), max) };
        vec.into_iter().take(len as usize).map(|cstr| string_from_cstr!(cstr)).collect()
    }

    /// Sets an interface element attribute.
    ///
    /// See also the [IUP Attributes Guide][1].
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/attrib_guide.html
    fn set_attrib<S1, S2>(&mut self, name: S1, value: S2) -> Self
                                        where S1: Into<String>, S2: Into<String> {
        // The way IupSetAttribute works is infeasible to safety. Use IupSetStrAttribute.
        let cname = CString::new(name.into()).unwrap();
        let cvalue = CString::new(value.into()).unwrap();
        unsafe { iup_sys::IupSetStrAttribute(self.raw(), cname.as_ptr(), cvalue.as_ptr()) };
        self.clone()
    }

    /// Gets an interface element attribute.
    ///
    /// See also the [IUP Attributes Guide][1].
    /// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/attrib_guide.html
    fn attrib<S: Into<String>>(&self, name: S) -> Option<String> {
        // Notice IupGetAttribute does not really give strings but pointers (that may be anything)
        // most (if not all) the default IUP attributes are string values, so we are safe by
        // defaulting to IupGetAttribute. A method should be defined to deal with raw attributes.
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupGetAttribute(self.raw(), cname.as_ptr()) } {
            cvalue if cvalue.is_null() => None,
            cvalue => Some(string_from_cstr!(cvalue)),
        }
    }

    /// Sets a raw interface element attribute.
    ///
    /// # Safety
    /// While this function is not unsafe, care must be taken while using it, prefer to use
    /// `Element::set_attrib` instead. The `data` pointer must live long enough (most of the time
    /// statically).
    fn set_attrib_data<S1>(&mut self, name: S1, data: *const c_void) -> Self
                                                              where S1: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupSetAttribute(self.raw(), cname.as_ptr(), data as *const c_char) };
        self.clone()
    }

    /// Gets a raw interface element attribute.
    fn attrib_data<S1>(&mut self, name: S1) -> *mut c_void
                                       where S1: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupGetAttribute(self.raw(), cname.as_ptr()) as *mut c_void }
    }

    /// Associates a element with an attribute.
    ///
    /// Instead of using `Element::add_handle_name` and `Element::set_attrib` with a new creative
    /// name, this function automatically creates a non conflict handle name and associates that
    /// with the attribute.
     fn set_attrib_handle<S1, E>(&mut self, name: S1, elem: E) -> Self
                                                where S1: Into<String>, E: Element {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupSetAttributeHandle(self.raw(), cname.as_ptr(), elem.raw()) };
        self.clone()
    }   

    /// Gets the handle associated with an attribute.
    fn attrib_handle<S1>(&mut self, name: S1) -> Option<Handle>
                                    where S1: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupGetAttributeHandle(self.raw(), cname.as_ptr()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Clears the value associated with an attribute and use the default value.
    fn clear_attrib<S: Into<String>>(&mut self, name: S) -> Self {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupSetAttribute(self.raw(), cname.as_ptr(), ptr::null()) };
        self.clone()
    }

    /// Removes an attribute from element and its children if the attrib is inheritable.
    ///
    /// It is useful to reset the state of inheritable attributes in a tree of elements.
    fn reset_attrib<S: Into<String>>(&mut self, name: S) -> Self {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupResetAttribute(self.raw(), cname.as_ptr()) };
        self.clone()
    }

    /// Returns the identifier of an interface element that has an associated handle name using
    /// `Element::add_handle_name` or using LED.
    ///
    /// Handle names shouldn't be confused with the NAME attribute.
    fn handle_name(&self) -> Option<String> {
        match unsafe { iup_sys::IupGetName(self.raw()) } {
            name if name.is_null() => None,
            name => Some(string_from_cstr!(name)),
        }
    }

    /// Associates a handle name with an interface element.
    ///
    /// Can be called several times with the same element and different names.
    /// There is no restriction for the number of names a pointer can have, but `Element::handle_name`
    /// will return the first name found.
    ///
    /// Returns the handle of the interface element previously associated to the parameter name.
    ///
    /// Handle names shouldn't be confused with the NAME attribute.
    fn add_handle_name<S: Into<String>>(&self, name: S) -> Option<Handle> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupSetHandle(cname.as_ptr(), self.raw()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }

    /// Clears the handle name association on the specified name.
    ///
    /// Note this will not destroy associated elements, just remove a name from the
    /// association table.
    ///
    /// Returns the handle of the interface element previously associated to the parameter name.
    ///
    /// Handle names shouldn't be confused with the NAME attribute.
    fn clear_handle_name<S: Into<String>>(name: S) -> Option<Handle> {
        let cname = CString::new(name.into()).unwrap();
        match unsafe { iup_sys::IupSetHandle(cname.as_ptr(), ptr::null_mut()) } {
            ptr if ptr.is_null() => None,
            ptr => Some(Handle::from_raw(ptr)),
        }
    }



    #[doc(hidden)]
    fn set_attrib_rgb<S1>(&mut self, name: S1, rgb: (u8, u8, u8)) -> Self
                                                              where S1: Into<String> {
        let cname = CString::new(name.into()).unwrap();
        unsafe { iup_sys::IupSetRGB(self.raw(), cname.as_ptr(), rgb.0, rgb.1, rgb.2) };
        self.clone()
    }

    #[doc(hidden)]
    fn attrib_rgb<S1>(&self, name: S1) -> Option<(u8, u8, u8)>
                                       where S1: Into<String> {
        unsafe {
            let cname = CString::new(name.into()).unwrap();
            if self.does_attrib_exist(&cname) {
                let mut rgb: (u8, u8, u8) = mem::MaybeUninit::uninit().assume_init();
                iup_sys::IupGetRGB(self.raw(), cname.as_ptr(), &mut rgb.0, &mut rgb.1, &mut rgb.2);
                Some(rgb)
            } else {
                None
            }
        }
    }

    #[doc(hidden)]
    fn attrib_bool<S1>(&self, name: S1) -> Option<bool>
                                       where S1: Into<String> {
        unsafe {
            let cname = CString::new(name.into()).unwrap();
            if self.does_attrib_exist(&cname) {
                Some(iup_sys::IupGetInt(self.raw(), cname.as_ptr()) != 0)
            } else {
                None
            }
        }
    }

    #[doc(hidden)]
    fn attrib_parse<T, S1>(&self, name: S1) -> Option<T>
                            where T: FromStr, S1: Into<String> {
        self.attrib(name).and_then(|s| s.parse::<T>().ok())
    }

}

pub trait ConvertXYToPos : Element {
    /// Converts a x,y coordinate in an item position in the container.
    ///
    /// The x,y coordinates are relative to the left corner and top corner of the element.
    ///
    /// This have a different effect for each control it is applied to, check their documentation.
    fn convert_xy_to_pos(&self, x: i32, y: i32) -> Option<i32> {
        match unsafe { iup_sys::IupConvertXYToPos(self.raw(), x as c_int, y as c_int) } {
            -1 => None,
            id => Some(id),
        }
    }
}


/// Sets an attribute in the global environment.
///
/// If the driver process the attribute then it will not be stored internally.
pub fn set_global<S1, S2>(name: S1, value: S2) 
                                where S1: Into<String>, S2: Into<String> {
    let cname = CString::new(name.into()).unwrap();
    let cvalue = CString::new(value.into()).unwrap();
    unsafe { iup_sys::IupSetStrGlobal(cname.as_ptr(), cvalue.as_ptr()) };
}

/// Returns an attribute value from the global environment.
///
/// The value can be returned from the driver or from the internal storage.
///
/// This function’s return value is not necessarily the same one used by the application to
/// set the attribute’s value.
pub fn global<S: Into<String>>(name: S) -> Option<String> {
    let cname = CString::new(name.into()).unwrap();
    match unsafe { iup_sys::IupGetGlobal(cname.as_ptr()) } {
        cvalue if cvalue.is_null() => None,
        cvalue => Some(string_from_cstr!(cvalue)),
    }
}

/// Clears the value associated with an global attribute.
pub fn clear_attrib<S: Into<String>>(name: S) {
    let cname = CString::new(name.into()).unwrap();
    unsafe { iup_sys::IupSetGlobal(cname.as_ptr(), ptr::null()) };
}

pub fn set_global_data<S1>(name: S1, data: *const c_void)
                                            where S1: Into<String> {
    let cname = CString::new(name.into()).unwrap();
    unsafe { iup_sys::IupSetGlobal(cname.as_ptr(), data as *const _) };
}

pub fn global_data<S1>(name: S1) -> *mut c_void
                           where S1: Into<String> {
    let cname = CString::new(name.into()).unwrap();
    unsafe { iup_sys::IupGetGlobal(cname.as_ptr()) as *mut c_void }
}


/// Called whenever a Element gets destroyed.
///
/// Use this to perform frees related to the Rust binding that are per-element.
extern fn on_element_destroy(ih: *mut iup_sys::Ihandle) -> c_int {
    unsafe { ::callback::drop_callbacks(ih); }
    iup_sys::IUP_DEFAULT
}
