extern crate zmq;

fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send_str("Hello", 0).unwrap();
    assert!(socket.recv_string(0) == Ok(Ok("World".into())));

}
