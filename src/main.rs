//! BORUIX fpcheck: dedicated user-mode FP demo process for the phase-4 acceptance.
//! Spawned by init and distributed to an AP queue by the round-robin scheduler.
#![no_std]
#![no_main]
extern crate alloc;
use libsys::write;
use libsys::STDOUT;
#[unsafe(no_mangle)]
pub extern "C" fn user_main(_argc: isize, _argv: *const *const u8) -> i32 {
    let _ = write(STDOUT, b"[fpcheck] user-mode FP demo starting on this CPU\n");
    let mut v: f64 = 1.0;
    for i in 0u32..30 {
        v = v * 1.01 + (i as f64) * 0.001;
        let mut buf = [0u8; 96];
        let n = unsafe {
            libc::stdio::snprintf(buf.as_mut_ptr() as *mut i8, buf.len(),
                b"[fpcheck] iter=%u val=%.2f\0".as_ptr() as *const i8, i, v)
        };
        if n > 0 { let _ = write(STDOUT, &buf[..n as usize]); let _ = write(STDOUT, b"\n"); }
        let _ = libsys::sleep(2_000_000_000);
    }
    let _ = write(STDOUT, b"[fpcheck] done\n");
    0
}