/////////////////////
/// CONFIG VALUES ///
/// /////////////////

// NEGATIVE VALUES = undefined behavior
const MEM_TARGET_PERCENTAGE: i64 = 20; // 20% usage
const MEM_MAX_ALLOC_PER_UPDATE_BYTES: i64 = 100 * 1000 * 1000; // 100MB
const MEM_MAX_HOLD_BYTES: i64 = 6 * 1000 * 1000 * 1000; // 6 gigs max  

const MIN_BYTES_CHANGED_BEFORE_REALLOC_BYTES: i64 = 5 * 1000 * 1000; // 5 MB 

/// END CONFIG ///


// use std::thread; 
use tokio::time::{self, Duration};
use sysinfo::{System, SystemExt}; 
use std::alloc::{self, Layout};


macro_rules! println {
    ($($rest:tt)*) => {
        #[cfg(feature = "logging")]
        std::println!($($rest)*)
    }
}

#[tokio::main]
async fn main() {
    std::println!("Constant loader (memory) for mitigating oci instance reclamation ©️Alvin He 2023");

    let update_interval_time_secs:u64 = 5;
    let mut system = System::new_all(); 
    let mut mem_holder = SmartMemoryHold::initialize(); 
    loop {
        system.refresh_all(); 
        mem_holder.update_hold(&system);  

        time::sleep(Duration::from_secs(update_interval_time_secs)).await;
    }; 
}

struct SmartMemoryHold {
        layout: Layout, 
        mem_ptr: *mut u8, 
    }

impl SmartMemoryHold {
    fn initialize() -> SmartMemoryHold {
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
    fn update_hold(&mut self, system: &System) {
        let mem_available: i64 = system.available_memory() as i64;
        // 100000000 bytes = 100 MB
        if mem_available < 100000000 {
            return; // hard core memory limit, don't let the system run out of memory 
        }

        // check if we need to adjust the memory allocation
        let memory_used_percent: i64 = ((system.used_memory() as f64) / (system.total_memory() as f64) * 100.0) as i64;
        // println!("Memory % currently used: {}", memory_used_percent); 
        let mut new_allocation_amount: i64;
        if memory_used_percent < MEM_TARGET_PERCENTAGE {
            let amount_targeted: f64 = ((MEM_TARGET_PERCENTAGE - memory_used_percent) as f64) * (1.0/100.0) * (system.available_memory() as f64);
            new_allocation_amount = if (amount_targeted as i64).abs() < MEM_MAX_ALLOC_PER_UPDATE_BYTES { amount_targeted as i64  } else { MEM_MAX_ALLOC_PER_UPDATE_BYTES };
        } else if memory_used_percent > MEM_TARGET_PERCENTAGE {
            let amount_targeted: f64 = ((memory_used_percent - MEM_TARGET_PERCENTAGE) as f64) * (1.0/100.0) * (system.available_memory() as f64);
            new_allocation_amount = if (amount_targeted as i64).abs() < MEM_MAX_ALLOC_PER_UPDATE_BYTES { amount_targeted as i64  } else { -MEM_MAX_ALLOC_PER_UPDATE_BYTES }; // default is negated as amount_targeted is negative 
        } else {
            println!("Current System Memory Usage equals to: {}, No change", MEM_TARGET_PERCENTAGE);
            return; 
        } 


        let current_holding_amount: i64 = self.layout.size() as i64; 
        let new_holding_amount: i64 = current_holding_amount + new_allocation_amount;
        if new_holding_amount >= MEM_MAX_HOLD_BYTES {
            new_allocation_amount = new_allocation_amount - (new_holding_amount - MEM_MAX_HOLD_BYTES); 
        } else if new_holding_amount <= 0 {
            new_allocation_amount = -current_holding_amount
        }

        if new_allocation_amount.abs() < MIN_BYTES_CHANGED_BEFORE_REALLOC_BYTES {
            println!("Memory changed too small to satisfy a reallocation");
            return;
        }

        // println!("Change to be applied: {}", new_allocation_amount);

        self.reallocate_mem_buffer(new_allocation_amount); 
        // adjust accordingly 

        println!("Current System Memory Usage: {}, Current Memory Held: {} MB, Next Target Amount: {} MB", memory_used_percent, (current_holding_amount as f64) / 1000.0 / 1000.0, (new_allocation_amount as f64) / 1000.0 / 1000.0); 
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


