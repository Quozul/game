#ifndef GAME_TYPE_HPP
#define GAME_TYPE_HPP

namespace packets {

	enum Type {
		SPAWN,
		MOVE,
		POSITION,
	};

	Type get_packet_type(char *data);

}

#endif //GAME_TYPE_HPP
