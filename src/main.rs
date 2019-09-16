use sled::Db;
use std::str;
use std::time::Instant;


fn get(t: sled::Db, key: &[u8]) -> std::option::Option<sled::IVec> {
	let result = t.get(key).unwrap();
	result
}
fn set(t: sled::Db, key: &[u8], value: &[u8]) -> std::option::Option<sled::IVec> {
	let result = t.insert(key, value).unwrap();
	result
}

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    assert!(responder.bind("tcp://*:5555").is_ok());
    let mut msg = zmq::Message::new();
	let t = Db::open("my_db").unwrap();
	println!("{}", "
	  _______ ___ ___ ___ 
	 |_  / __|   \\_ _/ __|
	  / /| _|| |) | |\\__ \\ 
	 /___|___|___/___|___/ 


	Welcome to zedis a lightweight
	super simple datasore. 

	transport: tpc://localhost:5555 
	database file: my_db");

    loop {
        responder.recv(&mut msg, 0).unwrap();
        let msg_text = msg.as_str().unwrap();
        let start = Instant::now();
        let split = msg_text.split_whitespace();
        let vec: Vec<&str> = split.collect();
        if vec[0] == "GET" {
        	let r = get(t.clone(), vec[1].as_bytes());
        	let byts = r.unwrap();
        	let s = String::from_utf8(byts.to_vec()).expect("Found invalid UTF-8");
        	responder.send(s.as_str(), 0).unwrap();
        }
        if vec[0] == "SET" {
        	let joined = vec[2..vec.len()].to_vec().join(" ");
        	let _r = set(t.clone(), vec[1].as_bytes(), joined.as_bytes());
        	responder.send("done.", 0).unwrap();
        }
        let _duration = start.elapsed();
        // println!("{} {:#?}", msg_text, _duration);
        // println!("{:#?}", _duration);
    }
}