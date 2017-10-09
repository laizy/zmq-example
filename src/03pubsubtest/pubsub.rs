extern crate zmq;

use std::thread;

const ADDR: &'static str = "tcp://127.0.0.1:5556";
const FILTER: &'static str = "msglosttest ";
const N: u32 = 10;

fn start_client() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();

    socket.connect(ADDR).unwrap();
    socket.set_subscribe(FILTER.as_bytes()).unwrap();


    let mut n = 0;
    // 服务端退出后recv_string会阻塞，后面服务端再次启动时可以继续接收。
    while let Ok(Ok(_string)) = socket.recv_string(0) {
        n += 1;
        println!("{:?} {}", n, _string);
    }


    println!("{:?}", n);

}

fn start_server() {
    let mut ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PUB).unwrap();
    socket.bind(ADDR).unwrap();

    for i in 0..N {
        socket.send_str(&format!("{} {}", FILTER, i), 0).unwrap();
        thread::sleep_ms(80);
    }

    println!("exit server");

}

fn main() {
    let handle = thread::spawn(|| { start_client(); });

    start_server();

    handle.join();
}
