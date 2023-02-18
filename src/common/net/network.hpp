#ifndef GAME_NETWORK_HPP
#define GAME_NETWORK_HPP

#include <openssl/types.h>
#include <openssl/ssl.h>
#include <openssl/err.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <fcntl.h>
#include <raylib.h>

namespace net {

	static const int server_port = 4433;

	int create_socket(bool isServer);

	SSL_CTX *create_context(bool isServer);

	void configure_server_context(SSL_CTX *ctx);

	void configure_client_context(SSL_CTX *ctx);

}

#endif //GAME_NETWORK_HPP
