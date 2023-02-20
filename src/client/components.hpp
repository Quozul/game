#ifndef GAME_COMPONENTS_HPP
#define GAME_COMPONENTS_HPP

#include <string>
#include <raylib.h>

struct Position {
	int x;
	int y;
};

struct Input {
	std::string text;
	Rectangle rectangle;
};

struct Button {
	std::string text;
	Rectangle rectangle;
};

struct Form {};

#endif //GAME_COMPONENTS_HPP
