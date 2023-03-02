#define RAYGUI_IMPLEMENTATION

#include <raylib.h>
#include <cstdlib>
#include <box2d/box2d.h>
#include "config.hpp"
#include "scenes/state_machine.hpp"
#include "../common/net/packets/move.hpp"

int main() {
    std::cout << std::type_index(typeid(packets::move)).hash_code() << std::endl;
	InitWindow(config::SCREEN_WIDTH, config::SCREEN_HEIGHT, "Window title");
	SetTargetFPS(60);

	entt::registry registry;
	events::EventLoop events;

	b2Vec2 gravity(0.0f, -10.0f);
	b2World world(gravity);

	resources::ResourceHolder resource_holder;
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