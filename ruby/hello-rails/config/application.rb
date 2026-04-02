require 'rails'
require 'action_controller/railtie'

module HelloRails
  class Application < Rails::Application
    config.load_defaults 8.0
    config.api_only = true
  end
end
