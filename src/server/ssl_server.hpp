#ifndef GAME_SSL_SERVER_HPP
#define GAME_SSL_SERVER_HPP


#include <netinet/in.h>
#include <csignal>
#include <openssl/types.h>
#include <iostream>
#include "../common/network.cpp"

class ClientConnection {
private:
	SSL *ssl;

	int client_skt = -1;

	char rxbuf[128];
	int rxcap = sizeof(rxbuf);
	int rxlen;

public:
	ClientConnection(int client_skt, SSL *ssl) {
		this->client_skt = client_skt;
		this->ssl = ssl;
	}

	void Read() {
		if ((rxlen = SSL_read(ssl, rxbuf, rxcap)) <= 0) {
			if (rxlen == 0) {
				printf("Client closed connection\n");
			}
			ERR_print_errors_fp(stderr);
		}

		/* Insure null terminated input */
		rxbuf[rxlen] = 0;

		/* Show received message */
		printf("Received: %s", rxbuf);
	}

	void Write() {
		/* Echo it back */
		if (SSL_write(ssl, rxbuf, rxlen) <= 0) {
			ERR_print_errors_fp(stderr);
		}
	}

	void Shutdown() {
		/* Cleanup for next client */
		SSL_shutdown(ssl);
		SSL_free(ssl);
		close(client_skt);
	}
};

class SslServer {
private:
	SSL_CTX *ssl_ctx = nullptr;

	int server_skt = -1;

	struct sockaddr_in addr;
	unsigned int addr_len = sizeof(addr);

public:
	SslServer() {
		ssl_ctx = net::create_context(true);

		/* Configure net context with appropriate key files */
		net::configure_server_context(ssl_ctx);

		/* Create net socket; will bind with net port and listen */
		server_skt = net::create_socket(true);

		std::cout << "Server listening..." << std::endl;
	}

	void Accept(entt::registry &registry) {
		int client_skt = accept(server_skt, (struct sockaddr *) &addr, &addr_len);
		if (client_skt < 0) {
			return;
		}

		printf("Client TCP connection accepted\n");

		/* Create net SSL structure using newly accepted client socket */
		SSL *ssl = SSL_new(ssl_ctx);
		SSL_set_fd(ssl, client_skt);

		/* Wait for SSL connection from the client */
		if (SSL_accept(ssl) <= 0) {
			ERR_print_errors_fp(stderr);
		} else {
			printf("Client SSL connection accepted\n\n");
		}
	}
};


#endif //GAME_SSL_SERVER_HPP
