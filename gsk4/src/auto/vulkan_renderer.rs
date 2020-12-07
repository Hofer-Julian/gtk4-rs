// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Renderer;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct VulkanRenderer(Object<ffi::GskVulkanRenderer, ffi::GskVulkanRendererClass>) @extends Renderer;

    match fn {
        get_type => || ffi::gsk_vulkan_renderer_get_type(),
    }
}

impl VulkanRenderer {
    pub fn new() -> VulkanRenderer {
        assert_initialized_main_thread!();
        unsafe { Renderer::from_glib_full(ffi::gsk_vulkan_renderer_new()).unsafe_cast() }
    }
}

impl Default for VulkanRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for VulkanRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("VulkanRenderer")
    }
}
