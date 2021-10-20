use bp7::{Bundle, EndpointID};
use dtn7_plus::client::DtnClient;
use serde::Deserialize;
use std::{convert::TryInto, env};
use tinyfiledialogs::MessageBoxIcon;
use web_view::*;

#[derive(Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Publish { data: String },
    Error { msg: String },
}

fn make_bundle(client: &DtnClient, data: String) -> Bundle {
    let sender = client.local_node_id().unwrap();
    let cts = client.creation_timestamp().unwrap();

    let mut bndl = bp7::bundle::new_std_payload_bundle(
        sender,
        "dtn://warnings/~dwd".try_into().unwrap(),
        data.as_bytes().to_vec(),
    );

    //    bndl.primary.bundle_control_flags = BUNDLE_MUST_NOT_FRAGMENTED | BUNDLE_STATUS_REQUEST_DELIVERY;
    bndl.primary.creation_timestamp = cts;

    bndl
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let port: u16 = if args.len() > 1 {
        args[1].parse::<u16>().unwrap()
    } else {
        3000
    };

    let client = DtnClient::with_host_and_port("127.0.0.1".into(), port);
    let sender = client.local_node_id().unwrap();
    let title = format!("DTN DWD Command Center @ {}", sender);
    //    bndl.primary.lifetime = std::time::Duration::from_secs(lifetime);

    let html_content = include_str!("../www/index.html");
    web_view::builder()
        .title(&title)
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
                    let mut bndl = make_bundle(&client, data);
                    let res = attohttpc::post(&format!("http://127.0.0.1:{}/insert", port))
                        .bytes(bndl.to_cbor())
                        .send()
                        .expect("error send bundle to dtnd")
                        .text()
                        .unwrap();
                    println!("Result: {}", res);
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
        .run()
        .unwrap();
}
