# Module 6 - Rust

## Commit 1 Reflection Notes

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

### Code Analysis

The initial implementation focused on setting up a TCP listener and reading HTTP requests:

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
```

This implementation demonstrates several important Rust features:

- **Ownership and borrowing**: The `&mut stream` syntax borrows the stream mutably, allowing the BufReader to read from it without taking ownership
- **Iterator chaining**: The method chains multiple iterator methods (lines, map, take_while, collect) to process the input efficiently
- **Type inference**: Using `Vec<_>` allows Rust to infer the actual type of the vector elements
- **Result handling**: The `unwrap()` method extracts the value from a `Result` type, assuming success

### TCP and HTTP Protocol Basics

In this implementation, we're working with:

1. **TCP (Transmission Control Protocol)**: A connection-oriented protocol that establishes a reliable communication channel between two network endpoints.

   - The `TcpListener` binds to a specific IP address and port (127.0.0.1:7878)
   - The `incoming()` method provides an iterator over connection attempts
   - Each successful connection results in a `TcpStream` that can be read from and written to

2. **HTTP (Hypertext Transfer Protocol)**: An application-level protocol that runs on top of TCP.
   - HTTP follows a request-response pattern
   - Requests and responses have a specific format with headers and body
   - Each line in an HTTP message ends with CRLF (`\r\n`)
   - Headers and body are separated by an empty line (`\r\n\r\n`)

## Commit 2 Reflection Notes

In this update, I enhanced the web server to actually respond to client requests with HTML content.

### Enhanced `handle_connection` Method Analysis

The updated `handle_connection` method now does the following:

1. **Reading Request**: It still reads and parses the HTTP request from the client using `BufReader`.

2. **Creating HTTP Response**: It constructs a proper HTTP response with these components:

   - **Status Line**: `"HTTP/1.1 200 OK"` - This tells the browser the request was successful.
   - **Content-Length Header**: This specifies the size of the HTML content in bytes, which is important for the browser to know how much data to expect.
   - **Empty Line**: The `\r\n\r\n` sequence marks the end of headers and beginning of the content.
   - **HTML Content**: The actual HTML content that will be displayed in the browser.

3. **Serving HTML**: Instead of just printing the request, it now:

   - Reads the HTML file from disk using `fs::read_to_string("hello.html").unwrap()`
   - Formats a proper HTTP response including the HTML content
   - Sends the response back to the client using `stream.write_all(response.as_bytes()).unwrap()`

4. **HTTP Headers**: The `Content-Length` header tells the browser how many bytes to expect in the response body. This allows the browser to know when the entire response has been received.

The response format follows HTTP protocol standards, with headers separated from the body by a blank line, and each header on its own line with a name-value format.

### Code Deep Dive

The enhanced implementation shows how to construct and send an HTTP response:

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
```

Notable aspects of this implementation:

1. **File System Operations**: Rust's `fs` module provides functions for interacting with the file system. The `read_to_string` function reads a file's contents into a String.

2. **Error Handling**: The code uses `unwrap()` which will panic if the file doesn't exist or can't be read. In a production system, proper error handling would be essential.

3. **String Formatting**: The `format!` macro enables combining multiple values into a single string, similar to printf in C or String.format in Java.

4. **Binary Data Handling**: The `as_bytes()` method converts the string to a byte array, and `write_all()` ensures all bytes are written to the stream.

### HTTP Response Structure

The HTTP response we're sending follows this structure:

```
HTTP/1.1 200 OK\r\n         <- Status line
Content-Length: {length}\r\n <- Headers
\r\n                         <- Empty line separating headers from body
<!DOCTYPE html>...          <- Body (HTML content)
```

Each component serves a specific purpose:

- **Status Line**: Includes HTTP version, status code (200), and reason phrase (OK)
- **Headers**: Provide metadata about the response (here, just Content-Length)
- **Body**: The actual content being sent to the client

The status code 200 indicates that the request was successful. Other common status codes include:

- 404: Not Found
- 500: Internal Server Error
- 301/302: Redirects
- 403: Forbidden
- 401: Unauthorized

The Content-Length header is critical for the browser to know exactly how many bytes to read. Without it, the browser wouldn't know when the response ends, potentially causing timeouts or incomplete rendering.

### HTTP/1.1 Protocol Features

Our implementation leverages several key features of the HTTP/1.1 protocol:

1. **Text-based protocol**: HTTP/1.1 messages are human-readable text
2. **Stateless design**: Each request-response cycle is independent
3. **Connection handling**: HTTP/1.1 supports persistent connections (keep-alive)
4. **Request methods**: Our example handles GET requests, but HTTP defines many methods (POST, PUT, DELETE, etc.)

### Rust's Strengths for Server Implementation

This implementation showcases several of Rust's strengths for server-side applications:

1. **Memory safety**: Rust's ownership system prevents memory leaks and data races
2. **Performance**: Rust's zero-cost abstractions allow high-level code without runtime overhead
3. **Reliability**: Strong type system catches many errors at compile time
4. **Concurrency**: Though not shown here, Rust's ownership model makes concurrent programming safer

### Result in Browser

After implementing this change, the browser now properly displays the HTML content instead of showing a loading indicator indefinitely.

![Commit 2 screen capture](/assets/images/commit2.png)

This successful implementation demonstrates how a few lines of Rust code can create a functional web server that follows HTTP protocol standards and delivers content to browsers efficiently.
