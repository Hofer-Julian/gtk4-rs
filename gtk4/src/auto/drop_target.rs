// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
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
    pub struct DropTarget(Object<ffi::GtkDropTarget, ffi::GtkDropTargetClass>) @extends EventController;

    match fn {
        get_type => || ffi::gtk_drop_target_get_type(),
    }
}

impl DropTarget {
    pub fn new(type_: glib::types::Type, actions: gdk::DragAction) -> DropTarget {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_drop_target_new(type_.to_glib(), actions.to_glib())) }
    }

    pub fn get_actions(&self) -> gdk::DragAction {
        unsafe { from_glib(ffi::gtk_drop_target_get_actions(self.to_glib_none().0)) }
    }

    pub fn get_drop(&self) -> Option<gdk::Drop> {
        unsafe { from_glib_none(ffi::gtk_drop_target_get_drop(self.to_glib_none().0)) }
    }

    pub fn get_formats(&self) -> Option<gdk::ContentFormats> {
        unsafe { from_glib_full(ffi::gtk_drop_target_get_formats(self.to_glib_none().0)) }
    }

    //pub fn get_gtypes(&self) -> /*Unimplemented*/Option<CArray TypeId { ns_id: 0, id: 30 }> {
    //    unsafe { TODO: call ffi:gtk_drop_target_get_gtypes() }
    //}

    pub fn get_preload(&self) -> bool {
        unsafe { from_glib(ffi::gtk_drop_target_get_preload(self.to_glib_none().0)) }
    }

    pub fn get_value(&self) -> Option<glib::Value> {
        unsafe { from_glib_none(ffi::gtk_drop_target_get_value(self.to_glib_none().0)) }
    }

    pub fn reject(&self) {
        unsafe {
            ffi::gtk_drop_target_reject(self.to_glib_none().0);
        }
    }

    pub fn set_actions(&self, actions: gdk::DragAction) {
        unsafe {
            ffi::gtk_drop_target_set_actions(self.to_glib_none().0, actions.to_glib());
        }
    }

    //pub fn set_gtypes(&self, types: /*Unimplemented*/Option<&CArray TypeId { ns_id: 0, id: 30 }>) {
    //    unsafe { TODO: call ffi:gtk_drop_target_set_gtypes() }
    //}

    pub fn set_preload(&self, preload: bool) {
        unsafe {
            ffi::gtk_drop_target_set_preload(self.to_glib_none().0, preload.to_glib());
        }
    }

    pub fn connect_accept<F: Fn(&DropTarget, &gdk::Drop) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_trampoline<F: Fn(&DropTarget, &gdk::Drop) -> bool + 'static>(
            this: *mut ffi::GtkDropTarget,
            drop: *mut gdk::ffi::GdkDrop,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_enter<F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<
            F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut ffi::GtkDropTarget,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> gdk::ffi::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    enter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_leave<F: Fn(&DropTarget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"leave\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    leave_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_motion<F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<
            F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut ffi::GtkDropTarget,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> gdk::ffi::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"motion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_actions_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_actions_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
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
                b"notify::actions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_actions_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_drop_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_drop_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
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
                b"notify::drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_formats_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
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
                b"notify::formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_formats_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_preload_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_preload_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
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
                b"notify::preload\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_preload_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_value_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut ffi::GtkDropTarget,
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
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct DropTargetBuilder {
    actions: Option<gdk::DragAction>,
    preload: Option<bool>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl DropTargetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DropTarget {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref actions) = self.actions {
            properties.push(("actions", actions));
        }
        if let Some(ref preload) = self.preload {
            properties.push(("preload", preload));
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
        let ret = glib::Object::new(DropTarget::static_type(), &properties)
            .expect("object new")
            .downcast::<DropTarget>()
            .expect("downcast");
        ret
    }

    pub fn actions(mut self, actions: gdk::DragAction) -> Self {
        self.actions = Some(actions);
        self
    }

    pub fn preload(mut self, preload: bool) -> Self {
        self.preload = Some(preload);
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

impl fmt::Display for DropTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DropTarget")
    }
}
