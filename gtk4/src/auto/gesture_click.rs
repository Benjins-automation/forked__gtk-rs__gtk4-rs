// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::GestureSingle;
use crate::PropagationLimit;
use crate::PropagationPhase;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkGestureClick")]
    pub struct GestureClick(Object<ffi::GtkGestureClick, ffi::GtkGestureClickClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_click_get_type(),
    }
}

impl GestureClick {
    #[doc(alias = "gtk_gesture_click_new")]
    pub fn new() -> GestureClick {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_click_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GestureClick`] objects.
    ///
    /// This method returns an instance of [`GestureClickBuilder`](crate::builders::GestureClickBuilder) which can be used to create [`GestureClick`] objects.
    pub fn builder() -> GestureClickBuilder {
        GestureClickBuilder::default()
    }

    #[doc(alias = "pressed")]
    pub fn connect_pressed<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pressed_trampoline<F: Fn(&GestureClick, i32, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureClick,
            n_press: libc::c_int,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), n_press, x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pressed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pressed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "released")]
    pub fn connect_released<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn released_trampoline<F: Fn(&GestureClick, i32, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureClick,
            n_press: libc::c_int,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), n_press, x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"released\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    released_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stopped")]
    pub fn connect_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stopped_trampoline<F: Fn(&GestureClick) + 'static>(
            this: *mut ffi::GtkGestureClick,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stopped\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stopped_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "unpaired-release")]
    pub fn connect_unpaired_release<
        F: Fn(&Self, f64, f64, u32, Option<&gdk::EventSequence>) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn unpaired_release_trampoline<
            F: Fn(&GestureClick, f64, f64, u32, Option<&gdk::EventSequence>) + 'static,
        >(
            this: *mut ffi::GtkGestureClick,
            x: libc::c_double,
            y: libc::c_double,
            button: libc::c_uint,
            sequence: *mut gdk::ffi::GdkEventSequence,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                x,
                y,
                button,
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unpaired-release\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unpaired_release_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureClick {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GestureClick`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GestureClickBuilder {
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl GestureClickBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GestureClickBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureClick`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GestureClick {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        glib::Object::new::<GestureClick>(&properties)
            .expect("Failed to create an instance of GestureClick")
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn propagation_limit(mut self, propagation_limit: PropagationLimit) -> Self {
        self.propagation_limit = Some(propagation_limit);
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }
}

impl fmt::Display for GestureClick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureClick")
    }
}
