defmodule HelloPhoenix.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      HelloPhoenixWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: HelloPhoenix.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
