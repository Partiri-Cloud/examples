import Config

config :hello_phoenix, HelloPhoenixWeb.Endpoint,
  http: [
    ip: {0, 0, 0, 0},
    port: String.to_integer(System.get_env("PORT") || "10000")
  ],
  server: true
