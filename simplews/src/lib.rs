use kinode_process_lib::{
    await_message, call_init, get_blob, http, println, Address, Message, Request, Response,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

#[derive(Serialize, Deserialize)]
enum CounterMessage {
    Get,
    Increment,
    Decrement,
}

#[derive(Serialize, Deserialize)]
struct CounterResponse {
    value: i32,
}

struct State {
    counter: i32,
    ws_channels: HashSet<u32>,
}

impl State {
    fn new() -> Self {
        Self {
            counter: 0,
            ws_channels: HashSet::new(),
        }
    }
}

fn handle_message(our: &Address, state: &mut State) -> anyhow::Result<()> {
    let message = await_message()?;

    if message.source().node != our.node {
        return Ok(());
    }

    match message {
        Message::Request { ref body, .. } => {
            let http_request: http::HttpServerRequest = serde_json::from_slice(body)?;
            match http_request {
                http::HttpServerRequest::Http(req) => handle_http_request(our, state, req),
                http::HttpServerRequest::WebSocketOpen { channel_id, .. } => {
                    state.ws_channels.insert(channel_id);
                    Ok(())
                }
                http::HttpServerRequest::WebSocketClose(channel_id) => {
                    state.ws_channels.remove(&channel_id);
                    Ok(())
                }
                http::HttpServerRequest::WebSocketPush { channel_id, .. } => {
                    handle_ws_message(state, channel_id)
                }
            }
        }
        _ => Ok(()),
    }
}

fn handle_http_request(
    our: &Address,
    state: &State,
    req: http::IncomingHttpRequest,
) -> anyhow::Result<()> {
    let path = req.path()?;
    match path.as_str() {
        "/" => {
            http::serve_ui(our, "ui", false, false, vec!["/"])?;
        }
        _ => {
            http::send_response(http::StatusCode::NOT_FOUND, None, Vec::new());
        }
    }
    Ok(())
}

fn handle_ws_message(state: &mut State, channel_id: u32) -> anyhow::Result<()> {
    let blob = get_blob().ok_or_else(|| anyhow::anyhow!("Failed to get blob"))?;
    let message: CounterMessage = serde_json::from_slice(&blob.bytes)?;

    match message {
        CounterMessage::Get => {}
        CounterMessage::Increment => state.counter += 1,
        CounterMessage::Decrement => state.counter -= 1,
    }

    let response = CounterResponse {
        value: state.counter,
    };
    let response_bytes = serde_json::to_vec(&response)?;

    http::send_ws_push(
        channel_id,
        http::WsMessageType::Text,
        kinode_process_lib::LazyLoadBlob {
            mime: Some("application/json".to_string()),
            bytes: response_bytes,
        },
    );
    Ok(())
}

call_init!(init);

fn init(our: Address) {
    println!("Counter process started");

    http::bind_http_path("/", false, false).expect("Failed to bind HTTP path");
    http::bind_ws_path("/ws", false, false).expect("Failed to bind WebSocket path");

    let mut state = State::new();

    loop {
        if let Err(e) = handle_message(&our, &mut state) {
            println!("Error: {:?}", e);
        }
    }
}
