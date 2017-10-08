extern crate zmq;

fn main() {
    let ctx = zmq::Context::new();
    let sub = ctx.socket(zmq::SUB).unwrap();

    sub.connect("tcp://localhost:5556").unwrap();
    let filter = "101 ";
    sub.set_subscribe(filter.as_bytes()).unwrap();

    let mut total_temp = 0;
    for i in 0..100 {
        let string = sub.recv_string(0).unwrap().unwrap();
        println!("recv msg {:?}", string);
        let temp = string.split_whitespace().skip(1).next().unwrap();

        total_temp += temp.parse().unwrap();
    }
    println!(
        "average temp for zipcode {:?} was {}",
        filter,
        total_temp / 100
    );
}
