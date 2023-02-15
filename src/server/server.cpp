#include <entt/entity/registry.hpp>
#include <raylib.h>
#include <netinet/in.h>
#include "../common/socket.hpp"

#define PORT 44444

void accept_connection(entt::registry &registry, const int sockfd) {
	struct sockaddr_in address;
	socklen_t len = sizeof(address);

	int client = accept(sockfd, (struct sockaddr *) &address, &len);  // Accept connection as usual
}

int main() {
	entt::registry registry;
	SetTargetFPS(60);

	int sockfd = create_socket(PORT);

	while (!WindowShouldClose()) {
		accept_connection(registry, sockfd);
	}

	return EXIT_SUCCESS;
}