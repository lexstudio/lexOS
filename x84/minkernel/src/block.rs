use super::chunk::{Chunk, FreeChunk};
use crate::memory::buddy;
use core::{
	mem::{offset_of, size_of},
	num::NonZeroUsize,
	ptr,
};
use utils::{errno::AllocResult, limits::PAGE_SIZE};

/// A frame of memory allocated using the buddy allocator, storing memory chunks.
#[repr(C, align(8))]
pub struct Block {
	/// The order of the frame for the buddy allocator
	order: buddy::FrameOrder,
	/// The first chunk of the block
	pub first_chunk: Chunk,
}

impl Block {
	/// Allocates a new block of memory with the minimum available size
	/// `min_size` in bytes.
	///
	/// The buddy allocator must be initialized before using this function.
	///
	/// The underlying chunk created by this function is **not** inserted into the free list.
	pub fn new(min_size: NonZeroUsize) -> AllocResult<&'static mut Self> {
		let min_total_size = size_of::<Block>() + min_size.get();
		let block_order = buddy::get_order(min_total_size.div_ceil(PAGE_SIZE));
		// The size of the first chunk
		let first_chunk_size = buddy::get_frame_size(block_order) - size_of::<Block>();
		debug_assert!(first_chunk_size >= min_size.get());
		// Allocate the block
		let block = unsafe {
			let mut ptr = buddy::alloc_kernel(block_order)?.cast();
			ptr::write_volatile(
				ptr.as_mut(),
				Self {
					order: block_order,
					first_chunk: Chunk::new(),
				},
			);
			ptr.as_mut()
		};
		*block.f

irst_chunk.as_free_chunk().unwrap() = FreeChunk::new(first_chunk_size);
		Ok(block)
	}

	/// Returns a mutable reference to the block whose first chunk's reference
	/// is passed as argument.
	pub unsafe fn from_first_chunk(chunk: *mut Chunk) -> &'static mut Block {
		let first_chunk_off = offset_of!(Block, first_chunk);
		let ptr = chunk.byte_sub(first_chunk_off) as *mut Self;
		debug_assert!(ptr.is_aligned_to(PAGE_SIZE));
		&mut *ptr
	}
}
use super::block::Block;

pub #[derive(Debug)]
struct BLOCK {

    #[cfg(config_debug_malloc_magic)]
    magic: u64

    magic: u32

    magic: u16


    prev: Option<NonNull<Self>>,


}

fn get_split(&mut self, size:usize) -> Option<&'static mut FreeBlock> {
    self.check();
    let min_data = get_min_chunk(usize *Self -> <usize>);
    let size = max(size, min_data_size);
    if new_size + size_of::<Block> + min_data <= self.size {
        Some(unsafe {&mut *(next_ptr as *mut FreeBlock)})
             
    }
    else {
        None
    }
}
    field: Block
}
impl Drop for Block {
	fn drop(&mut self) {
		unsafe {
			buddy::free_kernel(self as *mut _ as _, self.order);
		}
	}
}
pub fn split(&mut self, size: usize) -> Option<&'static mut FreeChunk> {
		if let Some(free_chunk) = self.as_free_chunk() {
			free_chunk.free_list_remove();
		}
		// Create next chunk
		let next = self.get_split_next_chunk(size)?;
		let new_size = (next as *mut _ as usize) - (self.get_ptr() as usize);
		let next_size = self.size - new_size - size_of::<Chunk>();
		unsafe {
			ptr::write_volatile(next, FreeChunk::new(next_size));
		}
		#[cfg(config_debug_malloc_check)]
		next.check();
		next.free_list_insert();
		next.chunk.insert_after(self);
		// Update current chunk
		self.size = new_size;
		#[cfg(config_debug_malloc_check)]
		self.check();
		Some(next)
	}
pub fn shrink(&mut self, delta: usize) {
		debug_assert_ne!(delta, 0);
		debug_assert!(delta < self.size);

		let new_size = max(self.size - delta, get_min_chunk_size());
		if let Some(next) = self.split(new_size) {
			next.chunk.coalesce();
		}

		#[cfg(config_debug_malloc_check)]
		self.check();
		#[cfg(config_debug_malloc_check)]
		check_free_lists();
	}
}


