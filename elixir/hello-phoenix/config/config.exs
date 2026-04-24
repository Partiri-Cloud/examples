import Config

config :hello_phoenix, HelloPhoenixWeb.Endpoint,
  http: [ip: {0, 0, 0, 0}, port: 10000],
  server: false

config :phoenix, :json_library, Jason

import_config "#{config_env()}.exs"
