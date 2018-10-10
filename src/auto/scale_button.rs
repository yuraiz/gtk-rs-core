// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Adjustment;
use Bin;
use Buildable;
use Button;
use Container;
use IconSize;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ScaleButton(Object<ffi::GtkScaleButton, ffi::GtkScaleButtonClass>): Button, Bin, Container, Widget, Buildable, Actionable, Orientable;

    match fn {
        get_type => || ffi::gtk_scale_button_get_type(),
    }
}

impl ScaleButton {
    pub fn new(size: IconSize, min: f64, max: f64, step: f64, icons: &[&str]) -> ScaleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_button_new(size.to_glib(), min, max, step, icons.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ScaleButtonExt {
    fn get_adjustment(&self) -> Adjustment;

    fn get_minus_button(&self) -> Option<Button>;

    fn get_plus_button(&self) -> Option<Button>;

    fn get_popup(&self) -> Option<Widget>;

    fn get_value(&self) -> f64;

    fn set_adjustment(&self, adjustment: &Adjustment);

    fn set_icons(&self, icons: &[&str]);

    fn set_value(&self, value: f64);

    fn get_property_icons(&self) -> Vec<String>;

    fn get_property_size(&self) -> IconSize;

    fn set_property_size(&self, size: IconSize);

    fn connect_popdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popdown(&self);

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popup(&self);

    fn connect_value_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ScaleButton> + IsA<glib::object::Object> + glib::object::ObjectExt> ScaleButtonExt for O {
    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_adjustment(self.to_glib_none().0))
        }
    }

    fn get_minus_button(&self) -> Option<Button> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_minus_button(self.to_glib_none().0))
        }
    }

    fn get_plus_button(&self) -> Option<Button> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_plus_button(self.to_glib_none().0))
        }
    }

    fn get_popup(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_popup(self.to_glib_none().0))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_scale_button_get_value(self.to_glib_none().0)
        }
    }

    fn set_adjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_scale_button_set_adjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    fn set_icons(&self, icons: &[&str]) {
        unsafe {
            ffi::gtk_scale_button_set_icons(self.to_glib_none().0, icons.to_glib_none().0);
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_scale_button_set_value(self.to_glib_none().0, value);
        }
    }

    fn get_property_icons(&self) -> Vec<String> {
        unsafe {
            let mut value = Value::from_type(<Vec<String> as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icons".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_size(&self) -> IconSize {
        unsafe {
            let mut value = Value::from_type(<IconSize as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "size".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_size(&self, size: IconSize) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "size".to_glib_none().0, Value::from(&size).to_glib_none().0);
        }
    }

    fn connect_popdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popdown",
                transmute(popdown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_popdown(&self) {
        let _ = self.emit("popdown", &[]).unwrap();
    }

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popup",
                transmute(popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_popup(&self) {
        let _ = self.emit("popup", &[]).unwrap();
    }

    fn connect_value_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "value-changed",
                transmute(value_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::adjustment",
                transmute(notify_adjustment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icons",
                transmute(notify_icons_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::size",
                transmute(notify_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn popdown_trampoline<P>(this: *mut ffi::GtkScaleButton, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn popup_trampoline<P>(this: *mut ffi::GtkScaleButton, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn value_changed_trampoline<P>(this: *mut ffi::GtkScaleButton, value: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &&(Fn(&P, f64) + 'static) = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).downcast_unchecked(), value)
}

unsafe extern "C" fn notify_adjustment_trampoline<P>(this: *mut ffi::GtkScaleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icons_trampoline<P>(this: *mut ffi::GtkScaleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_size_trampoline<P>(this: *mut ffi::GtkScaleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::GtkScaleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ScaleButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ScaleButton::from_glib_borrow(this).downcast_unchecked())
}
