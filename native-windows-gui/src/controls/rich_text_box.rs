use winapi::shared::minwindef::{WPARAM, LPARAM};
use winapi::um::winuser::{ES_AUTOVSCROLL, ES_AUTOHSCROLL, WS_VISIBLE, WS_DISABLED};
use crate::win32::window_helper as wh;
use crate::{Font, NwgError};
use super::{ControlBase, ControlHandle};
use std::ops::Range;

const NOT_BOUND: &'static str = "RichTextBox is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: RichTextBox handle is not HWND!";


bitflags! {
    pub struct RichTextBoxFlags: u32 {
        const VSCROLL = ES_AUTOVSCROLL;
        const HSCROLL = ES_AUTOHSCROLL;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}


/**
An edit control is a rectangular control window to permit the user to enter and edit text by typing on the keyboard
This control allow multi line input. For a single line of text, use `TextInput`.

A rich text box is almost a superset of the normal textbox. Unlike text box, rich text box has more features and can support Component Object Model (COM) objects.

See: https://docs.microsoft.com/en-us/windows/win32/controls/about-rich-edit-controls#rich-edit-version-41

Note: Use `\r\n` to input a new line not just `\n`.
*/
#[derive(Default, PartialEq, Eq)]
pub struct RichTextBox {
    pub handle: ControlHandle
}

impl RichTextBox {

    pub fn builder<'a>() -> RichTextBoxBuilder<'a> {
        RichTextBoxBuilder {
            text: "",
            size: (100, 25),
            position: (0, 0),
            flags: None,
            limit: 0,
            readonly: false,
            font: None,
            parent: None
        }
    }

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return the number of maximum character allowed in this text input
    pub fn limit(&self) -> u32 {
        use winapi::um::winuser::EM_GETLIMITTEXT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_GETLIMITTEXT as u32, 0, 0) as u32
    }

    /// Set the number of maximum character allowed in this text input
    pub fn set_limit(&self, limit: usize) {
        use winapi::um::winuser::EM_SETLIMITTEXT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_SETLIMITTEXT as u32, limit, 0);
    }

    /// Check if the content of the text input was modified after it's creation
    pub fn modified(&self) -> bool {
        use winapi::um::winuser::EM_GETMODIFY;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_GETMODIFY as u32, 0, 0) != 0
    }

    /// Manually set modified flag of the text input
    pub fn set_modified(&self, e: bool) {
        use winapi::um::winuser::EM_SETMODIFY;
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_SETMODIFY as u32, e as usize, 0);
    }

    /// Undo the last action by the user in the control
    pub fn undo(&self) {
        use winapi::um::winuser::EM_UNDO;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_UNDO as u32, 0, 0);
    }

    /// Return the selected range of characters by the user in the text input
    pub fn selection(&self) -> Range<u32> {
        use winapi::um::winuser::EM_GETSEL;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let (mut out1, mut out2) = (0u32, 0u32);
        let (ptr1, ptr2) = (&mut out1 as *mut u32, &mut out2 as *mut u32);
        wh::send_message(handle, EM_GETSEL as u32, ptr1 as WPARAM, ptr2 as LPARAM);

        Range { start: out1 as u32, end: out2 as u32 }
    }

    /// Return the selected range of characters by the user in the text input
    pub fn set_selection(&self, r: Range<u32>) {
        use winapi::um::winuser::EM_SETSEL;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        wh::send_message(handle, EM_SETSEL as u32, r.start as usize, r.end as isize);
    }

    /// Return the length of the user input in the control. This is better than test.len() as it
    /// does not allocate a string in memory
    pub fn len(&self) -> u32 {
        use winapi::um::winuser::EM_LINELENGTH;
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_LINELENGTH as u32, 0, 0) as u32
    }

    /// Return true if the TextInput value cannot be edited. Retrurn false otherwise.
    /// A user can still copy text from a readonly TextEdit (unlike disabled)
    pub fn readonly(&self) -> bool {
        use winapi::um::winuser::ES_READONLY;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        wh::get_style(handle) & ES_READONLY == ES_READONLY
    }

    /// Set the readonly flag of the text input
    /// A user can still copy text from a readonly TextEdit (unlike disabled)
    pub fn set_readonly(&self, r: bool) {
        use winapi::um::winuser::EM_SETREADONLY;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        wh::send_message(handle, EM_SETREADONLY as u32, r as WPARAM, 0);
    }

    /// Remove all text from the textbox
    pub fn clear(&self) {
        self.set_text("");
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the text displayed in the TextInput
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the text displayed in the TextInput
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "RICHEDIT50W"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | ES_AUTOVSCROLL | ES_AUTOHSCROLL
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_BORDER, WS_CHILD, ES_MULTILINE};
        
        WS_BORDER | WS_CHILD | ES_MULTILINE
    }

}


pub struct RichTextBoxBuilder<'a> {
    text: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<RichTextBoxFlags>,
    limit: usize,
    readonly: bool,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>
}

impl<'a> RichTextBoxBuilder<'a> {

    pub fn flags(mut self, flags: RichTextBoxFlags) -> RichTextBoxBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn text(mut self, text: &'a str) -> RichTextBoxBuilder<'a> {
        self.text = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> RichTextBoxBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> RichTextBoxBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn limit(mut self, limit: usize) -> RichTextBoxBuilder<'a> {
        self.limit = limit;
        self
    }

    pub fn readonly(mut self, read: bool) -> RichTextBoxBuilder<'a> {
        self.readonly = read;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> RichTextBoxBuilder<'a> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> RichTextBoxBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut RichTextBox) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("RichTextBox"))
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .text(self.text)
            .parent(Some(parent))
            .build()?;

        if self.limit > 0 {
            out.set_limit(self.limit);
        }

        if self.readonly {
            out.set_readonly(self.readonly);
        }

        if self.font.is_some() {
            out.set_font(self.font);
        }

        Ok(())
    }

}