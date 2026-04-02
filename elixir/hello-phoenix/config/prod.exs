import Config

config :hello_phoenix, HelloPhoenixWeb.Endpoint,
  http: [
    ip: {0, 0, 0, 0},
    port: String.to_integer(System.get_env("PORT") || "3000")
  ],
  server: true
