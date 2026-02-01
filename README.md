# TV Prog Rust API

This is a Rust implementation of a TV program API. 

## Motivation

The original API was written in Java Spring Boot, but it consumes too much memory. 
This Rust version aims to provide a more efficient and faster alternative.

The current API runs on a small VPS with only 2GB of RAM. The deployed version consumes at least 1.5GB of RAM, which is too much for such a simple API.
The goal is to reduce the memory footprint while maintaining functionality.

## Getting started

### Install Rust

This project require Rust on your machine. Please visit [rust-lang.org](https://www.rust-lang.org/tools/install) for installation instructions.

### Clone the repository

```bash
git clone github.com/PiotrFLEURY/tv_prog_rust_api.git
cd tv_prog_rust_api
```

### VsCode Setup

It is recommended to use [VsCode](https://code.visualstudio.com/) with the [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension for development.

### Build and run the project

```bash
cargo build
cargo run
```

### Run tests

```bash
cargo test
```

## Configuration

### Environment Variables

The application can be configured using the following environment variables:

- `CONNECTION_STRING`: The connection string for the PostgreSQL database. Default is `postgres://postgres:postgres@localhost:5432/postgres`.
- `XMLTV_BASE_URL`: The base URL for XMLTV data. Default is `https://xmltvfr.fr/xmltv/`.
- `TZ`: The timezone for the application. Default is `Europe/Paris`.