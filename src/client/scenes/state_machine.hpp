#ifndef GAME_STATE_MACHINE_HPP
#define GAME_STATE_MACHINE_HPP

#include <iostream>
#include "events.hpp"
#include "../systems.hpp"
#include "../../common/states/StateMachine.hpp"
#include "../components.hpp"

namespace scene {

	struct MenuState;
	struct GameState;

	struct MenuState: public state::ByDefault<state::Nothing> {
		using ByDefault::handle;

		state::TransitionTo<GameState> handle(const scene::PlayEvent &event, entt::registry &registry, resources::ResourceHolder &resource_holder) const {
			auto view1 = registry.view<Button>();
			std::cout << view1.size() << std::endl;
			for (auto [entity, button]: view1.each()) {
				registry.remove<Button>(entity);
				registry.destroy(entity);
			}

			auto view2 = registry.view<Input>();
			for (auto [entity, input]: view2.each()) {
				registry.remove<Input>(entity);
				registry.destroy(entity);
			}

			auto view3 = registry.view<Form>();
			for (auto [entity]: view3.each()) {
				registry.remove<Form>(entity);
				registry.destroy(entity);
			}

			setup(registry);

			auto* socket = new net::ClientSocket(event.hostname);
			std::cout << "Connecting to: '" << event.hostname << "'" << std::endl;

			if (socket->connected) {
				socket->start_loop();
				resource_holder.add_ptr<net::ClientSocket>(socket);
			}
			std::cout << "transition to game" << std::endl;
			return {};
		}
	};

	struct GameState: public state::ByDefault<state::Nothing> {
		using ByDefault::handle;

		state::TransitionTo<MenuState> handle(const scene::MenuEvent &, entt::registry &registry, resources::ResourceHolder &resource_holder) const {
			std::cout << "transition to menu" << std::endl;
			registry.clear<Position>();

			setup_inputs(registry);
			return {};
		}
	};
}

#endif //GAME_STATE_MACHINE_HPP
