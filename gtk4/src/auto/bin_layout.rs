// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::LayoutManager;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct BinLayout(Object<ffi::GtkBinLayout, ffi::GtkBinLayoutClass>) @extends LayoutManager;

    match fn {
        get_type => || ffi::gtk_bin_layout_get_type(),
    }
}

impl BinLayout {
    pub fn new() -> BinLayout {
        assert_initialized_main_thread!();
        unsafe { LayoutManager::from_glib_full(ffi::gtk_bin_layout_new()).unsafe_cast() }
    }
}

impl Default for BinLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BinLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BinLayout")
    }
}
