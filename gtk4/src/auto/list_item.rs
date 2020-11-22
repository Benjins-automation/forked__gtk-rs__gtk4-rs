// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct ListItem(Object<ffi::GtkListItem, ffi::GtkListItemClass>);

    match fn {
        get_type => || ffi::gtk_list_item_get_type(),
    }
}

impl ListItem {
    pub fn get_activatable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_list_item_get_activatable(self.to_glib_none().0)) }
    }

    pub fn get_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_list_item_get_child(self.to_glib_none().0)) }
    }

    pub fn get_item(&self) -> Option<glib::Object> {
        unsafe { from_glib_none(ffi::gtk_list_item_get_item(self.to_glib_none().0)) }
    }

    pub fn get_position(&self) -> u32 {
        unsafe { ffi::gtk_list_item_get_position(self.to_glib_none().0) }
    }

    pub fn get_selectable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_list_item_get_selectable(self.to_glib_none().0)) }
    }

    pub fn get_selected(&self) -> bool {
        unsafe { from_glib(ffi::gtk_list_item_get_selected(self.to_glib_none().0)) }
    }

    pub fn set_activatable(&self, activatable: bool) {
        unsafe {
            ffi::gtk_list_item_set_activatable(self.to_glib_none().0, activatable.to_glib());
        }
    }

    pub fn set_child<P: IsA<Widget>>(&self, child: Option<&P>) {
        unsafe {
            ffi::gtk_list_item_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_list_item_set_selectable(self.to_glib_none().0, selectable.to_glib());
        }
    }

    pub fn connect_property_activatable_notify<F: Fn(&ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activatable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_child_notify<F: Fn(&ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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

    pub fn connect_property_item_notify<F: Fn(&ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_item_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_position_notify<F: Fn(&ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_selectable_notify<F: Fn(&ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_selectable_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::selectable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selectable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_selected_notify<F: Fn(&ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct ListItemBuilder {
    activatable: Option<bool>,
    child: Option<Widget>,
    selectable: Option<bool>,
}

impl ListItemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ListItem {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref activatable) = self.activatable {
            properties.push(("activatable", activatable));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref selectable) = self.selectable {
            properties.push(("selectable", selectable));
        }
        let ret = glib::Object::new(ListItem::static_type(), &properties)
            .expect("object new")
            .downcast::<ListItem>()
            .expect("downcast");
        ret
    }

    pub fn activatable(mut self, activatable: bool) -> Self {
        self.activatable = Some(activatable);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListItem")
    }
}
