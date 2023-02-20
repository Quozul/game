#ifndef GAME_SYSTEMS_HPP
#define GAME_SYSTEMS_HPP

#include <entt/entt.hpp>
#include <raygui.h>
#include "../common/net/ClientSocket.hpp"

void setup(entt::registry &registry);

void setup_inputs(entt::registry &registry);

void text_drawing(entt::registry &registry);

void tick_inputs(entt::registry &registry);

void tick_forms(entt::registry &registry);

void drawing_squares(entt::registry &registry);

#endif //GAME_SYSTEMS_HPP
