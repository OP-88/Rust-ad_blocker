🦀 Rust Traffic Inspector

A low-level, User-Space Network Traffic Filter built from scratch.

    "To understand the Web, you have to break it down to the bytes."

This isn't a wrapper around Nginx or a high-level proxy library. This is a manual implementation of a Traffic Inspector using raw TCP streams in Rust. It captures, decodes, and filters HTTP packets at the byte level—all running in user space without sudo.
⚡ Key Features

    Zero Dependencies (Almost): Built primarily using Rust's standard library (std::net, std::io).

    Raw Packet Capture: Reads network traffic directly into a 1024-byte stack buffer.

    Manual HTTP Decoding: Converts raw u8 byte streams into readable text using lossy UTF-8 decoding.

    Active Traffic Control: Detects signatures (like ad or banner) and injects a custom HTTP 403 Forbidden response in real-time.

    Non-Root Execution: Runs entirely in user space (Layer 7) via TcpListener, bypassing the need for raw sockets or admin privileges.

🛠️ How It Works (The "First Principles" Approach)

Most ad blockers work as browser extensions. Most firewalls work at the Kernel level. This tool sits in the middle.

    The Listener: It binds to 127.0.0.1:3000 acting as a "Man-in-the-Middle" proxy.

    The Buffer: When a request hits, we don't wait for the full HTTP handshake. We grab the first 1024 bytes of the raw stream.

    The Inspection:

        The tool scans the raw text for banned keywords (currently hardcoded: "ad", "banner").

        If Clean: It forwards a 200 OK response.

        If Dirty: It kills the connection immediately with a custom 403 FORBIDDEN header and a "🚫 AD BLOCKED" HTML payload.

🚀 Quick Start
Prerequisites

    Rust and Cargo installed (rustup).

Running the Inspector

    Clone the repo:
    Bash

git clone https://github.com/OP-88/Rust-ad_blocker.git
cd Rust-ad_blocker

Fire it up:
Bash

cargo run

You should see: 🛡️ Traffic Inspector Online at http://127.0.0.1:3000

Test the Filter:

    Clean Traffic: Open your browser or run:
    Bash

curl http://127.0.0.1:3000/hello

Result: ✅ Traffic Clean

Malicious Traffic: Try to sneak in an "ad":
Bash

        curl http://127.0.0.1:3000/banner_ads

        Result: 🚫 AD BLOCKED

🧠 Why Build This?

I built this as a Capstone project to bridge the gap between High-Level Web Development and Systems Programming.

I initially wanted to write a raw packet sniffer, but running raw sockets requires Root/Admin privileges. By pivoting to a TcpListener architecture, I engineered a solution that mimics a "User Space Firewall"—proving you don't need kernel access to control network traffic if you understand how TCP streams work.
📜 License

Open Source. Use it to learn, break things, and rebuild them better.
