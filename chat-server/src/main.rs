
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

use std::collections::HashMap;


fn main () {
    let addr = "127.0.0.1:8080";
    let clients = Arc::new(Mutex::new(HashMap::<usize, TcpStream>::new()));
    let listener = TcpListener::bind(addr).unwrap();
    println!("running server at {}", addr);
    let (broadcast_tx, broadcast_rx) = channel();
    spawn_broadcast_thread(clients.clone(), broadcast_rx);

    for (id, stream) in listener.incoming().enumerate() {
        match stream {
            Ok(stream) => {
                clients.lock().unwrap().insert(id, stream.try_clone().unwrap());
                spawn_client_thread(stream, broadcast_tx.clone());
            }
            Err(_) => { /* connection failed */ }
        }
    }
}

fn spawn_client_thread (mut stream: TcpStream, tx: Sender<String>) {
    let addr = stream.peer_addr().unwrap();
    thread::spawn(move || {
        let mut buffer = [0; 2048];
        println!("{} connected", addr);
        tx.send(format!("{} connected\n", addr)).unwrap();
        loop {
            match stream.read(&mut buffer) {
                Err(_) => break,
                Ok(0) => break, // EOF
                Ok(bytes) => {
                    match std::str::from_utf8(&buffer[0..bytes]) {
                        Err(_) => break,
                        Ok(utf8_str) => tx.send(format!("{}: {}", addr, utf8_str)).unwrap(),
                    }
                },
            }
        }
        println!("{} disconnected", addr);
        tx.send(format!("{} disconnected\n", addr)).unwrap();
    });
}

fn spawn_broadcast_thread (clients: Arc<Mutex<HashMap<usize, TcpStream>>>, rx: Receiver<String>) {
    thread::spawn(move || {
        loop {
            let message = rx.recv().unwrap();
            let mut writers = clients.lock().unwrap();
            let mut invalid_writer_ids = Vec::new();

            for (&id, stream) in writers.iter_mut() {
                match stream.write_all(message.as_bytes()) {
                    Ok(()) => { /* sucess */ },
                    Err(_) => invalid_writer_ids.push(id),
                };
            }

            for id in invalid_writer_ids {
                writers.remove(&id);
            }
        }
    });
}
