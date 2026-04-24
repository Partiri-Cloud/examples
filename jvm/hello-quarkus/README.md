# hello-quarkus

A minimal Quarkus application ready to deploy on [Partiri](https://partiri.cloud).

## What's Included
- Quarkus 3.17 with a single JAX-RS resource
- Health check at `/health`
- Production Dockerfile (multi-stage, non-root UID 1001)
- Listens on `PORT` (default `10000`, matches the Partiri platform default)

## Deploy via the Partiri Dashboard

The Partiri service form has two modes. Choose one.

### Option A — Repository mode (deploy from Git)

1. Fork [partiri-cloud/examples](https://github.com/partiri-cloud/examples) or push this example to your own repo.
2. In the [Partiri dashboard](https://partiri.cloud), click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-quarkus` |
   | Project | *your project* |
   | Source | **Repository** |
   | Runtime | `jvm` |
   | Repository URL | `https://github.com/<you>/examples` |
   | Branch | `main` |
   | Root directory | `jvm/hello-quarkus` |
   | Build command | `./mvnw package -DskipTests` |
   | Run command | `java -jar target/quarkus-app/quarkus-run.jar` |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**. The platform builds and deploys automatically.

### Option B — Registry mode (deploy a pre-built image)

1. Build and push the image to your own registry:
   ```bash
   docker build -t ghcr.io/<you>/hello-quarkus:latest .
   docker push ghcr.io/<you>/hello-quarkus:latest
   ```
2. In the Partiri dashboard, click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-quarkus` |
   | Project | *your project* |
   | Source | **Registry image** |
   | Registry URL | `ghcr.io/<you>/hello-quarkus:latest` |
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
cd examples/jvm/hello-quarkus
partiri init
partiri service create
partiri service deploy
```

## Local Development

**Requirements:** Java 21, Maven 3.9 (or use the included wrapper)

```bash
./mvnw quarkus:dev
```

The server listens on port **10000** by default. Override with the `PORT` env var:

```bash
PORT=8080 ./mvnw quarkus:dev
```

Endpoints:
- `GET /` — welcome message
- `GET /health` — `{"status":"ok"}`

## Learn More
- [Deploying JVM on Partiri](https://partiri.cloud/documentation/frameworks#jvm)
- [Partiri CLI Reference](https://partiri.cloud/documentation/cli)
