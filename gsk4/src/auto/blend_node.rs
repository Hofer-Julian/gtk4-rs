// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::BlendMode;
use crate::RenderNode;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct BlendNode(Object<ffi::GskBlendNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_blend_node_get_type(),
    }
}

impl BlendNode {
    pub fn new<P: IsA<RenderNode>, Q: IsA<RenderNode>>(
        bottom: &P,
        top: &Q,
        blend_mode: BlendMode,
    ) -> BlendNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_blend_node_new(
                bottom.as_ref().to_glib_none().0,
                top.as_ref().to_glib_none().0,
                blend_mode.to_glib(),
            ))
        }
    }

    pub fn get_blend_mode(&self) -> BlendMode {
        unsafe { from_glib(ffi::gsk_blend_node_get_blend_mode(self.to_glib_none().0)) }
    }

    pub fn get_bottom_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_blend_node_get_bottom_child(self.to_glib_none().0)) }
    }

    pub fn get_top_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_blend_node_get_top_child(self.to_glib_none().0)) }
    }
}

impl fmt::Display for BlendNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BlendNode")
    }
}
