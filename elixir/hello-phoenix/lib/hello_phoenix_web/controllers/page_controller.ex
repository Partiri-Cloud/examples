defmodule HelloPhoenixWeb.PageController do
  use Phoenix.Controller, formats: [:json]

  def index(conn, _params) do
    json(conn, %{message: "Hello from Phoenix on Partiri!"})
  end

  def health(conn, _params) do
    json(conn, %{status: "ok"})
  end
end
