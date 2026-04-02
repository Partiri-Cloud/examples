defmodule HelloPhoenixWeb.Router do
  use Phoenix.Router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", HelloPhoenixWeb do
    pipe_through :api

    get "/", PageController, :index
    get "/health", PageController, :health
  end
end
