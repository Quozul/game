#include <raylib.h>
#include <entt/entity/registry.hpp>
#include "../common/net/ServerSocket.hpp"
#include <fmt/core.h>
#include <iostream>
#include <box2d/box2d.h>
#include "../common/packets/move.hpp"
#include "../common/net/serialization.hpp"

#define SCREEN_WIDTH  800.0
#define SCREEN_HEIGHT 450.0

struct RigidBody {
	b2Body *body;
};

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

void tick_positions(entt::registry &registry) {
	auto view = registry.view<RigidBody, net::Channel>();
	for (auto [entity, rigid_body, channel]: view.each()) {
		auto position = rigid_body.body->GetPosition();
		auto velocity = rigid_body.body->GetLinearVelocity();
		auto packet = packets::position{
			channel.fd,
			position.x,
			position.y,
			velocity.x,
			velocity.y,
		};
		auto serialized = net::serialize(packets::Type::POSITION, packet);
		channel.write(serialized);
		std::cout << fmt::format("Sent position packet: {} {}", position.x, position.y) << std::endl;
	}
}

int main() {
	entt::registry registry;
	resources::ResourceHolder resource_holder;
	events::EventLoop events(&resource_holder);
	net::ServerSocket socket{registry, events};

	b2Vec2 gravity(0.0f, 0.0f);
	b2World world(gravity);

	resource_holder.add(events);
	resource_holder.add(world);
	resource_holder.add_ptr(&registry);
	resource_holder.add(socket);

	InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Server");
	SetTargetFPS(60);

	events.on<events::server::DataReceived>([](auto &event, auto &resources) {
		auto type = packets::get_packet_type(event.buffer);
		switch (type) {
			case packets::Type::MOVE: {
				auto move = net::deserialize<packets::move>(event.buffer);
				entt::registry* registry = resources.template get_ptr<entt::registry>();
				RigidBody rigid_body = registry->get<RigidBody>(event.entity);
				b2Vec2 force(move.x, move.y);
				rigid_body.body->ApplyForceToCenter(force, true);
				std::cout << fmt::format("Move packet received: {} {}", move.x, move.y) << std::endl;
				break;
			}
			default: {
				break;
			}
		}
	});

	events.on<events::server::Connected>([](auto &event, auto &resources) {
		auto world = resources.template get<b2World>();
		auto registry = resources.template get_ptr<entt::registry>();
		net::Channel channel = registry->template get<net::Channel>(event.entity);
		std::cout << fmt::format("New client connected: {}", channel.fd) << std::endl;
		b2BodyDef bodyDef;
		bodyDef.type = b2_dynamicBody;
		bodyDef.position.Set(0.0f, 4.0f);
		b2Body *body = world.CreateBody(&bodyDef);
		registry->template emplace<RigidBody>(event.entity, body);
		auto packet = packets::spawn{channel.fd};
		auto serialized = net::serialize(packets::Type::SPAWN, packet);
		channel.write(serialized);
	});

	events.on<events::server::Disconnected>([](auto &event, auto &resources) {
		std::cout << fmt::format("Client disconnected: {}", event.fd) << std::endl;
	});

	// start network loop
	socket.start_loop();

	while (!WindowShouldClose()) {
		BeginDrawing();

		ClearBackground(RAYWHITE);

		text_drawing(registry);
		tick_positions(registry);

		EndDrawing();
	}

	return EXIT_SUCCESS;
}
