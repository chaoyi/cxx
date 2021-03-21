#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    unsafe extern "C++" {
        include!("demo/include/blobstore.h");

        fn make_string(str: &str) -> UniquePtr<CxxString>;

    }
}


fn main() {
    let _ = ffi::make_string("hello");
}
