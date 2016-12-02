#[cfg(feature = "serde_derive")]
include!("test_data.rs.in");

#[cfg(not(feature = "serde_derive"))]
include!(concat!(env!("OUT_DIR"), "/test_data.rs"));
