#include <raylib.h>
#include <entt/entity/registry.hpp>
#include "../common/net/ServerSocket.hpp"
#include <fmt/core.h>

#define SCREEN_WIDTH  800.0
#define SCREEN_HEIGHT 450.0

void text_drawing(entt::registry &registry) {
	auto view = registry.view<net::Channel>();

	std::string title = fmt::format("SERVER RUNNING! {} clients connected.", view.size());
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

int main() {
	entt::registry registry;

	InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Server");
	SetTargetFPS(60);

	net::ServerSocket socket{registry};
	socket.start_loop();

	while (!WindowShouldClose()) {
		BeginDrawing();

		ClearBackground(RAYWHITE);

		text_drawing(registry);

		EndDrawing();
	}

	return EXIT_SUCCESS;
}