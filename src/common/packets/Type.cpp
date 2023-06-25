#include <cstring>
#include "Type.hpp"

namespace packets {

	Type get_packet_type(char *data) {
		packets::Type result;
		std::memcpy(&result, data, sizeof(packets::Type));
		return result;
	}

}