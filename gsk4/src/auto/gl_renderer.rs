// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Renderer;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct GLRenderer(Object<ffi::GskGLRenderer, ffi::GskGLRendererClass>) @extends Renderer;

    match fn {
        get_type => || ffi::gsk_gl_renderer_get_type(),
    }
}

impl GLRenderer {
    pub fn new() -> GLRenderer {
        assert_initialized_main_thread!();
        unsafe { Renderer::from_glib_full(ffi::gsk_gl_renderer_new()).unsafe_cast() }
    }
}

impl Default for GLRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GLRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLRenderer")
    }
}
