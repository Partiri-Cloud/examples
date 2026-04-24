#include "crow.h"
#include <cstdlib>
#include <string>

int main() {
    crow::SimpleApp app;

    CROW_ROUTE(app, "/")([]() {
        return "Hello from Crow on Partiri!";
    });

    CROW_ROUTE(app, "/health")([]() {
        crow::json::wvalue result;
        result["status"] = "ok";
        return crow::response(200, result);
    });

    const char* port_env = std::getenv("PORT");
    uint16_t port = port_env ? static_cast<uint16_t>(std::stoi(port_env)) : 10000;

    app.port(port).multithreaded().run();

    return 0;
}
