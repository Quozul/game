#ifndef GAME_BASESOCKET_HPP
#define GAME_BASESOCKET_HPP

#include <cstdio>
#include <cstdlib>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <netinet/in.h>
#include <cerrno>
#include <csignal>
#include <cstring>
#include <openssl/types.h>
#include "network.hpp"

namespace net {

	class BaseSocket {
	protected:
		int i, rc, on = 1;
		int listen_sd, max_sd, new_sd;
		int desc_ready = false;
		struct sockaddr_in addr;
		fd_set master_set, working_set;
		SSL_CTX *ssl_ctx = nullptr;
		unsigned int addr_len = sizeof(addr);

	protected:
		void set_non_blocking() {
			rc = ioctl(listen_sd, FIONBIO, (char *) &on);
			if (rc < 0) {
				perror("ioctl() failed");
				close(listen_sd);
				exit(-1);
			}
		}

		void init_ssl(bool isServer = false) {
			ssl_ctx = net::create_context(isServer);
			net::configure_server_context(ssl_ctx);
		}

	public:
		BaseSocket() {
			listen_sd = socket(AF_INET, SOCK_STREAM, 0);
			if (listen_sd < 0) {
				perror("socket() failed");
				exit(-1);
			}

			rc = setsockopt(listen_sd, SOL_SOCKET, SO_REUSEADDR, (char *) &on, sizeof(on));
			if (rc < 0) {
				perror("setsockopt() failed");
				close(listen_sd);
				exit(-1);
			}

			FD_ZERO(&master_set);
			max_sd = listen_sd;
			FD_SET(listen_sd, &master_set);
		}

		~BaseSocket() {
			for (i = 0; i <= max_sd; ++i) {
				if (FD_ISSET(i, &master_set)) {
					close(i);
				}
			}

			SSL_CTX_free(ssl_ctx);
		}
	};

} // net

#endif //GAME_BASESOCKET_HPP
