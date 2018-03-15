extern crate greeter_example;

extern crate grpcio;
extern crate grpcio_proto;

#[macro_use]
extern crate ferris_print;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use grpcio_proto::greeter::helloworld::HelloRequest;
use grpcio_proto::greeter::helloworld_grpc::GreeterClient;

fn main() {
    let client = build_client("localhost", 50051);

    let mut req = HelloRequest::new();
    req.set_name("World".to_owned());
    req.set_age(12 as i32);

    let reply = client.say_hello(&req).expect("rpc");
    ferrisprint!(
        "{} in {} year will be {}",
        reply.get_message(),
        reply.get_age(),
        reply.get_age() * 2
    );
}

fn build_client(host: &str, port: u32) -> GreeterClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&format!("{}:{}", host, port));
    GreeterClient::new(ch)
}
