// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Event;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkProximityEvent")]
    pub struct ProximityEvent(Shared<ffi::GdkProximityEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

impl glib::StaticType for ProximityEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_proximity_event_get_type()) }
    }
}

impl ProximityEvent {}

impl fmt::Display for ProximityEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ProximityEvent")
    }
}
