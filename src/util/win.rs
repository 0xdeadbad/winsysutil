use windows::Win32::System::Memory::{GetProcessHeap, HeapAlloc, HeapFree, HeapHandle, HEAP_FLAGS};
use std::ffi::c_void;

pub struct HeapCalloc {
    mem: *mut c_void,
    freed: bool,
}

impl Drop for HeapCalloc {
    fn drop(&mut self) {
        if !self.freed {
            heapfree(self.mem).unwrap();
        }
    }
}

impl HeapCalloc {
    pub fn alloc(size: usize) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let mem = heapalloc(size)?;
        Ok(Self { mem, freed: false })
    }

    pub fn free(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if !self.freed {
            heapfree(self.mem)?;
            self.freed = true;
        }

        Ok(())
    }

    pub fn get_mem(&self) -> std::result::Result<*mut c_void, Box<dyn std::error::Error>> {
        if !self.freed {
            Ok(self.mem)
        } else {
            Err("Memory already freed".into())
        }
    }

    pub fn alloc_again(&mut self, size: usize) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if !self.freed {
            self.free()?;
        }
        self.mem = heapalloc(size)?;
        self.freed = false;

        Ok(())
    }
}

#[inline]
fn heapalloc(size: usize) -> std::result::Result<*mut c_void, Box<dyn std::error::Error>> {
    let ph = unsafe { GetProcessHeap() }?;
    let ha = unsafe { HeapAlloc::<HeapHandle>(ph, HEAP_FLAGS(0), size) };

    for i in 0..size {
        unsafe { *(ha.offset(i as isize) as *mut u8) = 0 };
    }

    Ok(ha)
}

#[inline]
fn heapfree(memory: *mut c_void) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let ph = unsafe { GetProcessHeap() }?;
    let _ha = unsafe { HeapFree::<HeapHandle>(ph, HEAP_FLAGS(0), Some(memory)) };

    Ok(())
}