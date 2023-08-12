// CONFIG FILE for oci-loader, read comments to see what each option does
// This is already tuned for the limits as of 08/12/2023, only edit if you know what you are doing  

// WARNING: NEGATIVE VALUES = undefined behavior
// Percentages GREATER THAN 1 will also cause a full memory leak / system memory exhaustion 

//////////////
/// Memory ///
//////////////

// The target percentage that this loader is trying to hit, decimal form percentages only
pub const MEM_TARGET_PERCENTAGE: f64 = 0.6; // 20% usage

// Max update size per cycle, used to spread out the load on the cpu due to allocations 
// increase to make the system more responsive (if you have applications that may very quickly spike in memory usage)
// increasing will also place more load to the system in a short period of time 
pub const MEM_MAX_ALLOC_PER_UPDATE_BYTES: i64 = 100 * 1000 * 1000; // 100MB

// How much the memory usage is allowed to deviate from MEM_TARGET_PERCENTAGE
// decreasing this too low could cause constant small allocations due to floating point math inaccuracies and could cause ±5% memory leaks  
pub const MIN_PERCENT_CHANGED_BEFORE_REALLOC: f64 = 0.01; // ±1% buffer

