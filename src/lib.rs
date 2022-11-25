#![no_std]

pub type Handle=*const();
pub type Status=usize;

pub type UnusedPointer=*const();

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct MemoryDescriptor{
	pub t:u32,
	pub physical_start:u64,
	pub virtual_start:u64,
	pub number_of_pages:u64,
	pub attribute:u64,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct TableHeader{
	pub signature:u64,
	pub revision:u32,
	pub header_size:u32,
	pub crc32:u32,
	pub reserved:u32,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct SimpleTextOutputProtocol{
	pub _unused0:[UnusedPointer;1],
	pub output_string:extern "C" fn(*const Self,*const u16)->Status,
	pub _unused1:[UnusedPointer;8],
}

#[derive(Copy,Clone,Debug)]
#[repr(C)]
pub struct BootServices{
	pub hdr:TableHeader,
	pub _unused0:[UnusedPointer;4],
	pub get_memory_map:extern "C" fn(*mut usize,*mut MemoryDescriptor,*mut usize,*mut usize,*mut u32)->Status,
}

#[derive(Copy,Clone,Debug)]
#[repr(C)]
pub struct SystemTable{
	pub hdr:TableHeader,
	pub firmware_vendor:*const u16,
	pub firmware_revision:u32,
	pub _unused0:[UnusedPointer;3],
	pub con_out:*const SimpleTextOutputProtocol,
	pub _unused1:[UnusedPointer;3],
	pub boot_services:*const BootServices,
	pub number_of_table_entries:usize,
	pub _unused2:[UnusedPointer;1],
}

