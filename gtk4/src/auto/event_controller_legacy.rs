// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct EventControllerLegacy(Object<ffi::GtkEventControllerLegacy, ffi::GtkEventControllerLegacyClass>) @extends EventController;

    match fn {
        get_type => || ffi::gtk_event_controller_legacy_get_type(),
    }
}

impl EventControllerLegacy {
    pub fn new() -> EventControllerLegacy {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_legacy_new()).unsafe_cast()
        }
    }

    pub fn connect_event<F: Fn(&EventControllerLegacy, &gdk::Event) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            F: Fn(&EventControllerLegacy, &gdk::Event) -> bool + 'static,
        >(
            this: *mut ffi::GtkEventControllerLegacy,
            event: *mut gdk::ffi::GdkEvent,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(event)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerLegacy {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EventControllerLegacy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventControllerLegacy")
    }
}
