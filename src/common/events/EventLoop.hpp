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
		std::map<std::type_index, std::function<void(const void *)>> handlers;
		resources::ResourceHolder *resource_holder;

	public:
		explicit EventLoop(resources::ResourceHolder *holder) : resource_holder(holder) {}

		template<typename Event>
		void on(void (*handler)(const Event &event, resources::ResourceHolder &resource_holder)) {
			if (handlers.contains(typeid(Event))) {
				throw std::invalid_argument("A handler for this scene is already defined.");
			}

			auto lambda = [handler, this](const void *event_ptr) {
				const auto *event = static_cast<const Event *>(event_ptr);
				handler(*event, *this->resource_holder);
			};

			handlers[typeid(Event)] = lambda;
		}

		template<typename Event>
		void fire(const Event &event) {
			std::cout << "event firing" << std::endl;
			auto it = handlers.find(typeid(event));
			if (it != handlers.end()) {
				std::cout << "handler found" << std::endl;
				it->second(&event);
			} else {
				std::cout << "handler NOT found" << std::endl;
			};
		}
	};

}


#endif //GAME_EVENTLOOP_HPP
