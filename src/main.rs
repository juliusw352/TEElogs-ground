use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, ErrorKind};
use std::thread;
use rand::Rng;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").expect("Networking error");
    println!("Server listening on port 9090");

    for stream_incoming in listener.incoming() {
        match stream_incoming {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {

    let peer_addr = stream
        .peer_addr()
        .map_or_else(|_| "Unknown".to_string(), |addr| addr.to_string());
    println!("New connection from {}", peer_addr);

    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Connection closed");
                }
                let res = &buffer[..n];
                println!("Received public key: {:?}", String::from_utf8(res.to_vec()).unwrap());
                respond(res);
            },
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => {
                match e.kind() {
                    ErrorKind::ConnectionReset => {
                        println!("Client connection reset")
                    },
                    _ => {
                        eprintln!("Unexpected error: {}", e);
                    }
                };
            }
        };
        break;
    }
    println!("Connection closed")
}

fn respond(res: &[u8]) {
    // Generate large prime number
    let mut rng = rand::rng();
    let p: u64 = 345466091;
    let base = 124717;
    let received_public_key: u64 = String::from_utf8(res.to_vec())
        .unwrap()
        .parse()
        .unwrap();
    
    let secret_key: u64 = rng.random_range(0..p - 1);
    let public_key: u64 = power_mod(base, secret_key, p);
    
    
    let mut response_stream = TcpStream::connect("127.0.0.1:9091")
    .expect("Failed to connect to response stream");

response_stream.write_all(format!("{}", public_key).as_bytes())
.expect("Failed to send response");
println!("Sent public key: {}", public_key);

let derived_symmetric_key: u64 = power_mod(received_public_key, secret_key, p);
println!("Derived symmetric key: {:?}", derived_symmetric_key);
}


// DHKE helper function
fn power_mod(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    return result;
}
