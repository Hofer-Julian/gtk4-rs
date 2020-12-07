// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::IMContext;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct IMContextSimple(Object<ffi::GtkIMContextSimple, ffi::GtkIMContextSimpleClass>) @extends IMContext;

    match fn {
        get_type => || ffi::gtk_im_context_simple_get_type(),
    }
}

impl IMContextSimple {
    pub fn new() -> IMContextSimple {
        assert_initialized_main_thread!();
        unsafe { IMContext::from_glib_full(ffi::gtk_im_context_simple_new()).unsafe_cast() }
    }
}

impl Default for IMContextSimple {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_IM_CONTEXT_SIMPLE: Option<&IMContextSimple> = None;

impl fmt::Display for IMContextSimple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IMContextSimple")
    }
}
