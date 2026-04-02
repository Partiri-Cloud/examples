# Partiri Examples

A collection of minimal, deploy-ready example applications for the [Partiri](https://partiri.com) platform, organized by runtime.

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

Every example follows the same workflow:

```bash
# 1. Clone the repo and navigate to an example
git clone https://github.com/partiri-cloud/examples.git
cd examples/node/hello-express

# 2. Initialize your Partiri config
partiri init

# 3. Create and deploy the service
partiri service create
partiri service deploy
```

## Prerequisites

- [Partiri CLI](https://partiri.com/docs/cli) installed
- A Partiri account with a workspace and project

## Documentation

- [Partiri Documentation](https://partiri.com/docs)
- [CLI Reference](https://partiri.com/docs/cli)
- [Framework Guides](https://partiri.com/docs/frameworks)
