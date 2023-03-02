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

		int getBytes() {
			return bytes;
		}

		void write(char*data) {
			auto buffer_length = sizeof(data);

			if (SSL_write(ssl, data, buffer_length) <= 0) {
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
				// Connection closed
				return true;
			}

			return false;
		}
	};

}


#endif //GAME_CHANNEL_HPP
