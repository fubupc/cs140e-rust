
use core::arch::asm;

use crate::console::kprint;

#[no_mangle]
#[lang = "panic_impl"]
pub extern "C" fn panic_impl(info: &core::panic::PanicInfo) -> ! {
    // FIXME: Print `fmt`, `file`, and `line` to the console.
    let header = r#"            (
        (      )     )
          )   (    (
         (          `
     .-""^"""^""^"""^""-.
   (//\\//\\//\\//\\//\\//)
    ~\^^^^^^^^^^^^^^^^^^/~
      `================`
 
     The pi is overdone.
 
 ---------- PANIC ----------
 "#;
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

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}
