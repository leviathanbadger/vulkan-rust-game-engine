

pub trait IntoBufferData<T> {
    fn element_count(&self) -> usize;
    fn as_buffer_ptr(&self) -> *const T;
}

impl<T: Sized> IntoBufferData<T> for Vec<T> {
    fn element_count(&self) -> usize {
        self.len()
    }

    fn as_buffer_ptr(&self) -> *const T {
        self.as_ptr()
    }
}

impl<T: Sized, const SIZE: usize> IntoBufferData<T> for [T; SIZE] {
    fn element_count(&self) -> usize {
        SIZE
    }

    fn as_buffer_ptr(&self) -> *const T {
        self.as_ptr()
    }
}

impl<T: Sized> IntoBufferData<T> for T {
    fn element_count(&self) -> usize {
        1
    }

    fn as_buffer_ptr(&self) -> *const T {
        self
    }
}
