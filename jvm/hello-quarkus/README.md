# hello-quarkus

A minimal Quarkus application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Quarkus 3.17 with a single JAX-RS resource
- Health check at `/health`
- Production Dockerfile (multi-stage, non-root UID 1001)

## Deploy to Partiri

### Prerequisites
- [Partiri CLI](https://partiri.com/docs/cli) installed
- A Partiri account with a workspace and project

### Steps
1. Clone and navigate to this example
   ```bash
   git clone https://github.com/partiri-cloud/examples.git
   cd examples/jvm/hello-quarkus
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

**Requirements:** Java 21, Maven 3.9 (or use the included wrapper)

```bash
./mvnw quarkus:dev
```

The server starts on port 10000 by default. Set the `PORT` environment variable to override:

```bash
PORT=8080 ./mvnw quarkus:dev
```

Endpoints:
- `GET /` — welcome message
- `GET /health` — `{"status":"ok"}`

## Learn More
- [Deploying Quarkus on Partiri](https://partiri.com/docs/frameworks/quarkus)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
