// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! `IMPL` BoxedInline wrapper implementation.

// rustdoc-stripper-ignore-next
/// Wrapper implementations for BoxedInline types. See `wrapper!`.
#[macro_export]
macro_rules! glib_boxed_inline_wrapper {
    ([$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty
     $(, @type_ $get_type_expr:expr)?) => {
        $(#[$attr])*
        #[derive(Copy, Clone)]
        #[repr(transparent)]
        $visibility struct $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? {
            pub(crate) inner: $ffi_name,
            phantom: std::marker::PhantomData<($($($generic),+)?)>,
        }

        $crate::glib_boxed_inline_wrapper!(
            @generic_impl [$($attr)*] $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name,
            @copy ptr unsafe { let copy = $crate::ffi::g_malloc0(std::mem::size_of::<$ffi_name>()) as *mut $ffi_name; std::ptr::copy_nonoverlapping(ptr, copy, 1); copy },
            @free ptr unsafe { $crate::ffi::g_free(ptr as *mut _); },
            @init _ptr (), @copy_into dest src std::ptr::copy_nonoverlapping(src, dest, 1), @clear _ptr ()
        );

        $crate::glib_boxed_inline_wrapper!(@value_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name $(, @type_ $get_type_expr)?);
    };

    ([$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty,
     @copy $copy_arg:ident $copy_expr:expr, @free $free_arg:ident $free_expr:expr
     $(, @type_ $get_type_expr:expr)?) => {
        $(#[$attr])*
        #[derive(Copy, Clone)]
        #[repr(transparent)]
        $visibility struct $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? {
            pub(crate) inner: $ffi_name,
            phantom: std::marker::PhantomData<($($($generic),+)?)>,
        }

        $crate::glib_boxed_inline_wrapper!(
            @generic_impl [$($attr)*] $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name,
            @copy $copy_arg $copy_expr, @free $free_arg $free_expr,
            @init _ptr (), @copy_into dest src std::ptr::copy_nonoverlapping(src, dest, 1), @clear _ptr ()
        );

        $crate::glib_boxed_inline_wrapper!(@value_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name $(, @type_ $get_type_expr)?);
    };

    ([$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty,
     @init $init_arg:ident $init_expr:expr, @copy_into $copy_into_arg_dest:ident $copy_into_arg_src:ident $copy_into_expr:expr, @clear $clear_arg:ident $clear_expr:expr
     $(, @type_ $get_type_expr:expr)?) => {
        $(#[$attr])*
        #[repr(transparent)]
        $visibility struct $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? {
            pub(crate) inner: $ffi_name,
            phantom: std::marker::PhantomData<($($($generic),+)?)>,
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::clone::Clone for $name $(<$($generic),+>)? {
            fn clone(&self) -> Self {
                unsafe {
                    $crate::translate::from_glib_none(&self.inner as *const $ffi_name)
                }
            }
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? Drop for $name $(<$($generic),+>)? {
            fn drop(&mut self) {
                unsafe {
                    let clear = |$clear_arg: *mut $ffi_name| $clear_expr;
                    clear(&mut self.inner as *mut $ffi_name);
                }
            }
        }

        $crate::glib_boxed_inline_wrapper!(
            @generic_impl [$($attr)*] $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name,
            @copy ptr unsafe { let copy = $crate::ffi::g_malloc0(std::mem::size_of::<$ffi_name>()) as *mut $ffi_name; let c = |$copy_into_arg_dest, $copy_into_arg_src| $copy_into_expr; c(copy, ptr); copy },
            @free ptr unsafe { let c = |$clear_arg| $clear_expr; c(ptr); $crate::ffi::g_free(ptr as *mut _); },
            @init $init_arg $init_expr, @copy_into $copy_into_arg_dest $copy_into_arg_src $copy_into_expr, @clear $clear_arg $clear_expr
        );

        $crate::glib_boxed_inline_wrapper!(@value_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name $(, @type_ $get_type_expr)?);
    };


    ([$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty,
     @copy $copy_arg:ident $copy_expr:expr, @free $free_arg:ident $free_expr:expr,
     @init $init_arg:ident $init_expr:expr, @copy_into $copy_into_arg_dest:ident $copy_into_arg_src:ident $copy_into_expr:expr, @clear $clear_arg:ident $clear_expr:expr
     $(, @type_ $get_type_expr:expr)?) => {
        $(#[$attr])*
        #[repr(transparent)]
        $visibility struct $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? {
            pub(crate) inner: $ffi_name,
            phantom: std::marker::PhantomData<($($($generic),+)?)>,
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::clone::Clone for $name $(<$($generic),+>)? {
            fn clone(&self) -> Self {
                unsafe {
                    $crate::translate::from_glib_none(&self.inner as *const $ffi_name)
                }
            }
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? Drop for $name $(<$($generic),+>)? {
            fn drop(&mut self) {
                unsafe {
                    let clear = |$clear_arg: *mut $ffi_name| $clear_expr;
                    clear(&mut self.inner as *mut $ffi_name);
                }
            }
        }

        $crate::glib_boxed_inline_wrapper!(
            @generic_impl [$($attr)*] $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name,
            @copy $copy_arg $copy_expr, @free $free_arg $free_expr,
            @init $init_arg $init_expr, @copy_into $copy_into_arg_dest $copy_into_arg_src $copy_into_expr, @clear $clear_arg $clear_expr
        );

        $crate::glib_boxed_inline_wrapper!(@value_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name $(, @type_ $get_type_expr)?);
    };

    (@generic_impl [$($attr:meta)*] $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty,
     @copy $copy_arg:ident $copy_expr:expr, @free $free_arg:ident $free_expr:expr,
     @init $init_arg:ident $init_expr:expr, @copy_into $copy_into_arg_dest:ident $copy_into_arg_src:ident $copy_into_expr:expr, @clear $clear_arg:ident $clear_expr:expr) => {

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::GlibPtrDefault for $name $(<$($generic),+>)? {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::Uninitialized for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn uninitialized() -> Self {
                let mut v = std::mem::MaybeUninit::zeroed();
                let init = |$init_arg: *mut $ffi_name| $init_expr;
                init(v.as_mut_ptr());
                Self {
                    inner: v.assume_init(),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::UnsafeFrom<$ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn unsafe_from(t: $ffi_name) -> Self {
                Self {
                    inner: t,
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = &'a $name $(<$($generic),+>)?;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                $crate::translate::Stash(&self.inner as *const $ffi_name, self)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                unsafe {
                    let copy = |$copy_arg: *const $ffi_name| $copy_expr;
                    copy(&self.inner as *const $ffi_name)
                }
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtrMut<'a, *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = &'a mut $name $(<$($generic),+>)?;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> $crate::translate::StashMut<'a, *mut $ffi_name, Self> {
                let ptr = &mut self.inner as *mut $ffi_name;
                $crate::translate::StashMut(ptr, self)
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *mut *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = Option<Vec<*const $ffi_name>>;

            fn to_glib_none_from_slice(t: &'a [$name $(<$($generic),+>)?]) -> (*mut *const $ffi_name, Self::Storage) {
                let mut v: Vec<_> = t.iter().map(|s| &s.inner as *const $ffi_name).collect();
                v.push(std::ptr::null_mut() as *const $ffi_name);

                (v.as_mut_ptr(), Some(v))
            }

            fn to_glib_container_from_slice(t: &'a [$name $(<$($generic),+>)?]) -> (*mut *const $ffi_name, Self::Storage) {
                let v_ptr = unsafe {
                    let v_ptr = $crate::ffi::g_malloc0(std::mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), &s.inner as *const $ffi_name);
                    }

                    v_ptr
                };

                (v_ptr, None)
            }

            fn to_glib_full_from_slice(t: &[$name $(<$($generic),+>)?]) -> *mut *const $ffi_name {
                unsafe {
                    let v_ptr = $crate::ffi::g_malloc0(std::mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), $crate::translate::ToGlibPtr::to_glib_full(s));
                    }

                    v_ptr
                }
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *const *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = Option<Vec<*const $ffi_name>>;

            fn to_glib_none_from_slice(t: &'a [$name $(<$($generic),+>)?]) -> (*const *const $ffi_name, Self::Storage) {
                let (ptr, stash) = $crate::translate::ToGlibContainerFromSlice::<'a, *mut *const $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const *const $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [$name $(<$($generic),+>)?]) -> (*const *const $ffi_name, Self::Storage) {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[$name $(<$($generic),+>)?]) -> *const *const $ffi_name {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = Option<&'a [$name $(<$($generic),+>)?]>;

            fn to_glib_none_from_slice(t: &'a [$name $(<$($generic),+>)?]) -> (*mut $ffi_name, Self::Storage) {
                (t.as_ptr() as *mut $ffi_name, Some(t))
            }

            fn to_glib_container_from_slice(t: &'a [$name $(<$($generic),+>)?]) -> (*mut $ffi_name, Self::Storage) {
                (
                    $crate::translate::ToGlibContainerFromSlice::<'a, *mut $ffi_name>::to_glib_full_from_slice(t),
                    None,
                )
            }

            fn to_glib_full_from_slice(t: &[$name $(<$($generic),+>)?]) -> *mut $ffi_name {
                let v_ptr = unsafe {
                    let v_ptr = $crate::ffi::g_malloc0(std::mem::size_of::<$ffi_name>()) as *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        let copy_into = |$copy_into_arg_dest: *mut $ffi_name, $copy_into_arg_src: *const $ffi_name| $copy_into_expr;
                        copy_into(v_ptr.add(i), &s.inner as *const $ffi_name);
                    }

                    v_ptr
                };

                v_ptr
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = Option<&'a [$name $(<$($generic),+>)?]>;

            fn to_glib_none_from_slice(t: &'a [$name $(<$($generic),+>)?]) -> (*const $ffi_name, Self::Storage) {
                let (ptr, stash) = $crate::translate::ToGlibContainerFromSlice::<'a, *mut $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [$name $(<$($generic),+>)?]) -> (*const $ffi_name, Self::Storage) {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[$name $(<$($generic),+>)?]) -> *const $ffi_name {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrNone<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                assert!(!ptr.is_null());

                let mut v = <$name $(<$($generic),+>)? as $crate::translate::Uninitialized>::uninitialized();
                let copy_into = |$copy_into_arg_dest: *mut $ffi_name, $copy_into_arg_src: *const $ffi_name| $copy_into_expr;
                copy_into(&mut v.inner as *mut $ffi_name, ptr as *const $ffi_name);

                v
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrNone<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                $crate::translate::from_glib_none::<_, $name $(<$($generic),+>)?>(ptr as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrFull<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                assert!(!ptr.is_null());

                let mut v = <$name $(<$($generic),+>)? as $crate::translate::Uninitialized>::uninitialized();
                let copy_into = |$copy_into_arg_dest: *mut $ffi_name, $copy_into_arg_src: *const $ffi_name| $copy_into_expr;
                copy_into(&mut v.inner as *mut $ffi_name, ptr as *const $ffi_name);

                let free = |$free_arg: *mut $ffi_name| $free_expr;
                free(ptr);

                v
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrFull<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_full(ptr: *const $ffi_name) -> Self {
                $crate::translate::from_glib_full::<_, $name $(<$($generic),+>)?>(ptr as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> $crate::translate::Borrowed<Self> {
                assert!(!ptr.is_null());

                $crate::translate::Borrowed::new(Self {
                    inner: std::ptr::read(ptr),
                    phantom: std::marker::PhantomData,
                })
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrBorrow<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> $crate::translate::Borrowed<Self> {
                $crate::translate::from_glib_borrow::<_, $name $(<$($generic),+>)?>(ptr as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut $ffi_name, num: usize) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut $ffi_name, num: usize) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr as *const _, num)
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut $ffi_name, num: usize) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr as *const _, num)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *const $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_num_as_vec(ptr: *const $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::translate::from_glib_none(ptr.add(i)));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *const $ffi_name, num: usize) -> Vec<Self> {
                let res = $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *const $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::translate::from_glib_full(ptr.add(i)));
                }
                $crate::ffi::g_free(ptr as *mut _);
                res
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::translate::from_glib_none(std::ptr::read(ptr.add(i))));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::translate::from_glib_full(std::ptr::read(ptr.add(i))));
                }
                $crate::ffi::g_free(ptr as *mut _);
                res
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }
        }
    };

    (@value_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty) => { };

    (@value_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty, @type_ $get_type_expr:expr) => {
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::types::StaticType for $name $(<$($generic),+>)? {
            fn static_type() -> $crate::types::Type {
                #[allow(unused_unsafe)]
                unsafe { $crate::translate::from_glib($get_type_expr) }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ValueType for $name $(<$($generic),+>)? {
            type Type = $name $(<$($generic),+>)?;
        }

        #[doc(hidden)]
        unsafe impl<'a> $crate::value::FromValue<'a> for $name $(<$($generic),+>)? {
            type Checker = $crate::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a $crate::Value) -> Self {
                let ptr = $crate::gobject_ffi::g_value_get_boxed($crate::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                <$name $(<$($generic),+>)? as $crate::translate::FromGlibPtrNone<*const $ffi_name>>::from_glib_none(ptr as *const $ffi_name)
            }
        }

        #[doc(hidden)]
        unsafe impl<'a> $crate::value::FromValue<'a> for &'a $name $(<$($generic),+>)? {
            type Checker = $crate::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a $crate::Value) -> Self {
                let ptr = $crate::gobject_ffi::g_value_get_boxed($crate::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                &*(ptr as *const $ffi_name as *const $name $(<$($generic),+>)?)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ToValue for $name $(<$($generic),+>)? {
            fn to_value(&self) -> $crate::Value {
                unsafe {
                    let mut value = $crate::Value::from_type(<$name $(<$($generic),+>)? as $crate::StaticType>::static_type());
                    $crate::gobject_ffi::g_value_set_boxed(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(self).0 as *mut _,
                    );
                    value
                }
            }

            fn value_type(&self) -> $crate::Type {
                <$name $(<$($generic),+>)? as $crate::StaticType>::static_type()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ToValueOptional for $name $(<$($generic),+>)? {
            fn to_value_optional(s: Option<&Self>) -> $crate::Value {
                let mut value = $crate::Value::for_value_type::<Self>();
                unsafe {
                    $crate::gobject_ffi::g_value_set_boxed(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&s).0 as *mut _,
                    );
                }

                value
            }
        }
    };
}
