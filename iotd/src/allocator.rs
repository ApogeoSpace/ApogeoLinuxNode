use alloc::alloc::*;
use core::ffi::{c_void};
use core::mem::MaybeUninit;
use crate::interface::callbacks::{__shim_kzalloc, __shim_kfree};

#[derive(Default)]
pub struct Allocator;


unsafe impl GlobalAlloc for Allocator {
     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { __shim_kzalloc(layout.size() as u32, 208 /* GFP_KERNEL */) as *mut u8 }
     }
     unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { __shim_kfree(ptr as *mut c_void); }
     }
}

impl Allocator {
   pub fn kernel_alloc<T: Sized>() -> &'static mut MaybeUninit<T> {
      unsafe { &mut core::slice::from_raw_parts_mut( 
         __shim_kzalloc(size_of::<MaybeUninit<T>>() as u32, 208 /* GFP_KERNEL */) as *mut MaybeUninit<T>,
         1
      )[0] }
   }
   pub fn kernel_free<T: Sized>(x: *mut T) -> () {
      let ptr = x as *mut _  as *mut c_void;
      unsafe { __shim_kfree(ptr); }
   }
}

/// If there is an out of memory error, just panic.
#[alloc_error_handler]
fn my_allocator_error(_layout: Layout) -> ! {
    panic!("out of memory");
}

/// The static global allocator.
#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;
