// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib::object::{Cast, IsA};
use glib::translate::*;
use std::cmp::Ordering;
use std::fmt;
use std::mem;

use crate::{SortType, TreeIter, TreeModel, TreeSortable};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SortColumn {
    Default,
    Index(u32),
}

#[doc(hidden)]
impl ToGlib for SortColumn {
    type GlibType = i32;

    #[inline]
    fn to_glib(&self) -> i32 {
        match *self {
            SortColumn::Default => ffi::GTK_TREE_SORTABLE_DEFAULT_SORT_COLUMN_ID,
            SortColumn::Index(x) => {
                assert!(x <= i32::max_value() as u32, "column index is too big");
                x as i32
            }
        }
    }
}

#[doc(hidden)]
impl FromGlib<i32> for SortColumn {
    #[inline]
    fn from_glib(val: i32) -> SortColumn {
        skip_assert_initialized!();
        match val {
            ffi::GTK_TREE_SORTABLE_DEFAULT_SORT_COLUMN_ID => SortColumn::Default,
            x => {
                assert!(x >= 0, "invalid column index");
                SortColumn::Index(x as u32)
            }
        }
    }
}

impl fmt::Display for SortColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SortColumn::{}",
            match *self {
                SortColumn::Default => "Default",
                SortColumn::Index(_) => "Index",
            }
        )
    }
}

pub trait TreeSortableExtManual: 'static {
    fn set_default_sort_func<F>(&self, sort_func: F)
    where
        F: Fn(&Self, &TreeIter, &TreeIter) -> Ordering + 'static;
    fn set_sort_func<F>(&self, sort_column_id: SortColumn, sort_func: F)
    where
        F: Fn(&Self, &TreeIter, &TreeIter) -> Ordering + 'static;
    fn get_sort_column_id(&self) -> Option<(SortColumn, SortType)>;
    fn set_sort_column_id(&self, sort_column_id: SortColumn, order: SortType);
    fn set_unsorted(&self);
}

unsafe extern "C" fn trampoline<T, F: Fn(&T, &TreeIter, &TreeIter) -> Ordering>(
    this: *mut ffi::GtkTreeModel,
    iter: *mut ffi::GtkTreeIter,
    iter2: *mut ffi::GtkTreeIter,
    f: glib::ffi::gpointer,
) -> i32
where
    T: IsA<TreeSortable>,
{
    let f: &F = &*(f as *const F);
    f(
        &TreeModel::from_glib_none(this).unsafe_cast(),
        &from_glib_borrow(iter),
        &from_glib_borrow(iter2),
    )
    .to_glib()
}

unsafe extern "C" fn destroy_closure<T, F: Fn(&T, &TreeIter, &TreeIter) -> Ordering>(
    ptr: glib::ffi::gpointer,
) {
    Box::<F>::from_raw(ptr as *mut _);
}

fn into_raw<F, T>(func: F) -> glib::ffi::gpointer
where
    F: Fn(&T, &TreeIter, &TreeIter) -> Ordering + 'static,
{
    skip_assert_initialized!();
    let func: Box<F> = Box::new(func);
    Box::into_raw(func) as glib::ffi::gpointer
}

impl<O: IsA<TreeSortable>> TreeSortableExtManual for O {
    #[inline]
    fn get_sort_column_id(&self) -> Option<(SortColumn, SortType)> {
        unsafe {
            let mut sort_column_id = mem::uninitialized();
            let mut order = mem::uninitialized();
            ffi::gtk_tree_sortable_get_sort_column_id(
                self.as_ref().to_glib_none().0,
                &mut sort_column_id,
                &mut order,
            );
            if sort_column_id != ffi::GTK_TREE_SORTABLE_UNSORTED_SORT_COLUMN_ID {
                Some((from_glib(sort_column_id), from_glib(order)))
            } else {
                None
            }
        }
    }

    fn set_default_sort_func<F>(&self, sort_func: F)
    where
        F: Fn(&Self, &TreeIter, &TreeIter) -> Ordering + 'static,
    {
        unsafe {
            ffi::gtk_tree_sortable_set_default_sort_func(
                self.as_ref().to_glib_none().0,
                Some(trampoline::<Self, F>),
                into_raw(sort_func),
                Some(destroy_closure::<Self, F>),
            )
        }
    }

    #[inline]
    fn set_sort_column_id(&self, sort_column_id: SortColumn, order: SortType) {
        unsafe {
            ffi::gtk_tree_sortable_set_sort_column_id(
                self.as_ref().to_glib_none().0,
                sort_column_id.to_glib(),
                order.to_glib(),
            );
        }
    }

    fn set_unsorted(&self) {
        unsafe {
            ffi::gtk_tree_sortable_set_sort_column_id(
                self.as_ref().to_glib_none().0,
                ffi::GTK_TREE_SORTABLE_UNSORTED_SORT_COLUMN_ID,
                SortType::Ascending.to_glib(),
            );
        }
    }

    fn set_sort_func<F>(&self, sort_column_id: SortColumn, sort_func: F)
    where
        F: Fn(&Self, &TreeIter, &TreeIter) -> Ordering + 'static,
    {
        unsafe {
            ffi::gtk_tree_sortable_set_sort_func(
                self.as_ref().to_glib_none().0,
                sort_column_id.to_glib(),
                Some(trampoline::<Self, F>),
                into_raw(sort_func),
                Some(destroy_closure::<Self, F>),
            )
        }
    }
}
