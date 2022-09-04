// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskContainerNode")]
    pub struct ContainerNode(Shared<ffi::GskContainerNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for ContainerNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_container_node_get_type()) }
    }
}

impl ContainerNode {
    #[doc(alias = "gsk_container_node_new")]
    pub fn new(children: &[RenderNode]) -> ContainerNode {
        assert_initialized_main_thread!();
        let n_children = children.len() as u32;
        unsafe {
            from_glib_full(ffi::gsk_container_node_new(
                children.to_glib_none().0,
                n_children,
            ))
        }
    }

    #[doc(alias = "gsk_container_node_get_n_children")]
    #[doc(alias = "get_n_children")]
    pub fn n_children(&self) -> u32 {
        unsafe { ffi::gsk_container_node_get_n_children(self.to_glib_none().0) }
    }
}

impl fmt::Display for ContainerNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ContainerNode")
    }
}
