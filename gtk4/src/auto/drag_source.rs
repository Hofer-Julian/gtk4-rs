// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::GestureSingle;
use crate::PropagationLimit;
use crate::PropagationPhase;
use glib::object::Cast;
use glib::object::IsA;
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
    pub struct DragSource(Object<ffi::GtkDragSource, ffi::GtkDragSourceClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_drag_source_get_type(),
    }
}

impl DragSource {
    pub fn new() -> DragSource {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_drag_source_new()) }
    }

    pub fn drag_cancel(&self) {
        unsafe {
            ffi::gtk_drag_source_drag_cancel(self.to_glib_none().0);
        }
    }

    pub fn get_actions(&self) -> gdk::DragAction {
        unsafe { from_glib(ffi::gtk_drag_source_get_actions(self.to_glib_none().0)) }
    }

    pub fn get_content(&self) -> Option<gdk::ContentProvider> {
        unsafe { from_glib_none(ffi::gtk_drag_source_get_content(self.to_glib_none().0)) }
    }

    pub fn get_drag(&self) -> Option<gdk::Drag> {
        unsafe { from_glib_none(ffi::gtk_drag_source_get_drag(self.to_glib_none().0)) }
    }

    pub fn set_actions(&self, actions: gdk::DragAction) {
        unsafe {
            ffi::gtk_drag_source_set_actions(self.to_glib_none().0, actions.to_glib());
        }
    }

    pub fn set_content<P: IsA<gdk::ContentProvider>>(&self, content: Option<&P>) {
        unsafe {
            ffi::gtk_drag_source_set_content(
                self.to_glib_none().0,
                content.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_icon<P: IsA<gdk::Paintable>>(&self, paintable: Option<&P>, hot_x: i32, hot_y: i32) {
        unsafe {
            ffi::gtk_drag_source_set_icon(
                self.to_glib_none().0,
                paintable.map(|p| p.as_ref()).to_glib_none().0,
                hot_x,
                hot_y,
            );
        }
    }

    pub fn connect_drag_begin<F: Fn(&DragSource, &gdk::Drag) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_begin_trampoline<F: Fn(&DragSource, &gdk::Drag) + 'static>(
            this: *mut ffi::GtkDragSource,
            drag: *mut gdk::ffi::GdkDrag,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drag))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_begin_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //pub fn connect_drag_cancel<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored reason: Gdk.DragCancelReason
    //}

    pub fn connect_drag_end<F: Fn(&DragSource, &gdk::Drag, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_end_trampoline<F: Fn(&DragSource, &gdk::Drag, bool) + 'static>(
            this: *mut ffi::GtkDragSource,
            drag: *mut gdk::ffi::GdkDrag,
            delete_data: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(drag),
                from_glib(delete_data),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_end_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_prepare<
        F: Fn(&DragSource, f64, f64) -> Option<gdk::ContentProvider> + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn prepare_trampoline<
            F: Fn(&DragSource, f64, f64) -> Option<gdk::ContentProvider> + 'static,
        >(
            this: *mut ffi::GtkDragSource,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> *mut gdk::ffi::GdkContentProvider {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prepare\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prepare_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_actions_notify<F: Fn(&DragSource) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_actions_trampoline<F: Fn(&DragSource) + 'static>(
            this: *mut ffi::GtkDragSource,
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

    pub fn connect_property_content_notify<F: Fn(&DragSource) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<F: Fn(&DragSource) + 'static>(
            this: *mut ffi::GtkDragSource,
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
                b"notify::content\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for DragSource {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct DragSourceBuilder {
    actions: Option<gdk::DragAction>,
    content: Option<gdk::ContentProvider>,
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl DragSourceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DragSource {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref actions) = self.actions {
            properties.push(("actions", actions));
        }
        if let Some(ref content) = self.content {
            properties.push(("content", content));
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
        let ret = glib::Object::new(DragSource::static_type(), &properties)
            .expect("object new")
            .downcast::<DragSource>()
            .expect("downcast");
        ret
    }

    pub fn actions(mut self, actions: gdk::DragAction) -> Self {
        self.actions = Some(actions);
        self
    }

    pub fn content<P: IsA<gdk::ContentProvider>>(mut self, content: &P) -> Self {
        self.content = Some(content.clone().upcast());
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

impl fmt::Display for DragSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DragSource")
    }
}
