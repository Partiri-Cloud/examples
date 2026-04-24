# hello-crow

A minimal C++ web application using [Crow](https://crowcpp.org) ready to deploy on [Partiri](https://partiri.cloud).

## What's Included
- Crow v1.2.0 (header-only HTTP framework) with a single route
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
   | Name | `hello-crow` |
   | Project | *your project* |
   | Source | **Repository** |
   | Runtime | `cpp` |
   | Repository URL | `https://github.com/<you>/examples` |
   | Branch | `main` |
   | Root directory | `cpp/hello-crow` |
   | Build command | `cmake -B build -DCMAKE_BUILD_TYPE=Release && cmake --build build` |
   | Run command | `./build/hello-crow` |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**. The platform builds and deploys automatically.

### Option B — Registry mode (deploy a pre-built image)

1. Build and push the image to your own registry:
   ```bash
   docker build -t ghcr.io/<you>/hello-crow:latest .
   docker push ghcr.io/<you>/hello-crow:latest
   ```
2. In the Partiri dashboard, click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-crow` |
   | Project | *your project* |
   | Source | **Registry image** |
   | Registry URL | `ghcr.io/<you>/hello-crow:latest` |
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
cd examples/cpp/hello-crow
partiri init
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

The server listens on port **10000** by default. Override with the `PORT` env var:

```bash
PORT=8080 ./build/hello-crow
```

Endpoints:
- `GET /` — welcome message
- `GET /health` — `{"status":"ok"}`

## Learn More
- [Deploying C++ on Partiri](https://partiri.cloud/documentation/frameworks#cpp)
- [Partiri CLI Reference](https://partiri.cloud/documentation/cli)
