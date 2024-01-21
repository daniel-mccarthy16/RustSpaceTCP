# Custom TCP/IP Stack with Rust

## Overview
This project provides a custom implementation of the TCP/IP stack in Rust, leveraging the kernel's TUN interface feature. It allows users to route TCP/IP traffic through a Rust-based stack, offering a deeper insight into network protocol handling and Rust programming.

## Features
- **Custom TCP/IP Implementation**: A tailor-made TCP/IP stack written in Rust, providing hands-on experience with low-level network protocol handling.
- **TUN Interface Integration**: Utilizes the kernel's TUN interface to intercept and manage TCP/IP traffic, offering a practical approach to network programming.
- **Transparent Socket API Override**: Includes a C library that overrides the standard socket API. It can be dynamically loaded at runtime, enabling existing C applications to use the custom Rust TCP/IP stack without modifying the original source code.

## Project Goals
This project is designed for educational purposes, aiming to:
- Gain some practical experience with Rust programming, particularly in the context of network protocol implementation.
- Gain a deep understanding of UNIX socket programming and the TCP protocol.

## Getting Started
### Prerequisites
- Rust programming environment
- C compiler (e.g., GCC)
- Knowledge of TCP/IP protocols and UNIX socket programming

### Installation and Usage
1. **Clone the repository:**
   ```sh
   git clone https://github.com/daniel-mccarthy16/wtf-is-tcp
   cd wtf_is_tcp
   ```

2. **Build and start the server**
 This script (`build.sh`) automates the process of setting up and running the server. Here's what it does:
    - **Compiles the Rust Project:** Uses `cargo build` to compile the project.
    - **Sets Necessary Capabilities:** Grants the binary the necessary capabilities for network operations (`cap_net_raw`, `cap_net_admin`) using `setcap`.
    - **Starts the Server:** Runs the compiled server in the background.
    - **Waits for the Virtual Interface:** Checks for the creation of the `mytun` virtual interface and waits until it is ready.
    - **Configures the Network Interface:** Assigns an IP address to `mytun` and brings the interface up.
    - **Clean-Up on Termination:** Handles the clean-up process when the server is terminated, including killing the server process, removing the Unix domain socket file, and optionally deleting the `mytun` interface if it exists.
    ```sh
    ./build.sh
    ```

3. **Compile the C socket override library:**
    ```sh 
    gcc -shared -o libsocketoverride.so -fPIC ./shared_sockets_lib/socket_override.c
    ```

4. **Clone the repository:**
    ```sh 
    LD_PRELOAD=./libsocketoverride.so ./your_c_application
    ```
