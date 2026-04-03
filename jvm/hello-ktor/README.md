# hello-ktor

A minimal Ktor application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Ktor 3.1 with Netty engine and a single routing file
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
   cd examples/jvm/hello-ktor
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

**Requirements:** Java 21, Gradle 8 (or use the included wrapper)

```bash
./gradlew run
```

The server starts on port 10000 by default. Set the `PORT` environment variable to override:

```bash
PORT=8080 ./gradlew run
```

Endpoints:
- `GET /` — welcome message
- `GET /health` — `{"status":"ok"}`

## Learn More
- [Deploying Ktor on Partiri](https://partiri.com/docs/frameworks/ktor)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
