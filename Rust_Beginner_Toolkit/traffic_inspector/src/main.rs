use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    // 1. Setup the "Sensor" on Port 3000
    // We unwrap() here because if we can't bind to the port, the tool SHOULD crash.
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    
    println!("🛡️  Traffic Inspector Online at http://127.0.0.1:3000");
    println!("   -> Waiting for incoming packets...\n");

    // 2. The Capture Loop
    // This loops forever, processing every connection that hits the port.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // If connection works, hand it off to our inspector function
                handle_connection(stream);
            }
            Err(e) => {
                println!("❌ Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // A bucket to hold the packet data

    // 3. Read ("Scan") the packet
    // We read the bytes from the network stream into our buffer
    stream.read(&mut buffer).unwrap();
    
    // Convert bytes to text so we can read it (lossy = ignore weird characters)
    let request_text = String::from_utf8_lossy(&buffer);

    println!("🔎 Scanning Packet...");

    // 4. THE FILTER LOGIC
    // If the URL contains "ad", "banner", or "track", we block it.
    if request_text.contains("ad") || request_text.contains("banner") {
        println!("🚨 [BLOCKED] Ad Signature Detected!");
        
        // RESPONSE: 403 Forbidden
        // Note the 'Content-Type' header which fixes the emoji display
        let response = "HTTP/1.1 403 FORBIDDEN\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<html><body style='background-color:black; color:red; font-family:sans-serif;'><h1>🚫 AD BLOCKED</h1><p>Malicious signature detected by Rust Inspector.</p></body></html>";
        stream.write_all(response.as_bytes()).unwrap();
    
    } else {
        println!("✅ [CLEAN] Traffic Allowed.");
        
        // RESPONSE: 200 OK
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<html><body style='font-family:sans-serif;'><h1>✅ Traffic Clean</h1><p>This packet has been inspected and approved.</p></body></html>";
        stream.write_all(response.as_bytes()).unwrap();
    }
    
    // Flush ensures the data is sent immediately
    stream.flush().unwrap();
    println!("------------------------------------------------");
}
