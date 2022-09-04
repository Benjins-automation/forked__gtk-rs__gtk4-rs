// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::EventSequenceState;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkGesture")]
    pub struct Gesture(Object<ffi::GtkGesture, ffi::GtkGestureClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_gesture_get_type(),
    }
}

impl Gesture {
    pub const NONE: Option<&'static Gesture> = None;
}

pub trait GestureExt: 'static {
    #[doc(alias = "gtk_gesture_get_bounding_box")]
    #[doc(alias = "get_bounding_box")]
    fn bounding_box(&self) -> Option<gdk::Rectangle>;

    #[doc(alias = "gtk_gesture_get_bounding_box_center")]
    #[doc(alias = "get_bounding_box_center")]
    fn bounding_box_center(&self) -> Option<(f64, f64)>;

    #[doc(alias = "gtk_gesture_get_device")]
    #[doc(alias = "get_device")]
    fn device(&self) -> Option<gdk::Device>;

    #[doc(alias = "gtk_gesture_get_group")]
    #[doc(alias = "get_group")]
    fn group(&self) -> Vec<Gesture>;

    #[doc(alias = "gtk_gesture_get_last_event")]
    #[doc(alias = "get_last_event")]
    fn last_event(&self, sequence: Option<&gdk::EventSequence>) -> Option<gdk::Event>;

    #[doc(alias = "gtk_gesture_get_last_updated_sequence")]
    #[doc(alias = "get_last_updated_sequence")]
    fn last_updated_sequence(&self) -> Option<gdk::EventSequence>;

    #[doc(alias = "gtk_gesture_get_point")]
    #[doc(alias = "get_point")]
    fn point(&self, sequence: Option<&gdk::EventSequence>) -> Option<(f64, f64)>;

    #[doc(alias = "gtk_gesture_get_sequence_state")]
    #[doc(alias = "get_sequence_state")]
    fn sequence_state(&self, sequence: &gdk::EventSequence) -> EventSequenceState;

    #[doc(alias = "gtk_gesture_get_sequences")]
    #[doc(alias = "get_sequences")]
    fn sequences(&self) -> Vec<gdk::EventSequence>;

    #[doc(alias = "gtk_gesture_group")]
    #[doc(alias = "group")]
    fn group_with(&self, gesture: &impl IsA<Gesture>);

    #[doc(alias = "gtk_gesture_handles_sequence")]
    fn handles_sequence(&self, sequence: Option<&gdk::EventSequence>) -> bool;

    #[doc(alias = "gtk_gesture_is_active")]
    fn is_active(&self) -> bool;

    #[doc(alias = "gtk_gesture_is_grouped_with")]
    fn is_grouped_with(&self, other: &impl IsA<Gesture>) -> bool;

    #[doc(alias = "gtk_gesture_is_recognized")]
    fn is_recognized(&self) -> bool;

    #[doc(alias = "gtk_gesture_set_sequence_state")]
    fn set_sequence_state(&self, sequence: &gdk::EventSequence, state: EventSequenceState) -> bool;

    #[doc(alias = "gtk_gesture_set_state")]
    fn set_state(&self, state: EventSequenceState) -> bool;

    #[doc(alias = "gtk_gesture_ungroup")]
    fn ungroup(&self);

    #[doc(alias = "n-points")]
    fn n_points(&self) -> u32;

    #[doc(alias = "begin")]
    fn connect_begin<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "cancel")]
    fn connect_cancel<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "end")]
    fn connect_end<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "sequence-state-changed")]
    fn connect_sequence_state_changed<
        F: Fn(&Self, Option<&gdk::EventSequence>, EventSequenceState) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "update")]
    fn connect_update<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Gesture>> GestureExt for O {
    fn bounding_box(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_get_bounding_box(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none_mut().0,
            ));
            if ret {
                Some(rect)
            } else {
                None
            }
        }
    }

    fn bounding_box_center(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_get_bounding_box_center(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            if ret {
                Some((x.assume_init(), y.assume_init()))
            } else {
                None
            }
        }
    }

    fn device(&self) -> Option<gdk::Device> {
        unsafe { from_glib_none(ffi::gtk_gesture_get_device(self.as_ref().to_glib_none().0)) }
    }

    fn group(&self) -> Vec<Gesture> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_gesture_get_group(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn last_event(&self, sequence: Option<&gdk::EventSequence>) -> Option<gdk::Event> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_get_last_event(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
            ))
        }
    }

    fn last_updated_sequence(&self) -> Option<gdk::EventSequence> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_get_last_updated_sequence(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn point(&self, sequence: Option<&gdk::EventSequence>) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_get_point(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            if ret {
                Some((x.assume_init(), y.assume_init()))
            } else {
                None
            }
        }
    }

    fn sequence_state(&self, sequence: &gdk::EventSequence) -> EventSequenceState {
        unsafe {
            from_glib(ffi::gtk_gesture_get_sequence_state(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
            ))
        }
    }

    fn sequences(&self) -> Vec<gdk::EventSequence> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_gesture_get_sequences(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn group_with(&self, gesture: &impl IsA<Gesture>) {
        unsafe {
            ffi::gtk_gesture_group(
                self.as_ref().to_glib_none().0,
                gesture.as_ref().to_glib_none().0,
            );
        }
    }

    fn handles_sequence(&self, sequence: Option<&gdk::EventSequence>) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_handles_sequence(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe { from_glib(ffi::gtk_gesture_is_active(self.as_ref().to_glib_none().0)) }
    }

    fn is_grouped_with(&self, other: &impl IsA<Gesture>) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_grouped_with(
                self.as_ref().to_glib_none().0,
                other.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_recognized(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_recognized(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_sequence_state(&self, sequence: &gdk::EventSequence, state: EventSequenceState) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_set_sequence_state(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
                state.into_glib(),
            ))
        }
    }

    fn set_state(&self, state: EventSequenceState) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_set_state(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
            ))
        }
    }

    fn ungroup(&self) {
        unsafe {
            ffi::gtk_gesture_ungroup(self.as_ref().to_glib_none().0);
        }
    }

    fn n_points(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "n-points")
    }

    fn connect_begin<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn begin_trampoline<
            P: IsA<Gesture>,
            F: Fn(&P, Option<&gdk::EventSequence>) + 'static,
        >(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    begin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_cancel<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<
            P: IsA<Gesture>,
            F: Fn(&P, Option<&gdk::EventSequence>) + 'static,
        >(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_end<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn end_trampoline<
            P: IsA<Gesture>,
            F: Fn(&P, Option<&gdk::EventSequence>) + 'static,
        >(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_sequence_state_changed<
        F: Fn(&Self, Option<&gdk::EventSequence>, EventSequenceState) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn sequence_state_changed_trampoline<
            P: IsA<Gesture>,
            F: Fn(&P, Option<&gdk::EventSequence>, EventSequenceState) + 'static,
        >(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            state: ffi::GtkEventSequenceState,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
                from_glib(state),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"sequence-state-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    sequence_state_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_update<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn update_trampoline<
            P: IsA<Gesture>,
            F: Fn(&P, Option<&gdk::EventSequence>) + 'static,
        >(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    update_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Gesture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Gesture")
    }
}
