use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use crate::win32::window_helper as wh;
use crate::win32::resources_helper as rh;
use super::{ControlBase, ControlHandle};
use crate::{Bitmap, Icon, NwgError, RawEventHandler, unbind_raw_event_handler};
use std::cell::RefCell;

const NOT_BOUND: &'static str = "ImageFrame is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: ImageFrame handle is not HWND!";


bitflags! {
    pub struct ImageFrameFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}

/**
An image frame is a control that displays a `Bitmap` or a `Icon` image resource. It can also triggers mouse clicks.

**Builder parameters:**
  * `parent`:           **Required.** The image frame parent container.
  * `size`:             The image frame size.
  * `position`:         The image frame position.
  * `flags`:            A combination of the ImageFrameFlags values.
  * `background_color`: The background color of the image frame. Used if the image is smaller than the control
  * `bitmap`:           A bitmap to display. If this value is set, icon is ignored.
  * `icon`:             An icon to display

**Control events:**
  * `OnImageFrameClick`: When the image frame is clicked once by the user
  * `OnImageFrameDoubleClick`: When the image frame is clicked twice rapidly by the user
  * `MousePress(_)`: Generic mouse press events on the button
  * `OnMouseMove`: Generic mouse mouse event

```rust
use native_windows_gui as nwg;
fn build_frame(button: &mut nwg::ImageFrame, window: &nwg::Window, ico: &nwg::Icon) {
    nwg::ImageFrame::builder()
        .parent(window)
        .build(button);
}
```
*/
#[derive(Default)]
pub struct ImageFrame {
    pub handle: ControlHandle,
    handler0: RefCell<Option<RawEventHandler>>,
}

impl ImageFrame {

    pub fn builder<'a>() -> ImageFrameBuilder<'a> {
        ImageFrameBuilder {
            size: (100, 100),
            position: (0, 0),
            flags: None,
            bitmap: None,
            icon: None,
            parent: None,
            background_color: None
        }
    }

    /// Sets the bitmap image of the image frame. Replace the current bitmap or icon.
    /// Set `image` to `None` to remove the image
    pub fn set_bitmap<'a>(&self, image: Option<&'a Bitmap>) {
        use winapi::um::winuser::{STM_SETIMAGE, IMAGE_BITMAP};
        use winapi::shared::minwindef::{WPARAM, LPARAM};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let image_handle = image.map(|i| i.handle as LPARAM).unwrap_or(0);
        wh::send_message(handle, STM_SETIMAGE, IMAGE_BITMAP as WPARAM, image_handle);
    }

    /// Sets the bitmap image of the image frame. Replace the current bitmap or icon.
    /// Set `image` to `None` to remove the image
    pub fn set_icon<'a>(&self, image: Option<&'a Icon>) {
        use winapi::um::winuser::{STM_SETIMAGE, IMAGE_ICON};
        use winapi::shared::minwindef::{WPARAM, LPARAM};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let image_handle = image.map(|i| i.handle as LPARAM).unwrap_or(0);
        wh::send_message(handle, STM_SETIMAGE, IMAGE_ICON as WPARAM, image_handle);
    }

    /// Returns the current image in the image frame.
    /// If the image frame has a bitmap, the value will be returned in `bitmap`
    /// If the image frame has a icon, the value will be returned in `icon`
    pub fn image<'a>(&self, bitmap: &mut Option<Bitmap>, icon: &mut Option<Icon>) {
        use winapi::um::winuser::{STM_GETIMAGE, IMAGE_BITMAP, IMAGE_ICON};
        use winapi::shared::minwindef::WPARAM;
        use winapi::shared::windef::HBITMAP;
        use winapi::um::winnt::HANDLE;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let bitmap_handle = wh::send_message(handle, STM_GETIMAGE, IMAGE_BITMAP as WPARAM, 0);
        let icon_handle = wh::send_message(handle, STM_GETIMAGE, IMAGE_ICON as WPARAM, 0);

        *bitmap = None;
        *icon = None;

        if bitmap_handle != 0 && rh::is_bitmap(bitmap_handle as HBITMAP) {
            *bitmap = Some(Bitmap { handle: bitmap_handle as HANDLE, owned: false });
        } else if icon_handle != 0 {
            *icon = Some(Icon { handle: icon_handle as HANDLE, owned: false });
        }
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

    /// Return the size of the image frame in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the image frame in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the image frame in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the image frame in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "STATIC"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{SS_NOTIFY, WS_CHILD, SS_CENTERIMAGE};

        WS_CHILD | SS_NOTIFY | SS_CENTERIMAGE
    }

    /// Change the label background color to transparent.
    /// Change the checkbox background color.
    fn hook_background_color(&self, c: [u8; 3]) {
        use crate::bind_raw_event_handler;
        use winapi::um::winuser::{WM_CTLCOLORSTATIC};
        use winapi::shared::{basetsd::UINT_PTR, windef::{HWND}, minwindef::LRESULT};
        use winapi::um::wingdi::{CreateSolidBrush, RGB};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let parent_handle = ControlHandle::Hwnd(wh::get_window_parent(handle));
        let brush = unsafe { CreateSolidBrush(RGB(c[0], c[1], c[2])) };
        
        let handler = bind_raw_event_handler(&parent_handle, handle as UINT_PTR, move |_hwnd, msg, _w, l| {
            match msg {
                WM_CTLCOLORSTATIC => {
                    let child = l as HWND;
                    if child == handle {
                        return Some(brush as LRESULT);
                    }
                },
                _ => {}
            }

            None
        });

        *self.handler0.borrow_mut() = Some(handler);
    }

}

impl Drop for ImageFrame {
    fn drop(&mut self) {
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            unbind_raw_event_handler(h);
        }
    }
}

pub struct ImageFrameBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<ImageFrameFlags>,
    bitmap: Option<&'a Bitmap>,
    icon: Option<&'a Icon>,
    parent: Option<ControlHandle>,
    background_color: Option<[u8; 3]>,
}

impl<'a> ImageFrameBuilder<'a> {

    pub fn flags(mut self, flags: ImageFrameFlags) -> ImageFrameBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> ImageFrameBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> ImageFrameBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn bitmap(mut self, bit: Option<&'a Bitmap>) -> ImageFrameBuilder<'a> {
        self.bitmap = bit;
        self
    }

    pub fn icon(mut self, ico: Option<&'a Icon>) -> ImageFrameBuilder<'a> {
        self.icon = ico;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ImageFrameBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> ImageFrameBuilder<'a> {
        self.background_color = color;
        self
    }

    pub fn build(self, out: &mut ImageFrame) -> Result<(), NwgError> {
        use winapi::um::winuser::{SS_BITMAP, SS_ICON};

        let mut flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());
        if self.icon.is_some() {
            flags |= SS_ICON;
        } else {
            flags |= SS_BITMAP;
        }
        
        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("ImageFrame"))
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        if self.bitmap.is_some() {
            out.set_bitmap(self.bitmap);
        } else if self.icon.is_some() {
            out.set_icon(self.icon);
        }

        if self.background_color.is_some() {
            out.hook_background_color(self.background_color.unwrap());
        }

        Ok(())
    }

}
