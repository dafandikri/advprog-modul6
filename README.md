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

## Commit 3 Reflection Notes

In this update, I enhanced the web server to validate requests and respond differently based on the requested path.

### Request Validation and Selective Responses

The updated implementation now:

1. **Parses the request line** to determine which path the client is requesting
2. **Checks the path**:
   - If the path is `/` (root), it serves the `hello.html` page with a `200 OK` status
   - If the path is anything else (like `/bad`), it serves the `404.html` page with a `404 NOT FOUND` status

This is a basic implementation of routing, a fundamental concept in web servers that maps URLs to specific handlers or resources.

### Code Analysis

Here's the key part of the updated implementation:

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
```

Important changes and patterns in this code:

1. **Request parsing**: Instead of collecting all request headers, we only read the first line with `buf_reader.lines().next()` which contains the HTTP method, path, and version.

2. **Conditional logic**: The code uses a simple `if-else` pattern for routing decisions, which works for our basic case but would need a more sophisticated approach for a real-world application.

3. **Tuple assignment**: The line `let (status_line, filename) = if...` demonstrates Rust's tuple pattern matching, allowing us to get both values at once from the condition.

4. **HTTP status codes**: We're now sending different status codes (200 or 404) depending on the request path.

### Need for Refactoring

As our server grows more complex, having all logic in the `handle_connection` function would make the code harder to maintain. Refactoring helps by:

1. **Separating concerns**: Different functions can handle different aspects of the request-response cycle
2. **Improving readability**: Smaller, focused functions are easier to understand
3. **Enabling reuse**: Common operations can be extracted into reusable functions
4. **Facilitating testing**: Isolated functions are easier to test

A common refactoring approach for web servers is to separate:

- Request parsing
- Route handling
- Response generation
- Error handling

### Result in Browser

When accessing the root path (`http://127.0.0.1:7878/`), the browser displays the welcome page:

![Normal page](/assets/images/commit2.png)

When accessing an invalid path (`http://127.0.0.1:7878/bad`), the browser displays the 404 error page:

![404 page](/assets/images/commit3.png)

## Commit 4 Reflection Notes

In this update, I enhanced the web server to simulate a slow response, which helps demonstrate the limitations of a single-threaded server architecture.

### Simulating Slow Responses

The updated implementation now includes a special path `/sleep` that intentionally delays the response by 10 seconds:

```rust
let (status_line, filename) = match &request_line[..] {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
    "GET /sleep HTTP/1.1" => {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "hello.html")
    }
    _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
};
```

This simulation demonstrates a critical limitation of single-threaded servers: while one request is being processed, all other requests are blocked.

### The Problem with Single-Threaded Servers

When testing with two browser windows simultaneously - one accessing `/sleep` and the other accessing the root path `/` - I observed that the second request doesn't complete until the first one finishes. This happens because:

1. **Sequential Processing**: Our server processes one request at a time in the order they're received
2. **Blocking Operations**: When the server encounters the `thread::sleep(Duration::from_secs(10))` call, the entire server is blocked
3. **No Concurrency**: There's no ability to handle multiple clients simultaneously

In a real-world scenario, this would mean that:

- A single slow request could significantly degrade server performance
- During high traffic, users would experience increasing delays
- Resource-intensive operations would block all other users
- The server could easily become overwhelmed with even moderate traffic

### Thread Starvation Explained

What we're experiencing is called "thread starvation" - when a single long-running operation prevents other operations from being processed. In our case:

1. The main thread is tied up handling the `/sleep` request
2. All other incoming connections are queued by the OS
3. No progress can be made on other requests until the sleep operation completes
4. The server becomes effectively unresponsive for the duration of the slow operation

### Performance Implications

This experiment highlights several performance concerns:

1. **Poor Scalability**: As the number of clients increases, response times grow linearly or worse
2. **Vulnerability to Abuse**: Malicious clients could intentionally make slow requests to perform a denial of service attack
3. **Inefficient Resource Utilization**: While the CPU is mostly idle during the `sleep` call, it can't be used to serve other requests
4. **Timeout Problems**: Browser requests might timeout if too many slow requests are in the queue

### Potential Solutions

To overcome these limitations, several approaches could be implemented:

1. **Multi-threading**: Create a new thread for each connection, allowing concurrent processing

   ```rust
   for stream in listener.incoming() {
       let stream = stream.unwrap();
       thread::spawn(|| {
           handle_connection(stream);
       });
   }
   ```

2. **Thread Pools**: Pre-create a pool of worker threads to handle incoming requests, limiting resource consumption

   ```rust
   let pool = ThreadPool::new(4); // Create a pool with 4 threads
   for stream in listener.incoming() {
       let stream = stream.unwrap();
       pool.execute(|| {
           handle_connection(stream);
       });
   }
   ```

3. **Asynchronous I/O**: Use non-blocking I/O operations with Rust's async/await features
   ```rust
   async fn handle_connection(mut stream: TcpStream) {
       // Async implementation
   }
   ```

Each approach has its own advantages and trade-offs in terms of performance, resource usage, and complexity.

### Conclusion

This experiment clearly demonstrates why production web servers need to implement concurrency mechanisms. The single-threaded approach, while simple to understand and implement, is fundamentally limited in its ability to handle real-world web traffic where multiple clients need to be served simultaneously and some operations may take longer than others.

## Commit 5 Reflection Notes

In this update, I transformed our single-threaded server into a multithreaded one using a ThreadPool implementation. This allows the server to handle multiple requests concurrently, significantly improving its performance under load.

### Understanding ThreadPool

A ThreadPool is a collection of pre-initialized threads that stand ready to execute tasks. This approach offers several advantages over creating a new thread for each incoming request:

1. **Resource Management**: Creating and destroying threads is expensive. A thread pool reuses threads, reducing this overhead.
2. **Controlled Concurrency**: By limiting the number of threads, we prevent the system from being overwhelmed by too many concurrent operations.
3. **Load Balancing**: Work can be distributed among available threads efficiently.

### ThreadPool Implementation

The ThreadPool implementation consists of several key components:

#### 1. ThreadPool Structure

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
```

The ThreadPool holds:

- A vector of Worker instances (each owning a thread)
- A sender for a channel to communicate with the workers

#### 2. Job Type

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
```

This type represents a task to be executed, using:

- `Box<dyn FnOnce()>`: A boxed closure that can be executed once
- `Send`: Makes the Job transferable between threads
- `'static`: Ensures the Job lives as long as needed

#### 3. Worker Structure

```rust
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
```

Each Worker has:

- An identifier for debugging
- A JoinHandle to its thread

#### 4. Channel-based Communication

The implementation uses a channel (`mpsc`) to communicate between the ThreadPool and its Workers:

1. The ThreadPool sends jobs through the `sender`
2. Workers share an `Arc<Mutex<Receiver>>` to receive jobs
3. Each worker continuously attempts to receive and execute jobs

### Shared State Architecture

The implementation demonstrates several concurrent programming patterns:

1. **Message Passing**: The channel (sender/receiver) allows communication between threads
2. **Shared Ownership**: `Arc<T>` (Atomic Reference Counting) enables multiple threads to share ownership of the receiver
3. **Mutual Exclusion**: `Mutex<T>` ensures only one thread can access the receiver at a time, preventing race conditions
4. **Thread Synchronization**: The receiver blocks when no jobs are available, effectively synchronizing worker threads

### Updated Main Function

The server now creates a ThreadPool and uses it to handle incoming connections:

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

### Benefits of the Multithreaded Approach

With this implementation, our server can now:

1. **Handle Concurrent Requests**: Multiple clients can be served simultaneously
2. **Overcome the Slow Response Issue**: A single slow request no longer blocks the entire server
3. **Utilize Multiple CPU Cores**: The workload can be distributed across multiple cores
4. **Scale More Effectively**: The server can handle higher traffic loads with better response times

### Testing the Improvement

When testing with the slow response simulation (accessing `/sleep` in one browser and `/` in another), both requests are now processed concurrently:

- The `/sleep` route still takes 10 seconds to respond
- The `/` route responds immediately, regardless of ongoing slow requests
- The server remains responsive to new requests even while processing long-running ones

### Thread Safety Considerations

Rust's ownership system ensures our multithreaded implementation is safe:

1. **No Data Races**: The compiler prevents sharing mutable data across threads without proper synchronization
2. **Ownership Transfer**: When a closure is sent to a worker, ownership of captured variables is transferred
3. **Thread-Safe Primitives**: `Arc` and `Mutex` provide safe cross-thread access to shared data
4. **Safe Concurrency**: Rust guarantees memory safety and thread safety at compile time

### Further Improvements

While our ThreadPool implementation is functional, it could be enhanced with:

1. **Graceful Shutdown**: Adding a way to send a termination signal to all workers
2. **Dynamic Thread Scaling**: Adjusting the number of threads based on system load
3. **Task Prioritization**: Adding support for prioritized job execution
4. **Error Handling**: Improving error handling for worker threads
5. **Performance Metrics**: Collecting statistics on thread usage and response times

This multithreaded implementation demonstrates how Rust's powerful concurrency features can be used to build efficient, safe concurrent systems without the common pitfalls found in other languages.

## Commit Bonus Reflection Notes

In this bonus update, I implemented an alternative constructor method for the ThreadPool called `build`. This method demonstrates how to improve error handling compared to the original `new` constructor.

### The Builder Pattern and Error Handling

The new `build` function follows a pattern commonly seen in Rust libraries where:

- `new` functions are simple and may panic for invalid inputs
- `build` functions return a `Result` type for more graceful error handling

```rust
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size == 0 {
        return Err(PoolCreationError::ZeroSize);
    }

    // Create thread pool...

    Ok(ThreadPool { workers, sender })
}
```

### Custom Error Type

I implemented a custom error type to better represent the specific errors that might occur during thread pool creation:

```rust
#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSize,
    ThreadCreationError(String),
}
```

This provides more context about what went wrong compared to a simple assertion or generic error.

### Comparing `new` vs `build`

| Aspect                   | `new` Method                       | `build` Method                      |
| ------------------------ | ---------------------------------- | ----------------------------------- |
| **Error Handling**       | Panics on invalid size             | Returns `Result` with error details |
| **Recovery**             | None - program crashes             | Caller can handle errors gracefully |
| **Usage Complexity**     | Simpler - no error checking needed | Requires error handling code        |
| **Debugging**            | Less informative (just panic)      | More informative error details      |
| **Production Readiness** | Good for internal code             | Better for library interfaces       |

### Rust's "Constructor" Patterns

Unlike languages with dedicated constructors, Rust uses static methods as constructors. This provides flexibility in how objects are created:

1. **Default Constructor**: Using the `Default` trait
2. **Simple Constructor**: The conventional `new` function
3. **Error-Handling Constructor**: Functions like `build` that return `Result`
4. **Builder Pattern**: A separate builder struct with chainable methods

### Application in Our Server

Our updated main function demonstrates how to use the `build` method with proper error handling:

```rust
let pool = match ThreadPool::build(4) {
    Ok(pool) => pool,
    Err(e) => {
        eprintln!("Failed to create thread pool: {:?}", e);
        return;
    }
};
```

This pattern is more robust because:

1. Errors are explicitly handled rather than crashing the program
2. Error details are logged to help with debugging
3. The program can gracefully exit or try alternative approaches

### Worker Improvements

The Worker struct also got a `build` method that properly handles potential thread creation failures:

```rust
fn build(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)
    -> Result<Worker, std::io::Error> {
    let thread = match std::thread::Builder::new()
        .name(format!("worker-{}", id))
        .spawn(/* ... */) {
            Ok(thread) => thread,
            Err(e) => return Err(e),
        };

    Ok(Worker { id, thread })
}
```

This allows us to:

1. Name our threads for better debugging
2. Properly propagate thread creation errors
3. Handle resource exhaustion scenarios

### When to Use Each Approach

- Use `new` when:

  - Invalid inputs would indicate a programming error
  - The function is used internally where failures should crash tests
  - Simplicity is preferred and requirements are well-defined

- Use `build` when:
  - Failures might legitimately occur at runtime
  - The caller needs to handle errors gracefully
  - More detailed error information is beneficial
  - Creating a robust public API

### Conclusion

Adding this alternative constructor demonstrates a more production-ready approach to creating the ThreadPool. While the `new` method is simpler to use, the `build` method offers better error handling and more flexibility. This pattern is common in professional Rust libraries and follows the language's philosophy of making errors explicit and recoverable.
