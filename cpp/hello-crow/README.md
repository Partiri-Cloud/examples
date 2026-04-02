# hello-crow

A minimal C++ web application using [Crow](https://crowcpp.org) ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Crow v1.2.0 (header-only HTTP framework) with a single route
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
   cd examples/cpp/hello-crow
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

**Requirements:** GCC or Clang, CMake 3.20+, git, libssl-dev, zlib1g-dev

```bash
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
./build/hello-crow
```

The server starts on port 3000 by default. Set the `PORT` environment variable to override:

```bash
PORT=8080 ./build/hello-crow
```

Endpoints:
- `GET /` — welcome message
- `GET /health` — `{"status":"ok"}`

## Learn More
- [Deploying C++ on Partiri](https://partiri.com/docs/frameworks/cpp)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
