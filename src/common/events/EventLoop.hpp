#ifndef GAME_EVENTLOOP_HPP
#define GAME_EVENTLOOP_HPP


#include <typeindex>
#include <map>
#include <functional>
#include <cstdio>
#include <iostream>
#include "../ResourceHolder.hpp"

namespace events {

	class EventLoop {
	private:

		std::map<std::string, std::function<void(const void*)>> handlers;
		resources::ResourceHolder *resource_holder;

	public:
		explicit EventLoop(resources::ResourceHolder *holder) : resource_holder(holder) {}

		template<typename Event>
		void on(void handler(const Event &event, resources::ResourceHolder &resource_holder)) {
			auto type_name = typeid(Event).name();
			if (handlers.contains(type_name)) {
				throw std::invalid_argument("A handler for this scene is already defined.");
			}

			auto lambda = [handler, this](const void *event_ptr) {
				const auto *event = static_cast<const Event *>(event_ptr);
				handler(*event, *this->resource_holder);
			};

			handlers[type_name] = lambda;

			printf("Registered event for %s\n", typeid(Event).name());
		}

		template<typename Event>
		void fire(const Event &event) {
			auto type_name = typeid(Event).name();
			printf("Received event %s\n", type_name);

			auto &handler = handlers[type_name];
			if (handler != nullptr) {
				handler(&event);
			}
		}

		~EventLoop() {
			std::cout << "EventLoop destroyed" << std::endl;
		}
	};

}


#endif //GAME_EVENTLOOP_HPP
