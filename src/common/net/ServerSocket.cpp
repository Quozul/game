#include "ServerSocket.hpp"

namespace net {

	void ServerSocket::accept_connection() {
		printf("  Listening socket is readable\n");
		do {
			new_sd = accept(listen_sd, (struct sockaddr *) &addr, &addr_len);
			if (new_sd < 0) {
				if (errno != EWOULDBLOCK) {
					perror("  accept() failed");
				}
				break;
			}

			printf("  New incoming connection - %d %d\n", new_sd, i);
			FD_SET(new_sd, &master_set);
			if (new_sd > max_sd) {
				max_sd = new_sd;
			}

			// Create the SSL context
			SSL *ssl = SSL_new(ssl_ctx);
			SSL_set_fd(ssl, new_sd);

			/* Wait for SSL connection from the client */
			if (SSL_accept(ssl) <= 0) {
				ERR_print_errors_fp(stderr);
			} else {
				printf("Client SSL connection accepted\n\n");
			}

			auto entity = registry->create();
			registry->emplace<net::Channel>(entity, new_sd, ssl);
			auto channel = registry->get<net::Channel>(entity);
			channel.write("Welcome!");
		} while (new_sd != -1);
	}

	[[noreturn]] void ServerSocket::server_loop(ServerSocket *server) {
		while (true) {
			server->loop();
		}
	}

	void ServerSocket::loop() {
		memcpy(&working_set, &master_set, sizeof(master_set));

		printf("Waiting on select()...\n");
		rc = select(max_sd + 1, &working_set, nullptr, nullptr, nullptr);

		if (rc < 0) {
			perror("  select() failed");
			return;
		}

		// FIXME: Select should never timeout since the last parameter is null
		if (rc == 0) {
			printf("  select() timed out.  End program.\n");
			return;
		}

		desc_ready = rc;
		for (i = 0; i <= max_sd && desc_ready > 0; ++i) {
			if (!FD_ISSET(i, &working_set)) continue;

			desc_ready -= 1;

			if (i == listen_sd) {
				accept_connection();
			} else {
				printf("  Descriptor %d is readable\n", i);

				auto view = registry->view<net::Channel>();

				// TODO: Optimize using a map
				for (auto [entity, channel]: view.each()) {
					if (channel.fd == i) {
						if (channel.read()) {
							close_connection(i);
							registry->destroy(entity);
						}
						break;
					}
				}
			} /* End of existing connection is readable */
		} /* End of loop through selectable descriptors */
	}

	// TODO: Move to Channel class
	void ServerSocket::close_connection(int fd) {
		close(fd);
		FD_CLR(fd, &master_set);
		if (fd == max_sd) {
			while (FD_ISSET(max_sd, &master_set) == false) {
				max_sd -= 1;
			}
		}
		printf("  Connection %d closed\n", fd);
	}


	void ServerSocket::start_loop() {
		std::thread handler(std::bind(server_loop, this));
		handler.detach();
	}

	ServerSocket::ServerSocket(entt::registry &reg) : BaseSocket() {
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


}
