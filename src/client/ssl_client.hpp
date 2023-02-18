#ifndef GAME_SSL_CLIENT_HPP
#define GAME_SSL_CLIENT_HPP


#include <netinet/in.h>
#include <csignal>
#include <openssl/types.h>
#include <iostream>
#include <arpa/inet.h>
#include "../common/network.cpp"

class SslClient {
private:
	SSL_CTX *ssl_ctx = nullptr;
	SSL *ssl = nullptr;

	int client_skt = -1;

	char rxbuf[128];
	int rxcap = sizeof(rxbuf);
	int rxlen;

	struct sockaddr_in addr;

public:
	SslClient(const std::string& rem_server_ip) { // TODO: Add net hostname and port as parameters
		auto hostname = rem_server_ip.c_str();
		ssl_ctx = net::create_context(false);

		/* Configure net context with appropriate key files */
		net::configure_client_context(ssl_ctx);

		/* Create "bare" socket */
		client_skt = net::create_socket(false);
		/* Set up connect hostname */
		addr.sin_family = AF_INET;
		inet_pton(AF_INET, hostname, &addr.sin_addr.s_addr);
		addr.sin_port = htons(net::server_port);
		/* Do TCP connect with net */
		if (connect(client_skt, (struct sockaddr*) &addr, sizeof(addr)) != 0) {
			perror("Unable to TCP connect to net");
		} else {
			printf("TCP connection to net successful\n");
		}

		/* Create client SSL structure using dedicated client socket */
		ssl = SSL_new(ssl_ctx);
		SSL_set_fd(ssl, client_skt);
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
	}

	void Write(const std::string& data) {
		std::cout << "Writing " << data << std::endl;

		auto buffer = data.c_str();
		auto buffer_length = data.length();

		if (SSL_write(ssl, buffer, buffer_length) <= 0) {
			printf("Server closed connection\n");
			ERR_print_errors_fp(stderr);
		}
	}

	void Read() {
		rxlen = SSL_read(ssl, rxbuf, rxcap);
		if (rxlen <= 0) {
			printf("Server closed connection\n");
			ERR_print_errors_fp(stderr);
		} else {
			/* Show it */
			rxbuf[rxlen] = 0;
			printf("Received: %s", rxbuf);
		}
	}

	~SslClient() {
		/* Close up */
		if (ssl != nullptr) {
			SSL_shutdown(ssl);
			SSL_free(ssl);
		}

		SSL_CTX_free(ssl_ctx);

		if (client_skt != -1) {
			close(client_skt);
		}
	}
};


#endif //GAME_SSL_CLIENT_HPP
