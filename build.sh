#!/bin/bash
cleanup() {
    echo "cleaning up server and virtual tunnel interface\n"
    kill "$rust_space_tcp_pid"
    wait "$rust_space_tcp_pid"

    # Remove the Unix domain socket file
    echo "Removing Unix domain socket file..."
    rm -f /tmp/wtfistcp_unix_socket

    # Sometimes this isn't needed??
    # But let's make sure it's clean if it exists
    if ip link show mytun > /dev/null 2>&1; then
        sudo ip link delete mytun
    fi

    echo "Cleanup complete."
    exit
}

trap cleanup SIGINT SIGTERM

# Build the Rust project
if ! cargo build; then
    exit 1
fi

# Set capabilities on the binary
sudo setcap 'cap_net_raw,cap_net_admin+eip' ./target/debug/rust_space_tcp

# Run server in the background
./target/debug/rust_space_tcp &
rust_space_tcp_pid=$!

# Function to check if the interface exists
interface_exists() {
    ip link show mytun > /dev/null 2>&1
}

# Wait for the interface to be created by the application
while ! interface_exists; do
    echo "Waiting for mytun interface to be created...\n"
    sleep 1
done

# Configure the network interface
sudo ip addr add 10.0.0.1/24 dev mytun
sudo ip link set up dev mytun

wait "$rust_space_tcp_pid"
