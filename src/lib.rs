#![no_std]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![doc = include_str!("../README.md")]

/// A low-latency, SIMD-oriented, non-cryptographic hasher, optimized for short buffers
///
/// # Example
/// ```
/// use dohl::Dohl;
///
/// const fn assert_send_sync<T: Send + Sync>() {}
/// const _: () = assert_send_sync::<Dohl>();
/// ```
pub struct Dohl;

unsafe impl Send for Dohl {}
unsafe impl Sync for Dohl {}
