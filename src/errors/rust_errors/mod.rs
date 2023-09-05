#[cfg(feature = "no_std")]
pub mod core_lib;

#[cfg(not(feature = "no_std"))]
pub mod std_lib;