// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use std::fmt;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SettingsSchemaKey(Shared<ffi::GSettingsSchemaKey>);

    match fn {
        ref => |ptr| ffi::g_settings_schema_key_ref(ptr),
        unref => |ptr| ffi::g_settings_schema_key_unref(ptr),
        get_type => || ffi::g_settings_schema_key_get_type(),
    }
}

impl SettingsSchemaKey {
    #[doc(alias = "g_settings_schema_key_get_default_value")]
    pub fn get_default_value(&self) -> glib::Variant {
        unsafe {
            from_glib_full(ffi::g_settings_schema_key_get_default_value(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_settings_schema_key_get_description")]
    pub fn get_description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_description(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    #[doc(alias = "g_settings_schema_key_get_name")]
    pub fn get_name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::g_settings_schema_key_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_settings_schema_key_get_range")]
    pub fn get_range(&self) -> glib::Variant {
        unsafe { from_glib_full(ffi::g_settings_schema_key_get_range(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_settings_schema_key_get_summary")]
    pub fn get_summary(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_summary(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_settings_schema_key_get_value_type")]
    pub fn get_value_type(&self) -> glib::VariantType {
        unsafe {
            from_glib_none(ffi::g_settings_schema_key_get_value_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_settings_schema_key_range_check")]
    pub fn range_check(&self, value: &glib::Variant) -> bool {
        unsafe {
            from_glib(ffi::g_settings_schema_key_range_check(
                self.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }
}

#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
impl fmt::Display for SettingsSchemaKey {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.get_name())
    }
}
