# hello-spring

A minimal Spring Boot application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Spring Boot 3.4 with a single REST controller
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
   cd examples/jvm/hello-spring
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
./gradlew bootRun
```

The server starts on port 3000 by default. Set the `PORT` environment variable to override:

```bash
PORT=8080 ./gradlew bootRun
```

Endpoints:
- `GET /` — welcome message
- `GET /health` — `{"status":"ok"}`

## Learn More
- [Deploying Spring Boot on Partiri](https://partiri.com/docs/frameworks/spring)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
