pub trait VectorTarget<T> {
    fn get_unchecked(v: &RealVector<T>, pos: usize) -> &T
    where
        Self: Sized;
    fn vector_length(v: &RealVector<T>) -> usize
    where
        Self: Sized;
    fn push_back(v: &RealVector<T>, item: &T)
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
pub struct RealVector<T> {
    _private: [T; 0],
}

impl<T: VectorTarget<T>> RealVector<T> {
    /// Returns the length of the vector in bytes.
    pub fn size(&self) -> usize {
        T::vector_length(self)
    }

    pub fn get_unchecked(&self, pos: usize) -> &T {
        T::get_unchecked(self, pos)
    }

    /// Returns true if `self` has a length of zero bytes.
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn get(&self, pos: usize) -> Option<&T> {
        if pos < self.size() {
            Some(self.get_unchecked(pos))
        } else {
            None
        }
    }

    pub fn push_back(&mut self, item: &T) {
        T::push_back(self, item);
    }
}

pub struct VectorIntoIterator<'a, T> {
    v: &'a RealVector<T>,
    index: usize,
}

impl<'a, T: VectorTarget<T>> IntoIterator for &'a RealVector<T> {
    type Item = &'a T;
    type IntoIter = VectorIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        VectorIntoIterator { v: self, index: 0 }
    }
}

impl<'a, T: VectorTarget<T>> Iterator for VectorIntoIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.index + 1;
        self.v.get(self.index - 1)
    }
}

// Attempted to put this in expand_vector() but ran into trait impl problems
impl VectorTarget<u8> for u8 {
    fn get_unchecked(v: &RealVector<u8>, pos: usize) -> &u8 {
        unsafe {
            extern "C" {
                #[link_name = "cxxbridge01$std$vector$u8$get_unchecked"]
                fn __get_unchecked(_: &RealVector<u8>, _: usize) -> &u8;
            }
            __get_unchecked(v, pos)
        }
    }
    fn vector_length(v: &RealVector<u8>) -> usize {
        unsafe {
            extern "C" {
                #[link_name = "cxxbridge01$std$vector$u8$length"]
                fn __vector_length(_: &RealVector<u8>) -> usize;
            }
            __vector_length(v)
        }
    }
    fn push_back(v: &RealVector<u8>, item: &u8) {
        unsafe {
            extern "C" {
                #[link_name = "cxxbridge01$std$vector$u8$push_back"]
                fn __push_back(_: &RealVector<u8>, _: &u8) -> usize;
            }
            __push_back(v, item);
        }
    }
}
