mod protector;
use protector::{get_session_key, get_hash_str, next_session_key};
use std::{
    net::{TcpListener, TcpStream, Shutdown},
    io::{Read, Write},
    io,
    str::from_utf8,
    thread,
    env,
};

fn handle_client(mut stream: TcpStream) {
    let mut hash = [0 as u8; 5];
    let mut key = [0 as u8; 10];
    let mut message = [0 as u8; 50];

    while match stream.read(&mut hash) {
        Ok(_) => {
            stream.read(&mut key).unwrap();
            stream.read(&mut message).unwrap();
            
            let received_hash = from_utf8(&hash).unwrap();
            let received_key = from_utf8(&key).unwrap();
            let new_key = next_session_key(&received_hash, &received_key);
            let result = new_key.clone().into_bytes();
            
            stream.write(&result).unwrap();
            stream.write(&message).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn server(port: String, n: u64) {
    let ip = "127.0.0.1:".to_string();
    let listener = TcpListener::bind((ip + &port).to_string()).unwrap();
    println!("Server listening on port {}", port);
    
    let mut limit = n;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if limit > 0 {
                    limit -= 1;
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || handle_client(stream));
                } else {
                    println!("The server is stopped. Overflow.");
                    break;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
    drop(listener);
}

fn client(addr: String) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let mut data = [0 as u8; 50];
            let mut rep = [0 as u8; 50];

            loop {
                let hash_str = get_hash_str();
                let session_key = get_session_key();
                let next_key = next_session_key(&hash_str, &session_key);
                
                let mut message = String::new();
                println!("Enter your message to server: ");
                io::stdin().read_line(&mut message).unwrap();
                
                stream.write(&hash_str.into_bytes()).unwrap();
                stream.write(&session_key.into_bytes()).unwrap();
                stream.write(&message.into_bytes()).unwrap();
                
                match stream.read(&mut data) {
                    Ok(size) => {
                        stream.read(&mut rep).unwrap();
                        let received_key = from_utf8(&data[0..size]).unwrap();
                        let response = from_utf8(&rep).unwrap();
                        
                        if received_key == next_key {
                            println!("Client key: {}", next_key);
                            println!("Server key: {}", received_key);
                        }   else {
                                break;
                        }
                        println!("Response: {}", response);
                    }, 
                    Err(err) => {
                        println!("Failed to receive data: {}", err);
                    }
                }
            }
        },
        Err(err) => {
            println!("Failed to connect: {}", err);
        }
    }
    println!("Terminated");
}
fn main() {
    let args: Vec<String> = env::args().collect();
    //port or ip:port - 1 argument
    //-n - 2 argument
    //number of simultaneous connections - 3 argument

    if args[1].find(":") != None {
        println!("Trying to connect with server on adress {}", &args[1]);
        client(args[1].to_string());
    } else {
        println!("Trying run server");
        server(args[1].to_string(), args[3].parse::<u64>().unwrap());
    }
}