#ifndef GAME_SERVER_EVENTS_HPP
#define GAME_SERVER_EVENTS_HPP

namespace events::server {
		struct Connected {
			entt::entity entity;
			int fd;
		};

		struct DataReceived {
			entt::entity entity;
			char* buffer;
			int length;
		};
	}

#endif //GAME_SERVER_EVENTS_HPP
