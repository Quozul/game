#include "systems.hpp"
#include "components.hpp"
#include "config.hpp"
#include "scenes/state_machine.hpp"
#include "../common/packets/move.hpp"
#include "../common/net/serialization.hpp"

void setup(entt::registry &registry) {
	const auto entity = registry.create();
	registry.emplace<Position>(entity, 0, 0);
}

void setup_inputs(entt::registry &registry) {
	Rectangle input_rectangle{
			0, 100, 300, 50
	};
	Rectangle button_rectangle{
			0, 200, 300, 50
	};

	const auto entity = registry.create();
	registry.emplace<Input>(entity, "127.0.0.1", input_rectangle);
	registry.emplace<Button>(entity, "Connect", button_rectangle);
	registry.emplace<Form>(entity);
}

void text_drawing(entt::registry &registry) {
	const char *text = "Client";
	const Vector2 text_size = MeasureTextEx(GetFontDefault(), text, 20, 1);
	DrawText(text, config::SCREEN_WIDTH / 2 - text_size.x / 2, 10, 20, BLACK);
}

void tick_inputs(entt::registry &registry) {
	auto view = registry.view<Input>();

	for (auto [entity, input]: view.each()) {
		char *data = (char *) (input.text.c_str());
		GuiTextBox(input.rectangle, data, 15, true);

		DrawText(data, 0, 200, 20, BLACK);
	}
}

using SceneManager = state::StateMachine<scene::MenuState, scene::GameState>;

void tick_forms(entt::registry &registry, resources::ResourceHolder &resource_holder) {
	auto view = registry.view<Input, Button, Form>();

	auto manager = resource_holder.get_ptr<SceneManager>();

	for (auto [entity, input, button]: view.each()) {
		if (GuiButton(button.rectangle, button.text.c_str())) {
			manager->handle(scene::PlayEvent{input.text});
		}
	}
}

void controls(entt::registry &registry, resources::ResourceHolder &resource_holder) {
	net::ClientSocket *client;
	if ((client = resource_holder.get_ptr<net::ClientSocket>()) == nullptr) {
		return;
	}

	if (IsKeyPressed(KEY_D)) {
		packets::move packet{1, 0};
		char *serialized = net::serialize(packets::Type::MOVE, packet);
		client->write(serialized);
	} else if (IsKeyPressed(KEY_A)) {
		packets::move packet{-1, 0};
		char *serialized = net::serialize(packets::Type::MOVE, packet);
		client->write(serialized);
	}

	if (IsKeyPressed(KEY_W)) {
		packets::move packet{0, -1};
		char *serialized = net::serialize(packets::Type::MOVE, packet);
		client->write(serialized);
	} else if (IsKeyPressed(KEY_S)) {
		packets::move packet{0, 1};
		char *serialized = net::serialize(packets::Type::MOVE, packet);
		client->write(serialized);
	}
}

void drawing_squares(entt::registry &registry) {
	auto view = registry.view<Position>();

	for (auto [entity, position]: view.each()) {
		DrawRectangle(position.x, position.y, 16, 16, RED);
	}
}

