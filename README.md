# 🦀 Rust Traffic Inspector

**A Low-Level, User-Space Network Traffic Filter Built from Scratch**

> *"To understand the Web, you have to break it down to the bytes."*

This isn't a wrapper around Nginx or a high-level proxy library. This is a **manual implementation** of a Traffic Inspector using raw TCP streams in Rust. It captures, decodes, and filters HTTP packets at the byte level—all running in user space without `sudo`.

---

## ⚡ Key Features

<table>
<tr>
<td width="50%">

### 🚀 **Zero Dependencies (Almost)**
Built primarily using Rust's standard library (`std::net`, `std::io`)

### 📡 **Raw Packet Capture**
Reads network traffic directly into a 1024-byte stack buffer

### 🔍 **Manual HTTP Decoding**
Converts raw `u8` byte streams into readable text using lossy UTF-8 decoding

</td>
<td width="50%">

### 🛡️ **Active Traffic Control**
Detects signatures (like `ad` or `banner`) and injects custom HTTP `403 Forbidden` in real-time

### 👤 **Non-Root Execution**
Runs entirely in user space (Layer 7) via `TcpListener`, bypassing raw sockets

### ⚙️ **First Principles Engineering**
No black boxes—every byte is manually inspected and processed

</td>
</tr>
</table>

---

## 🛠️ How It Works

Most ad blockers work as browser extensions. Most firewalls work at the Kernel level. **This tool sits in the middle.**

```
┌─────────────┐
│   Client    │
│  (Browser)  │
└──────┬──────┘
       │ HTTP Request
       ▼
┌─────────────────────────────────────┐
│  🦀 Rust Traffic Inspector          │
│  (User Space Proxy)                 │
│                                     │
│  1️⃣ Bind to 127.0.0.1:3000         │
│  2️⃣ Accept TCP connection          │
│  3️⃣ Read 1024 bytes into buffer    │
│  4️⃣ Scan for banned keywords       │
│  5️⃣ Inject 403 OR forward 200      │
└─────────────────────────────────────┘
```

### The Architecture

1. **The Listener**: Binds to `127.0.0.1:3000` acting as a "Man-in-the-Middle" proxy

2. **The Buffer**: Grabs the first 1024 bytes of the raw TCP stream (no high-level HTTP parsing)

3. **The Inspection**:
   - Scans raw text for banned keywords (currently: `"ad"`, `"banner"`)
   - **If Clean**: Forwards `200 OK` response
   - **If Dirty**: Kills connection with custom `403 FORBIDDEN` + `🚫 AD BLOCKED` HTML

---

## 🚀 Quick Start

### Prerequisites

- Rust and Cargo installed via [rustup](https://rustup.rs/)

### Installation

```bash
# Clone the repository
git clone https://github.com/OP-88/Rust-ad_blocker.git
cd Rust-ad_blocker

# Run the inspector
cargo run
```

**Expected Output:**
```
🛡️ Traffic Inspector Online at http://127.0.0.1:3000
```

---

## 🧪 Testing

### ✅ Clean Traffic

```bash
curl http://127.0.0.1:3000/hello
```

**Response:**
```
✅ Traffic Clean
```

---

### 🚫 Blocked Traffic

```bash
curl http://127.0.0.1:3000/banner_ads
```

**Response:**
```html
HTTP/1.1 403 Forbidden

<h1>🚫 AD BLOCKED</h1>
<p>This request was flagged by the Traffic Inspector.</p>
```

---

## 🧠 Why Build This?

I built this as a **Capstone project** to bridge the gap between:
- High-Level Web Development
- Low-Level Systems Programming

**The Challenge:**  
I initially wanted to write a raw packet sniffer, but raw sockets require `root`/admin privileges.

**The Solution:**  
By pivoting to a `TcpListener` architecture, I engineered a **User Space Firewall**—proving you don't need kernel access to control network traffic if you understand how TCP streams work.

---

## 📚 What You'll Learn

- ✅ How to read raw TCP byte streams in Rust
- ✅ Manual HTTP request parsing (no external crates)
- ✅ Building a Layer 7 proxy without root access
- ✅ Real-time packet inspection and filtering
- ✅ Designing lightweight, dependency-minimal systems

---

## 🔧 Technical Stack

| Component | Technology |
|-----------|------------|
| **Language** | Rust 🦀 |
| **Network Layer** | Layer 7 (Application) |
| **Core Libraries** | `std::net::TcpListener`, `std::io` |
| **Buffer Size** | 1024 bytes (stack-allocated) |
| **Execution** | User space (no sudo required) |

---

## 🎯 Roadmap

- [ ] Add configurable blocklist (JSON/TOML file)
- [ ] Implement regex pattern matching for advanced filtering
- [ ] Add HTTPS support with TLS termination
- [ ] Create WebUI dashboard for real-time traffic monitoring
- [ ] Benchmark performance vs traditional ad blockers

---

## 👤 Author

**OP-88**

Built with ❤️ for systems programmers who want to understand the Web at the byte level.

---

## 📜 License

Open Source. Use it to learn, break things, and rebuild them better.

---

## 🌟 Like This Project?

If you found this helpful or learned something new:
- ⭐ Star the repository
- 🔄 Fork it and build your own version
- 📢 Share it with fellow Rustaceans

---

**"Security through understanding, not abstraction."**
