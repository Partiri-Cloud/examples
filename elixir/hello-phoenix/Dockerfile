FROM elixir:1.18-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY mix.exs mix.lock* ./
RUN mix local.hex --force && mix local.rebar --force
RUN MIX_ENV=prod mix deps.get --only prod

COPY . .
RUN MIX_ENV=prod mix compile

RUN useradd -u 1001 -m appuser
USER 1001

ENV MIX_ENV=prod
ENV PORT=10000

EXPOSE 10000

CMD ["mix", "phx.server"]
