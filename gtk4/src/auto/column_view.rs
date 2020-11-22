// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Adjustment;
use crate::Align;
use crate::Buildable;
use crate::ColumnViewColumn;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::Overflow;
use crate::Scrollable;
use crate::ScrollablePolicy;
use crate::SelectionModel;
use crate::SortType;
use crate::Sorter;
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
    pub struct ColumnView(Object<ffi::GtkColumnView, ffi::GtkColumnViewClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Scrollable;

    match fn {
        get_type => || ffi::gtk_column_view_get_type(),
    }
}

impl ColumnView {
    pub fn new<P: IsA<SelectionModel>>(model: Option<&P>) -> ColumnView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_column_view_new(
                model.map(|p| p.as_ref()).to_glib_full(),
            ))
            .unsafe_cast()
        }
    }

    pub fn append_column(&self, column: &ColumnViewColumn) {
        unsafe {
            ffi::gtk_column_view_append_column(self.to_glib_none().0, column.to_glib_none().0);
        }
    }

    pub fn get_columns(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_column_view_get_columns(self.to_glib_none().0)) }
    }

    pub fn get_enable_rubberband(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_get_enable_rubberband(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_model(&self) -> Option<SelectionModel> {
        unsafe { from_glib_none(ffi::gtk_column_view_get_model(self.to_glib_none().0)) }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_column_view_get_reorderable(self.to_glib_none().0)) }
    }

    pub fn get_show_column_separators(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_get_show_column_separators(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_show_row_separators(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_get_show_row_separators(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_single_click_activate(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_get_single_click_activate(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_sorter(&self) -> Option<Sorter> {
        unsafe { from_glib_none(ffi::gtk_column_view_get_sorter(self.to_glib_none().0)) }
    }

    pub fn insert_column(&self, position: u32, column: &ColumnViewColumn) {
        unsafe {
            ffi::gtk_column_view_insert_column(
                self.to_glib_none().0,
                position,
                column.to_glib_none().0,
            );
        }
    }

    pub fn remove_column(&self, column: &ColumnViewColumn) {
        unsafe {
            ffi::gtk_column_view_remove_column(self.to_glib_none().0, column.to_glib_none().0);
        }
    }

    pub fn set_enable_rubberband(&self, enable_rubberband: bool) {
        unsafe {
            ffi::gtk_column_view_set_enable_rubberband(
                self.to_glib_none().0,
                enable_rubberband.to_glib(),
            );
        }
    }

    pub fn set_model<P: IsA<SelectionModel>>(&self, model: Option<&P>) {
        unsafe {
            ffi::gtk_column_view_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_column_view_set_reorderable(self.to_glib_none().0, reorderable.to_glib());
        }
    }

    pub fn set_show_column_separators(&self, show_column_separators: bool) {
        unsafe {
            ffi::gtk_column_view_set_show_column_separators(
                self.to_glib_none().0,
                show_column_separators.to_glib(),
            );
        }
    }

    pub fn set_show_row_separators(&self, show_row_separators: bool) {
        unsafe {
            ffi::gtk_column_view_set_show_row_separators(
                self.to_glib_none().0,
                show_row_separators.to_glib(),
            );
        }
    }

    pub fn set_single_click_activate(&self, single_click_activate: bool) {
        unsafe {
            ffi::gtk_column_view_set_single_click_activate(
                self.to_glib_none().0,
                single_click_activate.to_glib(),
            );
        }
    }

    pub fn sort_by_column(&self, column: Option<&ColumnViewColumn>, direction: SortType) {
        unsafe {
            ffi::gtk_column_view_sort_by_column(
                self.to_glib_none().0,
                column.to_glib_none().0,
                direction.to_glib(),
            );
        }
    }

    pub fn connect_activate<F: Fn(&ColumnView, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&ColumnView, u32) + 'static>(
            this: *mut ffi::GtkColumnView,
            position: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), position)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_columns_notify<F: Fn(&ColumnView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_columns_trampoline<F: Fn(&ColumnView) + 'static>(
            this: *mut ffi::GtkColumnView,
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
                b"notify::columns\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_columns_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_enable_rubberband_notify<F: Fn(&ColumnView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_rubberband_trampoline<F: Fn(&ColumnView) + 'static>(
            this: *mut ffi::GtkColumnView,
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
                b"notify::enable-rubberband\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_rubberband_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_model_notify<F: Fn(&ColumnView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&ColumnView) + 'static>(
            this: *mut ffi::GtkColumnView,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_reorderable_notify<F: Fn(&ColumnView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_reorderable_trampoline<F: Fn(&ColumnView) + 'static>(
            this: *mut ffi::GtkColumnView,
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
                b"notify::reorderable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reorderable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_column_separators_notify<F: Fn(&ColumnView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_column_separators_trampoline<
            F: Fn(&ColumnView) + 'static,
        >(
            this: *mut ffi::GtkColumnView,
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
                b"notify::show-column-separators\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_column_separators_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_row_separators_notify<F: Fn(&ColumnView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_row_separators_trampoline<F: Fn(&ColumnView) + 'static>(
            this: *mut ffi::GtkColumnView,
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
                b"notify::show-row-separators\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_row_separators_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_single_click_activate_notify<F: Fn(&ColumnView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_single_click_activate_trampoline<
            F: Fn(&ColumnView) + 'static,
        >(
            this: *mut ffi::GtkColumnView,
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
                b"notify::single-click-activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_single_click_activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_sorter_notify<F: Fn(&ColumnView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_sorter_trampoline<F: Fn(&ColumnView) + 'static>(
            this: *mut ffi::GtkColumnView,
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
}

#[derive(Clone, Default)]
pub struct ColumnViewBuilder {
    enable_rubberband: Option<bool>,
    model: Option<SelectionModel>,
    reorderable: Option<bool>,
    show_column_separators: Option<bool>,
    show_row_separators: Option<bool>,
    single_click_activate: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
    hadjustment: Option<Adjustment>,
    hscroll_policy: Option<ScrollablePolicy>,
    vadjustment: Option<Adjustment>,
    vscroll_policy: Option<ScrollablePolicy>,
}

impl ColumnViewBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ColumnView {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref enable_rubberband) = self.enable_rubberband {
            properties.push(("enable-rubberband", enable_rubberband));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        if let Some(ref reorderable) = self.reorderable {
            properties.push(("reorderable", reorderable));
        }
        if let Some(ref show_column_separators) = self.show_column_separators {
            properties.push(("show-column-separators", show_column_separators));
        }
        if let Some(ref show_row_separators) = self.show_row_separators {
            properties.push(("show-row-separators", show_row_separators));
        }
        if let Some(ref single_click_activate) = self.single_click_activate {
            properties.push(("single-click-activate", single_click_activate));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref hadjustment) = self.hadjustment {
            properties.push(("hadjustment", hadjustment));
        }
        if let Some(ref hscroll_policy) = self.hscroll_policy {
            properties.push(("hscroll-policy", hscroll_policy));
        }
        if let Some(ref vadjustment) = self.vadjustment {
            properties.push(("vadjustment", vadjustment));
        }
        if let Some(ref vscroll_policy) = self.vscroll_policy {
            properties.push(("vscroll-policy", vscroll_policy));
        }
        let ret = glib::Object::new(ColumnView::static_type(), &properties)
            .expect("object new")
            .downcast::<ColumnView>()
            .expect("downcast");
        ret
    }

    pub fn enable_rubberband(mut self, enable_rubberband: bool) -> Self {
        self.enable_rubberband = Some(enable_rubberband);
        self
    }

    pub fn model<P: IsA<SelectionModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }

    pub fn reorderable(mut self, reorderable: bool) -> Self {
        self.reorderable = Some(reorderable);
        self
    }

    pub fn show_column_separators(mut self, show_column_separators: bool) -> Self {
        self.show_column_separators = Some(show_column_separators);
        self
    }

    pub fn show_row_separators(mut self, show_row_separators: bool) -> Self {
        self.show_row_separators = Some(show_row_separators);
        self
    }

    pub fn single_click_activate(mut self, single_click_activate: bool) -> Self {
        self.single_click_activate = Some(single_click_activate);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn hadjustment<P: IsA<Adjustment>>(mut self, hadjustment: &P) -> Self {
        self.hadjustment = Some(hadjustment.clone().upcast());
        self
    }

    pub fn hscroll_policy(mut self, hscroll_policy: ScrollablePolicy) -> Self {
        self.hscroll_policy = Some(hscroll_policy);
        self
    }

    pub fn vadjustment<P: IsA<Adjustment>>(mut self, vadjustment: &P) -> Self {
        self.vadjustment = Some(vadjustment.clone().upcast());
        self
    }

    pub fn vscroll_policy(mut self, vscroll_policy: ScrollablePolicy) -> Self {
        self.vscroll_policy = Some(vscroll_policy);
        self
    }
}

impl fmt::Display for ColumnView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColumnView")
    }
}
