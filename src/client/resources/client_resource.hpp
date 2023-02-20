#ifndef GAME_CLIENT_RESOURCE_HPP
#define GAME_CLIENT_RESOURCE_HPP

#include <memory>
#include "../../common/net/ClientSocket.hpp"

struct ClientLoader final {
	using result_type = std::shared_ptr<net::ClientSocket>;

	result_type operator()(std::string hostname) const {
		return std::make_shared<net::ClientSocket>(hostname);
	}
};

#endif //GAME_CLIENT_RESOURCE_HPP
