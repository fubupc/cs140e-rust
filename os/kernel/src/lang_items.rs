use core::arch::asm;

use crate::console::kprint;

#[no_mangle]
#[lang = "panic_impl"]
pub extern "Rust" fn panic_impl(info: &core::panic::PanicInfo) -> ! {
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
    kprint!("\n{}\n", info.message());

    loop {
        unsafe { asm!("wfe") }
    }
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}
