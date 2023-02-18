#ifndef GAME_SERVERSOCKET_HPP
#define GAME_SERVERSOCKET_HPP

#include <thread>
#include <functional>
#include <entt/entity/registry.hpp>
#include "BaseSocket.hpp"
#include "network.hpp"
#include "../server/Channel.hpp"

namespace net {

	// Source: https://www.ibm.com/docs/en/i/7.1?topic=designs-example-nonblocking-io-select
	class ServerSocket : private BaseSocket {
	private:
		SSL_CTX *ssl_ctx = nullptr;
		entt::registry *registry;
		unsigned int addr_len = sizeof(addr);

		void accept_connection();

		[[noreturn]] static void server_loop(ServerSocket *server);

		void loop();

		// TODO: Move to Channel class
		void close_connection(int fd);

	public:
		explicit ServerSocket(entt::registry &reg) : BaseSocket() {
			this->registry = &reg;
			addr.sin_family = AF_INET;
			addr.sin_addr.s_addr = INADDR_ANY;
			addr.sin_port = htons(SERVER_PORT);
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

			ssl_ctx = net::create_context(true);

			/* Configure net context with appropriate key files */
			net::configure_server_context(ssl_ctx);
		}

		void start_loop();

	};

}


#endif //GAME_SERVERSOCKET_HPP
