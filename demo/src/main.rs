#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    // Shared structs with fields visible to both languages.
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type MultiBuf;

        unsafe fn next_chunk<'a>(buf: &'a mut MultiBuf, func: &MyFunction, func2: &MyFunction2) -> &'a [u8];
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("demo/include/blobstore.h");

        type BlobstoreClient;
        type MyFunction;
        type MyFunction2;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(self: &BlobstoreClient, parts: &mut MultiBuf) -> u64;
        fn tag(self: &BlobstoreClient, blobid: u64, tag: &str);
        fn metadata(self: &BlobstoreClient, blobid: u64) -> BlobMetadata;

        #[cxx_name="operatorINVOKE"]
        fn call(self: &MyFunction);
        #[cxx_name="operatorINVOKE"]
        fn call(self: &MyFunction2, param: i32);
    }
}

// An iterator over contiguous chunks of a discontiguous file object.
//
// Toy implementation uses a Vec<Vec<u8>> but in reality this might be iterating
// over some more complex Rust data structure like a rope, or maybe loading
// chunks lazily from somewhere.
pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}
pub fn next_chunk<'a>(buf: &'a mut MultiBuf, function: &ffi::MyFunction, fn2: &ffi::MyFunction2) -> &'a [u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    function.call();
    fn2.call(3);
    next.map_or(&[], Vec::as_slice)
}

fn main() {
    let client = ffi::new_blobstore_client();

    // Upload a blob.
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {}", blobid);

    // Add a tag.
    client.tag(blobid, "rust");

    // Read back the tags.
    let metadata = client.metadata(blobid);
    println!("tags = {:?}", metadata.tags);
}
