# bamboo_api

A Rust client library for the [Bamboo Data Center REST API](https://docs.atlassian.com/bamboo/REST/latest/).

The code in `src/generated/` is generated from the official Bamboo OpenAPI spec using the `codegen` tool in this workspace.

## Requirements

- Rust 1.85+ (edition 2024)

## Building

```sh
cargo build
```

## Regenerating the client

If the OpenAPI spec or the codegen fixes change, regenerate `src/generated/` with:

```sh
make generate
```

This will:
1. Download the Bamboo OpenAPI spec into `openapi/swagger.v3.json` (skipped if already present).
2. Run the `codegen` tool, which applies spec fixes and writes new source files to `src/generated/`.

To force a fresh download of the spec, delete `openapi/swagger.v3.json` before running `make generate`.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
bamboo_api = { path = "path/to/bamboo_api" }
```

Create a client and call API methods:

```rust
use bamboo_api::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::new()
        .with_base_url("https://bamboo.example.com")
        .with_api_key("your-api-token");

    let projects = client.get_projects(None, None).await?;
    println!("{:?}", projects);

    Ok(())
}
```

## Workspace structure

| Crate | Description |
|-------|-------------|
| `.` (`bamboo_api`) | Published library — re-exports everything from `src/generated/` |
| `codegen/` | Binary tool that downloads the spec and generates `src/generated/` |
