## 6. 🧠 AI Prompt Journal
*I used Generative AI to bridge the gap between my web development knowledge and systems programming. Here is the record of my learning process:*

### Entry 1: Architecture Decisions
* **Prompt Used:**
    > "I want to build a packet filter in Rust. Should I use a raw packet sniffer or a simple TCP listener? I am on a university computer without admin rights."
* **AI's Response Summary:**
    The AI explained that raw packet sniffing (Layer 3) requires root/administrator privileges to access the network card driver. It suggested using a `TcpListener` (Layer 7) instead, which behaves like a web server and runs in "User Space" without special permissions.
* **My Evaluation:**
    **Very Helpful.** This saved me from failing the project. If I had tried to use a "Sniffer" library (like `pcap`), my demo would have crashed on any computer where I didn't have Admin rights.

### Entry 2: Handling Network Data
* **Prompt Used:**
    > "How do I read data from a TcpStream in Rust and check if it contains a specific word like 'ad'?"
* **AI's Response Summary:**
    The AI provided the `stream.read(&mut buffer)` syntax. Crucially, it taught me that network data arrives as raw bytes (`u8`), not text. It introduced me to `String::from_utf8_lossy`, which safely converts those bytes into a String I can search using `.contains()`.
* **My Evaluation:**
    **Critical.** Coming from JavaScript, I expected data to just "be text" automatically. The AI clarified how low-level memory buffers work in Rust.

### Entry 3: Debugging Encoding (The "Mojibake" Bug)
* **Prompt Used:**
    > "Why are my emojis showing as weird symbols `ðŸš«` in the browser instead of the 🚫 icon?"
* **AI's Response Summary:**
    The AI identified this as a text encoding issue. Since a raw TCP stream doesn't send HTTP headers by default, the browser guessed the wrong encoding (ISO-8859-1). The AI suggested manually adding the header `Content-Type: text/html; charset=utf-8` to my response string.
* **My Evaluation:**
    **Educational.** It turned a frustrating bug into a lesson about how HTTP headers actually control browser rendering.
