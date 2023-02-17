#ifndef GAME_SERVERSOCKET_HPP
#define GAME_SERVERSOCKET_HPP

#include "BaseSocket.hpp"

namespace net {

	class ServerSocket : private BaseSocket {
	public:
		ServerSocket(): BaseSocket() {
			/*************************************************************/
			/* Bind the socket                                           */
			/*************************************************************/
			memset(&addr, 0, sizeof(addr));
			addr.sin_family = AF_INET;
			memcpy(&addr.sin_addr, &in6addr_any, sizeof(in6addr_any));
			addr.sin_port = htons(SERVER_PORT);
			rc = bind(listen_sd, (struct sockaddr *) &addr, sizeof(addr));
			if (rc < 0) {
				perror("bind() failed");
				close(listen_sd);
				exit(-1);
			}

			/*************************************************************/
			/* Set the listen back log                                   */
			/*************************************************************/
			rc = listen(listen_sd, 32);
			if (rc < 0) {
				perror("listen() failed");
				close(listen_sd);
				exit(-1);
			}

			/*************************************************************/
			/* Initialize the timeval struct to 3 minutes.  If no        */
			/* activity after 3 minutes this program will end.           */
			/*************************************************************/
			timeout.tv_sec = 3 * 60;
			timeout.tv_usec = 0;
		}

		void loop() {
			/**********************************************************/
			/* Copy the master fd_set over to the working fd_set.     */
			/**********************************************************/
			memcpy(&working_set, &master_set, sizeof(master_set));

			/**********************************************************/
			/* Call select() and wait 3 minutes for it to complete.   */
			/**********************************************************/
			printf("Waiting on select()...\n");
			rc = select(max_sd + 1, &working_set, nullptr, nullptr, &timeout);

			/**********************************************************/
			/* Check to see if the select call failed.                */
			/**********************************************************/
			if (rc < 0) {
				perror("  select() failed");
				return;
			}

			/**********************************************************/
			/* Check to see if the 3 minute time out expired.         */
			/**********************************************************/
			if (rc == 0) {
				printf("  select() timed out.  End program.\n");
				return;
			}

			/**********************************************************/
			/* One or more descriptors are readable.  Need to         */
			/* determine which ones they are.                         */
			/**********************************************************/
			desc_ready = rc;
			for (i = 0; i <= max_sd && desc_ready > 0; ++i) {
				/*******************************************************/
				/* Check to see if this descriptor is ready            */
				/*******************************************************/
				if (FD_ISSET(i, &working_set)) {
					/****************************************************/
					/* A descriptor was found that was readable - one   */
					/* less has to be looked for.  This is being done   */
					/* so that we can stop looking at the working set   */
					/* once we have found all of the descriptors that   */
					/* were ready.                                      */
					/****************************************************/
					desc_ready -= 1;

					/****************************************************/
					/* Check to see if this is the listening socket     */
					/****************************************************/
					if (i == listen_sd) {
						printf("  Listening socket is readable\n");
						/*************************************************/
						/* Accept all incoming connections that are      */
						/* queued up on the listening socket before we   */
						/* loop back and call select again.              */
						/*************************************************/
						do {
							/**********************************************/
							/* Accept each incoming connection.  If       */
							/* accept fails with EWOULDBLOCK, then we     */
							/* have accepted all of them.  Any other      */
							/* failure on accept will cause us to end the */
							/* server.                                    */
							/**********************************************/
							new_sd = accept(listen_sd, NULL, NULL);
							if (new_sd < 0) {
								if (errno != EWOULDBLOCK) {
									perror("  accept() failed");
								}
								break;
							}

							/**********************************************/
							/* Add the new incoming connection to the     */
							/* master read set                            */
							/**********************************************/
							printf("  New incoming connection - %d\n", new_sd);
							FD_SET(new_sd, &master_set);
							if (new_sd > max_sd)
								max_sd = new_sd;

							/**********************************************/
							/* Loop back up and accept another incoming   */
							/* connection                                 */
							/**********************************************/
						} while (new_sd != -1);
					}

						/****************************************************/
						/* This is not the listening socket, therefore an   */
						/* existing connection must be readable             */
						/****************************************************/
					else {
						printf("  Descriptor %d is readable\n", i);
						close_conn = false;
						/*************************************************/
						/* Receive all incoming data on this socket      */
						/* before we loop back and call select again.    */
						/*************************************************/
						do {
							/**********************************************/
							/* Receive data on this connection until the  */
							/* recv fails with EWOULDBLOCK.  If any other */
							/* failure occurs, we will close the          */
							/* connection.                                */
							/**********************************************/
							rc = recv(i, buffer, sizeof(buffer), 0);
							if (rc < 0) {
								if (errno != EWOULDBLOCK) {
									perror("  recv() failed");
									close_conn = true;
								}
								break;
							}

							/**********************************************/
							/* Check to see if the connection has been    */
							/* closed by the client                       */
							/**********************************************/
							if (rc == 0) {
								printf("  Connection closed\n");
								close_conn = true;
								break;
							}

							/**********************************************/
							/* Data was received                          */
							/**********************************************/
							len = rc;
							printf("  %d bytes received : %s\n", len, buffer);

							/**********************************************/
							/* Echo the data back to the client           */
							/**********************************************/
							rc = send(i, buffer, len, 0);
							if (rc < 0) {
								perror("  send() failed");
								close_conn = true;
								break;
							}

						} while (true);

						/*************************************************/
						/* If the close_conn flag was turned on, we need */
						/* to clean up this active connection.  This     */
						/* clean up process includes removing the        */
						/* descriptor from the master set and            */
						/* determining the new maximum descriptor value  */
						/* based on the bits that are still turned on in */
						/* the master set.                               */
						/*************************************************/
						if (close_conn) {
							close(i);
							FD_CLR(i, &master_set);
							if (i == max_sd) {
								while (FD_ISSET(max_sd, &master_set) == false)
									max_sd -= 1;
							}
						}
					} /* End of existing connection is readable */
				} /* End of if (FD_ISSET(i, &working_set)) */
			} /* End of loop through selectable descriptors */
		}

	};

}


#endif //GAME_SERVERSOCKET_HPP
