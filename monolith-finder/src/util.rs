use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::{mem, ptr};

pub struct DerefSliceArray<'a, T, const I: usize>(pub &'a mut [T; I]);

impl<'a, T, const I: usize> Deref for DerefSliceArray<'a, T, I> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T, const I: usize> DerefMut for DerefSliceArray<'a, T, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct DerefSliceArrayVal<T, const I: usize>(pub [T; I]);

impl<T, const I: usize> Deref for DerefSliceArrayVal<T, I> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const I: usize> DerefMut for DerefSliceArrayVal<T, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn generate_array<T, const I: usize>(mut make_element: impl FnMut() -> T) -> [T; I] {
    // SAFETY: This is the "Initializing an array element-by-element" example from the MaybeUninit docs, which is supposedly safe.
    let mut data: [MaybeUninit<T>; I] = unsafe { MaybeUninit::uninit().assume_init() };
    for elem in &mut data[..] {
        unsafe {
            ptr::write(elem.as_mut_ptr(), make_element());
        }
    }
    // Oh god, I can't use transmute here because I isn't fixed.
    unsafe {
        let result: [T; I] = mem::transmute_copy(&data);
        mem::forget(data);
        result
    }
}
