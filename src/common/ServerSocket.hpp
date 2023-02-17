#ifndef GAME_SERVERSOCKET_HPP
#define GAME_SERVERSOCKET_HPP

#include "BaseSocket.hpp"

namespace net {

	// Source: https://www.ibm.com/docs/en/i/7.1?topic=designs-example-nonblocking-io-select
	class ServerSocket : private BaseSocket {
	public:
		ServerSocket() : BaseSocket() {
			memset(&addr, 0, sizeof(addr));
			addr.sin_family = AF_INET;
			memcpy(&addr.sin_addr, &in6addr_any, sizeof(in6addr_any));
			addr.sin_port = htons(SERVER_PORT);
			rc = bind(listen_sd, (struct sockaddr *) &addr, sizeof(addr));

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

		void loop() {
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
					printf("  Listening socket is readable\n");
					do {
						new_sd = accept(listen_sd, nullptr, nullptr);
						if (new_sd < 0) {
							if (errno != EWOULDBLOCK) {
								perror("  accept() failed");
							}
							break;
						}

						printf("  New incoming connection - %d\n", new_sd);
						FD_SET(new_sd, &master_set);
						if (new_sd > max_sd) {
							max_sd = new_sd;
						}
					} while (new_sd != -1);
				} else {
					printf("  Descriptor %d is readable\n", i);
					close_conn = false;
					rc = recv(i, buffer, sizeof(buffer), 0);
					if (rc < 0) {
						if (errno != EWOULDBLOCK) {
							perror("  recv() failed");
							close_connection(i);
						}
						return;
					}

					if (rc == 0) {
						printf("  Connection closed\n");
						close_connection(i);
						return;
					}

					len = rc;
					printf("  %d bytes received : %s\n", len, buffer);

					rc = send(i, buffer, len, 0);
					if (rc < 0) {
						perror("  send() failed");
						close_connection(i);
						return;
					}
				} /* End of existing connection is readable */
			} /* End of loop through selectable descriptors */
		}

		void close_connection(int fd) {
			close(fd);
			FD_CLR(fd, &master_set);
			if (fd == max_sd) {
				while (FD_ISSET(max_sd, &master_set) == false) {
					max_sd -= 1;
				}
			}
			printf("  Connection %d closed\n", fd);
		}

	};

}


#endif //GAME_SERVERSOCKET_HPP
