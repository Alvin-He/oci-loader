// NEGATIVE VALUES = undefined behavior
pub const MEM_TARGET_PERCENTAGE: f64 = 0.6; // 20% usage
pub const MEM_MAX_ALLOC_PER_UPDATE_BYTES: i64 = 100 * 1000 * 1000; // 100MB
// pub const MEM_MAX_HOLD_BYTES: i64 = 6 * 1000 * 1000 * 1000; // 6 gigs max  

pub const MIN_PERCENT_CHANGED_BEFORE_REALLOC: f64 = 0.01; // ±1% buffer

