cmake_minimum_required(VERSION 3.15)
project(game CXX)

find_package(raylib 4.0 REQUIRED)
find_package(EnTT 3.0 REQUIRED)
find_package(OpenSSL 3.0 REQUIRED)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(LIBRARIES raylib EnTT::EnTT OpenSSL::SSL)

# Common library
file(GLOB_RECURSE COMMON_DEPENDS "src/common/*.cpp")
add_library(common STATIC ${COMMON_DEPENDS})
target_link_libraries(common ${LIBRARIES})

# Server
file(GLOB_RECURSE SERVER_DEPENDS "src/server/*.cpp")
add_executable(server ${SERVER_DEPENDS})
target_link_libraries(server ${LIBRARIES} common)

# Client
file(GLOB_RECURSE CLIENT_DEPENDS "src/client/*.cpp")
add_executable(client ${CLIENT_DEPENDS})
target_link_libraries(client ${LIBRARIES} common)
