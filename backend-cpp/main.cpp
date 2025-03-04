#include "crow.h"
#include "crow/middlewares/cors.h"
#include "modules.hpp"
#include <fstream>
#include <sstream>
#include <iostream>
#include <nlohmann/json.hpp>
#include <thread>
#include <chrono>
#include <cstdlib>

// Define Windows version for Crow
#define _WIN32_WINNT 0x0601

// for convenience
using json = nlohmann::json;

// Helper function to check string endings (C++17 compatible)
bool ends_with(const std::string& str, const std::string& suffix) {
    if (str.length() < suffix.length()) {
        return false;
    }
    return str.compare(str.length() - suffix.length(), suffix.length(), suffix) == 0;
}

json generate_dummy_netlist() {
    json j;
    j["modules"] = json::array();

    // Example module "alu"
    json alu;
    alu["name"] = "alu";
    alu["signals"] = json::array({
        {{"name", "A"}, {"direction", "input"}, {"width", 32}},
        {{"name", "B"}, {"direction", "input"}, {"width", 32}},
        {{"name", "Result"}, {"direction", "output"}, {"width", 32}}
    });
    alu["submodules"] = json::array();
    alu["complexity"] = 120;

    // Example module "cpu" instantiates alu
    json cpu;
    cpu["name"] = "cpu";
    cpu["signals"] = json::array({
        {{"name", "clk"}, {"direction", "input"}, {"width", 1}},
        {{"name", "reset"}, {"direction", "input"}, {"width", 1}},
        {{"name", "data_out"}, {"direction", "output"}, {"width", 32}}
    });
    json cpu_submodules = json::array();
    
    // Add ALU instance to CPU
    json alu_instance;
    alu_instance["instanceName"] = "alu1";
    alu_instance["moduleName"] = "alu";
    alu_instance["portConnections"] = {
        {"A", "alu_in_a"},
        {"B", "alu_in_b"},
        {"Result", "alu_result"}
    };
    cpu_submodules.push_back(alu_instance);
    
    cpu["submodules"] = cpu_submodules;
    cpu["complexity"] = 500;
    
    // Add modules to the netlist
    j["modules"].push_back(alu);
    j["modules"].push_back(cpu);
    
    return j;
}

int main() {
    // Create app with CORS middleware
    crow::App<crow::CORSHandler> app;

    // Configure CORS
    auto& cors = app.get_middleware<crow::CORSHandler>();
    cors
        .global()
        .headers("Content-Type", "Authorization")
        .methods("POST"_method, "GET"_method, "PUT"_method, "DELETE"_method)
        .origin("*");

    // API endpoint to get netlist data
    CROW_ROUTE(app, "/api/netlist")
    ([]() {
        return crow::response(generate_dummy_netlist().dump());
    });

    // Serve static files from the React build directory
    CROW_ROUTE(app, "/<path>")
    ([](std::string path) {
        // Default to index.html for the root path
        if (path.empty() || path == "/") {
            path = "index.html";
        }

        // Construct the path to the frontend build directory
        std::string filePath = "../frontend-react/build/" + path;
        
        // Try to open the file
        std::ifstream file(filePath, std::ios::binary);
        if (!file.good()) {
            // If file not found, serve index.html (for client-side routing)
            file.close();
            file.open("../frontend-react/build/index.html", std::ios::binary);
            if (!file.good()) {
                return crow::response(404, "File not found");
            }
        }

        // Read the file content
        std::stringstream buffer;
        buffer << file.rdbuf();
        std::string content = buffer.str();

        // Set content type based on file extension
        std::string contentType = "text/plain";
        if (ends_with(path, ".html")) contentType = "text/html";
        else if (ends_with(path, ".js")) contentType = "application/javascript";
        else if (ends_with(path, ".css")) contentType = "text/css";
        else if (ends_with(path, ".json")) contentType = "application/json";
        else if (ends_with(path, ".png")) contentType = "image/png";
        else if (ends_with(path, ".jpg") || ends_with(path, ".jpeg")) contentType = "image/jpeg";
        else if (ends_with(path, ".svg")) contentType = "image/svg+xml";

        crow::response res(content);
        res.set_header("Content-Type", contentType);
        return res;
    });

    // Default route for the root path
    CROW_ROUTE(app, "/")
    ([]() {
        // Redirect to the React app's index.html
        std::ifstream file("../frontend-react/build/index.html", std::ios::binary);
        if (!file.good()) {
            return crow::response(404, "Frontend not built. Please build the React frontend first.");
        }

        std::stringstream buffer;
        buffer << file.rdbuf();
        std::string content = buffer.str();

        crow::response res(content);
        res.set_header("Content-Type", "text/html");
        return res;
    });

    // Start the server
    std::cout << "Starting server on http://localhost:8080" << std::endl;
    std::cout << "API available at http://localhost:8080/api/netlist" << std::endl;
    std::cout << "Frontend served from ../frontend-react/build/" << std::endl;
    app.port(8080).multithreaded().run();

    return 0;
}
