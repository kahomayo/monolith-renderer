use std::ops::{Deref, DerefMut};

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

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
