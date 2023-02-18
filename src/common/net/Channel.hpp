#ifndef GAME_CHANNEL_HPP
#define GAME_CHANNEL_HPP

#include <cstring>
#include <openssl/ssl.h>
#include <sys/socket.h>
#include <cerrno>
#include <csignal>

namespace net {

	class Channel {
	private:
		char buffer[80];
		int bytes = 0;

	public:
		int fd;
		SSL * ssl;

		Channel(int fd, SSL * ssl) {
			this->fd = fd;
			this->ssl = ssl;
		}

		char * getBuffer() {
			return buffer;
		}

		void write(std::string data) {
			printf("Writing %s\n", data.c_str());

			auto buffer = data.c_str();
			auto buffer_length = data.length();

			if (SSL_write(ssl, buffer, buffer_length) <= 0) {
				printf("Server closed connection\n");
				ERR_print_errors_fp(stderr);
			}
		}

		bool read() {
			// Clear the buffer
			memset(buffer, 0, sizeof(buffer));

			bytes = SSL_read(ssl, buffer, sizeof(buffer));

			if (bytes < 0) {
				if (errno != EWOULDBLOCK) {
					perror("  recv() failed");
					return true;
				}
				return false;
			}

			if (bytes == 0) {
				printf("  Connection closed\n");
				return true;
			}

			printf("  %d bytes received : %s\n", bytes, buffer);

			return false;
		}
	};

}


#endif //GAME_CHANNEL_HPP
