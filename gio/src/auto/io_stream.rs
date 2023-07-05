// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AsyncResult, Cancellable, InputStream, OutputStream};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GIOStream")]
    pub struct IOStream(Object<ffi::GIOStream, ffi::GIOStreamClass>);

    match fn {
        type_ => || ffi::g_io_stream_get_type(),
    }
}

impl IOStream {
    pub const NONE: Option<&'static IOStream> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::IOStream>> Sealed for T {}
}

pub trait IOStreamExt: IsA<IOStream> + sealed::Sealed + 'static {
    #[doc(alias = "g_io_stream_clear_pending")]
    fn clear_pending(&self) {
        unsafe {
            ffi::g_io_stream_clear_pending(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_io_stream_close")]
    fn close(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_io_stream_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_io_stream_close_async")]
    fn close_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn close_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = close_async_trampoline::<P>;
        unsafe {
            ffi::g_io_stream_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn close_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.close_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_io_stream_get_input_stream")]
    #[doc(alias = "get_input_stream")]
    fn input_stream(&self) -> InputStream {
        unsafe {
            from_glib_none(ffi::g_io_stream_get_input_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_io_stream_get_output_stream")]
    #[doc(alias = "get_output_stream")]
    fn output_stream(&self) -> OutputStream {
        unsafe {
            from_glib_none(ffi::g_io_stream_get_output_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_io_stream_has_pending")]
    fn has_pending(&self) -> bool {
        unsafe { from_glib(ffi::g_io_stream_has_pending(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_io_stream_is_closed")]
    fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::g_io_stream_is_closed(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_io_stream_set_pending")]
    fn set_pending(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_io_stream_set_pending(self.as_ref().to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "closed")]
    fn connect_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_closed_trampoline<P: IsA<IOStream>, F: Fn(&P) + 'static>(
            this: *mut ffi::GIOStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(IOStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<IOStream>> IOStreamExt for O {}

impl fmt::Display for IOStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IOStream")
    }
}
