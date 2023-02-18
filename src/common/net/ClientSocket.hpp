#ifndef GAME_CLIENTSOCKET_HPP
#define GAME_CLIENTSOCKET_HPP


#include <netinet/in.h>
#include <csignal>
#include <openssl/types.h>
#include <iostream>
#include <arpa/inet.h>
#include <thread>
#include "network.hpp"
#include "Channel.hpp"
#include "BaseSocket.hpp"

namespace net {

	class ClientSocket: private BaseSocket {
	private:
		SSL *ssl = nullptr;
		std::string data;

	public:
		net::Channel *channel;

		explicit ClientSocket(const std::string &rem_server_ip): BaseSocket() {
			init_ssl();
			auto hostname = rem_server_ip.c_str();

			/* Set up connect hostname */
			addr.sin_family = AF_INET;
			inet_pton(AF_INET, hostname, &addr.sin_addr.s_addr);
			addr.sin_port = htons(net::server_port);

			/* Do TCP connect with net */
			rc = connect(listen_sd, (struct sockaddr *) &addr, sizeof(addr));

			if (rc < 0) {
				std::cout << rc << std::endl;
				perror("connect() failed");
				close(listen_sd);
				exit(-1);
			}

			printf("TCP connection to net successful\n");

			/* Create client SSL structure using dedicated client socket */
			ssl = SSL_new(ssl_ctx);
			SSL_set_fd(ssl, listen_sd);
			/* Set hostname for SNI */
			SSL_set_tlsext_host_name(ssl, hostname);
			/* Configure net hostname check */
			SSL_set1_host(ssl, hostname);

			int result;
			if ((result = SSL_connect(ssl)) == 1) {
				std::cout << "Client connected!" << std::endl;
			} else {
				std::cerr << "Client was not able to connect! Error " << result << std::endl;
			}

			channel = new net::Channel{listen_sd, ssl};
		}

		void loop() {
			memcpy(&working_set, &master_set, sizeof(master_set));

			printf("Waiting on select()...\n");
			rc = select(listen_sd + 1, &working_set, nullptr, nullptr, nullptr);

			if (rc < 0) {
				perror("  select() failed");
				return;
			}

			// FIXME: Select should never timeout since the last parameter is null
			if (rc == 0) {
				printf("  select() timed out.  End program.\n");
				return;
			}

			if (FD_ISSET(listen_sd, &master_set)) {
				channel->read();
			}
		}

		[[noreturn]] static void client_loop(ClientSocket *client) {
			while (true) {
				client->loop();
			}
		}

		void start_loop() {
			std::thread handler(std::bind(client_loop, this));
			handler.detach();
		}

		~ClientSocket() {
			/* Close up */
			if (ssl != nullptr) {
				SSL_shutdown(ssl);
				SSL_free(ssl);
			}
		}
	};

}


#endif //GAME_CLIENTSOCKET_HPP
