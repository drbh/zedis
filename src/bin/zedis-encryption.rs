use sodiumoxide::crypto::aead;
use std::env;

fn execute(socket: &zmq::Socket, cmd: String) -> String {
    let mut msg = zmq::Message::new();
    socket.send(&cmd, 0).unwrap();
    socket.recv(&mut msg, 0).unwrap();
    let msg_text = msg.as_str().unwrap();
    // println!("{}", msg_text);
    msg_text.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut port = "5555";
    if args.len() > 1 {
        port = &args[1];
    };
    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::REQ).unwrap();
    let address = format!("tcp://127.0.0.1:{}", port);
    socket.connect(&address).unwrap();

    let k = aead::gen_key();
    let n = aead::gen_nonce();
    let m = b"david holtz";
    let ad = b"Some additional data";

    let c = aead::seal(m, Some(ad), &n, &k);
    let m2 = aead::open(&c, Some(ad), &n, &k).unwrap();

    let y = String::from_utf8(m2.clone()).unwrap();

    execute(&socket, y.clone());

    println!("{:?}", y);
}
