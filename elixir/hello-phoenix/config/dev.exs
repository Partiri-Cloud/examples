import Config

config :hello_phoenix, HelloPhoenixWeb.Endpoint,
  http: [ip: {0, 0, 0, 0}, port: 10000],
  server: true,
  debug_errors: true
