#include <entt/entity/registry.hpp>
#include "ssl_server.hpp"

void read_system(entt::registry &registry) {
	auto view = registry.view<ClientConnection>();

	for (auto [entity, conn]: view.each()) {
		conn.Read();
	}
}

int main() {
	entt::registry registry;

	SslServer server;

	while (true) {
		server.Accept(registry);
		read_system(registry);
	}

	return EXIT_SUCCESS;
}