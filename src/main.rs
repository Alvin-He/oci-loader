/////////////////////
/// CONFIG VALUES ///
/// /////////////////

// // NEGATIVE VALUES = undefined behavior
// const MEM_TARGET_PERCENTAGE: i64 = 20; // 20% usage
// const MEM_MAX_ALLOC_PER_UPDATE_BYTES: i64 = 100 * 1000 * 1000; // 100MB
// const MEM_MAX_HOLD_BYTES: i64 = 6 * 1000 * 1000 * 1000; // 6 gigs max  

// const MIN_BYTES_CHANGED_BEFORE_REALLOC_BYTES: i64 = 5 * 1000 * 1000; // 5 MB 

/// END CONFIG ///


// use std::thread; 
use tokio::time::{self, Duration};
use sysinfo::{System, SystemExt}; 
#[macro_use]
mod extera;
mod memory;
use crate::memory::{SmartMemoryHold}; 
mod config;

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


