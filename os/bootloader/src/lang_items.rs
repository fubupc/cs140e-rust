use core::alloc::GlobalAlloc;

#[no_mangle]
#[lang = "panic_impl"]
pub extern "C" fn panic_impl(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static DUMMY_ALLOCATOR: DummyAllocator = DummyAllocator;

struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _: core::alloc::Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {
        unimplemented!()
    }
}
