# hello-fiber

A minimal [Fiber](https://gofiber.io) application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Fiber with a single HTTP endpoint
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
   cd examples/go/hello-fiber
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
go mod tidy
go run .
```

The server starts on port 3000 by default. Set the `PORT` environment variable to override:
```bash
PORT=8080 go run .
```

## Learn More
- [Deploying Go on Partiri](https://partiri.com/docs/frameworks/go)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
