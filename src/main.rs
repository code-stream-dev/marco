extern crate diesel;
extern crate futures;
extern crate tokio_io;
extern crate tokio_core;

use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

static OEB_IP: &'static str = "192.168.0.14";
static KAI_IP: &'static str = "192.168.0.37";

fn main() {
	let mut core = Core::new().expect("unable to create tokio core");
	let handle = core.handle();

	let addr = "0.0.0.0:1234".parse().unwrap();
	let tcp = TcpListener::bind(&addr, &handle).expect("unable to create tcp listener");

	let server = tcp.incoming().for_each(|(tcp, _)| {
		let (reader, writer) = tcp.split();

		let bytes_copied = io::copy(reader, writer);

		let handle_conn = bytes_copied.map(|(n, _, _)| {
            println!("wrote {} bytes", n)
        }).map_err(|err| {
            println!("IO error {:?}", err)
        });

        handle.spawn(handle_conn);

	    Ok(())
	});

	core.run(server).unwrap();
}
