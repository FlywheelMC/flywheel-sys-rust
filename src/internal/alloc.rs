use std::alloc::{ Layout, alloc, handle_alloc_error };


#[unsafe(no_mangle)]
pub fn flywheel_alloc(len : u32, align : u32) -> u32 {
    let layout = unsafe { Layout::from_size_align_unchecked(len as usize, align as usize) };
    let ptr    = unsafe { alloc(layout) };
    if (ptr.is_null()) {
        handle_alloc_error(layout);
    }
    ptr as u32
}
