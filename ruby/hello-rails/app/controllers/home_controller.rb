class HomeController < ApplicationController
  def index
    render json: { message: 'Hello from Rails on Partiri!' }
  end

  def health
    render json: { status: 'ok' }
  end
end
