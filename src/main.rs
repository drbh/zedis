use sled::Db;
use std::env;
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
    let command_count = commands.clone().count();
    let cmd = commands.next().ok_or(Error::InvalidCommand)?;

    let mut key: &str = "default";
    let mut val: String = "default".to_string();
    if command_count > 1 {
        key = commands.next().ok_or(Error::InvalidKey)?;
        val = commands.collect::<Vec<&str>>().join(" ");
    }

    match cmd {
        "GET" => match t.get(key.as_bytes()).map_err(|_| Error::Database)? {
            Some(val) => String::from_utf8(val.to_vec()).map_err(|_| Error::Database),
            None => Err(Error::InvalidKey),
        },
        "PRE" => {
            let keys = t
                .scan_prefix(key.as_bytes())
                .keys()
                .map(|x| String::from_utf8(x.unwrap().to_vec()).map_err(|_| Error::Database))
                .map(|y| y.unwrap())
                .collect::<Vec<String>>()
                .join("\", \"");
            Ok(format!("[\"{}\"]", keys))
        }
        "SET" => {
            match t
                .insert(key.as_bytes(), val.as_bytes())
                .map_err(|_| Error::Database)?
            {
                Some(val) => String::from_utf8(val.to_vec()).map_err(|_| Error::Database),
                None => Ok(String::from("done")),
            }
        }
        "DEL" => match t.remove(key.as_bytes()).map_err(|_| Error::Database)? {
            Some(val) => String::from_utf8(val.to_vec()).map_err(|_| Error::Database),
            None => Ok(String::from("done")),
        },
        "KEYS" => {
            let keys = t
                .iter()
                .keys()
                .map(|x| String::from_utf8(x.unwrap().to_vec()).map_err(|_| Error::Database))
                .map(|y| y.unwrap())
                .collect::<Vec<String>>()
                .join("\", \"");
            Ok(format!("[\"{}\"]", keys))
        }

        _ => Err(Error::InvalidCommand),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut port = "5555";
    if args.len() > 1 {
        port = &args[1];
    };
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    let address = format!("tcp://*:{}", port);
    assert!(responder.bind(&address).is_ok());
    let mut msg = zmq::Message::new();
    let t = Db::open("my_db").unwrap();
    println!(
        "
      _______ ___ ___ ___ 
     |_  / __|   \\_ _/ __|
      / /| _|| |) | |\\__ \\ 
     /___|___|___/___|___/ 


    Welcome to zedis a lightweight
    super simple datasore. 

    transport: tcp://localhost:{} 
    database file: my_db",
        &port
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
