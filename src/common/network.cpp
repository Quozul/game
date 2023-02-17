#include <openssl/types.h>
#include <openssl/ssl.h>
#include <openssl/err.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <fcntl.h>

static const int server_port = 4433;

int create_socket(bool isServer) {
	int socketfd, max_sd;
	int optval = 1;
	struct sockaddr_in addr;
	struct timeval timeout;
	fd_set master_set, working_set;

	socketfd = socket(AF_INET, SOCK_STREAM, 0);
	if (socketfd < 0) {
		perror("Unable to create socket");
		exit(EXIT_FAILURE);
	}

	if (isServer) {
		if (fcntl(socketfd, F_SETFL, SOCK_NONBLOCK) == -1) {
			printf("Could not switch to non-blocking.\n");
			return -1;
		}

		FD_ZERO(&master_set);
		max_sd = socketfd;
		FD_SET(socketfd, &master_set);

		timeout.tv_sec  = 3 * 60;
		timeout.tv_usec = 0;

		addr.sin_family = AF_INET;
		addr.sin_port = htons(server_port);
		addr.sin_addr.s_addr = INADDR_ANY;

		/* Reuse the address; good for quick restarts */
		if (setsockopt(socketfd, SOL_SOCKET, SO_REUSEADDR, &optval, sizeof(optval)) < 0) {
			perror("setsockopt(SO_REUSEADDR) failed");
			exit(EXIT_FAILURE);
		}

		if (bind(socketfd, (struct sockaddr *) &addr, sizeof(addr)) < 0) {
			perror("Unable to bind");
			exit(EXIT_FAILURE);
		}

		if (listen(socketfd, 1) < 0) {
			perror("Unable to listen");
			exit(EXIT_FAILURE);
		}
	}

	return socketfd;
}

SSL_CTX *create_context(bool isServer) {
	const SSL_METHOD *method;
	SSL_CTX *ctx;

	if (isServer)
		method = TLS_server_method();
	else
		method = TLS_client_method();

	ctx = SSL_CTX_new(method);
	if (ctx == nullptr) {
		perror("Unable to create SSL context");
		ERR_print_errors_fp(stderr);
		exit(EXIT_FAILURE);
	}

	return ctx;
}

void configure_server_context(SSL_CTX *ctx) {
	/* Set the key and cert */
	if (SSL_CTX_use_certificate_chain_file(ctx, "cert.pem") <= 0) {
		ERR_print_errors_fp(stderr);
		exit(EXIT_FAILURE);
	}

	if (SSL_CTX_use_PrivateKey_file(ctx, "key.pem", SSL_FILETYPE_PEM) <= 0) {
		ERR_print_errors_fp(stderr);
		exit(EXIT_FAILURE);
	}
}

void configure_client_context(SSL_CTX *ctx) {
	/*
	 * Configure the client to abort the handshake if certificate verification
	 * fails
	 */
	// FIXME: Uncomment
	// SSL_CTX_set_verify(ctx, SSL_VERIFY_PEER, nullptr);

	/*
	 * In a real application you would probably just use the default system certificate trust store and call:
	 *     SSL_CTX_set_default_verify_paths(ctx);
	 * In this demo though we are using a self-signed certificate, so the client must trust it directly.
	 */
	if (!SSL_CTX_load_verify_locations(ctx, "cert.pem", nullptr)) {
		ERR_print_errors_fp(stderr);
		exit(EXIT_FAILURE);
	}
}