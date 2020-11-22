// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Rectangle(Boxed<ffi::GdkRectangle>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gdk_rectangle_get_type(), ptr as *mut _) as *mut ffi::GdkRectangle,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gdk_rectangle_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || ffi::gdk_rectangle_get_type(),
    }
}

impl Rectangle {
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        unsafe {
            from_glib(ffi::gdk_rectangle_contains_point(
                self.to_glib_none().0,
                x,
                y,
            ))
        }
    }

    fn equal(&self, rect2: &Rectangle) -> bool {
        unsafe {
            from_glib(ffi::gdk_rectangle_equal(
                self.to_glib_none().0,
                rect2.to_glib_none().0,
            ))
        }
    }

    pub fn intersect(&self, src2: &Rectangle) -> Option<Rectangle> {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            let ret = from_glib(ffi::gdk_rectangle_intersect(
                self.to_glib_none().0,
                src2.to_glib_none().0,
                dest.to_glib_none_mut().0,
            ));
            if ret {
                Some(dest)
            } else {
                None
            }
        }
    }

    pub fn union(&self, src2: &Rectangle) -> Rectangle {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            ffi::gdk_rectangle_union(
                self.to_glib_none().0,
                src2.to_glib_none().0,
                dest.to_glib_none_mut().0,
            );
            dest
        }
    }
}

impl PartialEq for Rectangle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Rectangle {}
