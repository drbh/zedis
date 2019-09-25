use colored::*;
use std::env;

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn execute(socket: &zmq::Socket, cmd: String) {
    let mut msg = zmq::Message::new();
    socket.send(&cmd, 0).unwrap();
    socket.recv(&mut msg, 0).unwrap();
    let msg_text = msg.as_str().unwrap();
    println!("{}", msg_text);
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

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(format!("{} >> ", "zedis".blue().bold()).as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                execute(&socket, line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
