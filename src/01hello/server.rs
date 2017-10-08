extern crate zmq;

fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REP).unwrap();
    socket.bind("tcp://127.0.0.1:1234").unwrap();

    loop {
        let msg = match socket.recv_msg(0) {
            Ok(msg) => msg,
            Err(_) => break,
        };

        println!("recv msg {:?}", msg.as_str());

        assert!(msg.as_str() == Some("Hello"));

        socket.send_str("World", 0).unwrap();
    }

}
