use anyhow::Error;
use futures::StreamExt;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex as AsyncMutex;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::WebSocketStream;

struct ServerState {
    devices: std::collections::HashMap<usize /* device id */, DeviceState>,
}

struct DeviceState {
    disconnect_tx: tokio::sync::oneshot::Sender<()>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    // Listen to incoming WebSocket connections
    let socket = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let mut incoming = TcpListenerStream::new(socket);

    // Init state
    let server_state = ServerState { devices: std::collections::HashMap::new() };
    let server_state = Arc::new(AsyncMutex::new(server_state));

    // Dispatch connections
    while let Some(stream) = incoming.next().await {
        let stream = stream.expect("Failed to accept stream");
        tokio::spawn(accept_connection(stream, server_state.clone()));
    }

    Ok(())
}

/// This function accepts the connection, does the WS handshake and does error
/// handling once the connection handling returns.
async fn accept_connection(stream: TcpStream, server_state: Arc<AsyncMutex<ServerState>>) {
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

    match accept_connection_inner(&mut ws_stream, server_state.clone()).await {
        Ok(_) => println!("Connection terminated"),
        Err(e) => println!("Error: {:?}", e),
    }
}

/// This function accepts the stream and processes it.
async fn accept_connection_inner(
    ws_stream: &mut WebSocketStream<TcpStream>,
    server_state: Arc<AsyncMutex<ServerState>>,
) -> Result<(), Error> {
    println!("Accept connection");

    // Here, in the real code, we establish an additional connection to another
    // TCP server (called "proxy socket" from now on). Messages from the server
    // are framed and relayed to the WebSocket server, and vice versa. This
    // happens even without authentication (since the TCP connection has its
    // own auth).

    // 1. Send server-hello message

    // 2. Auth loop: select! between ws stream and proxy socket.
    let device_id = 42; // Determined in the auth handshake

    // 3. If successful, create a one-shot channel for disconnecting the
    //    connection. Then register it in the state.
    let (disconnect_tx, disconnect_rx) = tokio::sync::oneshot::channel();
    let device = DeviceState { disconnect_tx };
    server_state.lock().await.devices.insert(device_id, device);

    // 4. Send server-info message

    // 5. Check disconnect_rx.try_recv()

    // 6. Send queued messages in a loop, check disconnect_rx after every message

    // 7. Check disconnect_rx.try_recv()

    // 8. Send another control message

    // 9. Main loop: select! between ws stream, proxy socket and disconnect_rx

    Ok(())
}
