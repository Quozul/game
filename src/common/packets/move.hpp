#ifndef GAME_MOVE_PACKET_HPP
#define GAME_MOVE_PACKET_HPP

namespace packets {

	// client sent packets

	struct move {
		short x;
		short y;
	};

	// server sent packets

	struct position {
		int id;
		float x;
		float y;
		float vx;
		float vy;
	};

	struct spawn {
		int id;
	};

}

#endif //GAME_MOVE_PACKET_HPP
