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

#define SERVER_PORT 4433

namespace net {

	class BaseSocket {
	protected:
		int i, len, rc, on = 1;
		int listen_sd, max_sd, new_sd;
		int desc_ready = false;
		int close_conn;
		char buffer[80];
		struct sockaddr_in addr;
		struct timeval timeout;
		fd_set master_set, working_set;

	public:
		BaseSocket() {
			/*************************************************************/
			/* Create an AF_INET6 stream socket to receive incoming      */
			/* connections on                                            */
			/*************************************************************/
			listen_sd = socket(AF_INET, SOCK_STREAM, 0);
			if (listen_sd < 0) {
				perror("socket() failed");
				exit(-1);
			}

			/*************************************************************/
			/* Allow socket descriptor to be reuseable                   */
			/*************************************************************/
			rc = setsockopt(listen_sd, SOL_SOCKET, SO_REUSEADDR, (char *) &on, sizeof(on));
			if (rc < 0) {
				perror("setsockopt() failed");
				close(listen_sd);
				exit(-1);
			}

			/*************************************************************/
			/* Set socket to be nonblocking. All of the sockets for      */
			/* the incoming connections will also be nonblocking since   */
			/* they will inherit that state from the listening socket.   */
			/*************************************************************/
			rc = ioctl(listen_sd, FIONBIO, (char *) &on);
			if (rc < 0) {
				perror("ioctl() failed");
				close(listen_sd);
				exit(-1);
			}

			/*************************************************************/
			/* Initialize the master fd_set                              */
			/*************************************************************/
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
		}
	};

} // net

#endif //GAME_BASESOCKET_HPP
