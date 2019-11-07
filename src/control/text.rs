use iup_sys;
use libc::{c_char, c_int};
use std::{mem, ptr};

use Element;

/// See the [IUP Text Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iuptext.html
pub struct Text(*mut iup_sys::Ihandle);

impl Text {
    /// Creates a editable text-field.
    pub fn new() -> Text {
        unsafe { Text::from_raw(iup_sys::IupText(ptr::null_mut())) }
    }

    /// Creates a spin control.
    ///
    /// The spin increments and decrements an integer number. 
    pub fn new_spin() -> Text {
        Text::new().set_attrib_data("SPIN", cstr!("YES") as *const _)
    }

    /// Converts a (lin, col) character positioning into an absolute position.
    ///
    /// lin and col starts at 1, pos starts at 0. For single line controls pos is always *col-1*.
    pub fn convert_lincol_to_pos(&self, lin: i32, col: i32) -> usize {
        unsafe {
            let mut r: c_int = mem::MaybeUninit::uninit().assume_init();
            iup_sys::IupTextConvertLinColToPos(self.raw(), lin, col, &mut r);
            r as usize
        }
    }

    /// Converts an absolute position into a (lin, col) character positioning.
    ///
    /// lin and col starts at 1, pos starts at 0.
    /// For single line controls lin is always 1, and col is always *pos+1*.
    pub fn convert_pos_to_lincol(&self, pos: usize) -> (i32, i32) {
        unsafe {
            let (mut lin, mut col): (c_int, c_int) = (mem::MaybeUninit::uninit().assume_init(), mem::MaybeUninit::uninit().assume_init());
            iup_sys::IupTextConvertPosToLinCol(self.raw(), pos as c_int, &mut lin, &mut col);
            (lin as i32, col as i32)
        }
    }
}

impl_widget!(Text, "text");

/// Returns a position in the string.
impl ::element::ConvertXYToPos for Text {}

impl ::callback::MapCb for Text {}
impl ::callback::UnmapCb for Text {}
impl ::callback::GetFocusCb for Text {}
impl ::callback::KillFocusCb for Text {}
impl ::callback::EnterWindowCb for Text {}
impl ::callback::LeaveWindowCb for Text {}
impl ::callback::HelpCb for Text {}
// TODO impl K_ callbacks when it's implemented.

// TODO impl future DragSource and DragTarget traits.

/// Action generated when any mouse button is pressed or released.
///
/// Use `convert_xy_to_pos` to convert (x,y) coordinates in character positioning.
impl ::callback::button::ButtonCb for Text {}

/// Action generated when the caret/cursor position is changed. 
///
/// For single line controls `lin` (2nd param) is always *1*, and `pos` (3rd param) is always *col-1*.
impl ::callback::CaretCb for Text {}

/// Action generated when one or more files are dropped in the element.
impl ::callback::DropFilesCb for Text {}

/// Action generated when the mouse is moved.
///
/// Use `convert_xy_to_pos` to convert (x,y) coordinates in character positioning.
impl ::callback::button::MotionCb for Text {}

/// Action generated when a spin button is pressed.
///
/// Valid only when the element is a spin (`Text::new_spin` or SPIN=YES attribute).
/// When this callback is called the ACTION callback is not called.
/// The VALUE attribute can be changed during this callback only if SPINAUTO=NO.
///
/// The `i32` parameter received by the callback is the value of the spin (after incremented).
///
/// May return `CallbackReturn::Ignore` but that is only allowed on Windows and Motif.
impl ::callback::SpinCb for Text {}

/// Called after the value was interactively changed by the user.
impl ::callback::ValueChangedCb for Text {}

/// See the `TextAction` documentation.
impl self::TextAction for Text {}
impl_callback! {
    #[doc="Action generated when the text is edited, but before its value is actually changed."]
    #[doc=""]
    #[doc="Can be generated when using the keyboard, undo system or from the clipboard."]
    #[doc=""]
    #[doc="The following values can be returned from the callback:"]
    #[doc=" + `CallbackReturn::Default` or `()` for the default reaction."]
    #[doc=" + `CallbackReturn::Close` will be processed, but the change will be ignored."]
    #[doc=" + `CallbackReturn::Ignore` ignores the new value."]
    #[doc=" + An `CallbackReturn::Char`, if the received `c` is `None` or NUL is returned it'll act"]
    #[doc="   just like `CallbackReturn::Default` otherwise the returned character will be used"]
    #[doc="   instead of the one sent to the callback."]
    #[doc=""]
    #[doc="The VALUE attribute can be changed only if `CallbackReturn::Ignore` is returned."]
    #[doc=""]
    #[doc="**NOTE:** The **character** received and returned must be in the ASCII range of"]
    #[doc=" UTF-8 (0-127). No restriction on the `newvalue` string, when the value added to the VALUE"]
    #[doc="is a non-ASCII character the `c` parameter will be `None` but the value is still used for"]
    #[doc="updating the VALUE attribute."]
    pub trait TextAction where Self: Element {
        let name = "ACTION";
        extern fn listener(ih: *mut iup_sys::Ihandle, c: c_int, newvalue: *const c_char) -> CallbackReturn;
        fn set_action<F: Callback(Self, Option<char>, String)>(&mut self, cb: F) -> Self;
        fn remove_action(&mut self) -> Option<Box<_>>;
    }
}
