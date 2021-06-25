use anyhow::Error;
use futures::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::wrappers::TcpListenerStream;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::WebSocketStream;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    // Listen to incoming WebSocket connections
    let socket = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let mut incoming = TcpListenerStream::new(socket);

    // Dispatch connections
    while let Some(stream) = incoming.next().await {
        let stream = stream.expect("Failed to accept stream");
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

/// This function accepts the connection, does the WS handshake and does error
/// handling once the connection handling returns.
async fn accept_connection(stream: TcpStream) {
    // Do the WebSocket handshake
    let ws_callback = |_req: &Request, resp: Response| {
        /* Some registration logic omitted */
        Ok(resp)
    };
    let mut ws_stream = match tokio_tungstenite::accept_hdr_async(stream, ws_callback).await {
        Ok(stream) => stream,
        Err(e) => {
            // Most probably a non-websocket connection attempt
            println!("Error in stream: {:?}", e);
            return;
        }
    };

    match accept_connection_inner(&mut ws_stream).await {
        Ok(_) => println!("Connection terminated"),
        Err(e) => println!("Error: {:?}", e),
    }
}

/// This function accepts the stream and processes it.
async fn accept_connection_inner(ws_stream: &mut WebSocketStream<TcpStream>) -> Result<(), Error> {
    println!("Accept connection");
    Ok(())
}
