use crate::ShortcutsSection;
use glib::object::ObjectType;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib::ObjectExt;
use std::mem::transmute;

impl ShortcutsSection {
    pub fn connect_change_current_page<F: Fn(&ShortcutsSection, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"change-current-page\0".as_ptr() as *const _,
                Some(transmute(change_current_page_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn emit_change_current_page(&self, object: i32) -> bool {
        let stash: Stash<*mut ffi::GtkShortcutsSection, _> = self.to_glib_none();
        let gobject =
            unsafe { glib::Object::from_glib_borrow(stash.0 as *mut glib::gobject_ffi::GObject) };
        let res = gobject.emit("change-current-page", &[&object]).unwrap();
        res.unwrap()
            .get()
            .expect("Return Value for `emit_change_current_page`")
            .unwrap()
    }
}

unsafe extern "C" fn change_current_page_trampoline<
    F: Fn(&ShortcutsSection, i32) -> bool + 'static,
>(
    this: *mut ffi::GtkShortcutsSection,
    object: libc::c_int,
    f: glib::ffi::gpointer,
) -> glib::ffi::gboolean {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this), object).to_glib()
}
