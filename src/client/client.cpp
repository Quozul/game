#define RAYGUI_IMPLEMENTATION

#include <raylib.h>
#include <cstdlib>
#include <box2d/box2d.h>
#include <fmt/core.h>
#include "config.hpp"
#include "scenes/state_machine.hpp"
#include "../common/packets/move.hpp"
#include "../common/events/client_events.hpp"
#include "../common/net/serialization.hpp"

int main() {
	entt::registry registry;
	resources::ResourceHolder resource_holder;
	events::EventLoop events(&resource_holder);

	InitWindow(config::SCREEN_WIDTH, config::SCREEN_HEIGHT, "Window title");
	SetTargetFPS(60);

	events.on<events::client::DataReceived>([](auto &event, auto &resources) {
		auto type = packets::get_packet_type(event.buffer);
		switch (type) {
			case packets::Type::SPAWN: {
				auto move = net::deserialize<packets::spawn>(event.buffer);
				std::cout << fmt::format("Spawn packet received: {}", move.id) << std::endl;
				break;
			}
			case packets::Type::POSITION: {
				auto move = net::deserialize<packets::position>(event.buffer);
				std::cout << fmt::format("Position packet received: {} {} {}", move.id, move.x, move.y) << std::endl;
				break;
			}
			default:{
				break;
			}
		}
	});

	b2Vec2 gravity(0.0f, -10.0f);
	b2World world(gravity);

	resource_holder.add(events);
	resource_holder.add(world);
	resource_holder.add(registry);

	using SceneManager = state::StateMachine<scene::MenuState, scene::GameState>;
	SceneManager manager(&registry, &resource_holder);

	resource_holder.add(manager);

	setup_inputs(registry);

	while (!WindowShouldClose()) {
		BeginDrawing();

		ClearBackground(RAYWHITE);

		controls(registry, resource_holder);
		drawing_squares(registry);
		text_drawing(registry);
		tick_inputs(registry);
		tick_forms(registry, resource_holder);

		EndDrawing();
	}

	CloseWindow();

	return EXIT_SUCCESS;
}