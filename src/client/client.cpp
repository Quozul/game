#define RAYGUI_IMPLEMENTATION

#include <raylib.h>
#include <cstdlib>
#include "config.hpp"
#include "scenes/scene_state_machine.hpp"

int main() {
	InitWindow(config::SCREEN_WIDTH, config::SCREEN_HEIGHT, "Window title");
	SetTargetFPS(60);

	entt::registry registry;

	using SceneManager = state::StateMachine<scene::MenuState, scene::GameState>;
	SceneManager manager(&registry);

	auto entity = registry.create();
	registry.emplace<SceneManager>(entity, manager);

	setup_inputs(registry);

	while (!WindowShouldClose()) {
		BeginDrawing();

		ClearBackground(RAYWHITE);

		drawing_squares(registry);
		text_drawing(registry);
		tick_inputs(registry);
		tick_forms(registry);

		EndDrawing();
	}

	CloseWindow();

	return EXIT_SUCCESS;
}