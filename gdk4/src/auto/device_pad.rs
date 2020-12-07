// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Device;
use crate::DevicePadFeature;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct DevicePad(Interface<ffi::GdkDevicePad>) @requires Device;

    match fn {
        get_type => || ffi::gdk_device_pad_get_type(),
    }
}

pub const NONE_DEVICE_PAD: Option<&DevicePad> = None;

pub trait DevicePadExt: 'static {
    fn get_feature_group(&self, feature: DevicePadFeature, feature_idx: i32) -> i32;

    fn get_group_n_modes(&self, group_idx: i32) -> i32;

    fn get_n_features(&self, feature: DevicePadFeature) -> i32;

    fn get_n_groups(&self) -> i32;
}

impl<O: IsA<DevicePad>> DevicePadExt for O {
    fn get_feature_group(&self, feature: DevicePadFeature, feature_idx: i32) -> i32 {
        unsafe {
            ffi::gdk_device_pad_get_feature_group(
                self.as_ref().to_glib_none().0,
                feature.to_glib(),
                feature_idx,
            )
        }
    }

    fn get_group_n_modes(&self, group_idx: i32) -> i32 {
        unsafe { ffi::gdk_device_pad_get_group_n_modes(self.as_ref().to_glib_none().0, group_idx) }
    }

    fn get_n_features(&self, feature: DevicePadFeature) -> i32 {
        unsafe {
            ffi::gdk_device_pad_get_n_features(self.as_ref().to_glib_none().0, feature.to_glib())
        }
    }

    fn get_n_groups(&self) -> i32 {
        unsafe { ffi::gdk_device_pad_get_n_groups(self.as_ref().to_glib_none().0) }
    }
}

impl fmt::Display for DevicePad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DevicePad")
    }
}
