# hello-fastapi

A minimal FastAPI application ready to deploy on [Partiri](https://partiri.cloud).

## What's Included
- FastAPI 0.115.12 with a single HTTP endpoint
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
   | Name | `hello-fastapi` |
   | Project | *your project* |
   | Source | **Repository** |
   | Runtime | `python` |
   | Repository URL | `https://github.com/<you>/examples` |
   | Branch | `main` |
   | Root directory | `python/hello-fastapi` |
   | Build command | `pip install -r requirements.txt` |
   | Run command | `uvicorn main:app --host 0.0.0.0 --port $PORT` |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**. The platform builds and deploys automatically.

### Option B — Registry mode (deploy a pre-built image)

1. Build and push the image to your own registry:
   ```bash
   docker build -t ghcr.io/<you>/hello-fastapi:latest .
   docker push ghcr.io/<you>/hello-fastapi:latest
   ```
2. In the Partiri dashboard, click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-fastapi` |
   | Project | *your project* |
   | Source | **Registry image** |
   | Registry URL | `ghcr.io/<you>/hello-fastapi:latest` |
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
cd examples/python/hello-fastapi
partiri init
partiri service create
partiri service deploy
```

## Local Development

```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
uvicorn main:app --host 0.0.0.0 --port 10000 --reload
```

Visit `http://localhost:10000` and `http://localhost:10000/health`. Override the port with `--port <n>` as needed.

## Learn More
- [Deploying Python on Partiri](https://partiri.cloud/documentation/frameworks#python)
- [Partiri CLI Reference](https://partiri.cloud/documentation/cli)
