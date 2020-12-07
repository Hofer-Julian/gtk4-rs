// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::GestureDrag;
use crate::GestureSingle;
use crate::Orientation;
use crate::PanDirection;
use crate::PropagationLimit;
use crate::PropagationPhase;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct GesturePan(Object<ffi::GtkGesturePan, ffi::GtkGesturePanClass>) @extends GestureDrag, GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_pan_get_type(),
    }
}

impl GesturePan {
    pub fn new(orientation: Orientation) -> GesturePan {
        assert_initialized_main_thread!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_pan_new(orientation.to_glib())).unsafe_cast()
        }
    }

    pub fn get_orientation(&self) -> Orientation {
        unsafe { from_glib(ffi::gtk_gesture_pan_get_orientation(self.to_glib_none().0)) }
    }

    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_gesture_pan_set_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    pub fn connect_pan<F: Fn(&GesturePan, PanDirection, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pan_trampoline<F: Fn(&GesturePan, PanDirection, f64) + 'static>(
            this: *mut ffi::GtkGesturePan,
            direction: ffi::GtkPanDirection,
            offset: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(direction), offset)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pan\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pan_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_orientation_notify<F: Fn(&GesturePan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_orientation_trampoline<F: Fn(&GesturePan) + 'static>(
            this: *mut ffi::GtkGesturePan,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::orientation\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_orientation_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct GesturePanBuilder {
    orientation: Option<Orientation>,
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl GesturePanBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GesturePan {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        let ret = glib::Object::new(GesturePan::static_type(), &properties)
            .expect("object new")
            .downcast::<GesturePan>()
            .expect("downcast");
        ret
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn propagation_limit(mut self, propagation_limit: PropagationLimit) -> Self {
        self.propagation_limit = Some(propagation_limit);
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }
}

impl fmt::Display for GesturePan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GesturePan")
    }
}
