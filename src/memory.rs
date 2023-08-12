use sysinfo::{System, SystemExt}; 
use std::alloc::{self, Layout};

use crate::config::*;
// use crate::extera;

pub struct SmartMemoryHold {
    layout: Layout, 
    mem_ptr: *mut u8, 
}


impl SmartMemoryHold {
    pub fn initialize() -> SmartMemoryHold {
        let layout: Layout = Layout::from_size_align(16, 32).unwrap();
        let new_ptr: *mut u8 = unsafe { alloc::alloc(layout) };

        if new_ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        return SmartMemoryHold {
            layout: layout,
            mem_ptr: new_ptr
        };
    }
    pub fn update_hold(&mut self, system: &System) {
        let mem_available: i64 = system.available_memory() as i64;
        // 100000000 bytes = 100 MB
        if mem_available < 100000000 {
            return; // hard core memory limit, don't let the system run out of memory 
        }

        // check if we need to adjust the memory allocation
        let memory_used_percent: f64 = (system.used_memory() as f64) / (system.total_memory() as f64);
        let percentage_changed: f64 = MEM_TARGET_PERCENTAGE - memory_used_percent;

        // converting to integer percentage as we don't want the precision of float
        if percentage_changed.abs() <= MEM_MIN_CHANGED_BEFORE_REALLOC_PERCENTAGE {
            println!("Current System Memory Deviation: {}, less than the reallocation threshold, no change", percentage_changed);
            return; 
        } 

        let amount_targeted: f64 = percentage_changed * (system.available_memory() as f64);

        let mut new_allocation_amount: i64 = if (amount_targeted as i64).abs() > MEM_MAX_ALLOC_PER_UPDATE_BYTES {
                if amount_targeted.is_sign_negative() { -MEM_MAX_ALLOC_PER_UPDATE_BYTES } else { MEM_MAX_ALLOC_PER_UPDATE_BYTES }
            } else {
                amount_targeted as i64
            };
        
        // subtract one byte off as that's needed to keep the buffer existing 
        let current_holding_amount: i64 = self.layout.size() as i64 - 1; 
        let new_holding_amount: i64 = current_holding_amount + new_allocation_amount;
        if new_holding_amount <= 0 {
            new_allocation_amount = -current_holding_amount;
        }

        // println!("Change to be applied: {}", new_allocation_amount);

        self.reallocate_mem_buffer(new_allocation_amount); 
        // adjust accordingly 

        println!("Current System Memory Usage: {}, Current Memory Held: {} MB, Next Target Amount: {} MB", memory_used_percent, current_holding_amount / 1000 / 1000, new_allocation_amount / 1000 / 1000); 
    }
    
    fn reallocate_mem_buffer(&mut self, amount_changed_bytes: i64) {

        let mut target_size = self.layout.size() as i64 + amount_changed_bytes; 
        if target_size <= 0 { target_size = 1;} // just move to the smallest possible allocation amount if it wants to allocate non-existence 
        let new_layout = alloc::Layout::from_size_align(target_size as usize, 32).unwrap();
        
        let new_ptr = unsafe { alloc::realloc(self.mem_ptr, self.layout, new_layout.size()) }; 

        if new_ptr.is_null() { // panic on failure to allocate
            alloc::handle_alloc_error(new_layout);
        }
        
        self.layout = new_layout; 
        self.mem_ptr = new_ptr; 
    }
}

