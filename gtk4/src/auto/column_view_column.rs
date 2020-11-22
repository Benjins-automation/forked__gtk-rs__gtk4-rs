// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ColumnView;
use crate::ListItemFactory;
use crate::Sorter;
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
    pub struct ColumnViewColumn(Object<ffi::GtkColumnViewColumn, ffi::GtkColumnViewColumnClass>);

    match fn {
        get_type => || ffi::gtk_column_view_column_get_type(),
    }
}

impl ColumnViewColumn {
    pub fn new<P: IsA<ListItemFactory>>(
        title: Option<&str>,
        factory: Option<&P>,
    ) -> ColumnViewColumn {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_column_view_column_new(
                title.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }

    pub fn get_column_view(&self) -> Option<ColumnView> {
        unsafe {
            from_glib_none(ffi::gtk_column_view_column_get_column_view(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_expand(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_column_get_expand(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_factory(&self) -> Option<ListItemFactory> {
        unsafe {
            from_glib_none(ffi::gtk_column_view_column_get_factory(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_fixed_width(&self) -> i32 {
        unsafe { ffi::gtk_column_view_column_get_fixed_width(self.to_glib_none().0) }
    }

    pub fn get_header_menu(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_column_view_column_get_header_menu(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_column_get_resizable(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_sorter(&self) -> Option<Sorter> {
        unsafe {
            from_glib_none(ffi::gtk_column_view_column_get_sorter(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_column_view_column_get_title(self.to_glib_none().0)) }
    }

    pub fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_column_get_visible(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_column_view_column_set_expand(self.to_glib_none().0, expand.to_glib());
        }
    }

    pub fn set_factory<P: IsA<ListItemFactory>>(&self, factory: Option<&P>) {
        unsafe {
            ffi::gtk_column_view_column_set_factory(
                self.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_fixed_width(&self, fixed_width: i32) {
        unsafe {
            ffi::gtk_column_view_column_set_fixed_width(self.to_glib_none().0, fixed_width);
        }
    }

    pub fn set_header_menu<P: IsA<gio::MenuModel>>(&self, menu: Option<&P>) {
        unsafe {
            ffi::gtk_column_view_column_set_header_menu(
                self.to_glib_none().0,
                menu.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gtk_column_view_column_set_resizable(self.to_glib_none().0, resizable.to_glib());
        }
    }

    pub fn set_sorter<P: IsA<Sorter>>(&self, sorter: Option<&P>) {
        unsafe {
            ffi::gtk_column_view_column_set_sorter(
                self.to_glib_none().0,
                sorter.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::gtk_column_view_column_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_column_view_column_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    pub fn connect_property_column_view_notify<F: Fn(&ColumnViewColumn) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_view_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::column-view\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_view_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_expand_notify<F: Fn(&ColumnViewColumn) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_expand_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::expand\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expand_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_factory_notify<F: Fn(&ColumnViewColumn) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_factory_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::factory\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_factory_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_fixed_width_notify<F: Fn(&ColumnViewColumn) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_fixed_width_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::fixed-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fixed_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_header_menu_notify<F: Fn(&ColumnViewColumn) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_header_menu_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::header-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_header_menu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_resizable_notify<F: Fn(&ColumnViewColumn) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resizable_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::resizable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resizable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_sorter_notify<F: Fn(&ColumnViewColumn) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_sorter_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::sorter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sorter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_title_notify<F: Fn(&ColumnViewColumn) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_visible_notify<F: Fn(&ColumnViewColumn) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct ColumnViewColumnBuilder {
    expand: Option<bool>,
    factory: Option<ListItemFactory>,
    fixed_width: Option<i32>,
    header_menu: Option<gio::MenuModel>,
    resizable: Option<bool>,
    sorter: Option<Sorter>,
    title: Option<String>,
    visible: Option<bool>,
}

impl ColumnViewColumnBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ColumnViewColumn {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref factory) = self.factory {
            properties.push(("factory", factory));
        }
        if let Some(ref fixed_width) = self.fixed_width {
            properties.push(("fixed-width", fixed_width));
        }
        if let Some(ref header_menu) = self.header_menu {
            properties.push(("header-menu", header_menu));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref sorter) = self.sorter {
            properties.push(("sorter", sorter));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        let ret = glib::Object::new(ColumnViewColumn::static_type(), &properties)
            .expect("object new")
            .downcast::<ColumnViewColumn>()
            .expect("downcast");
        ret
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn factory<P: IsA<ListItemFactory>>(mut self, factory: &P) -> Self {
        self.factory = Some(factory.clone().upcast());
        self
    }

    pub fn fixed_width(mut self, fixed_width: i32) -> Self {
        self.fixed_width = Some(fixed_width);
        self
    }

    pub fn header_menu<P: IsA<gio::MenuModel>>(mut self, header_menu: &P) -> Self {
        self.header_menu = Some(header_menu.clone().upcast());
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn sorter<P: IsA<Sorter>>(mut self, sorter: &P) -> Self {
        self.sorter = Some(sorter.clone().upcast());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }
}

impl fmt::Display for ColumnViewColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColumnViewColumn")
    }
}
