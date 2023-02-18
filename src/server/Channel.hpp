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
		int i, len, rc = 1;

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

		bool read() {
			// Clear the buffer
			memset(buffer, 0, sizeof(buffer));

			rc = SSL_read(ssl, buffer, sizeof(buffer));
//			rc = recv(fd, buffer, sizeof(buffer), 0);

			if (rc < 0) {
				if (errno != EWOULDBLOCK) {
					perror("  recv() failed");
					return true;
				}
				return false;
			}

			if (rc == 0) {
				printf("  Connection closed\n");
				return true;
			}

			len = rc;
			printf("  %d bytes received : %s\n", len, buffer);

//			rc = send(i, buffer, len, 0);
//			if (rc < 0) {
//				perror("  send() failed");
//				return true;
//			}

			return false;
		}
	};

}


#endif //GAME_CHANNEL_HPP
