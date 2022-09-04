// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkX11Surface")]
    pub struct X11Surface(Object<ffi::GdkX11Surface, ffi::GdkX11SurfaceClass>) @extends gdk::Surface;

    match fn {
        type_ => || ffi::gdk_x11_surface_get_type(),
    }
}

impl X11Surface {
    #[doc(alias = "gdk_x11_surface_get_desktop")]
    #[doc(alias = "get_desktop")]
    pub fn desktop(&self) -> u32 {
        unsafe { ffi::gdk_x11_surface_get_desktop(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_x11_surface_get_group")]
    #[doc(alias = "get_group")]
    pub fn group(&self) -> Option<gdk::Surface> {
        unsafe { from_glib_none(ffi::gdk_x11_surface_get_group(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_x11_surface_move_to_current_desktop")]
    pub fn move_to_current_desktop(&self) {
        unsafe {
            ffi::gdk_x11_surface_move_to_current_desktop(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_x11_surface_move_to_desktop")]
    pub fn move_to_desktop(&self, desktop: u32) {
        unsafe {
            ffi::gdk_x11_surface_move_to_desktop(self.to_glib_none().0, desktop);
        }
    }

    #[doc(alias = "gdk_x11_surface_set_frame_sync_enabled")]
    pub fn set_frame_sync_enabled(&self, frame_sync_enabled: bool) {
        unsafe {
            ffi::gdk_x11_surface_set_frame_sync_enabled(
                self.to_glib_none().0,
                frame_sync_enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "gdk_x11_surface_set_group")]
    pub fn set_group(&self, leader: &impl IsA<gdk::Surface>) {
        unsafe {
            ffi::gdk_x11_surface_set_group(self.to_glib_none().0, leader.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_x11_surface_set_skip_pager_hint")]
    pub fn set_skip_pager_hint(&self, skips_pager: bool) {
        unsafe {
            ffi::gdk_x11_surface_set_skip_pager_hint(
                self.to_glib_none().0,
                skips_pager.into_glib(),
            );
        }
    }

    #[doc(alias = "gdk_x11_surface_set_skip_taskbar_hint")]
    pub fn set_skip_taskbar_hint(&self, skips_taskbar: bool) {
        unsafe {
            ffi::gdk_x11_surface_set_skip_taskbar_hint(
                self.to_glib_none().0,
                skips_taskbar.into_glib(),
            );
        }
    }

    #[doc(alias = "gdk_x11_surface_set_theme_variant")]
    pub fn set_theme_variant(&self, variant: &str) {
        unsafe {
            ffi::gdk_x11_surface_set_theme_variant(self.to_glib_none().0, variant.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_x11_surface_set_urgency_hint")]
    pub fn set_urgency_hint(&self, urgent: bool) {
        unsafe {
            ffi::gdk_x11_surface_set_urgency_hint(self.to_glib_none().0, urgent.into_glib());
        }
    }

    #[doc(alias = "gdk_x11_surface_set_user_time")]
    pub fn set_user_time(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_x11_surface_set_user_time(self.to_glib_none().0, timestamp);
        }
    }

    #[doc(alias = "gdk_x11_surface_set_utf8_property")]
    pub fn set_utf8_property(&self, name: &str, value: Option<&str>) {
        unsafe {
            ffi::gdk_x11_surface_set_utf8_property(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for X11Surface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11Surface")
    }
}
