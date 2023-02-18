#include <raylib.h>
#include <entt/entity/registry.hpp>
#include <thread>
#include "../common/ServerSocket.hpp"

#define SCREEN_WIDTH  800.0
#define SCREEN_HEIGHT 450.0

void text_drawing(entt::registry &registry) {
	const char *text = "SERVER RUNNING";
	const Vector2 text_size = MeasureTextEx(GetFontDefault(), text, 20, 1);
	DrawText(text, SCREEN_WIDTH / 2 - text_size.x / 2, 10, 20, BLACK);

	int y = 30;

	auto view = registry.view<net::Channel>();

	for (auto [entity, channel]: view.each()) {
		DrawText(channel.getBuffer(), SCREEN_WIDTH / 2 - text_size.x / 2, y, 10, BLACK);
		y += 10;
	}
}

[[noreturn]] void server_loop(entt::registry &registry) {
	net::ServerSocket socket{registry};

	while (true) {
		socket.loop();
	}
}

int main() {
	entt::registry registry;

	InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Server");
	SetTargetFPS(60);

	std::thread handler([&registry] { return server_loop(registry); });
	handler.detach();

	while (!WindowShouldClose()) {
		BeginDrawing();

		ClearBackground(RAYWHITE);

		text_drawing(registry);

		EndDrawing();
	}

	return EXIT_SUCCESS;
}