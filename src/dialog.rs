use super::BaseWidget;

use cstr_utils::AsCStr;

use ::iup_sys;

use std::convert::AsMut;
use std::ffi::CString;
use std::ptr;

pub struct Dialog(BaseWidget);

impl Dialog {
    pub fn new<W>(contents: W) -> Dialog where W: Into<BaseWidget> {
        let ptr = unsafe { iup_sys::IupDialog(contents.into().0) };
        Dialog(BaseWidget::from_ptr(ptr))
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.set_str_attribute(::attrs::TITLE, title);
        self
    }

    pub fn set_size_pixels(mut self, width: u32, height: u32) -> Self {
        let rastersize = format!("{}x{}", width, height);
        self.set_str_attribute(::attrs::RASTERSIZE, rastersize);
        self
    }
}

impl_base_widget! { Dialog, Dialog, "dialog" }
