# hello-axum

A minimal [Axum](https://github.com/tokio-rs/axum) application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Axum 0.8 with a Tokio async runtime, `/` and `/health` endpoints
- Health check at `/health`
- Production Dockerfile with multi-stage build

## Deploy to Partiri

### Prerequisites
- [Partiri CLI](https://partiri.com/docs/cli) installed
- A Partiri account with a workspace and project

### Steps
1. Clone and navigate to this example
   ```bash
   git clone https://github.com/partiri-cloud/examples.git
   cd examples/rust/hello-axum
   ```

2. Initialize your Partiri config
   ```bash
   partiri init
   ```

3. Create and deploy the service
   ```bash
   partiri service create
   partiri service deploy
   ```

## Local Development

```bash
cargo run
# or with a custom port
PORT=8080 cargo run
```

The server listens on `PORT` (default `3000`).

## Learn More
- [Deploying Axum on Partiri](https://partiri.com/docs/frameworks/axum)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
