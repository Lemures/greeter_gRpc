extern crate futures;
extern crate grpcio;
extern crate protobuf;

//NOTE UMBER IMPORTANT!!!! must include the mod that are built.
pub mod greeter {
    include!(concat!(env!("OUT_DIR"), "/greeter/mod.rs"));
}
