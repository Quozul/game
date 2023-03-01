#ifndef GAME_EVENTLOOP_HPP
#define GAME_EVENTLOOP_HPP


#include <typeindex>
#include <map>
#include <functional>
#include <cstdio>
#include <iostream>

namespace events {

	class EventLoop {
	private:
		std::map<std::type_index, std::function<void(const void *)>> handlers;

	public:
		template<typename Event>
		void on(void (*handler)(const Event &event)) {
			if (handlers.contains(typeid(Event))) {
				throw std::invalid_argument("A handler for this scene is already defined.");
			}

			auto lambda = [handler](const void *event_ptr) {
				const auto *event = static_cast<const Event *>(event_ptr);
				handler(*event);
			};

			handlers[typeid(Event)] = lambda;
		}

		template<typename Event>
		void fire(const Event &event) {
			auto &handler = handlers[typeid(Event)];
			handler(&event);
		}
	};

}


#endif //GAME_EVENTLOOP_HPP
