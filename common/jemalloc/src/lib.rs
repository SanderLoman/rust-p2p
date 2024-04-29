#[cfg(feature = "jemalloc")]
extern crate jemallocator;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
