//                         ______(pull) worker (push) ______
//                        |                                 |
// task dispatch(push) ---|------(pull) worker (push) ------|------(pull) sink
//                        | _____(poll) worker (push) ______|

extern crate zmq;
extern crate rand;

use std::thread;
use std::time;

const TASK_DISPATCH: &'static str = "tcp://127.0.0.1:1234";
const RESULT_SINK: &'static str = "tcp://127.0.0.1:1235";

fn task_dispatch() {
    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::PUSH).unwrap();
    socket.bind(TASK_DISPATCH).unwrap();

    thread::sleep_ms(1000); // wait all workers be ready

    socket.send_str("0", 0).unwrap(); // 发送一个空的task，主要是让sink端好开始计时。

    let mut total_msec = 0;
    for i in 0..100 {
        let workload: u32 = rand::random::<u32>() % 100 + 1;
        total_msec += workload;
        socket.send_str(&format!("{}", workload), 0).unwrap();
    }
    println!("Total expected cost: {:?} msec", total_msec);

}

fn workers() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PULL).unwrap();
    socket.connect(TASK_DISPATCH).unwrap();

    let sink = ctx.socket(zmq::PUSH).unwrap();
    sink.connect(RESULT_SINK).unwrap();

    loop {
        let workload: u32 = socket.recv_string(0).unwrap().unwrap().parse().unwrap();

        thread::sleep_ms(workload);

        sink.send_str(&format!("{}", workload), 0).unwrap();

    }
}

fn sink() {
    let ctx = zmq::Context::new();

    let sink = ctx.socket(zmq::PULL).unwrap();
    sink.bind(RESULT_SINK).unwrap();

    sink.recv_string(0);

    let timestart = time::Instant::now();

    for i in 0..100 {
        let workload: u32 = sink.recv_string(0).unwrap().unwrap().parse().unwrap();
    }

    let timeend = time::Instant::now();
    let dur = timeend.duration_since(timestart);
    println!(
        "Actual take {:?} ms",
        dur.as_secs() * 1000 + (dur.subsec_nanos() / 1_000_000) as u64
    );
}


fn main() {
    thread::spawn(|| { workers(); });
    thread::spawn(|| { workers(); });
    thread::spawn(|| { workers(); });
    thread::spawn(|| { workers(); });

    thread::spawn(task_dispatch);

    sink();

    thread::sleep_ms(1000);
}
