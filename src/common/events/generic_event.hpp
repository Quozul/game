#ifndef GAME_GENERIC_EVENT_HPP
#define GAME_GENERIC_EVENT_HPP

#include "../net/Channel.hpp"

namespace events {

	template<typename T>
	struct generic_event {
		T event;
		entt::entity entity;
	};

}

#endif //GAME_GENERIC_EVENT_HPP
