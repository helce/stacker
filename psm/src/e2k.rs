extern crate libc;
use core::mem::{transmute, MaybeUninit};

struct UcontextT(libc::ucontext_t);

impl Drop for UcontextT {
    fn drop(&mut self) {
        unsafe {
            libc::freecontext_e2k(&mut self.0);
        }
    }
}

#[cfg(switchable_stack)]
#[inline(always)]
pub unsafe fn rust_psm_on_stack(
    data: usize,
    return_ptr: usize,
    callback: extern_item!(unsafe fn(usize, usize)),
    sp: *mut u8,
    base: *mut u8,
) {
    let mut uctx = MaybeUninit::<UcontextT>::uninit();
    let uctx_ptr = &mut uctx as *mut MaybeUninit<UcontextT> as *mut UcontextT;
    assert!(
        libc::getcontext(uctx_ptr.offset(0) as *mut libc::ucontext_t) != -1,
        "psm_on_stack: getcontext"
    );
    let mut uctx = uctx.assume_init();
    uctx.0.uc_stack.ss_sp = base as *mut libc::c_void;
    uctx.0.uc_stack.ss_size = sp.offset_from(base) as libc::size_t;
    let mut uctx_main = MaybeUninit::<UcontextT>::uninit();
    let uctx_main_ptr = &mut uctx_main as *mut MaybeUninit<UcontextT> as *mut UcontextT;
    uctx.0.uc_link = uctx_main_ptr.offset(0) as *mut libc::ucontext_t;
    assert!(
        libc::makecontext_e2k(
            &mut uctx.0 as *mut libc::ucontext_t,
            transmute(callback),
            2,
            data,
            return_ptr
        ) != -1,
        "psm_on_stack: makecontext_e2k"
    );
    assert!(
        libc::swapcontext(
            uctx_main_ptr.offset(0) as *mut libc::ucontext_t,
            &mut uctx.0 as *mut libc::ucontext_t
        ) != -1,
        "psm_on_stack: swapcontext"
    );
    let mut _uctx_main = uctx_main.assume_init();
}

#[cfg(switchable_stack)]
#[inline(always)]
pub unsafe fn rust_psm_replace_stack(
    _data: usize,
    _callback: extern_item!(unsafe fn(usize) -> !),
    _sp: *mut u8,
    _: *mut u8,
) -> ! {
    panic!("psm_replace_stack is not supported for e2k");
}
