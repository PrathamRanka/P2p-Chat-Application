# P2P Chat

A fully decentralized peer-to-peer chat application built with Rust, powered by Iroh and iroh-gossip.

No servers. No databases. No centralized infrastructure.

Users can create chat rooms, share invite tickets, and exchange messages directly over the Iroh network.

---

## Features

### Core Features

* Fully peer-to-peer architecture
* No central server required
* Real-time messaging
* Topic-based chat rooms
* Invite users using room tickets
* Persistent peer identity
* NAT traversal handled automatically by Iroh
* Multi-peer communication
* Cross-platform support

### User Experience

* User nicknames
* Message timestamps
* Colored terminal output
* Graceful disconnect handling
* Online peer indicators
* Local chat history persistence
* Room creation and joining

### Advanced Features

* Persistent identity using SecretKey
* Multiple chat rooms
* Reconnection support
* Room discovery through tickets
* Message history loading
* Terminal UI (TUI)

---

## Tech Stack

| Component     | Technology  |
| ------------- | ----------- |
| Language      | Rust        |
| Networking    | Iroh        |
| Messaging     | iroh-gossip |
| Runtime       | Tokio       |
| Serialization | Serde       |
| Storage       | JSON Files  |
| Terminal UI   | Crossterm   |

---

## Architecture

```text
┌─────────────┐
│   User A    │
└──────┬──────┘
       │
┌──────▼──────┐
│ Iroh Node A │
└──────┬──────┘
       │
════ Gossip Topic ════
       │
┌──────▼──────┐
│ Iroh Node B │
└──────┬──────┘
       │
┌──────▼──────┐
│   User B    │
└─────────────┘
```

Messages are propagated through Iroh Gossip, allowing all peers connected to the same topic to receive updates.

---

## Installation

### Prerequisites

* Rust 1.80+
* Cargo

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:

```bash
rustc --version
cargo --version
```

---

## Clone Repository

```bash
git clone https://github.com/yourusername/p2p-chat.git

cd p2p-chat
```

---

## Dependencies

```bash
cargo add iroh
cargo add iroh-gossip
cargo add tokio --features full
cargo add anyhow
cargo add serde
cargo add serde_json
cargo add chrono
cargo add crossterm
cargo add rand
```

---

## Running the Application

Build:

```bash
cargo build --release
```

Run:

```bash
cargo run
```

---

## Usage

### Create a Room

```bash
cargo run -- create
```

Output:

```text
Room created successfully

Ticket:
iroh://abcd1234xyz...
```

Share the generated ticket with other users.

---

### Join a Room

```bash
cargo run -- join <ticket>
```

Example:

```bash
cargo run -- join iroh://abcd1234xyz...
```

---

### Send Messages

```text
[Pratham]:
Hello everyone!
```

---

### Receive Messages

```text
[Alice] 22:14
Hi there!

[Bob] 22:15
Welcome!
```

---

## Project Structure

```text
src/
├── main.rs
├── cli.rs
├── chat.rs
├── gossip.rs
├── identity.rs
├── storage.rs
├── ticket.rs
├── ui.rs
└── types.rs

data/
├── identity.json
└── history/

Cargo.toml
README.md
```

---

## Development Roadmap

### Phase 1

* [x] Iroh endpoint creation
* [x] Gossip protocol setup
* [x] Topic creation
* [x] Broadcast messages
* [x] Receive messages

### Phase 2

* [ ] Room tickets
* [ ] User nicknames
* [ ] Improved CLI
* [ ] Room management

### Phase 3

* [ ] Local persistence
* [ ] Chat history
* [ ] Reconnection support
* [ ] Peer metadata

### Phase 4

* [ ] Full TUI
* [ ] Online users list
* [ ] Multiple rooms
* [ ] File sharing

---

## Important Iroh Concepts

### Endpoint

Represents a peer on the network and manages connections.

### Router

Routes protocol traffic between peers.

### Gossip

Handles message propagation across connected peers.

### TopicId

Unique identifier for a chat room.

### SecretKey

Persistent identity of a peer.

### Ticket

A shareable invite allowing other peers to join a room.

---

## Example Workflow

### User A

```bash
cargo run -- create
```

Generates:

```text
iroh://ticket123
```

---

### User B

```bash
cargo run -- join iroh://ticket123
```

---

### Start Chatting

```text
[UserA]
Hello!

[UserB]
Hey!
```

Messages are exchanged directly between peers without any server.

---

## Why Iroh?

Traditional chat applications rely on centralized infrastructure.

This project uses Iroh to provide:

* Direct peer-to-peer communication
* NAT traversal
* Secure identities
* Global connectivity
* Distributed messaging

Resulting in a serverless, censorship-resistant communication system.

---

## Future Enhancements

* End-to-end encryption
* File sharing
* Voice chat
* Message reactions
* Private channels
* Mobile support
* WebAssembly client
* Message synchronization
* Presence indicators

---

## License

MIT License

---

## References

* https://docs.iroh.computer
* https://docs.iroh.computer/examples/chat
* https://github.com/n0-computer/iroh
* https://github.com/n0-computer/iroh-gossip
