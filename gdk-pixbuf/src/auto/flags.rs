// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{bitflags::bitflags, translate::*};
use std::fmt;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GdkPixbufFormatFlags")]
    pub struct PixbufFormatFlags: u32 {
        #[doc(alias = "GDK_PIXBUF_FORMAT_WRITABLE")]
        const WRITABLE = ffi::GDK_PIXBUF_FORMAT_WRITABLE as _;
        #[doc(alias = "GDK_PIXBUF_FORMAT_SCALABLE")]
        const SCALABLE = ffi::GDK_PIXBUF_FORMAT_SCALABLE as _;
        #[doc(alias = "GDK_PIXBUF_FORMAT_THREADSAFE")]
        const THREADSAFE = ffi::GDK_PIXBUF_FORMAT_THREADSAFE as _;
    }
}

impl fmt::Display for PixbufFormatFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for PixbufFormatFlags {
    type GlibType = ffi::GdkPixbufFormatFlags;

    #[inline]
    fn into_glib(self) -> ffi::GdkPixbufFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPixbufFormatFlags> for PixbufFormatFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GdkPixbufFormatFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}
