#include <entt/entity/registry.hpp>
#include "ssl_server.hpp"
#include "../common/ServerSocket.hpp"

void read_system(entt::registry &registry) {
	auto view = registry.view<ClientConnection>();

	for (auto [entity, conn]: view.each()) {
		conn.Read();
	}
}

int main() {
	entt::registry registry;

//	SslServer server;
	net::ServerSocket socket;

	while (true) {
//		server.Accept(registry);
//		read_system(registry);
		socket.loop();
	}

	return EXIT_SUCCESS;
}