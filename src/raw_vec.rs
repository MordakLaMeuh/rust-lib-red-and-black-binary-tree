use libc::{free, malloc, realloc};
use std::marker::PhantomData;
use std::mem::{size_of, MaybeUninit};
use std::ptr::{copy, null_mut};

pub struct RawVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> RawVec<T> {
    pub fn new() -> Self {
        let ptr = unsafe { malloc(size_of::<T>()) } as *mut T;
        // assert!(ptr as usize & (size_of::<T>() - 1) == 0); // check align
        assert_ne!(ptr, null_mut());
        Self {
            ptr,
            len: 0,
            capacity: 1,
        }
    }
    pub fn push(&mut self, content: T) {
        if self.len != 0 && self.len & (self.len - 1) == 0 && self.len >= self.capacity {
            // println!("Realloc: {} to {}", self.len, self.len * 2);
            self.ptr =
                unsafe { realloc(self.ptr as *mut _, self.len * size_of::<T>() * 2) as *mut T };
            // assert!(self.ptr as usize & (size_of::<T>() - 1) == 0); // check align
            self.capacity = self.len * 2;
            assert_ne!(self.ptr, null_mut());
        }
        unsafe {
            *(self.ptr.add(self.len)) = content;
        }
        self.len += 1;
    }
    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.len, "request @{} with len {}", index, self.len);
        unsafe { self.ptr.add(index).as_ref().unwrap() }
    }
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.len, "request @{} with len {}", index, self.len);
        unsafe { self.ptr.add(index).as_mut().unwrap() }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "request @{} with len {}", index, self.len);
        unsafe {
            let mut t = MaybeUninit::<T>::uninit();
            copy::<T>(self.ptr.add(index), t.as_mut_ptr(), 1);
            if index != (self.len - 1) {
                copy::<T>(self.ptr.add(self.len - 1), self.ptr.add(index), 1);
            }
            self.len -= 1;
            t.assume_init()
        }
    }
    pub fn iter<'a>(&'a self) -> RawVecIterator<'a, T> {
        RawVecIterator {
            ptr: self.ptr,
            index: 0,
            len: self.len,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr as *mut _);
        }
    }
}

impl<T> std::ops::Index<usize> for RawVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        self.get(idx)
    }
}

impl<T> std::ops::IndexMut<usize> for RawVec<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.get_mut(idx)
    }
}

pub struct RawVecIterator<'a, T> {
    ptr: *mut T,
    index: usize,
    len: usize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for RawVecIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let output = unsafe { self.ptr.add(self.index).as_ref() };
            self.index += 1;
            output
        } else {
            None
        }
    }
}
