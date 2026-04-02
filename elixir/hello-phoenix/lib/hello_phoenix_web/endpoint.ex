defmodule HelloPhoenixWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :hello_phoenix

  plug Plug.RequestId
  plug Plug.Parsers,
    parsers: [:urlencoded, :json],
    json_decoder: Jason

  plug HelloPhoenixWeb.Router
end
