# Module 6 - Rust

### Commit 1 Reflection Notes: Handling Connection and Checking Response

In this commit, I implemented a basic TCP server that can listen for and process HTTP requests. The key addition was the `handle_connection` method, which processes incoming TCP streams.

### Understanding the `handle_connection` Method

The `handle_connection` method is responsible for processing each client connection to our web server. Here's a breakdown of how it works:

1. **Parameter**: It takes a mutable `TcpStream` that represents the connection to the client.

2. **BufReader**: The method creates a buffered reader from the stream using `BufReader::new(&mut stream)`. This provides an efficient way to read data from the stream line by line.

3. **Reading HTTP Request**:

   - The method reads lines from the buffered reader using `.lines()`.
   - It unwraps each result with `.map(|result| result.unwrap())`.
   - It continues reading lines as long as they're not empty using `.take_while(|line| !line.is_empty())`.
   - It collects all these lines into a vector with `.collect()`.

4. **Printing the Request**: Finally, it prints the HTTP request headers to the console with `println!("Request: {:#?}", http_request)`.

The HTTP request structure reveals important information about what the browser is requesting:

- The first line (typically something like `GET / HTTP/1.1`) shows the HTTP method, requested path, and protocol version.
- Subsequent lines contain headers with information about the client, accepted content types, and other HTTP-related metadata.

This implementation only receives and displays requests but doesn't send any response back to the browser yet, which is why the browser appears to be loading indefinitely when accessing the server.
