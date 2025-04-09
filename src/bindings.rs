// Generated by `wit-bindgen` 0.41.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod uprotocol {
    pub mod basic {
        #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
        pub mod uattributes {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Uattributes {
                pub foo: u32,
            }
            impl ::core::fmt::Debug for Uattributes {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Uattributes").field("foo", &self.foo).finish()
                }
            }
        }
        #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
        pub mod umessage {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type Uattributes = super::super::super::uprotocol::basic::uattributes::Uattributes;
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Umessage {
                pub uattributes: Uattributes,
            }
            impl ::core::fmt::Debug for Umessage {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Umessage")
                        .field("uattributes", &self.uattributes)
                        .finish()
                }
            }
        }
    }
}
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod uprotocol {
        pub mod basic {
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod uuri {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// Data model definition for source and destination addressing of messages sent to/from
                /// devices, services, methods, topics, etc...
                #[derive(Clone)]
                pub struct Uuri {
                    /// Authority Name.
                    ///
                    /// Could be the host name, ip address, device & domain names, etc..
                    pub authority_name: _rt::String,
                    /// Software Entity (uEntity) Identifiers.
                    pub ue_id: u32,
                    /// Software Entity (uEntity) major version number.
                    pub ue_version_major: u32,
                    /// uEntity resource id.
                    ///
                    /// Identifier used to represent either a method, publish topic, or notification topic.
                    pub resource_id: u32,
                }
                impl ::core::fmt::Debug for Uuri {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Uuri")
                            .field("authority-name", &self.authority_name)
                            .field("ue-id", &self.ue_id)
                            .field("ue-version-major", &self.ue_version_major)
                            .field("resource-id", &self.resource_id)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_create_uuri_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::create_uuri();
                    let ptr1 = (&raw mut _RET_AREA.0).cast::<u8>();
                    let Uuri {
                        authority_name: authority_name2,
                        ue_id: ue_id2,
                        ue_version_major: ue_version_major2,
                        resource_id: resource_id2,
                    } = result0;
                    let vec3 = (authority_name2.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr1.add(::core::mem::size_of::<*const u8>()).cast::<usize>() = len3;
                    *ptr1.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    *ptr1.add(2 * ::core::mem::size_of::<*const u8>()).cast::<i32>() = _rt::as_i32(
                        ue_id2,
                    );
                    *ptr1
                        .add(4 + 2 * ::core::mem::size_of::<*const u8>())
                        .cast::<i32>() = _rt::as_i32(ue_version_major2);
                    *ptr1
                        .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                        .cast::<i32>() = _rt::as_i32(resource_id2);
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_create_uuri<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0
                        .add(::core::mem::size_of::<*const u8>())
                        .cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    /// Add a simple function that uses the uuid type
                    /// This ensures the type is included in bindings
                    fn create_uuri() -> Uuri;
                }
                #[doc(hidden)]
                macro_rules! __export_uprotocol_basic_uuri_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "uprotocol:basic/uuri#create-uuri")] unsafe extern "C" fn
                        export_create_uuri() -> * mut u8 { unsafe { $($path_to_types)*::
                        _export_create_uuri_cabi::<$ty > () } } #[unsafe (export_name =
                        "cabi_post_uprotocol:basic/uuri#create-uuri")] unsafe extern "C"
                        fn _post_return_create_uuri(arg0 : * mut u8,) { unsafe {
                        $($path_to_types)*:: __post_return_create_uuri::<$ty > (arg0) } }
                        };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_uprotocol_basic_uuri_cabi;
                #[cfg_attr(target_pointer_width = "64", repr(align(8)))]
                #[cfg_attr(target_pointer_width = "32", repr(align(4)))]
                struct _RetArea(
                    [::core::mem::MaybeUninit<
                        u8,
                    >; 8 + 3 * ::core::mem::size_of::<*const u8>()],
                );
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8
                        + 3 * ::core::mem::size_of::<*const u8>()],
                );
            }
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod uuid {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// The UUID datamodel per https://www.rfc-editor.org/rfc/rfc9562[RFC 9562]
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Uuid {
                    pub msb: u64,
                    /// Most significant bits
                    pub lsb: u64,
                }
                impl ::core::fmt::Debug for Uuid {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Uuid")
                            .field("msb", &self.msb)
                            .field("lsb", &self.lsb)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_create_uuid_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::create_uuid();
                    let ptr1 = (&raw mut _RET_AREA.0).cast::<u8>();
                    let Uuid { msb: msb2, lsb: lsb2 } = result0;
                    *ptr1.add(0).cast::<i64>() = _rt::as_i64(msb2);
                    *ptr1.add(8).cast::<i64>() = _rt::as_i64(lsb2);
                    ptr1
                }
                pub trait Guest {
                    /// Add a simple function that uses the uuid type
                    /// This ensures the type is included in bindings
                    fn create_uuid() -> Uuid;
                }
                #[doc(hidden)]
                macro_rules! __export_uprotocol_basic_uuid_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "uprotocol:basic/uuid#create-uuid")] unsafe extern "C" fn
                        export_create_uuid() -> * mut u8 { unsafe { $($path_to_types)*::
                        _export_create_uuid_cabi::<$ty > () } } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_uprotocol_basic_uuid_cabi;
                #[repr(align(8))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod utransport {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Uuri = super::super::super::super::exports::uprotocol::basic::uuri::Uuri;
                pub type Umessage = super::super::super::super::uprotocol::basic::umessage::Umessage;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_register_listener_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: *mut u8,
                    arg6: usize,
                    arg7: i32,
                    arg8: i32,
                    arg9: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg6;
                    let bytes1 = _rt::Vec::from_raw_parts(arg5.cast(), len1, len1);
                    let result2 = T::register_listener(
                        super::super::super::super::exports::uprotocol::basic::uuri::Uuri {
                            authority_name: _rt::string_lift(bytes0),
                            ue_id: arg2 as u32,
                            ue_version_major: arg3 as u32,
                            resource_id: arg4 as u32,
                        },
                        super::super::super::super::exports::uprotocol::basic::uuri::Uuri {
                            authority_name: _rt::string_lift(bytes1),
                            ue_id: arg7 as u32,
                            ue_version_major: arg8 as u32,
                            resource_id: arg9 as u32,
                        },
                    );
                    (result2).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_unregister_listener_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: *mut u8,
                    arg6: usize,
                    arg7: i32,
                    arg8: i32,
                    arg9: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg6;
                    let bytes1 = _rt::Vec::from_raw_parts(arg5.cast(), len1, len1);
                    let result2 = T::unregister_listener(
                        super::super::super::super::exports::uprotocol::basic::uuri::Uuri {
                            authority_name: _rt::string_lift(bytes0),
                            ue_id: arg2 as u32,
                            ue_version_major: arg3 as u32,
                            resource_id: arg4 as u32,
                        },
                        super::super::super::super::exports::uprotocol::basic::uuri::Uuri {
                            authority_name: _rt::string_lift(bytes1),
                            ue_id: arg7 as u32,
                            ue_version_major: arg8 as u32,
                            resource_id: arg9 as u32,
                        },
                    );
                    (result2).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_send_cabi<T: Guest>(arg0: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::send(super::super::super::super::uprotocol::basic::umessage::Umessage {
                        uattributes: super::super::super::super::uprotocol::basic::uattributes::Uattributes {
                            foo: arg0 as u32,
                        },
                    });
                    (result0).take_handle() as i32
                }
                pub trait Guest {
                    fn register_listener(
                        source_filter: Uuri,
                        sink_filter: Uuri,
                    ) -> wit_bindgen_rt::async_support::FutureReader<_rt::String>;
                    fn unregister_listener(
                        source_filter: Uuri,
                        sink_filter: Uuri,
                    ) -> wit_bindgen_rt::async_support::FutureReader<_rt::String>;
                    fn send(
                        message: Umessage,
                    ) -> wit_bindgen_rt::async_support::FutureReader<_rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_uprotocol_basic_utransport_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "uprotocol:basic/utransport#register-listener")] unsafe extern
                        "C" fn export_register_listener(arg0 : * mut u8, arg1 : usize,
                        arg2 : i32, arg3 : i32, arg4 : i32, arg5 : * mut u8, arg6 :
                        usize, arg7 : i32, arg8 : i32, arg9 : i32,) -> i32 { unsafe {
                        $($path_to_types)*:: _export_register_listener_cabi::<$ty >
                        (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) } }
                        #[unsafe (export_name =
                        "uprotocol:basic/utransport#unregister-listener")] unsafe extern
                        "C" fn export_unregister_listener(arg0 : * mut u8, arg1 : usize,
                        arg2 : i32, arg3 : i32, arg4 : i32, arg5 : * mut u8, arg6 :
                        usize, arg7 : i32, arg8 : i32, arg9 : i32,) -> i32 { unsafe {
                        $($path_to_types)*:: _export_unregister_listener_cabi::<$ty >
                        (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) } }
                        #[unsafe (export_name = "uprotocol:basic/utransport#send")]
                        unsafe extern "C" fn export_send(arg0 : i32,) -> i32 { unsafe {
                        $($path_to_types)*:: _export_send_cabi::<$ty > (arg0) } } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_uprotocol_basic_utransport_cabi;
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    #![allow(dead_code, clippy::all)]
    pub use alloc_crate::string::String;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
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
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::boxed::Box;
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
pub mod wit_future {
    #![allow(dead_code, unused_variables, clippy::all)]
    #[doc(hidden)]
    pub trait FuturePayload: Unpin + Sized + 'static {
        const VTABLE: &'static wit_bindgen_rt::async_support::FutureVtable<Self>;
    }
    #[doc(hidden)]
    pub mod vtable0 {
        fn write(
            future: u32,
            value: super::super::_rt::String,
        ) -> ::core::pin::Pin<
            super::super::_rt::Box<dyn ::core::future::Future<Output = bool>>,
        > {
            super::super::_rt::Box::pin(async move {
                #[repr(align(4))]
                struct Buffer([::core::mem::MaybeUninit<u8>; 8]);
                let mut buffer = Buffer([::core::mem::MaybeUninit::uninit(); 8]);
                let address = buffer.0.as_mut_ptr() as *mut u8;
                unsafe {
                    let vec0 = &value;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    *address.add(::core::mem::size_of::<*const u8>()).cast::<usize>() = len0;
                    *address.add(0).cast::<*mut u8>() = ptr0.cast_mut();
                }
                match unsafe {
                    wit_bindgen_rt::async_support::await_future_result(
                            start_write,
                            future,
                            address,
                        )
                        .await
                } {
                    wit_bindgen_rt::async_support::AsyncWaitResult::Values(_) => true,
                    wit_bindgen_rt::async_support::AsyncWaitResult::End => false,
                    wit_bindgen_rt::async_support::AsyncWaitResult::Error(_) => {
                        unreachable!("received error while performing write")
                    }
                }
            })
        }
        fn read(
            future: u32,
        ) -> ::core::pin::Pin<
            super::super::_rt::Box<
                dyn ::core::future::Future<
                    Output = ::std::option::Option<
                        ::std::result::Result<
                            super::super::_rt::String,
                            wit_bindgen_rt::async_support::ErrorContext,
                        >,
                    >,
                >,
            >,
        > {
            super::super::_rt::Box::pin(async move {
                struct Buffer([::core::mem::MaybeUninit<u8>; 8]);
                let mut buffer = Buffer([::core::mem::MaybeUninit::uninit(); 8]);
                let address = buffer.0.as_mut_ptr() as *mut u8;
                match unsafe {
                    wit_bindgen_rt::async_support::await_future_result(
                            start_read,
                            future,
                            address,
                        )
                        .await
                } {
                    wit_bindgen_rt::async_support::AsyncWaitResult::Values(v) => {
                        let value = unsafe {
                            let l0 = *address.add(0).cast::<*mut u8>();
                            let l1 = *address
                                .add(::core::mem::size_of::<*const u8>())
                                .cast::<usize>();
                            let len2 = l1;
                            let bytes2 = super::super::_rt::Vec::from_raw_parts(
                                l0.cast(),
                                len2,
                                len2,
                            );
                            super::super::_rt::string_lift(bytes2)
                        };
                        Some(Ok(value))
                    }
                    wit_bindgen_rt::async_support::AsyncWaitResult::Error(e) => {
                        Some(
                            Err(
                                wit_bindgen_rt::async_support::ErrorContext::from_handle(e),
                            ),
                        )
                    }
                    wit_bindgen_rt::async_support::AsyncWaitResult::End => None,
                }
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        unsafe extern "C" fn cancel_write(_: u32) -> u32 {
            unreachable!()
        }
        #[cfg(not(target_arch = "wasm32"))]
        unsafe extern "C" fn cancel_read(_: u32) -> u32 {
            unreachable!()
        }
        #[cfg(not(target_arch = "wasm32"))]
        unsafe extern "C" fn close_writable(_: u32, _: u32) {
            unreachable!()
        }
        #[cfg(not(target_arch = "wasm32"))]
        unsafe extern "C" fn close_readable(_: u32, _: u32) {
            unreachable!()
        }
        #[cfg(not(target_arch = "wasm32"))]
        unsafe extern "C" fn new() -> u32 {
            unreachable!()
        }
        #[cfg(not(target_arch = "wasm32"))]
        unsafe extern "C" fn start_read(_: u32, _: *mut u8) -> u32 {
            unreachable!()
        }
        #[cfg(not(target_arch = "wasm32"))]
        unsafe extern "C" fn start_write(_: u32, _: *mut u8) -> u32 {
            unreachable!()
        }
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "[export]uprotocol:basic/utransport")]
        unsafe extern "C" {
            #[link_name = "[future-new-0]register-listener"]
            fn new() -> u32;
            #[link_name = "[future-cancel-write-0]register-listener"]
            fn cancel_write(_: u32) -> u32;
            #[link_name = "[future-cancel-read-0]register-listener"]
            fn cancel_read(_: u32) -> u32;
            #[link_name = "[future-close-writable-0]register-listener"]
            fn close_writable(_: u32, _: u32);
            #[link_name = "[future-close-readable-0]register-listener"]
            fn close_readable(_: u32, _: u32);
            #[link_name = "[async-lower][future-read-0]register-listener"]
            fn start_read(_: u32, _: *mut u8) -> u32;
            #[link_name = "[async-lower][future-write-0]register-listener"]
            fn start_write(_: u32, _: *mut u8) -> u32;
        }
        pub static VTABLE: wit_bindgen_rt::async_support::FutureVtable<
            super::super::_rt::String,
        > = wit_bindgen_rt::async_support::FutureVtable::<super::super::_rt::String> {
            write,
            read,
            cancel_write,
            cancel_read,
            close_writable,
            close_readable,
            new,
        };
        impl super::FuturePayload for super::super::_rt::String {
            const VTABLE: &'static wit_bindgen_rt::async_support::FutureVtable<Self> = &VTABLE;
        }
    }
    /// Creates a new Component Model `future` with the specified payload type.
    pub fn new<T: FuturePayload>() -> (
        wit_bindgen_rt::async_support::FutureWriter<T>,
        wit_bindgen_rt::async_support::FutureReader<T>,
    ) {
        unsafe { wit_bindgen_rt::async_support::future_new::<T>(T::VTABLE) }
    }
}
/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
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
macro_rules! __export_up_core_api_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::uprotocol::basic::uuri::__export_uprotocol_basic_uuri_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::uprotocol::basic::uuri);
        $($path_to_types_root)*::
        exports::uprotocol::basic::uuid::__export_uprotocol_basic_uuid_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::uprotocol::basic::uuid);
        $($path_to_types_root)*::
        exports::uprotocol::basic::utransport::__export_uprotocol_basic_utransport_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::uprotocol::basic::utransport);
    };
}
#[doc(inline)]
pub(crate) use __export_up_core_api_impl as export;
#[cfg(target_arch = "wasm32")]
#[unsafe(
    link_section = "component-type:wit-bindgen:0.41.0:uprotocol:basic:up-core-api:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 712] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xc6\x04\x01A\x02\x01\
A\x0d\x01B\x02\x01r\x01\x03fooy\x04\0\x0buattributes\x03\0\0\x03\0\x1buprotocol:\
basic/uattributes\x05\0\x02\x03\0\0\x0buattributes\x01B\x04\x02\x03\x02\x01\x01\x04\
\0\x0buattributes\x03\0\0\x01r\x01\x0buattributes\x01\x04\0\x08umessage\x03\0\x02\
\x03\0\x18uprotocol:basic/umessage\x05\x02\x01B\x04\x01r\x04\x0eauthority-names\x05\
ue-idy\x10ue-version-majory\x0bresource-idy\x04\0\x04uuri\x03\0\0\x01@\0\0\x01\x04\
\0\x0bcreate-uuri\x01\x02\x04\0\x14uprotocol:basic/uuri\x05\x03\x01B\x04\x01r\x02\
\x03msbw\x03lsbw\x04\0\x04uuid\x03\0\0\x01@\0\0\x01\x04\0\x0bcreate-uuid\x01\x02\
\x04\0\x14uprotocol:basic/uuid\x05\x04\x02\x03\0\x02\x04uuri\x02\x03\0\x01\x08um\
essage\x01B\x0a\x02\x03\x02\x01\x05\x04\0\x04uuri\x03\0\0\x02\x03\x02\x01\x06\x04\
\0\x08umessage\x03\0\x02\x01e\x01s\x01@\x02\x0dsource-filter\x01\x0bsink-filter\x01\
\0\x04\x04\0\x11register-listener\x01\x05\x04\0\x13unregister-listener\x01\x05\x01\
@\x01\x07message\x03\0\x04\x04\0\x04send\x01\x06\x04\0\x1auprotocol:basic/utrans\
port\x05\x07\x04\0\x1buprotocol:basic/up-core-api\x04\0\x0b\x11\x01\0\x0bup-core\
-api\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.227.1\
\x10wit-bindgen-rust\x060.41.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
