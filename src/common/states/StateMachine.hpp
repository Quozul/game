#ifndef GAME_STATEMACHINE_HPP
#define GAME_STATEMACHINE_HPP

#include <tuple>
#include <variant>
#include <iostream>
#include <entt/entity/registry.hpp>
#include "../events/EventLoop.hpp"
#include "../ResourceHolder.hpp"

namespace state {

	// Source: https://sii.pl/blog/en/implementing-a-state-machine-in-c17/

	template<typename... States>
	class StateMachine {
	private:
		entt::registry *registry;
		resources::ResourceHolder *resource_holder;

	public:
		StateMachine(entt::registry *reg, resources::ResourceHolder *ent) {
			this->registry = reg;
			this->resource_holder = ent;
		}

		template<typename State>
		void transition_to() {
			current_state = &std::get<State>(states);
		}

		template<typename Event>
		void handle(const Event &event) {
			auto passEventToState = [this, &event](auto statePtr) {
				statePtr->handle(event, *registry, *resource_holder).execute(*this);
			};
			std::visit(passEventToState, current_state);
		}

	private:
		std::tuple<States...> states;
		std::variant<States *...> current_state{&std::get<0>(states)};
	};

	template<typename State>
	struct TransitionTo {
		template<typename Machine>
		void execute(Machine &machine) {
			machine.template transition_to<State>();
		}
	};

	struct Nothing {
		template<typename Machine>
		void execute(Machine &) {
		}
	};

	template<typename Action>
	struct ByDefault {
		template<typename Event>
		Action handle(const Event &, entt::registry &registry, resources::ResourceHolder &resource_holder) const {
			return Action{};
		}
	};

}

#endif //GAME_STATEMACHINE_HPP
