// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskCairoNode")]
    pub struct CairoNode(Shared<ffi::GskCairoNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for CairoNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_cairo_node_get_type()) }
    }
}

impl CairoNode {
    #[doc(alias = "gsk_cairo_node_new")]
    pub fn new(bounds: &graphene::Rect) -> CairoNode {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gsk_cairo_node_new(bounds.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_cairo_node_get_draw_context")]
    #[doc(alias = "get_draw_context")]
    pub fn draw_context(&self) -> cairo::Context {
        unsafe { from_glib_full(ffi::gsk_cairo_node_get_draw_context(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_cairo_node_get_surface")]
    #[doc(alias = "get_surface")]
    pub fn surface(&self) -> cairo::Surface {
        unsafe { from_glib_none(ffi::gsk_cairo_node_get_surface(self.to_glib_none().0)) }
    }
}

impl fmt::Display for CairoNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CairoNode")
    }
}
