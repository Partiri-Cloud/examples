# hello-phoenix

A minimal Phoenix application ready to deploy on [Partiri](https://partiri.cloud).

## What's Included
- Phoenix 1.7 with a single HTTP endpoint
- Health check at `/health`
- Production Dockerfile
- Listens on `PORT` (default `10000`, matches the Partiri platform default)

## Deploy via the Partiri Dashboard

The Partiri service form has two modes. Choose one.

### Option A — Repository mode (deploy from Git)

1. Fork [partiri-cloud/examples](https://github.com/partiri-cloud/examples) or push this example to your own repo.
2. In the [Partiri dashboard](https://partiri.cloud), click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-phoenix` |
   | Project | *your project* |
   | Source | **Repository** |
   | Runtime | `elixir` |
   | Repository URL | `https://github.com/<you>/examples` |
   | Branch | `main` |
   | Root directory | `elixir/hello-phoenix` |
   | Build command | `mix deps.get --only prod && MIX_ENV=prod mix compile` |
   | Run command | `MIX_ENV=prod mix phx.server` |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**. The platform builds and deploys automatically.

### Option B — Registry mode (deploy a pre-built image)

1. Build and push the image to your own registry:
   ```bash
   docker build -t ghcr.io/<you>/hello-phoenix:latest .
   docker push ghcr.io/<you>/hello-phoenix:latest
   ```
2. In the Partiri dashboard, click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-phoenix` |
   | Project | *your project* |
   | Source | **Registry image** |
   | Registry URL | `ghcr.io/<you>/hello-phoenix:latest` |
   | Credentials | *a registry secret* (required for private images) |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**.

> No `Port` field is needed — the platform injects `PORT=10000` and this example listens on it.
>
> Other environment variables are optional — add them in the Environment Variables section as needed.

## Deploy via the CLI

```bash
git clone https://github.com/partiri-cloud/examples.git
cd examples/elixir/hello-phoenix
partiri init
partiri service create
partiri service deploy
```

## Local Development

```bash
mix deps.get
mix phx.server
```

The app will be available at `http://localhost:10000`. Override with the `PORT` env var:

```bash
PORT=8080 mix phx.server
```

## Learn More
- [Deploying Elixir on Partiri](https://partiri.cloud/documentation/frameworks#elixir)
- [Partiri CLI Reference](https://partiri.cloud/documentation/cli)
