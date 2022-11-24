#![no_main]
#![no_std]

extern crate uefi;

use core::panic::PanicInfo;
use uefi::*;

#[no_mangle]
pub unsafe extern "C" fn efi_main(_image_handle:Handle,system_table:*const SystemTable)->Status{
	let putc=|c:char|{
		if !c.is_ascii() {
			unimplemented!();
		}
		((*(*system_table).con_out).output_string)((*system_table).con_out,&[c as u16,0]as*const u16);
	};
	let puts=|s:&str|{
		for c in s.chars(){
			putc(c);
		}
	};
	puts("Hello, World!\r\n");
	loop{}
}

#[panic_handler]
fn panic(_:&PanicInfo)->!{
	loop{}
}

