use rand::seq::SliceRandom;
use std::io::Write;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::Arc;
use std::{env, fs, process, str, thread};

static ADDRESS: &str = "0.0.0.0:17";

fn get_quotes(file_path: String) -> Vec<String> {
    let all_quotes = fs::read_to_string(file_path).unwrap_or_else(|err| {
        println!("Couldn't open quotes file: {}", err);
        process::exit(1);
    });
    let all_quotes: Vec<String> = all_quotes.split('%').map(|s| s.to_string()).collect();
    println!("Got {} quotes from quote file", all_quotes.len());
    all_quotes
}

fn spawn_tcp_thread(_quotes: &Arc<Vec<String>>) -> thread::JoinHandle<()> {
    let quotes = Arc::clone(_quotes);

    thread::spawn(move || {
        let tcp_listener = TcpListener::bind(ADDRESS).unwrap_or_else(|err| {
            println!("Couldn't bind TCP socket to address: {}", err);
            process::exit(1);
        });
        println!("TCP listening");
        loop {
            if let Ok((tcp_stream, _)) = tcp_listener.accept() {
                let quote = quotes.choose(&mut rand::thread_rng());
                if handle_tcp(tcp_stream, quote.unwrap()).is_err() {
                    // TODO Logging/warning
                    continue;
                }
            }
        }
    })
}

fn handle_tcp(mut stream: TcpStream, quote: &str) -> std::io::Result<()> {
    stream.write_all(quote.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn spawn_udp_thread(_quotes: &Arc<Vec<String>>) -> thread::JoinHandle<()> {
    let quotes = Arc::clone(_quotes);

    thread::spawn(move || {
        let udp_socket = UdpSocket::bind(ADDRESS).unwrap_or_else(|err| {
            println!("Couldn't bind UDP socket to address: {}", err);
            process::exit(1);
        });
        println!("UDP listening");
        let mut throwaway_buf = [0; 1];

        loop {
            if let Ok((_, udp_addr)) = udp_socket.recv_from(&mut throwaway_buf) {
                let quote = quotes.choose(&mut rand::thread_rng());
                if handle_udp(udp_socket.try_clone().unwrap(), udp_addr, quote.unwrap()).is_err() {
                    // TODO Logging/warning
                    continue;
                }
            }
        }
    })
}

fn handle_udp(socket: UdpSocket, addr: SocketAddr, quote: &str) -> std::io::Result<()> {
    socket.send_to(quote.as_bytes(), addr)?;
    Ok(())
}

fn main() {
    let quotes_file_path = env::args().nth(1).unwrap_or_else(|| {
        println!("Specify quotes file './qotd <file_path>'");
        process::exit(1);
    });
    let quotes = Arc::new(get_quotes(quotes_file_path));

    let tcp_thread = spawn_tcp_thread(&quotes);
    let udp_thread = spawn_udp_thread(&quotes);
    for t in vec![tcp_thread, udp_thread] {
        t.join().unwrap();
    }
}
