cmake_minimum_required(VERSION 3.15)
project(veriview VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# If you have local copies of Crow and nlohmann/json, include them here.
include_directories(${PROJECT_SOURCE_DIR}/include)

# Find dependencies
find_package(Crow CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(Threads REQUIRED)

# Define the executable
add_executable(veriview main.cpp modules.hpp)

# Link dependencies
target_link_libraries(veriview PRIVATE
    Crow::Crow
    nlohmann_json::nlohmann_json
    Threads::Threads
)

# Enable compiler warnings
if(MSVC)
    target_compile_options(veriview PRIVATE /W4)
else()
    target_compile_options(veriview PRIVATE -Wall -Wextra)
endif()
