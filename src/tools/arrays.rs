use std::mem::MaybeUninit;

pub fn transform_array<T, U, F: FnMut(T) -> U, const N: usize>(
    source: [T; N],
    mut mapper: F,
) -> [U; N] {
    unsafe {
        let mut result = MaybeUninit::uninit();
        let start = result.as_mut_ptr() as *mut U;

        for (pos, item) in IntoIterator::into_iter(source).enumerate() {
            // SAFETY: safe because loop ensures `start.add(pos)`
            //         is always on an array element, of type U
            start.add(pos).write(mapper(item));
        }

        // SAFETY: safe because loop ensures entire array
        //         has been manually initialised
        result.assume_init()
    }
}

pub fn build_array<T, F: FnMut() -> T, const N: usize>(mut builder: F) -> [T; N] {
    unsafe {
        let mut result = MaybeUninit::uninit();
        let start = result.as_mut_ptr() as *mut T;

        for pos in 0..N {
            // SAFETY: safe because loop ensures `start.add(pos)`
            //         is always on an array element, of type T
            start.add(pos).write(builder());
        }

        // SAFETY: safe because loop ensures entire array
        //         has been manually initialised
        result.assume_init()
    }
}
