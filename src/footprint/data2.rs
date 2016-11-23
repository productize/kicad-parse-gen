#[cfg(feature = "serde_derive")]
include!("data2.rs.in");

#[cfg(not(feature = "serde_derive"))]
include!(concat!(env!("OUT_DIR"), "footprint/data2.rs"));
