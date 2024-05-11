---------------

<div>
    <img src="https://img.shields.io/badge/Language%20-Rust-orange.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Tool%20-Chat Server-darkgreen.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Operational Sys%20-Windows, Linux-darkred.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Crates%20-tokio, clap-lightgreen.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Type%20-Command line tools for utilities-beige.svg" style="max-width: 100%;">
</div>

----------------

# Madness Chat
Chat server designed for casual and simple use in the terminal. Built in rust lang with an H.P Lovecraft theme (just because I'm a fan). Focusing only on the occasional use of a simple terminal.

#

![Peek 11-05-2024 04-39](https://github.com/JMoreira2Dev/MadnessChat/assets/167461650/8bc05a8f-9d86-4156-bc78-4ff9a22bd22b)

- ***The server is designed not to store messages after the connection is closed. The messages sent by each user are identified with their respective IPs within the network, so it is recommended to implement a password***

## Installation:

```bash
  git clone https://github.com//JMoreira2Dev/MadnessChat.git
  cd MadnessChat
  cargo build --release --manifest-path madness_server/Cargo.toml --target-dir .
  cargo build --release --manifest-path madness_client/Cargo.toml --target-dir .
```

or

```bash
  cargo install --git https://github.com//JMoreira2Dev/MadnessChat.git madness_client
  cargo install --git https://github.com//JMoreira2Dev/MadnessChat.git madness_server
```

## Usage: 

<h3>Create Server</h3>

> ./madness_server localhost 6060 -p Password1234

<h3>Launch Client</h3>

> ./madness_client -i IP -p PORT -u Kyle -P Password1234

##

- Inspiration from: [Creating a Chat Server with async Rust and Tokio](https://youtu.be/T2mWg91sx-o?si=TM3OGfuXQkPaAI-Y)
