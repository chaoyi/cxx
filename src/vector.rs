pub trait VecOps<T> {
    fn get_unchecked(v: &Vector<T>, pos: usize) -> &T
    where
        Self: Sized;
    fn vector_length(v: &Vector<T>) -> usize
    where
        Self: Sized;
}

/// Binding to C++ `std::vector`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `Vector` by value. C++'s vector
/// requires a move constructor and may hold internal pointers, which is not
/// compatible with Rust's move behavior. Instead in Rust code we will only ever
/// look at a Vector through a reference or smart pointer, as in `&Vector`
/// or `UniquePtr<Vector>`.
#[repr(C)]
pub struct Vector<T> {
    _private: [T; 0],
}

impl<T: VecOps<T>> Vector<T> {
    /// Returns the length of the vector in bytes.
    ///
    /// Matches the behavior of C++ [std::vector::size][size].
    ///
    /// [size]: https://en.cppreference.com/w/cpp/vector/basic_vector/size
    pub fn len(&self) -> usize {
        T::vector_length(self)
    }

    pub fn get_unchecked(&self, pos: usize) -> &T {
        T::get_unchecked(self, pos)
    }

    /// Returns true if `self` has a length of zero bytes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, pos: usize) -> Option<&T> {
        if pos < self.len() {
            Some(self.get_unchecked(pos))
        } else {
            None
        }
    }
}

pub struct VectorIntoIterator<'a, T> {
    v: &'a Vector<T>,
    index: usize,
}

impl<'a, T: VecOps<T>> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = VectorIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        VectorIntoIterator { v: self, index: 0 }
    }
}

impl<'a, T: VecOps<T>> Iterator for VectorIntoIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.index + 1;
        self.v.get(self.index - 1)
    }
}

// Attempted to put this in expand_vector() but ran into trait impl problems
impl VecOps<u8> for u8 {
    fn get_unchecked(v: &Vector<u8>, pos: usize) -> &u8 {
        unsafe {
            extern "C" {
                #[link_name = "cxxbridge01$std$vector$u8$get_unchecked"]
                fn __get_unchecked(_: &Vector<u8>, _: usize) -> &u8;
            }
            __get_unchecked(v, pos)
        }
    }
    fn vector_length(v: &Vector<u8>) -> usize {
        unsafe {
            extern "C" {
                #[link_name = "cxxbridge01$std$vector$u8$length"]
                fn __vector_length(_: &Vector<u8>) -> usize;
            }
            __vector_length(v)
        }
    }
}
