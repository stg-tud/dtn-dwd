use anyhow::anyhow;
use bp7::{Bundle, EndpointID};
use dtn7_plus::client::DtnClient;
use json;
use serde::Deserialize;
use std::convert::TryFrom;
use std::env;
use tinyfiledialogs::MessageBoxIcon;
use web_view::*;
use ws::{Builder, CloseCode, Handler, Handshake, Message, Result, Sender};

struct Connection {
    endpoint: String,
    out: Sender,
    subscribed: bool,
    verbose: bool,
    last: u64,
    handle: web_view::Handle<()>,
}

impl Connection {
    fn handle_incoming_bundle(&mut self, bndl: &Bundle) -> anyhow::Result<()> {
        let cblock = bndl.payload().ok_or(anyhow!("no extension block"))?;
        let json_str = String::from_utf8(cblock.clone())?;
        let parsed = json::parse(&json_str)?;
        let last = &parsed["time"].as_u64().unwrap();

        if *last > self.last {
            self.last = *last;
            self.handle.dispatch(move |webview| {
                webview.eval(dbg!(&format!("handleBundle('{}');\n", json_str)))
            });
        }

        Ok(())
    }
}

impl Handler for Connection {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out
            .send(dbg!(format!("/subscribe {}", self.endpoint)))?;
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(txt) => {
                dbg!(&txt);
                if txt == "subscribed" {
                    self.subscribed = true;
                } else if txt.starts_with("200") {
                } else {
                    eprintln!("Unexpected response: {}", txt);
                    self.out.close(CloseCode::Error)?;
                }
            }
            Message::Binary(bin) => {
                let bndl: Bundle =
                    Bundle::try_from(bin).expect("Error decoding bundle from server");
                if bndl.is_administrative_record() {
                    eprintln!("Handling of administrative records not yet implemented!");
                } else {
                    if self.verbose {
                        eprintln!("Bundle-Id: {}", bndl.id());
                    }
                    if self.handle_incoming_bundle(&bndl).is_err() && self.verbose {
                        eprintln!("Not warning bundle: {}", bndl.id());
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Publish { data: String },
    Error { msg: String },
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port: u16 = if args.len() > 1 {
        args[1].parse::<u16>().unwrap()
    } else {
        3002
    };
    let local_url = format!("ws://127.0.0.1:{}/ws", port);
    let endpoint = EndpointID::with_dtn("warnings/dwd")?;

    let html_content = include_str!("../www/index.html");
    let client = DtnClient::with_host_and_port("127.0.0.1".into(), port);
    client.register_application_endpoint(&endpoint.to_string())?;

    let webview = web_view::builder()
        .title("DTN DWD Client")
        .content(Content::Html(html_content))
        .size(800, 600)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            println!("{}", arg);
            let cmd: Cmd = dbg!(serde_json::from_str(arg).unwrap());
            match cmd {
                Cmd::Publish { data } => {
                    println!("Publishing: {}", data);
                    tinyfiledialogs::message_box_ok(
                        "Information",
                        "Published updated warning information.",
                        MessageBoxIcon::Info,
                    );
                }
                Cmd::Error { msg } => {
                    eprintln!("{}", msg);
                    tinyfiledialogs::message_box_ok("Error", &msg, MessageBoxIcon::Error);
                }
            }
            Ok(())
        })
        .build()?;

    let handle = webview.handle();
    std::thread::spawn(move || {
        let mut ws = Builder::new()
            .build(|out| Connection {
                endpoint: endpoint.to_string(),
                out,
                subscribed: false,
                verbose: true,
                last: 0,
                handle: handle.clone(),
            })
            .unwrap();
        ws.connect(url::Url::parse(&local_url).unwrap()).unwrap();
        ws.run().unwrap();
    });
    webview.run()?;
    Ok(())
}
