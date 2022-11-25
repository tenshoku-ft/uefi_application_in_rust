#![no_main]
#![no_std]

extern crate uefi;

use core::mem::size_of_val;
use core::mem::transmute;
use core::panic::PanicInfo;
use uefi::*;

static mut MEMORY_MAP:&mut [u8]=&mut [0u8;0x4000];

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
	let bs=&(*(*system_table).boot_services);

	let mut memory_map_size=MEMORY_MAP.len();
	let mut map_key=0;
	let mut descriptor_size=0;
	let mut descriptor_version=0;
	let st=(bs.get_memory_map)(&mut memory_map_size,transmute(MEMORY_MAP.as_mut_ptr()),&mut map_key,&mut descriptor_size,&mut descriptor_version);
	puts(match st & (1<<(size_of_val(&st)*8-1)) {
		0=>{"OK"},
		_=>{"KO"},
	});
	puts("\r\n");
	loop{}
}

#[panic_handler]
fn panic(_:&PanicInfo)->!{
	loop{}
}

