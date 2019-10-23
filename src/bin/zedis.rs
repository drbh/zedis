use sled::Db;
use std::env;
use std::str;
use std::time::Instant;

#[derive(Debug)]
enum Error {
    InvalidCommand,
    Database,
    InvalidKey,
    NonIntValue,
}

fn increment(old: Option<&[u8]>) -> Option<Vec<u8>> {
    match old {
        Some(bytes) => {
            // return old value if is not int32
            let string_value = String::from_utf8(bytes.to_vec()).unwrap();
            let my_int = string_value.parse::<i32>().map_err(|_| Error::Database);
            let _int = my_int.unwrap_or(-1);
            if _int != -1 {
                return Some((_int + 1).to_string().as_bytes().to_vec());
            }
        }
        // None => return Some(0.to_string().as_bytes().to_vec()),
        None => return None,
    };
    Some(old.unwrap().to_vec())
}

fn handle(t: sled::Db, msg: &str, publ: &zmq::Socket) -> Result<String, Error> {
    let mut commands = msg.split_whitespace();
    let command_count = commands.clone().count();
    let cmd = commands.next().ok_or(Error::InvalidCommand)?;

    let mut key: &str = "default";
    let mut val: String = "default".to_string();
    if command_count > 1 {
        key = commands.next().ok_or(Error::InvalidKey)?;
        val = commands.collect::<Vec<&str>>().join(" ");
    }

    match cmd.to_ascii_uppercase().as_str() {
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
            if keys.len() < 1 {
                Err(Error::InvalidKey)?
            }
            Ok(format!("[\"{}\"]", keys))
        }
        "INC" => match t
            .update_and_fetch(key.as_bytes(), increment)
            .map_err(|_| Error::Database)?
        {
            Some(val) => {
                // println!("{:?}", "val");
                // println!("{:?}", val);

                //wasteful test - but returns error if failed
                let string_value = String::from_utf8(val.to_vec()).unwrap();
                let my_int = string_value.parse::<i32>().map_err(|_| Error::Database);
                let _int = my_int.unwrap_or(0);
                if _int == 0 {
                    return Err(Error::NonIntValue)?;
                }
                String::from_utf8(val.to_vec()).map_err(|_| Error::Database)
            }
            None => Err(Error::InvalidKey)?,
        },
        "SET" => {
            match t
                .insert(key.as_bytes(), val.as_bytes())
                .map_err(|_| Error::Database)?
            {
                Some(val) => {
                    // send the key
                    publ.send(key, 0).expect("ZMQ error");
                    String::from_utf8(val.to_vec()).map_err(|_| Error::Database)
                }
                None => Ok(String::from("done")),
            }
        }
        "DEL" => match t.remove(key.as_bytes()).map_err(|_| Error::Database)? {
            Some(val) => String::from_utf8(val.to_vec()).map_err(|_| Error::Database),
            None => Ok(String::from("done")),
        },
        "CLEAR" => {
            t.clear().unwrap();
            Ok(String::from("cleared"))
        }
        "FLUSH" => Ok(format!("[\"{}\"]", t.flush().unwrap())),
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

    let publisher = context.socket(zmq::PUB).unwrap();
    let address = format!("tcp://127.0.0.1:{}", "7894");
    assert!(publisher.bind(&address).is_ok());
    publisher.send("money", 0).expect("ZMQ error");

    let mut msg = zmq::Message::new();
    let t = Db::open("my_db").unwrap();
    println!(
        "
      _______ ___ ___ ___ 
     |_  / __|   \\_ _/ __|
      / /| _|| |) | |\\__ \\ 
     /___|___|___/___|___/ 

    version: 0.1.102

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
        let answer = match handle(t.clone(), msg_text, &publisher) {
            Ok(res) => res,
            Err(e) => format!("Error occurred: {:?}", e),
        };
        responder.send(answer.as_str(), 0).expect("ZMQ error");
        let _duration = start.elapsed();
    }
}
