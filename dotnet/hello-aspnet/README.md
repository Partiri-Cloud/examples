# hello-aspnet

A minimal ASP.NET Core application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- ASP.NET Core 9 Minimal API with a single HTTP endpoint
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
   cd examples/dotnet/hello-aspnet
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

**Requirements:** .NET 9 SDK

```bash
dotnet run
```

The server starts on port 3000 by default. Set the `PORT` environment variable to override:

```bash
PORT=8080 dotnet run
```

Endpoints:
- `GET /` — welcome message
- `GET /health` — `{"status":"ok"}`

## Learn More
- [Deploying ASP.NET Core on Partiri](https://partiri.com/docs/frameworks/aspnet)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
