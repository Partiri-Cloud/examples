# Partiri Examples

A collection of minimal, deploy-ready example applications for the [Partiri](https://partiri.cloud) platform, organized by runtime.

Every example listens on `PORT` (default **10000**, matches the Partiri platform default) and exposes a `/health` endpoint.

## Examples

### Node.js

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-express](node/hello-express/) | Express.js | Minimal Express HTTP server |
| [hello-nextjs](node/hello-nextjs/) | Next.js | Next.js with App Router |
| [hello-nestjs](node/hello-nestjs/) | NestJS | NestJS REST API |
| [hello-nuxt](node/hello-nuxt/) | Nuxt | Nuxt with server-side rendering |
| [hello-svelte](node/hello-svelte/) | SvelteKit | SvelteKit application |
| [hello-hono](node/hello-hono/) | Hono | Lightweight Hono HTTP server |
| [hello-astro](node/hello-astro/) | Astro | Astro with SSR adapter |
| [hello-remix](node/hello-remix/) | Remix | Remix full-stack application |

### Python

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-fastapi](python/hello-fastapi/) | FastAPI | Async API with FastAPI and Uvicorn |
| [hello-django](python/hello-django/) | Django | Django with Gunicorn |
| [hello-flask](python/hello-flask/) | Flask | Flask with Gunicorn |

### Go

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-gin](go/hello-gin/) | Gin | Gin HTTP server |
| [hello-fiber](go/hello-fiber/) | Fiber | Fiber HTTP server |

### Rust

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-actix](rust/hello-actix/) | Actix Web | Actix Web HTTP server |
| [hello-rocket](rust/hello-rocket/) | Rocket | Rocket HTTP server |
| [hello-axum](rust/hello-axum/) | Axum | Axum HTTP server |

### Ruby

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-rails](ruby/hello-rails/) | Rails | Rails application with Puma |

### Elixir

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-phoenix](elixir/hello-phoenix/) | Phoenix | Phoenix LiveView application |

### PHP

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-laravel](php/hello-laravel/) | Laravel | Laravel application |

### JVM

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-spring](jvm/hello-spring/) | Spring Boot | Spring Boot REST API |
| [hello-ktor](jvm/hello-ktor/) | Ktor | Kotlin Ktor HTTP server |
| [hello-quarkus](jvm/hello-quarkus/) | Quarkus | Quarkus JAX-RS application |

### .NET

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-aspnet](dotnet/hello-aspnet/) | ASP.NET Core | Minimal ASP.NET Core API |

### C++

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-crow](cpp/hello-crow/) | Crow | Crow HTTP server |

### Static

| Example | Framework | Description |
|---------|-----------|-------------|
| [hello-static](static/hello-static/) | HTML/CSS/JS | Static site served by Caddy |

## How to Deploy

Each example's README includes three deployment paths:

1. **Partiri Dashboard — Repository mode**: fork the repo, paste the repo URL into the service form with the example's build/run command values.
2. **Partiri Dashboard — Registry mode**: build the example's Dockerfile, push the image to your registry (GHCR, DockerHub, etc.), and paste the image URL into the service form.
3. **Partiri CLI**:

```bash
git clone https://github.com/partiri-cloud/examples.git
cd examples/node/hello-express
partiri init
partiri service create
partiri service deploy
```

## Prerequisites

- A Partiri account with a workspace and project — sign up at [partiri.cloud](https://partiri.cloud)
- [Partiri CLI](https://partiri.cloud/documentation/cli) installed (for the CLI path)
- Docker (for the Registry-mode path)

## Documentation

- [Partiri Documentation](https://partiri.cloud/documentation)
- [Framework Guides](https://partiri.cloud/documentation/frameworks)
- [CLI Reference](https://partiri.cloud/documentation/cli)
