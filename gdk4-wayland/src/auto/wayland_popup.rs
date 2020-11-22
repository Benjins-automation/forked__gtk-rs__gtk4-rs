// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::WaylandSurface;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct WaylandPopup(Object<ffi::GdkWaylandPopup>) @extends WaylandSurface, gdk::Surface, @implements gdk::Popup;

    match fn {
        get_type => || ffi::gdk_wayland_popup_get_type(),
    }
}

impl WaylandPopup {}

impl fmt::Display for WaylandPopup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WaylandPopup")
    }
}
