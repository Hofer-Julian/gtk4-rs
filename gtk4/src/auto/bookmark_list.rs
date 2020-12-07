// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

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
    pub struct BookmarkList(Object<ffi::GtkBookmarkList, ffi::GtkBookmarkListClass>) @implements gio::ListModel;

    match fn {
        get_type => || ffi::gtk_bookmark_list_get_type(),
    }
}

impl BookmarkList {
    pub fn new(filename: Option<&str>, attributes: Option<&str>) -> BookmarkList {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_bookmark_list_new(
                filename.to_glib_none().0,
                attributes.to_glib_none().0,
            ))
        }
    }

    pub fn get_attributes(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_bookmark_list_get_attributes(self.to_glib_none().0)) }
    }

    pub fn get_filename(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_bookmark_list_get_filename(self.to_glib_none().0)) }
    }

    pub fn get_io_priority(&self) -> i32 {
        unsafe { ffi::gtk_bookmark_list_get_io_priority(self.to_glib_none().0) }
    }

    pub fn is_loading(&self) -> bool {
        unsafe { from_glib(ffi::gtk_bookmark_list_is_loading(self.to_glib_none().0)) }
    }

    pub fn set_attributes(&self, attributes: Option<&str>) {
        unsafe {
            ffi::gtk_bookmark_list_set_attributes(
                self.to_glib_none().0,
                attributes.to_glib_none().0,
            );
        }
    }

    pub fn set_io_priority(&self, io_priority: i32) {
        unsafe {
            ffi::gtk_bookmark_list_set_io_priority(self.to_glib_none().0, io_priority);
        }
    }

    pub fn get_property_loading(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"loading\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `loading` getter")
                .unwrap()
        }
    }

    pub fn connect_property_attributes_notify<F: Fn(&BookmarkList) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_attributes_trampoline<F: Fn(&BookmarkList) + 'static>(
            this: *mut ffi::GtkBookmarkList,
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
                b"notify::attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_attributes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_io_priority_notify<F: Fn(&BookmarkList) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_io_priority_trampoline<F: Fn(&BookmarkList) + 'static>(
            this: *mut ffi::GtkBookmarkList,
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
                b"notify::io-priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_io_priority_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_loading_notify<F: Fn(&BookmarkList) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_loading_trampoline<F: Fn(&BookmarkList) + 'static>(
            this: *mut ffi::GtkBookmarkList,
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
                b"notify::loading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_loading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct BookmarkListBuilder {
    attributes: Option<String>,
    filename: Option<String>,
    io_priority: Option<i32>,
}

impl BookmarkListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> BookmarkList {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref attributes) = self.attributes {
            properties.push(("attributes", attributes));
        }
        if let Some(ref filename) = self.filename {
            properties.push(("filename", filename));
        }
        if let Some(ref io_priority) = self.io_priority {
            properties.push(("io-priority", io_priority));
        }
        let ret = glib::Object::new(BookmarkList::static_type(), &properties)
            .expect("object new")
            .downcast::<BookmarkList>()
            .expect("downcast");
        ret
    }

    pub fn attributes(mut self, attributes: &str) -> Self {
        self.attributes = Some(attributes.to_string());
        self
    }

    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_string());
        self
    }

    pub fn io_priority(mut self, io_priority: i32) -> Self {
        self.io_priority = Some(io_priority);
        self
    }
}

impl fmt::Display for BookmarkList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BookmarkList")
    }
}
