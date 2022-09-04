// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Native;
use crate::Root;
use crate::Widget;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkDragIcon")]
    pub struct DragIcon(Object<ffi::GtkDragIcon, ffi::GtkDragIconClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Native, Root;

    match fn {
        type_ => || ffi::gtk_drag_icon_get_type(),
    }
}

impl DragIcon {
    #[doc(alias = "gtk_drag_icon_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_drag_icon_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drag_icon_set_child")]
    pub fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_drag_icon_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_drag_icon_create_widget_for_value")]
    pub fn create_widget_for_value(value: &glib::Value) -> Option<Widget> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_drag_icon_create_widget_for_value(
                value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_drag_icon_get_for_drag")]
    #[doc(alias = "get_for_drag")]
    pub fn for_drag(drag: &gdk::Drag) -> Widget {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_drag_icon_get_for_drag(drag.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drag_icon_set_from_paintable")]
    pub fn set_from_paintable(
        drag: &gdk::Drag,
        paintable: &impl IsA<gdk::Paintable>,
        hot_x: i32,
        hot_y: i32,
    ) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_drag_icon_set_from_paintable(
                drag.to_glib_none().0,
                paintable.as_ref().to_glib_none().0,
                hot_x,
                hot_y,
            );
        }
    }

    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&DragIcon) + 'static>(
            this: *mut ffi::GtkDragIcon,
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
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DragIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DragIcon")
    }
}
