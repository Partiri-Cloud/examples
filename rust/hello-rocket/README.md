# hello-rocket

A minimal [Rocket](https://rocket.rs) application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Rocket 0.5 with `/` and `/health` endpoints
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
   cd examples/rust/hello-rocket
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
ROCKET_PORT=8080 cargo run
```

The server address and port are configured via `Rocket.toml`. Override at runtime with
`ROCKET_ADDRESS` and `ROCKET_PORT` environment variables. Partiri sets `PORT` — if you
need to forward it, set `ROCKET_PORT=$PORT` in the service env vars.

## Learn More
- [Deploying Rocket on Partiri](https://partiri.com/docs/frameworks/rocket)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
