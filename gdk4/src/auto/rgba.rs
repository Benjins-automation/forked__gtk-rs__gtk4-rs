// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;
use std::hash;

glib::wrapper! {
    pub struct RGBA(BoxedInline<ffi::GdkRGBA>);

    match fn {
        copy => |ptr| ffi::gdk_rgba_copy(ptr),
        free => |ptr| ffi::gdk_rgba_free(ptr),
        type_ => || ffi::gdk_rgba_get_type(),
    }
}

impl RGBA {
    #[doc(alias = "gdk_rgba_equal")]
    fn equal(&self, p2: &RGBA) -> bool {
        unsafe {
            from_glib(ffi::gdk_rgba_equal(
                ToGlibPtr::<*const ffi::GdkRGBA>::to_glib_none(self).0 as glib::ffi::gconstpointer,
                ToGlibPtr::<*const ffi::GdkRGBA>::to_glib_none(p2).0 as glib::ffi::gconstpointer,
            ))
        }
    }

    #[doc(alias = "gdk_rgba_hash")]
    fn hash(&self) -> u32 {
        unsafe {
            ffi::gdk_rgba_hash(
                ToGlibPtr::<*const ffi::GdkRGBA>::to_glib_none(self).0 as glib::ffi::gconstpointer,
            )
        }
    }

    #[doc(alias = "gdk_rgba_is_clear")]
    pub fn is_clear(&self) -> bool {
        unsafe { from_glib(ffi::gdk_rgba_is_clear(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_rgba_is_opaque")]
    pub fn is_opaque(&self) -> bool {
        unsafe { from_glib(ffi::gdk_rgba_is_opaque(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_rgba_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gdk_rgba_to_string(self.to_glib_none().0)) }
    }
}

impl PartialEq for RGBA {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for RGBA {}

impl fmt::Display for RGBA {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

impl hash::Hash for RGBA {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash(&self.hash(), state)
    }
}
