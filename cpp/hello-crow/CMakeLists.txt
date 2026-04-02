cmake_minimum_required(VERSION 3.20)
project(hello-crow CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)

FetchContent_Declare(
    crow
    GIT_REPOSITORY https://github.com/CrowCpp/Crow.git
    GIT_TAG        v1.2.0
)
FetchContent_MakeAvailable(crow)

add_executable(hello-crow main.cpp)

target_link_libraries(hello-crow PRIVATE Crow::Crow)
