#![windows_subsystem = "windows"]
use std::thread;
extern crate web_view;
use web_view::*;
extern crate json;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde_derive::{Deserialize, Serialize};
// use actix_web::{middleware};
use std::env;

pub static mut DB: Option<zmq::Socket> = None;

#[derive(Debug, Serialize, Deserialize)]
struct IncommingRequest {
    command: String,
    key: Option<String>,
    value: Option<String>,
}

fn homepage() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(
            r#"
<div id="root"></div>
<script src="https://cdnjs.cloudflare.com/ajax/libs/react/15.4.2/react.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/react/15.4.2/react-dom.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/babel-standalone/6.21.1/babel.min.js"></script>
<style>
    html {
        font-family: 'Source Code Pro', monospace;
    }
    pre {
        padding: 5px;
        font-size: 1.2em;
        
        white-space: pre-wrap;       /* css-3 */
        white-space: -moz-pre-wrap;  /* Mozilla, since 1999 */
        white-space: -pre-wrap;      /* Opera 4-6 */
        white-space: -o-pre-wrap;    /* Opera 7 */
        word-wrap: break-word;       /* Internet Explorer 5.5+ */
    }
    hr {
        width: 36vw;
    }
    ul {
        list-style: none;
    }
    li {
        padding-top: 3vh;
        height: 5vh;
    }
    h3 {
        padding-left: 2vw;
        color: #FFE600;
    }
    .top {
        position: absolute;
        top: 0;
        background: #5E44FF;
        height: 8vh;
        width: 100vw;
        z-index: 99;
    }
    .bottom {
        position: absolute;
        top: 8vh;
        height: 92vh;
    }
    .main {
        display: flex;
        position: absolute;
        left: 0;
        right: 0;
        top: 0;
        bottom: 0;
    }
    .keyWindow {
        background: #ddd;
        overflow-x: hidden;
        overflow-y: scroll;
        position: absolute;
        height: 100%;
        width: 45vw;
    }
    .valueWindow {
        overflow-x: hidden;
        overflow-y: scroll;
        position: absolute;
        left: 45vw;
        width: 55vw;
        height: 100%;
        color: #FFE600;
        background: #222;
    }
    .currentlyViewing {
        color: #5E44FF;
        font-weight: bold;
        font-size: 1.05em;
    }
</style>
<script type="text/babel">
    class Greeting extends React.Component {


    constructor(props) {
        // Required step: always call the parent class' constructor
        super(props);
        this.fetchAllKeys()
        // Set the state directly. Use props if necessary.
        this.state = {
            items: [1,2,3,4],
            currentValue: "",
            currentKey: "",
        }
        setInterval(function(){this.fetchAllKeys()}.bind(this),2000)
    }

    fetchAllKeys() {
        fetch("/wire", {
          "method": "POST",
          "headers": {
            "content-type": "application/json"
          },
          "body":JSON.stringify({
            "command": "KEYS"
          })
        })
        .then(response => {
            response.json().then(r =>{
                this.setState({items: r})
            })
        })
        .catch(err => {
          console.log(err);
        });
    }


    onKeyClick(e) {
        let currentKey = e.target.innerText

        fetch("/wire", {
          "method": "POST",
          "headers": {
            "content-type": "application/json"
          },
          "body":JSON.stringify({
            "command": "GET",
            "key": currentKey
          })
        })
        .then(response => {
            response.text().then(r =>{
                try {
                  this.setState({currentValue: JSON.parse(r), currentKey: currentKey})
                }
                catch(error) {
                  this.setState({currentValue: r, currentKey: currentKey})
                }
            })
        })
        .catch(err => {
          console.log(err);
        });
    }
    render() {
        return (
        <div className="main">
            <div className="top">
                <h3>ZEDIS SERVER</h3>
            </div>
            <div className="bottom">
                <div className="keyWindow">
                    <ul>{this.state.items.map(e=>{return(
                        <li 
                            onClick={this.onKeyClick.bind(this)}
                            className={e == this.state.currentKey && "currentlyViewing" }>
                                <div>{e}</div>
                        </li>
                        )})}</ul>
                </div>
                <div className="valueWindow"> 
                    <pre>
                        {
                            this.state.currentValue != "" && JSON.stringify(this.state.currentValue,null,2) 
                        }
                    </pre>
                </div>
            </div>
        </div>);
    }
}
ReactDOM.render(
    <Greeting />,
    document.getElementById('root')
);
</script>
    "#)
}

fn wire(item: web::Json<IncommingRequest>) -> HttpResponse {
    let mut command_for_zedis: String = "".to_string();
    match item.command.as_ref() {
        "GET" => {
            command_for_zedis = format!("{} {}", &item.command, &item.key.as_ref().unwrap());
        }
        "SET" => {
            command_for_zedis = format!(
                "{} {} {}",
                &item.command,
                &item.key.as_ref().unwrap(),
                &item.value.as_ref().unwrap()
            );
        }
        "DEL" => {
            command_for_zedis = format!("{} {}", &item.command, &item.key.as_ref().unwrap());
        }
        "KEYS" => {
            command_for_zedis = format!("{}", &item.command);
        }
        "PRE" => {
            command_for_zedis = format!("{} {}", &item.command, &item.key.as_ref().unwrap());
        }
        _ => {}
    }

    let mut resp: String = "yo".to_string();

    unsafe {
        match &DB {
            Some(socket) => {
                socket.send(&command_for_zedis, 0).unwrap();
                resp = socket.recv_string(0).unwrap().unwrap();
            }
            None => (),
        }
    }

    HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(resp)
}

fn main() -> Result<(), web_view::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut port_zedis: String = "5555".to_string();
    let mut port_server: String = "8080".to_string();
    if args.len() > 2 {
        port_zedis = args[1].to_string();
        port_server = args[2].to_string();
    };

    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REQ).unwrap();
    let address = format!("tcp://localhost:{}", port_zedis);
    socket.connect(&address).unwrap();
    unsafe { DB = Some(socket) }

    let my_port_server = port_server.clone();
    thread::spawn(move || {
        HttpServer::new(|| {
            App::new()
                // .wrap(middleware::Logger::default())
                .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
                .service(web::resource("/").route(web::get().to(homepage)))
                .service(web::resource("/wire").route(web::post().to(wire)))
        })
        .bind(format!("127.0.0.1:{}", my_port_server))?
        .run()
    });
    web_view::builder()
        .title("zedis server viewer")
        .content(Content::Url(format!("http://localhost:{}", port_server)))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
}
