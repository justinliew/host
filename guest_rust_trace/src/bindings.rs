#[allow(dead_code)]
pub mod fastly {
    #[allow(dead_code)]
    pub mod varnish {
        #[allow(dead_code, clippy::all)]
        pub mod trace_log {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            pub fn log(msg: &str, endpoint: &str, sid: &str) {
                unsafe {
                    let vec0 = msg;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let vec1 = endpoint;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let vec2 = sid;
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "fastly:varnish/trace-log")]
                    extern "C" {
                        #[link_name = "log"]
                        fn wit_import(
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        ptr0.cast_mut(),
                        len0,
                        ptr1.cast_mut(),
                        len1,
                        ptr2.cast_mut(),
                        len2,
                    );
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ReqResource {
                handle: _rt::Resource<ReqResource>,
            }
            impl ReqResource {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for ReqResource {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "fastly:varnish/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]req-resource"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl ReqResource {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_header_names(&self) -> _rt::Vec<_rt::Vec<u8>> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "fastly:varnish/types")]
                        extern "C" {
                            #[link_name = "[method]req-resource.get-header-names"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base6 = l1;
                        let len6 = l2;
                        let mut result6 = _rt::Vec::with_capacity(len6);
                        for i in 0..len6 {
                            let base = base6.add(i * 8);
                            let e6 = {
                                let l3 = *base.add(0).cast::<*mut u8>();
                                let l4 = *base.add(4).cast::<usize>();
                                let len5 = l4;
                                _rt::Vec::from_raw_parts(l3.cast(), len5, len5)
                            };
                            result6.push(e6);
                        }
                        _rt::cabi_dealloc(base6, len6 * 8, 4);
                        result6
                    }
                }
            }
            impl ReqResource {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get(&self, header: &str) -> Option<_rt::Vec<_rt::Vec<u8>>> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = header;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "fastly:varnish/types")]
                        extern "C" {
                            #[link_name = "[method]req-resource.get"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l3 = *ptr1.add(4).cast::<*mut u8>();
                                    let l4 = *ptr1.add(8).cast::<usize>();
                                    let base8 = l3;
                                    let len8 = l4;
                                    let mut result8 = _rt::Vec::with_capacity(len8);
                                    for i in 0..len8 {
                                        let base = base8.add(i * 8);
                                        let e8 = {
                                            let l5 = *base.add(0).cast::<*mut u8>();
                                            let l6 = *base.add(4).cast::<usize>();
                                            let len7 = l6;
                                            _rt::Vec::from_raw_parts(l5.cast(), len7, len7)
                                        };
                                        result8.push(e8);
                                    }
                                    _rt::cabi_dealloc(base8, len8 * 8, 4);
                                    result8
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl ReqResource {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_service_id(&self) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "fastly:varnish/types")]
                        extern "C" {
                            #[link_name = "[method]req-resource.get-service-id"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod queue {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn try_pop(timeout_secs: u64) -> bool {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "fastly:varnish/queue")]
                    extern "C" {
                        #[link_name = "try-pop"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(&timeout_secs));
                    _rt::bool_lift(ret as u8)
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod fastly {
        #[allow(dead_code)]
        pub mod varnish {
            #[allow(dead_code, clippy::all)]
            pub mod trace_hooks {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_enter_cabi<T: Guest>() {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::enter();
                }
                pub trait Guest {
                    fn enter();
                }
                #[doc(hidden)]
                macro_rules! __export_fastly_varnish_trace_hooks_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "fastly:varnish/trace-hooks#enter"] unsafe extern "C" fn
                        export_enter() { $($path_to_types)*:: _export_enter_cabi::<$ty >
                        () } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_fastly_varnish_trace_hooks_cabi;
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub use alloc_crate::string::String;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_trace_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::fastly::varnish::trace_hooks::__export_fastly_varnish_trace_hooks_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::fastly::varnish::trace_hooks);
    };
}
#[doc(inline)]
pub(crate) use __export_trace_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:trace:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 577] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xc5\x03\x01A\x02\x01\
A\x09\x01B\x02\x01@\x03\x03msgs\x08endpoints\x03sids\x01\0\x04\0\x03log\x01\0\x03\
\x01\x18fastly:varnish/trace-log\x05\0\x01B\x0b\x04\0\x0creq-resource\x03\x01\x01\
h\0\x01p}\x01p\x02\x01@\x01\x04self\x01\0\x03\x04\0%[method]req-resource.get-hea\
der-names\x01\x04\x01k\x03\x01@\x02\x04self\x01\x06headers\0\x05\x04\0\x18[metho\
d]req-resource.get\x01\x06\x01@\x01\x04self\x01\0s\x04\0#[method]req-resource.ge\
t-service-id\x01\x07\x03\x01\x14fastly:varnish/types\x05\x01\x02\x03\0\x01\x0cre\
q-resource\x01B\x04\x02\x03\x02\x01\x02\x04\0\x0creq-resource\x03\0\0\x01@\x01\x0c\
timeout-secsw\0\x7f\x04\0\x07try-pop\x01\x02\x03\x01\x14fastly:varnish/queue\x05\
\x03\x01B\x02\x01@\0\x01\0\x04\0\x05enter\x01\0\x04\x01\x1afastly:varnish/trace-\
hooks\x05\x04\x04\x01\x14fastly:varnish/trace\x04\0\x0b\x0b\x01\0\x05trace\x03\0\
\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.215.0\x10wit-bi\
ndgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
