cmake_minimum_required(VERSION 3.15)
project(game CXX)

find_package(raylib REQUIRED)
find_package(EnTT REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(Threads REQUIRED)
find_package(fmt REQUIRED)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(LIBRARIES raylib EnTT::EnTT OpenSSL::SSL Threads::Threads fmt::fmt)

# Common library
file(GLOB_RECURSE COMMON_DEPENDS "src/common/*.[hc]pp")
add_library(common STATIC ${COMMON_DEPENDS})
target_link_libraries(common ${LIBRARIES})

# Server
file(GLOB_RECURSE SERVER_DEPENDS "src/server/*.[hc]pp")
add_executable(server ${SERVER_DEPENDS})
target_link_libraries(server LINK_PUBLIC common)

# Client
file(GLOB_RECURSE CLIENT_DEPENDS "src/client/*.[hc]pp")
add_executable(client ${CLIENT_DEPENDS})
target_link_libraries(client LINK_PUBLIC common)
