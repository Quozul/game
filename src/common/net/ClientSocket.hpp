#ifndef GAME_CLIENTSOCKET_HPP
#define GAME_CLIENTSOCKET_HPP


#include <netinet/in.h>
#include <csignal>
#include <functional>
#include <openssl/types.h>
#include <iostream>
#include <arpa/inet.h>
#include <thread>
#include "network.hpp"
#include "Channel.hpp"
#include "BaseSocket.hpp"

namespace net {

	class ClientSocket : private BaseSocket {
	private:
		SSL *ssl = nullptr;
		net::Channel *channel;

	public:
		bool connected = false;

		explicit ClientSocket(const std::string &rem_server_ip, events::EventLoop &events);

		void write(char *data);

		bool read();

		void loop();

		[[noreturn]] static void client_loop(ClientSocket *client) {
			while (true) {
				client->loop();
			}
		}

		void start_loop() {
			std::thread handler(std::bind(client_loop, this));
			handler.detach();
		}

		~ClientSocket();
	};

}


#endif //GAME_CLIENTSOCKET_HPP
