use colored::*;
use std::env;

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate ring;

use ring::aead::*;
use ring::pbkdf2::*;
use ring::rand::SystemRandom;

fn execute(socket: &zmq::Socket, cmd: String) -> String {
    let mut msg = zmq::Message::new();
    socket.send(&cmd, 0).unwrap();
    socket.recv(&mut msg, 0).unwrap();
    let msg_text = msg.as_str().unwrap();
    // println!("{}", msg_text);
    msg_text
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

    let res = execute(&socket, "keys");

    // The password will be used to generate a key
    let password = b"nice password";

    // Usually the salt has some random data and something that relates to the user
    // like an username
    let salt = [0, 1, 2, 3, 4, 5, 6, 7];

    // Keys are sent as &[T] and must have 32 bytes
    let mut key = [0; 32];
    derive(&HMAC_SHA256, 100, &salt, &password[..], &mut key);

    // Your private data
    let content = b"content to encrypt".to_vec();
    println!("Content to encrypt's size {}", content.len());

    // Additional data that you would like to send and it would not be encrypted but it would
    // be signed
    let additional_data: [u8; 0] = [];

    // Ring uses the same input variable as output
    let mut in_out = content.clone();

    // The input/output variable need some space for a suffix
    println!("Tag len {}", CHACHA20_POLY1305.tag_len());
    for _ in 0..CHACHA20_POLY1305.tag_len() {
        in_out.push(0);
    }

    // Opening key used to decrypt data
    let opening_key = OpeningKey::new(&CHACHA20_POLY1305, &key).unwrap();

    // Sealing key used to encrypt data
    let sealing_key = SealingKey::new(&CHACHA20_POLY1305, &key).unwrap();

    // Random data must be used only once per encryption
    let mut nonce = vec![0; 12];

    // Fill nonce with random data
    let rand = SystemRandom::new();
    rand.fill(&mut nonce).unwrap();

    // Encrypt data into in_out variable
    let output_size = seal_in_place(
        &sealing_key,
        &nonce,
        &additional_data,
        &mut in_out,
        CHACHA20_POLY1305.tag_len(),
    )
    .unwrap();

    println!("Encrypted data's size {}", output_size);

    let decrypted_data =
        open_in_place(&opening_key, &nonce, &additional_data, 0, &mut in_out).unwrap();

    println!("{:?}", String::from_utf8(decrypted_data.to_vec()).unwrap());
}
