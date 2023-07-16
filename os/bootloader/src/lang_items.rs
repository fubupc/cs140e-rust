use core::{alloc::GlobalAlloc, arch::asm};

use crate::console::kprint;

#[no_mangle]
#[lang = "panic_impl"]
pub extern "C" fn panic_impl(info: &core::panic::PanicInfo) -> ! {
    let header = r#"---------- PANIC ----------"#;
    kprint!("{}", header);
    if let Some(loc) = info.location() {
        kprint!("\nFILE: {}\n", loc.file());
        kprint!("LINE: {}\n", loc.line());
        kprint!("COL: {}\n", loc.column());
    } else {
        kprint!("\npanic occurred but can't get location information...\n");
    }
    if let Some(message) = info.message() {
        kprint!("\n{}\n", message);
    } else if let Some(payload) = info.payload().downcast_ref::<&'static str>() {
        kprint!("\n{}\n", payload);
    }

    loop {
        unsafe { asm!("wfe") }
    }
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
