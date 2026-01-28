#![no_std]

use core::ptr::{ self };
use core::mem::{ self };
use core::slice::{ self };
use core::sync::atomic::{ Ordering };

#[cfg(target_has_atomic = "8")]
use core::sync::atomic::{ AtomicU8 };

#[cfg(target_has_atomic = "16")]
use core::sync::atomic::{ AtomicU16 };

#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{ AtomicU32 };

#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{ AtomicU64 };

pub struct ByteBuf<'a> {
    buf: &'a mut [u8],
}

impl ByteBuf<'_> {
    pub fn new(addr: usize, len: usize) -> Self {
        Self {
            buf: unsafe {
                slice::from_raw_parts_mut(ptr::with_exposed_provenance_mut(addr), len)
            },
        }
    }

    pub fn addr(&self) -> usize {
        self.buf.as_ptr().addr()
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn copy_to(&mut self, off: usize, buf: &mut [u8]) {
        buf.copy_from_slice(&self.buf[off..]);
    }

    pub fn copy_from(&mut self, off: usize, buf: &[u8]) {
        (&mut self.buf[off..]).copy_from_slice(buf);
    }

    fn off<T>(&self, off: usize) -> *const T {
        (&self.buf[off..off+mem::size_of::<T>()]).as_ptr().cast::<T>()
    }

    fn off_mut<T>(&mut self, off: usize) -> *mut T {
        (&mut self.buf[off..off+mem::size_of::<T>()]).as_mut_ptr().cast::<T>()
    }

    pub fn rd8(&self, off: usize) -> u8 {
        let addr = self.off::<u8>(off);
        unsafe { addr.read() }
    }

    pub fn rd8_volatile(&self, off: usize) -> u8 {
        let addr = self.off::<u8>(off);
        unsafe { addr.read_volatile() }
    }

    #[cfg(target_has_atomic = "8")]
    pub fn rd8_atomic(&mut self, off: usize) -> u8 {
        unsafe {
            let addr = AtomicU8::from_ptr(self.off_mut::<u8>(off));
            addr.load(Ordering::SeqCst)
        }
    }

    pub fn wr8(&mut self, off: usize, value: u8) {
        let addr = self.off_mut::<u8>(off);
        unsafe { addr.write(value); }
    }

    pub fn wr8_volatile(&mut self, off: usize, value: u8) {
        let addr = self.off_mut::<u8>(off);
        unsafe { addr.write_volatile(value); }
    }

    #[cfg(target_has_atomic = "8")]
    pub fn wr8_atomic(&mut self, off: usize, value: u8) {
        unsafe {
            let addr = AtomicU8::from_ptr(self.off_mut::<u8>(off));
            addr.store(value, Ordering::SeqCst)
        }
    }

    pub fn rd16(&self, off: usize) -> u16 {
        let addr = self.off::<u16>(off);
        unsafe { addr.read() }
    }

    pub fn rd16_volatile(&self, off: usize) -> u16 {
        let addr = self.off::<u16>(off);
        unsafe { addr.read_volatile() }
    }

    #[cfg(target_has_atomic = "16")]
    pub fn rd16_atomic(&mut self, off: usize) -> u16 {
        unsafe {
            let addr = AtomicU16::from_ptr(self.off_mut::<u16>(off));
            addr.load(Ordering::SeqCst)
        }
    }

    pub fn wr16(&mut self, off: usize, value: u16) {
        let addr = self.off_mut::<u16>(off);
        unsafe { addr.write(value); }
    }

    pub fn wr16_volatile(&mut self, off: usize, value: u16) {
        let addr = self.off_mut::<u16>(off);
        unsafe { addr.write_volatile(value); }
    }

    #[cfg(target_has_atomic = "16")]
    pub fn wr16_atomic(&mut self, off: usize, value: u16) {
        unsafe {
            let addr = AtomicU16::from_ptr(self.off_mut::<u16>(off));
            addr.store(value, Ordering::SeqCst)
        }
    }

    pub fn rd32(&self, off: usize) -> u32 {
        let addr = self.off::<u32>(off);
        unsafe { addr.read() }
    }

    pub fn rd32_volatile(&self, off: usize) -> u32 {
        let addr = self.off::<u32>(off);
        unsafe { addr.read_volatile() }
    }

    #[cfg(target_has_atomic = "32")]
    pub fn rd32_atomic(&mut self, off: usize) -> u32 {
        unsafe {
            let addr = AtomicU32::from_ptr(self.off_mut::<u32>(off));
            addr.load(Ordering::SeqCst)
        }
    }

    pub fn wr32(&mut self, off: usize, value: u32) {
        let addr = self.off_mut::<u32>(off);
        unsafe { addr.write(value); }
    }

    pub fn wr32_volatile(&mut self, off: usize, value: u32) {
        let addr = self.off_mut::<u32>(off);
        unsafe { addr.write_volatile(value); }
    }

    #[cfg(target_has_atomic = "32")]
    pub fn wr32_atomic(&mut self, off: usize, value: u32) {
        unsafe {
            let addr = AtomicU32::from_ptr(self.off_mut::<u32>(off));
            addr.store(value, Ordering::SeqCst)
        }
    }

    pub fn rd64(&self, off: usize) -> u64 {
        let addr = self.off::<u64>(off);
        unsafe { addr.read() }
    }

    pub fn rd64_volatile(&self, off: usize) -> u64 {
        let addr = self.off::<u64>(off);
        unsafe { addr.read_volatile() }
    }

    #[cfg(target_has_atomic = "64")]
    pub fn rd64_atomic(&mut self, off: usize) -> u64 {
        unsafe {
            let addr = AtomicU64::from_ptr(self.off_mut::<u64>(off));
            addr.load(Ordering::SeqCst)
        }
    }

    pub fn wr64(&mut self, off: usize, value: u64) {
        let addr = self.off_mut::<u64>(off);
        unsafe { addr.write(value); }
    }

    pub fn wr64_volatile(&mut self, off: usize, value: u64) {
        let addr = self.off_mut::<u64>(off);
        unsafe { addr.write_volatile(value); }
    }

    #[cfg(target_has_atomic = "64")]
    pub fn wr64_atomic(&mut self, off: usize, value: u64) {
        unsafe {
            let addr = AtomicU64::from_ptr(self.off_mut::<u64>(off));
            addr.store(value, Ordering::SeqCst)
        }
    }
}
