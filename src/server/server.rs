extern crate greeter_example;

extern crate futures;
extern crate grpcio;
extern crate grpcio_proto;

#[macro_use]
extern crate log;

#[macro_use]
extern crate ferris_print;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use greeter_example::init_log;

use futures::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

//Create after running build script.
//These modules will be found in ./target/`MODE`/build/
use grpcio_proto::greeter::helloworld::{HelloReply, HelloRequest};
use grpcio_proto::greeter::helloworld_grpc::{self, Greeter};

//TODO comment the shit out of this
#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
        //make a message to return to the client.
        let msg = format!("Hello {}!", req.get_name());
        let age = req.get_age();

        //`sink` is expecting a `HelloReply` so let make one.
        let mut resp = HelloReply::new();
        resp.set_message(msg);
        resp.set_age(age + age);

        //give Sink the msg if else map any errors.
        let f = sink.success(resp)
            .map_err(move |e| error!("failed to replay {:?} {:?}", req, e));
        ctx.spawn(f)
    }
}
fn main() {
    let _guard = init_log(None); //Allow the info! marco to display message
    let env = Arc::new(Environment::new(1));
    let service = helloworld_grpc::create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("localhost", 50051)
        .build()
        .unwrap();
    server.start();

    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });

    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
