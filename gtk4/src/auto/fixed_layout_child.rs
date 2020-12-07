// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::LayoutChild;
use crate::LayoutManager;
use crate::Widget;
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
    pub struct FixedLayoutChild(Object<ffi::GtkFixedLayoutChild, ffi::GtkFixedLayoutChildClass>) @extends LayoutChild;

    match fn {
        get_type => || ffi::gtk_fixed_layout_child_get_type(),
    }
}

impl FixedLayoutChild {
    pub fn get_transform(&self) -> Option<gsk::Transform> {
        unsafe {
            from_glib_none(ffi::gtk_fixed_layout_child_get_transform(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_transform(&self, transform: &gsk::Transform) {
        unsafe {
            ffi::gtk_fixed_layout_child_set_transform(
                self.to_glib_none().0,
                transform.to_glib_none().0,
            );
        }
    }

    pub fn connect_property_transform_notify<F: Fn(&FixedLayoutChild) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transform_trampoline<F: Fn(&FixedLayoutChild) + 'static>(
            this: *mut ffi::GtkFixedLayoutChild,
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
                b"notify::transform\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transform_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct FixedLayoutChildBuilder {
    transform: Option<gsk::Transform>,
    child_widget: Option<Widget>,
    layout_manager: Option<LayoutManager>,
}

impl FixedLayoutChildBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> FixedLayoutChild {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref transform) = self.transform {
            properties.push(("transform", transform));
        }
        if let Some(ref child_widget) = self.child_widget {
            properties.push(("child-widget", child_widget));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        let ret = glib::Object::new(FixedLayoutChild::static_type(), &properties)
            .expect("object new")
            .downcast::<FixedLayoutChild>()
            .expect("downcast");
        ret
    }

    pub fn transform(mut self, transform: &gsk::Transform) -> Self {
        self.transform = Some(transform.clone());
        self
    }

    pub fn child_widget<P: IsA<Widget>>(mut self, child_widget: &P) -> Self {
        self.child_widget = Some(child_widget.clone().upcast());
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }
}

impl fmt::Display for FixedLayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FixedLayoutChild")
    }
}
