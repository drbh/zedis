use sled::Db;
use std::str;
use std::time::Instant;

#[derive(Debug)]
enum Error {
    InvalidCommand,
    Database,
    InvalidKey,
}

fn handle(t: sled::Db, msg: &str) -> Result<String, Error> {
    let mut commands = msg.split_whitespace();
    let cmd = commands.next().ok_or(Error::InvalidCommand)?;
    let key = commands.next().ok_or(Error::InvalidKey)?;
    let val: String = commands.collect::<Vec<&str>>().join(" ");
    match cmd {
        "GET" => match t.get(key.as_bytes()).map_err(|_| Error::Database)? {
            Some(val) => String::from_utf8(val.to_vec()).map_err(|_| Error::Database),
            None => Err(Error::InvalidKey),
        },
        "SET" => {
            match t
                .insert(key.as_bytes(), val.as_bytes())
                .map_err(|_| Error::Database)?
            {
                Some(val) => String::from_utf8(val.to_vec()).map_err(|_| Error::Database),
                None => Ok(String::from("done")),
            }
        }
        _ => Err(Error::InvalidCommand),
    }
}

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    assert!(responder.bind("tcp://*:5555").is_ok());
    let mut msg = zmq::Message::new();
    let t = Db::open("my_db").unwrap();
    println!(
        "{}",
        "
      _______ ___ ___ ___ 
     |_  / __|   \\_ _/ __|
      / /| _|| |) | |\\__ \\ 
     /___|___|___/___|___/ 


    Welcome to zedis a lightweight
    super simple datasore. 

    transport: tcp://localhost:5555 
    database file: my_db"
    );

    loop {
        responder.recv(&mut msg, 0).unwrap();
        let msg_text = msg.as_str().unwrap();
        let start = Instant::now();
        let answer = match handle(t.clone(), msg_text) {
            Ok(res) => res,
            Err(e) => format!("Error occurred: {:?}", e),
        };
        responder.send(answer.as_str(), 0).expect("ZMQ error");
        let _duration = start.elapsed();
    }
}
