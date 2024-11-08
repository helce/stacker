extern crate stacker;

use std::sync::mpsc;
use std::thread;

#[inline(never)]
fn __stacker_black_box(_: *const u8) {}

#[test]
#[cfg_attr(target_arch = "e2k64", ignore)]
fn deep() {
    fn foo(n: usize, s: &mut [u8]) {
        __stacker_black_box(s.as_ptr());
        if n > 0 {
            stacker::maybe_grow(64 * 1024, 1024 * 1024, || {
                let mut s = [0u8; 1024];
                foo(n - 1, &mut s);
                __stacker_black_box(s.as_ptr());
            })
        } else {
            println!("bottom");
        }
    }

    let limit = if cfg!(target_arch = "wasm32") {
        2000
    } else {
        256 * 1024
    };
    foo(limit, &mut []);
}

#[test]
#[cfg_attr(any(target_arch = "wasm32", target_arch = "e2k64"), ignore)]
fn panic() {
    fn foo(n: usize, s: &mut [u8]) {
        __stacker_black_box(s.as_ptr());
        if n > 0 {
            stacker::maybe_grow(64 * 1024, 1024 * 1024, || {
                let mut s = [0u8; 1024];
                foo(n - 1, &mut s);
                __stacker_black_box(s.as_ptr());
            })
        } else {
            panic!("bottom");
        }
    }

    let (tx, rx) = mpsc::channel::<()>();
    thread::spawn(move || {
        foo(64 * 1024, &mut []);
        drop(tx);
    })
    .join()
    .unwrap_err();

    assert!(rx.recv().is_err());
}

fn recursive<F: FnOnce()>(n: usize, f: F) -> usize {
    if n > 0 {
        stacker::grow(64 * 1024, || recursive(n - 1, f) + 1)
    } else {
        f();
        0
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", ignore)]
fn catch_panic() {
    let panic_result = std::panic::catch_unwind(|| {
        recursive(100, || panic!());
    });
    assert!(panic_result.is_err());
}

#[test]
#[cfg_attr(target_arch = "wasm32", ignore)]
fn catch_panic_inside() {
    stacker::grow(64 * 1024, || {
        let panic_result = std::panic::catch_unwind(|| {
            recursive(100, || panic!());
        });
        assert!(panic_result.is_err());
    });
}

#[test]
#[cfg_attr(target_arch = "wasm32", ignore)]
fn catch_panic_leaf() {
    stacker::grow(64 * 1024, || {
        let panic_result = std::panic::catch_unwind(|| panic!());
        assert!(panic_result.is_err());
    });
}
