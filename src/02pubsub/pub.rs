extern crate zmq;
extern crate rand;

fn main() {
    let ctx = zmq::Context::new();
    let publisher = ctx.socket(zmq::PUB).unwrap();

    publisher.bind("tcp://*:5556").unwrap();
    // publisher.bind("ipc://weather.ipc").unwrap();

    loop {
        let zipcode = rand::random::<u32>() % 200;
        let temp = (rand::random::<u32>() % 215) as i32 - 80;
        let relhumidity = rand::random::<u32>() % 50 + 10;

        publisher
            .send_str(&format!("{} {} {}", zipcode, temp, relhumidity), 0)
            .unwrap();

    }

}
