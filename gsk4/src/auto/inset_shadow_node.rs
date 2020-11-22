// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use crate::RoundedRect;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct InsetShadowNode(Object<ffi::GskInsetShadowNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_inset_shadow_node_get_type(),
    }
}

impl InsetShadowNode {
    pub fn new(
        outline: &RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) -> InsetShadowNode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_inset_shadow_node_new(
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            ))
        }
    }

    pub fn get_blur_radius(&self) -> f32 {
        unsafe { ffi::gsk_inset_shadow_node_get_blur_radius(self.to_glib_none().0) }
    }

    pub fn get_dx(&self) -> f32 {
        unsafe { ffi::gsk_inset_shadow_node_get_dx(self.to_glib_none().0) }
    }

    pub fn get_dy(&self) -> f32 {
        unsafe { ffi::gsk_inset_shadow_node_get_dy(self.to_glib_none().0) }
    }

    pub fn get_spread(&self) -> f32 {
        unsafe { ffi::gsk_inset_shadow_node_get_spread(self.to_glib_none().0) }
    }

    pub fn peek_color(&self) -> Option<gdk::RGBA> {
        unsafe { from_glib_none(ffi::gsk_inset_shadow_node_peek_color(self.to_glib_none().0)) }
    }

    pub fn peek_outline(&self) -> Option<RoundedRect> {
        unsafe {
            from_glib_none(ffi::gsk_inset_shadow_node_peek_outline(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for InsetShadowNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InsetShadowNode")
    }
}
