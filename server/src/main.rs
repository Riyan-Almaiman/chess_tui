use tokio::{
    net::TcpListener,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,

};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
enum Color {
    White,
    Black,
}

#[derive(Serialize, Deserialize)]
enum NetMessage {
    AssignColor(Color),
    Move { from: (usize, usize), to: (usize, usize) },
}
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let clients: Arc<Mutex<Vec<Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>>>> =
        Arc::new(Mutex::new(Vec::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let clients = clients.clone();

        tokio::spawn(async move {
            let (mut reader, writer) = socket.into_split();
            let writer = Arc::new(Mutex::new(writer));

            // add client
let mut clients_guard = clients.lock().await;

let color = if clients_guard.len() == 0 {
    Color::White
} else {
    Color::Black
};

// send color immediately
{
    let mut w = writer.lock().await;
    let msg = serde_json::to_string(&NetMessage::AssignColor(color)).unwrap() + "\n";
    let _ = w.write_all(msg.as_bytes()).await;
}

clients_guard.push(writer.clone());
drop(clients_guard);
            let mut buf = [0; 1024];

            loop {
                let n = match reader.read(&mut buf).await {
                    Ok(0) | Err(_) => return,
                    Ok(n) => n,
                };

                // take snapshot (drop lock immediately)
                let snapshot = {
                    let clients = clients.lock().await;
                    clients.clone()
                };

                // broadcast
                for client in snapshot {
                    let mut w = client.lock().await;
let text = String::from_utf8_lossy(&buf[..n]);

for line in text.lines() {
    let msg = line.to_string() + "\n";
    let _ = w.write_all(msg.as_bytes()).await;
}                }
            }
        });
    }
}