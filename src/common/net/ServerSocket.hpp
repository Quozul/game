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
		explicit ServerSocket(entt::registry &reg) : BaseSocket() {
			set_non_blocking();
			init_ssl(true);

			this->registry = &reg;
			addr.sin_family = AF_INET;
			addr.sin_addr.s_addr = INADDR_ANY;
			addr.sin_port = htons(net::server_port);
			rc = bind(listen_sd, (struct sockaddr *) &addr, addr_len);

			if (rc < 0) {
				perror("bind() failed");
				close(listen_sd);
				exit(-1);
			}

			rc = listen(listen_sd, 32);
			if (rc < 0) {
				perror("listen() failed");
				close(listen_sd);
				exit(-1);
			}
		}

		void start_loop();

	};

}


#endif //GAME_SERVERSOCKET_HPP
