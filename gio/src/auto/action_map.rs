// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Action;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GActionMap")]
    pub struct ActionMap(Interface<ffi::GActionMap, ffi::GActionMapInterface>);

    match fn {
        type_ => || ffi::g_action_map_get_type(),
    }
}

impl ActionMap {
    pub const NONE: Option<&'static ActionMap> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ActionMap>> Sealed for T {}
}

pub trait ActionMapExt: IsA<ActionMap> + sealed::Sealed + 'static {
    #[doc(alias = "g_action_map_add_action")]
    fn add_action(&self, action: &impl IsA<Action>) {
        unsafe {
            ffi::g_action_map_add_action(
                self.as_ref().to_glib_none().0,
                action.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_action_map_lookup_action")]
    fn lookup_action(&self, action_name: &str) -> Option<Action> {
        unsafe {
            from_glib_none(ffi::g_action_map_lookup_action(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_action_map_remove_action")]
    fn remove_action(&self, action_name: &str) {
        unsafe {
            ffi::g_action_map_remove_action(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<ActionMap>> ActionMapExt for O {}

impl fmt::Display for ActionMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ActionMap")
    }
}
