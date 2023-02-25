#include <raylib.h>
#include <entt/entity/registry.hpp>
#include "../common/net/ServerSocket.hpp"
#include <fmt/core.h>

#define SCREEN_WIDTH  800.0
#define SCREEN_HEIGHT 450.0

void text_drawing(entt::registry &registry, Queue<events::generic_event<events::server_events>> &queue) {
	auto view = registry.view<net::Channel>();

	std::string title = fmt::format("SERVER RUNNING! {} clients connected. {}", view.size(), queue.size());
	const char *text = title.c_str();

	const Vector2 text_size = MeasureTextEx(GetFontDefault(), text, 20, 1);
	DrawText(text, SCREEN_WIDTH / 2 - text_size.x / 2, 10, 20, BLACK);

	int y = 30;

	for (auto [entity, channel]: view.each()) {
		std::string row = fmt::format("Client {} - {}", channel.fd, channel.getBuffer());
		DrawText(row.c_str(), SCREEN_WIDTH / 2 - text_size.x / 2, y, 10, BLACK);
		y += 10;
	}
}

[[noreturn]] void consume_events(entt::registry &registry, Queue<events::generic_event<events::server_events>> &queue) {
	while (true) {
		auto event = queue.pop();
		auto channel = registry.get<net::Channel>(event.entity);

		switch (event.event) {
			case events::server_events::CONNECTED:
				channel.write("Hello World!");
				break;
			case events::DATA_RECEIVED:
				printf("  %d bytes received : %s\n", channel.getBytes(), channel.getBuffer());
				break;
		}
	}
}

int main() {
	entt::registry registry;

	InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Server");
	SetTargetFPS(60);

	Queue<events::generic_event<events::server_events>> queue;

	// start network loop
	net::ServerSocket socket{registry, queue};
	socket.start_loop();

	// start event loop
	std::thread handler(std::bind(consume_events, std::ref(registry), std::ref(queue)));
	handler.detach();

	while (!WindowShouldClose()) {
		BeginDrawing();

		ClearBackground(RAYWHITE);

		text_drawing(registry, queue);

		EndDrawing();
	}

	return EXIT_SUCCESS;
}