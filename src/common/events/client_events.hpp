#ifndef GAME_CLIENT_EVENTS_HPP
#define GAME_CLIENT_EVENTS_HPP

namespace events::client {
		struct Connected{};

		struct DataReceived {
			char* buffer;
			int length;
		};

		struct Disconnected{};
	}

#endif //GAME_CLIENT_EVENTS_HPP
