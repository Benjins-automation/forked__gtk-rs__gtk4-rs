// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CrossingMode;
use crate::Event;
use crate::NotifyType;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct CrossingEvent(Object<ffi::GdkCrossingEvent>) @extends Event;

    match fn {
        get_type => || ffi::gdk_crossing_event_get_type(),
    }
}

impl CrossingEvent {
    pub fn get_detail(&self) -> NotifyType {
        unsafe { from_glib(ffi::gdk_crossing_event_get_detail(self.to_glib_none().0)) }
    }

    pub fn get_focus(&self) -> bool {
        unsafe { from_glib(ffi::gdk_crossing_event_get_focus(self.to_glib_none().0)) }
    }

    pub fn get_mode(&self) -> CrossingMode {
        unsafe { from_glib(ffi::gdk_crossing_event_get_mode(self.to_glib_none().0)) }
    }
}

impl fmt::Display for CrossingEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CrossingEvent")
    }
}
