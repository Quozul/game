#include <raylib.h>
#include <cstdlib>
#include <entt/entt.hpp>

#define SCREEN_WIDTH  800.0
#define SCREEN_HEIGHT 450.0

struct Position {
	int x;
	int y;
};

void text_drawing(entt::registry &registry) {
	const char *text = "OMG! IT WORKS!";
	const Vector2 text_size = MeasureTextEx(GetFontDefault(), text, 20, 1);
	DrawText(text, SCREEN_WIDTH / 2 - text_size.x / 2, 10, 20, BLACK);
}

void drawing_squares(entt::registry &registry) {
	auto view = registry.view<Position>();

	for (auto [entity, position]: view.each()) {
		if (IsKeyDown(KEY_D)) {
			position.x += 1;
		} else if (IsKeyDown(KEY_A)) {
			position.x -= 1;
		}

		if (IsKeyDown(KEY_W)) {
			position.y -= 1;
		} else if (IsKeyDown(KEY_S)) {
			position.y += 1;
		}

		DrawRectangle(position.x, position.y, 16, 16, RED);
	}
}

void setup(entt::registry &registry) {
	const auto entity = registry.create();
	registry.emplace<Position>(entity, 0, 0);
}

int main() {
	entt::registry registry;
	InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Window title");
	SetTargetFPS(60);

	setup(registry);

	while (!WindowShouldClose()) {
		BeginDrawing();

		ClearBackground(RAYWHITE);

		text_drawing(registry);
		drawing_squares(registry);

		EndDrawing();
	}

	CloseWindow();

	return EXIT_SUCCESS;
}