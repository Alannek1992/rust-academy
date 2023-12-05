# Messaging System

Messaging System is designed to showcase various concepts and techniques in Rust programming for building a simple yet functional messaging system. The project is organized as a workspace, including a shared library, `common`, and two binaries, `server` and `client`, to demonstrate the functionality of the messaging system.

## Features

- **Messaging Interface:** Send and receive messages using the `common` shared library.
- **Concurrency:** Utilize Rust's powerful concurrency features to handle multiple messages concurrently.
- **Networking:** Basic networking concepts in Rust allowing communication between the `server` and `client` binaries.

## Project Structure

The project is organized as a Rust workspace, consisting of the following components:

- **`common`:** A shared library containing the core messaging functionalities.
- **`server`:** Binary which operates as server receiving the messages from clients and broadcasting them
- **`client`:** Binary for sending the messages to the server and receiving other messages from the clients.