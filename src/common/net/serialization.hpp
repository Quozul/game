#ifndef GAME_SERIALIZATION_HPP
#define GAME_SERIALIZATION_HPP

#include <cstring>
#include "../packets/Type.hpp"

namespace net {

	template<typename T>
	char *serialize(packets::Type packet_type, T &data) {
		char *buffer = new char[sizeof(T) + sizeof(packets::Type)];
		std::memcpy(buffer, &packet_type, sizeof(packets::Type));
		std::memcpy(buffer + sizeof(packets::Type), &data, sizeof(T));
		return buffer;
	}

	template<typename T>
	T deserialize(char *data) {
		T result;
		std::memcpy(&result, data + sizeof(packets::Type), sizeof(T));
		return result;
	}

}

#endif //GAME_SERIALIZATION_HPP