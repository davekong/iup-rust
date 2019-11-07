use libc::c_char;
use std::path::PathBuf;

//
// The following regex can be used to convert from doc comments to attrib comments:
//     ([\t ]*)\/\/\/[ ]?(.*)
//     $1#[doc="$2"]
//
// See also PR #26
//

// Note: This callback is IUP-Rust specific, evaluated manually in `with_iup`.
impl_callback! {
    let name = "_IUPRUST_CLOSE_CB";
    extern fn listener() -> CallbackReturn;
    #[doc="Action generated when IUP closes (i.e. at the end of `with_iup`)."]
    pub fn set_close_cb<F: Callback()>(cb: F);
    #[doc="Removes a previosly set up close callback."]
    pub fn remove_close_cb() -> Option<Box<_>>;
}

impl_callback! {
    let name = "IDLE_ACTION";
    extern fn listener() -> CallbackReturn;
    #[doc="Action generated when there are no events or messages to be processed."]
    #[doc=""]
    #[doc="Often used to perform background operations."]
    pub fn set_idle<F: Callback()>(cb: F);
    #[doc="Removes a previosly set up idle_action callback."]
    pub fn remove_idle() -> Option<Box<_>>;
}

// Common Callbacks
// ----------------------------

// This is the common version of the ACTION callback, any so called ACTION that does not
// have the `(*mut iup_sys::Ihandle)` signature should be another trait.
impl_callback! {
    #[doc="Action generated when the element is activated. Affects each element differently."]
    #[doc=""]
    #[doc="See the documentation of the `Self` object for the effect of this callback on it."]
    pub trait Action where Self: Element {
        let name = "ACTION";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_action<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_action(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Action generated when the element is activated. Affects each element differently."]
    #[doc=""]
    #[doc="See the documentation of the `Self` object for the effect of this callback on it."]
    #[doc=""]
    #[doc="Note: This is different from the `ACTION` callback."]
    pub trait ActionCb where Self: Element {
        let name = "ACTION_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_action_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_action_cb(&mut self) -> Option<Box<_>>;
    }
}

// Note on using LDESTROY_CB instead of DESTROY_CB:
//
//    IUP calls LDESTROY_CB and then calls DESTROY_CB. It was thought LDESTROY_CB should be used
//    for our binding to free things up, but we're doing it the other way around for simplicity.
//
//    The binding free needs to be called after LDESTROY_CB (i.e. at DESTROY_CB) because
//    the LDESTROY_CB event call depends on the binding boxed data to work properly.
impl_callback! {
    #[doc="Called right before an element is destroyed."]
    pub trait DestroyCb where Self: Element {
        let name = "LDESTROY_CB";   // See comments above for reason behind LDESTROY_CB.
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_destroy_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_destroy_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Called right after an element is mapped and its attributes updated in `Widget::map`."]
    #[doc=""]
    #[doc="When the element is a dialog, it is called after the layout is updated. For all other"]
    #[doc="elements is called before the layout is updated, so the element current size will still"]
    #[doc="be 0x0 during MAP_CB."]
    pub trait MapCb where Self: Element {
        let name = "MAP_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_map_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_map_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Called right before an element is unmapped."]
    pub trait UnmapCb where Self: Element {
        let name = "UNMAP_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_unmap_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_unmap_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Action generated when an element is given keyboard focus."]
    #[doc=""]
    #[doc="This callback is called after the `KillFocusCb` of the element that loosed the focus."]
    #[doc="The IupGetFocus (TODO) function during the callback returns the element that loosed the focus."]
    pub trait GetFocusCb where Self: Element {
        let name = "GETFOCUS_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_getfocus_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_getfocus_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Action generated when an element loses keyboard focus."]
    #[doc=""]
    #[doc="This callback is called before the `GetFocusCb` of the element that gets the focus."]
    #[doc=""]
    #[doc="While processing this message, do not make any function calls that display or activate a"]
    #[doc="window. This causes the thread to yield control and can cause the application to stop"]
    #[doc="responding to messages."]
    pub trait KillFocusCb where Self: Element {
        let name = "KILLFOCUS_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_killfocus_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_killfocus_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Action generated when the mouse enters the native element."]
    #[doc=""]
    #[doc="When the cursor is moved from one element to another, the call order in all platforms will"]
    #[doc="be first the `LeaveWindowCb` callback of the old control followed by the `EnterWindowCb`"]
    #[doc="callback of the new control."]
    pub trait EnterWindowCb where Self: Element {
        let name = "ENTERWINDOW_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_enterwindow_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_enterwindow_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Action generated when the mouse leaves the native element."]
    #[doc=""]
    #[doc="When the cursor is moved from one element to another, the call order in all platforms will"]
    #[doc="be first the `LeaveWindowCb` callback of the old control followed by the `EnterWindowCb`"]
    #[doc="callback of the new control."]
    pub trait LeaveWindowCb where Self: Element {
        let name = "LEAVEWINDOW_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_leavewindow_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_leavewindow_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Action generated when the user press F1 at a control."]
    #[doc=""]
    #[doc="`CallbackReturn::Close` will be processed."]
    pub trait HelpCb where Self: Element {
        let name = "HELP_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_help_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_help_cb(&mut self) -> Option<Box<_>>;
    }
}

// Other Callbacks
// ----------------------------

impl_callback! {
    #[doc="Action generated when the caret/cursor position is changed."]
    #[doc=""]
    #[doc="The second and third parameters are the line and column number (start at 1)."]
    #[doc="The fourth parameter is a 0 based character position."]
    pub trait CaretCb where Self: Element {
        let name = "CARET_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, lin: c_int, col: c_int, pos: c_int) -> CallbackReturn;
        fn set_caret_cb<F: Callback(Self, i32, i32, usize)>(&mut self, cb: F) -> Self;
        fn remove_caret_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Action generated when a spin button is pressed."]
    pub trait SpinCb where Self: Element {
        let name = "SPIN_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, i: c_int) -> CallbackReturn;
        fn set_spin_cb<F: Callback(Self, i32)>(&mut self, cb: F) -> Self;
        fn remove_spin_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Usually called after the value of a control changed."]
    #[doc=""]
    #[doc="See the specific control documentation for more details."]
    pub trait ValueChangedCb where Self: Element {
        let name = "VALUECHANGED_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_valuechanged_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_valuechanged_cb(&mut self) -> Option<Box<_>>;
    }
}


impl_callback! {
    #[doc="Action called when a file is *dropped* into the control."]
    #[doc=""]
    #[doc="When several files are dropped at once, the callback is called several times, once for"]
    #[doc="each file. "]
    #[doc=""]
    #[doc="If defined after the element is mapped then the attribute DROPFILESTARGET must be set"]
    #[doc="to YES."]
    #[doc=""]
    #[doc="The third parameter of the callback is the number index of the dropped file. If several"]
    #[doc="files are dropped, it is the index of the dropped file starting from *total-1* to *0*."]
    #[doc="The fourth and fifth parameters are x,y coordinate of the point where the user"]
    #[doc="released the mouse button."]
    #[doc=""]
    #[doc="if `CallbackReturn::Ignore`  is returned the callback will **not** be called for the"]
    #[doc="next dropped files, and the processing of dropped files will be interrupted."]
    #[doc=""]
    #[doc="\\[Windows and GTK Only\\] (GTK 2.6)"]
    pub trait DropFilesCb where Self: Element {
        let name = "DROPFILES_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, filename: *const c_char,
                           num: c_int, x: c_int, y: c_int) -> CallbackReturn;
        fn set_dropfiles_cb<F: Callback(Self, PathBuf, usize, i32, i32)>(&mut self, cb: F) -> Self;
        fn remove_dropfiles_cb(&mut self) -> Option<Box<_>>;
        // TODO should it be plural (dropfiles_cb) just like IUP or singular (dropfile_cb)?
    }
}

impl_callback! {
    #[doc="Called just before a dialog is closed when the user clicks the close button of the title bar"]
    #[doc="or an equivalent action."]
    #[doc=""]
    #[doc="`CallbackReturn::Close` will be processed. If `CallbackReturn::Ignore`, it prevents the dialog"]
    #[doc="from being closed. If you destroy the dialog in this callback, you must"]
    #[doc="return `CallbackReturn::Ignore`."]
    pub trait CloseCb where Self: Element {
        let name = "CLOSE_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle) -> CallbackReturn;
        fn set_move_cb<F: Callback(Self)>(&mut self, cb: F) -> Self;
        fn remove_move_cb(&mut self) -> Option<Box<_>>;
    }
}

impl_callback! {
    #[doc="Called after the widget is moved, see it's documentation for more details."]
    pub trait MoveCb where Self: Element {
        let name = "MOVE_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, x: c_int, y: c_int) -> CallbackReturn;
        fn set_move_cb<F: Callback(Self, i32, i32)>(&mut self, cb: F) -> Self;
        fn remove_move_cb(&mut self) -> Option<Box<_>>;
    }
}


impl_callback! {
    #[doc="Action generated when the canvas or dialog size is changed."]
    pub trait ResizeCb where Self: Element {
        let name = "RESIZE_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, w: c_int, h: c_int) -> CallbackReturn;
        fn set_move_cb<F: Callback(Self, i32, i32)>(&mut self, cb: F) -> Self;
        fn remove_move_cb(&mut self) -> Option<Box<_>>;
    }
}
