#ifndef GAME_SERVERSOCKET_HPP
#define GAME_SERVERSOCKET_HPP

#include <thread>
#include <functional>
#include <entt/entity/registry.hpp>
#include "BaseSocket.hpp"
#include "network.hpp"
#include "Channel.hpp"
#include "../events/generic_event.hpp"
#include "../events/server_events.hpp"
#include "../Queue.hpp"

namespace net {

	// Source: https://www.ibm.com/docs/en/i/7.1?topic=designs-example-nonblocking-io-select
	class ServerSocket : private BaseSocket {
	private:
		entt::registry *registry;
		Queue<events::generic_event<events::server_events>> *queue;

		void accept_connection();

		[[noreturn]] static void server_loop(ServerSocket *server);

		void loop();

		// TODO: Move to Channel class
		void close_connection(int fd);

	public:
		explicit ServerSocket(entt::registry &reg, Queue<events::generic_event<events::server_events>> & queue);

		void start_loop();

	};

}


#endif //GAME_SERVERSOCKET_HPP
