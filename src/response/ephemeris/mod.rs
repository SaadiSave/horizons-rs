use std::ops::{Deref, DerefMut};

pub struct Vector3D<T>([T; 3]);

#[repr(C)]
pub struct Vector3DIndex<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Deref for Vector3D<T> {
    type Target = Vector3DIndex<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.as_ptr().cast() }
    }
}

impl<T> DerefMut for Vector3D<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.as_mut_ptr().cast() }
    }
}
