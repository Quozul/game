#include <raylib.h>
#include <entt/entity/registry.hpp>
#include "../common/net/ServerSocket.hpp"
#include <fmt/core.h>
#include <iostream>
#include "../common/net/packets/move.hpp"
#include "../common/net/serialization.hpp"

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
	events::EventLoop events;
	net::ServerSocket socket{registry, events};

	InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Server");
	SetTargetFPS(60);

	events.on<events::server::DataReceived>([](auto &event) {
		auto type = net::get_packet_type(event.buffer);
		switch (type) {
			case packets::Type::MOVE: {
				auto move = net::deserialize<packets::move>(event.buffer);
				std::cout << fmt::format("Move packet received: {} {}", move.x, move.y) << std::endl;
			}
		}
	});

	events.on<events::server::Connected>([](auto &event) {
		std::cout << fmt::format("New client connected: {}", event.fd) << std::endl;
	});

	events.on<events::server::Disconnected>([](auto &event) {
		std::cout << fmt::format("Client disconnected: {}", event.fd) << std::endl;
	});

	// start network loop
	socket.start_loop();

	while (!WindowShouldClose()) {
		BeginDrawing();

		ClearBackground(RAYWHITE);

		text_drawing(registry);

		EndDrawing();
	}

	return EXIT_SUCCESS;
}
