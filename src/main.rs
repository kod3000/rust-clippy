use std::net::{TcpListener, TcpStream, UdpSocket};
use std::io::{self, Read, Write};
use std::thread;
use clipboard::{ClipboardProvider, ClipboardContext};
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

const BROADCAST_PORT: u16 = 7879;
const TCP_PORT: u16 = 7878;
const DISCOVERY_MESSAGE: &str = "CLIPPY_SYNC_DISCOVER";

type IpList = Arc<Mutex<HashSet<String>>>;

fn broadcast_discovery() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    let broadcast_address = format!("255.255.255.255:{}", BROADCAST_PORT);
    socket.send_to(DISCOVERY_MESSAGE.as_bytes(), broadcast_address)?;
    Ok(())
}

fn listen_for_broadcasts(shared_ip_list: IpList) -> std::io::Result<()> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", BROADCAST_PORT))?;
    let mut buf = [0; 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let message = String::from_utf8_lossy(&buf[..amt]);
        if message == DISCOVERY_MESSAGE {
            // println!("Discovery message received from {}", src);
            let ip_address = src.ip().to_string();
            let mut ip_list = shared_ip_list.lock().unwrap();
            if ip_list.insert(ip_address.clone()) {
                println!("New connection added to list: {}", ip_address);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, local_clipboard: Arc<Mutex<String>>) {
    let mut buffer = String::new();
    match stream.read_to_string(&mut buffer) {
        Ok(_) => {
            if buffer.len() == 0 {
                println!("Clipboard is empty, no changes made");
                return;
            }
            let mut clipboard = local_clipboard.lock().unwrap();
            *clipboard = buffer.clone();
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(buffer).unwrap();
        }
        Err(e) => println!("Failed to receive data: {}", e),
    }
}

fn start_tcp_server() {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", TCP_PORT)).unwrap();
    let local_clipboard = Arc::new(Mutex::new(String::new()));
    println!("\n\n\tClippy instance is now running on port {}", TCP_PORT);
    println!("\n\n\tWaiting for incoming connections...\n\n");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clipboard_clone = Arc::clone(&local_clipboard);
                thread::spawn(move || handle_client(stream, clipboard_clone));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn main() {

    let shared_ip_list: IpList = Arc::new(Mutex::new(HashSet::new()));
    let thread_shared_ip_list = Arc::clone(&shared_ip_list);

    thread::spawn(move || {
        listen_for_broadcasts(thread_shared_ip_list).expect("Failed to listen for broadcasts");
    });

    thread::spawn(move || {
        loop{
            broadcast_discovery().expect("Failed to broadcast discovery message (-__-) ");
            std::thread::sleep(std::time::Duration::from_secs(30));
        }
    });

    thread::spawn(move || {
        start_tcp_server();
    });

    let mut counter = 0;
    let mut ctx_init: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut previous_clipboard_contents = ctx_init.get_contents().unwrap();
    loop{
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let clipboard_contents = ctx.get_contents().unwrap();
        if clipboard_contents != previous_clipboard_contents {
            let ip_list = shared_ip_list.lock().unwrap();
            for ip in ip_list.iter() {
                match TcpStream::connect(format!("{}:{}", ip, TCP_PORT)) {
                    Ok(mut stream) => {
                        stream.write_all(clipboard_contents.as_bytes()).unwrap();
                        // TODO: this counter is incorrect, get it working properly
                        counter += 1;
                    }
                    Err(e) => {
                        // TODO: if a failure happens, we may want to remove the IP from the list
                        println!("Failed to connect to {}: {}", ip, e);
                    }
                }
            }
            previous_clipboard_contents = clipboard_contents;
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
