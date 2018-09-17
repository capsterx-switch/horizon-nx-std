// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// *Implementation adapted from `/sys/redox/condvar.rs`

use cell::UnsafeCell;
use intrinsics::atomic_cxchg;
use ptr;
use time::Duration;

#[cfg(target_arch = "aarch64")]
use libnx_rs::{libnx};

use sys::mutex::{self, Mutex};

#[cfg(not(target_arch = "aarch64"))]
pub struct Condvar {
    lock: UnsafeCell<*mut ::libctru::LightLock>,
}

#[cfg(target_arch = "aarch64")]
pub struct Condvar {
    tag : u32, 
    lock: UnsafeCell<*mut libnx::CondVar>,
}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

#[cfg(not(target_arch = "aarch64"))]
impl Condvar {
    pub const fn new() -> Condvar {
        Condvar {
            lock: UnsafeCell::new(ptr::null_mut()),
        }
    }

    #[inline]
    pub unsafe fn init(&self) {
        *self.lock.get() = ptr::null_mut();
    }

    #[inline]
    pub fn notify_one(&self) {
        unsafe {
            let arbiter = ::libctru::__sync_get_arbiter();

            ::libctru::svcArbitrateAddress(arbiter,
                                *self.lock.get() as u32,
                                ::libctru::ARBITRATION_SIGNAL,
                                1,
                                0);
        }
    }

    #[inline]
    pub fn notify_all(&self) {
        unsafe {
            let lock = self.lock.get();

            if *lock == ptr::null_mut() {
                return;
            }

            let arbiter = ::libctru::__sync_get_arbiter();

            ::libctru::svcArbitrateAddress(arbiter,
                                *self.lock.get() as u32,
                                ::libctru::ARBITRATION_SIGNAL,
                                -1,
                                0);
        }
    }

    #[inline]
    pub fn wait(&self, mutex: &Mutex) {
        unsafe {
            let lock = self.lock.get();

            if *lock != mutex::raw(mutex) as *mut i32 {
                if *lock != ptr::null_mut() {
                    panic!("Condvar used with more than one Mutex");
                }

                atomic_cxchg(lock as *mut usize, 0, mutex::raw(mutex) as usize);
            }

            mutex.unlock();

            let arbiter = ::libctru::__sync_get_arbiter();

            ::libctru::svcArbitrateAddress(arbiter,
                                *self.lock.get() as u32,
                                ::libctru::ARBITRATION_WAIT_IF_LESS_THAN,
                                2,
                                0);

            mutex.lock();
        }
    }

    #[inline]
    pub fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        use time::Instant;

        unsafe {
            let lock = self.lock.get();

            if *lock != mutex::raw(mutex) as *mut i32 {
                if *lock != ptr::null_mut() {
                    panic!("Condvar used with more than one Mutex");
                }

                atomic_cxchg(lock as *mut usize, 0, mutex::raw(mutex) as usize);
            }

            let now = Instant::now();

            let nanos = dur.as_secs()
                           .saturating_mul(1_000_000_000)
                           .saturating_add(dur.subsec_nanos() as u64);

            mutex.unlock();

            let arbiter = ::libctru::__sync_get_arbiter();

            ::libctru::svcArbitrateAddress(arbiter,
                                *self.lock.get() as u32,
                                ::libctru::ARBITRATION_WAIT_IF_LESS_THAN_TIMEOUT,
                                2,
                                nanos as i64);

            mutex.lock();

            now.elapsed() < dur
        }
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        *self.lock.get() = ptr::null_mut();
    }
}

#[cfg(target_arch = "aarch64")]
impl Condvar {
    pub const fn new() -> Condvar {
        Condvar {
            tag : 0,
            lock: UnsafeCell::new(ptr::null_mut()),
        }
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        self.tag = 0;
        *self.lock.get() = ptr::null_mut();
    }

    #[inline]
    pub fn notify_one(&self) {
        unsafe {
            //libnx::condvarWake(*self.lock.get(), 1);
            //LibNX's condvarWake gets inlined to this
            libnx::svcSignalProcessWideKey(*self.lock.get(), 1);
        }
    }

    #[inline]
    pub fn notify_all(&self) {
        unsafe {
            let lock = self.lock.get();

            if *lock == ptr::null_mut() {
                return;
            }

            //libnx::condvarWake(*self.lock.get(), -1);
            //LibNX's condvarWake gets inlined to this
            libnx::svcSignalProcessWideKey(*self.lock.get(), -1);

        }
    }

    #[inline]
    pub fn wait(&self, mutex: &Mutex) {
        self.wait_timeout(mutex, Duration::from_millis(u64::max_value()));
    }

    #[inline]
    pub fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        let dur_millis = (dur.as_secs() * 1000) + (dur.subsec_millis() as u64);
        unsafe {

            mutex.unlock();

            let nx_inner_mut = &mutex.inner;
            libnx::condvarWaitTimeout(*self.lock.get(), nx_inner_mut.get() as *mut _, dur_millis);

            mutex.lock();
        }
        true
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        *self.lock.get() = ptr::null_mut();
    }
}