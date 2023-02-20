#ifndef GAME_SERVERSOCKET_HPP
#define GAME_SERVERSOCKET_HPP

#include <thread>
#include <functional>
#include <entt/entity/registry.hpp>
#include "BaseSocket.hpp"
#include "network.hpp"
#include "Channel.hpp"

namespace net {

	// Source: https://www.ibm.com/docs/en/i/7.1?topic=designs-example-nonblocking-io-select
	class ServerSocket : private BaseSocket {
	private:
		entt::registry *registry;

		void accept_connection();

		[[noreturn]] static void server_loop(ServerSocket *server);

		void loop();

		// TODO: Move to Channel class
		void close_connection(int fd);

	public:
		explicit ServerSocket(entt::registry &reg);

		void start_loop();

	};

}


#endif //GAME_SERVERSOCKET_HPP
