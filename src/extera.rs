#[macro_export]
macro_rules! println {
    ($($rest:tt)*) => {
        #[cfg(feature = "logging")]
        std::println!($($rest)*)
    }
}

// pub struct Config { // config structure for the loader
//     // NEGATIVE VALUES = undefined behavior
//     mem_target_percentage: i64, // = 20; // 20% usage
//     mem_max_alloc_per_update_bytes: i64, // = 100 * 1000 * 1000; // 100MB
//     mem_max_hold_bytes: i64, //= 6 * 1000 * 1000 * 1000; // 6 gigs max  
//     min_bytes_changed_before_realloc_bytes: i64 //= 5 * 1000 * 1000; // 5 MB 
// }
