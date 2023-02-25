#define RAYGUI_IMPLEMENTATION

#include <raylib.h>
#include <cstdlib>
#include <box2d/box2d.h>
#include "config.hpp"
#include "scenes/scene_state_machine.hpp"

int main() {
	InitWindow(config::SCREEN_WIDTH, config::SCREEN_HEIGHT, "Window title");
	SetTargetFPS(60);

	entt::registry registry;

	b2Vec2 gravity(0.0f, -10.0f);
	b2World world(gravity);

	auto entity = registry.create();

	using SceneManager = state::StateMachine<scene::MenuState, scene::GameState>;
	SceneManager manager(&registry, &entity);

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