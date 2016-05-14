// This file was generated by gir (4d68d19) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct SeparatorMenuItem(Object<ffi::GtkSeparatorMenuItem>): MenuItem, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_separator_menu_item_get_type(),
    }
}

impl SeparatorMenuItem {
    pub fn new() -> SeparatorMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_separator_menu_item_new()).downcast_unchecked()
        }
    }
}
