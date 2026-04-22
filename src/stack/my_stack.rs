extern crate alloc;

use alloc::boxed::Box;
use core::mem::{self, MaybeUninit};
use core::ptr;
use std::fmt::Debug;

pub struct MyStack<T> {
    buf: Box<[MaybeUninit<T>]>,
    len: usize,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        return Self::with_capacity(0);
    }

    pub fn with_capacity(cap: usize) -> Self {
        let buf = stack_with_capacity(cap);
        return Self { buf, len: 0 };
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        unsafe {
            let dst = self.buf.as_mut_ptr().add(self.len);
            ptr::write(dst, MaybeUninit::new(value));
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe {
            let src = self.buf.as_ptr().add(self.len);
            return Some(ptr::read(src).assume_init());
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.capacity() == 0 {
            4
        } else {
            self.capacity() * 2
        };
        let mut new_buf = stack_with_capacity(new_cap);

        // move initialized elements
        for i in 0..self.len {
            unsafe {
                let src = self.buf.as_ptr().add(i);
                let dst = new_buf.as_mut_ptr().add(i);
                ptr::write(dst, ptr::read(src));
            }
        }

        // old buffer will be dropped (uninit part ignored)
        self.buf = new_buf;
    }
}

impl<T> Drop for MyStack<T> {
    fn drop(&mut self) {
        // drop initialized elements
        for i in 0..self.len {
            unsafe {
                let ptr = self.buf.as_mut_ptr().add(i);
                ptr::drop_in_place((*ptr).as_mut_ptr());
            }
        }
        // buffer deallocates via Box
    }
}

impl Debug for MyStack<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("[");
        for val in &self.buf {
            unsafe {
                out.push_str(format!("{:?}, ", val.assume_init()).as_str());
            }
        }
        out.pop();
        out.pop();
        out.push(']');
        f.write_str(format!("{}", out).as_str())
    }
}

fn stack_with_capacity<T>(cap: usize) -> Box<[MaybeUninit<T>]> {
    let buf = if cap == 0 {
        // empty slice
        Box::new([])
    } else {
        // allocate uninitialized buffer
        let v: Box<[MaybeUninit<T>]> = {
            let layout = core::alloc::Layout::array::<MaybeUninit<T>>(cap).unwrap();
            unsafe {
                let ptr = alloc::alloc::alloc(layout) as *mut MaybeUninit<T>;
                let slice = core::slice::from_raw_parts_mut(ptr, cap);
                Box::from_raw(slice)
            }
        };
        v
    };
    return buf;
}
