# Fast LoL Client

Fast LoL Client is a custom client for League of Legends built using the Iced framework. This project is under active development and aims to provide a functional and efficient alternative to the official Riot client.

## Installation

### Prerequisites

- Rust and Cargo installed
- Riot Games League of Legends installed

### Steps

1. Clone the repository:

```bash
git clone https://github.com/gqsnt/fast_lol_client.git
cd fast_lol_client
```

2. Build the project:

```bash
cargo build --release
```

3. Configure the path to your League of Legends installation by creating a `config.toml` file in the root directory with the following content:

```toml
riot_path="C:\\Riot Games"
```

## Usage

To run the client, execute the following command:

```bash
cargo run --release
```
