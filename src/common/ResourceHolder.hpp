#ifndef GAME_RESOURCEHOLDER_HPP
#define GAME_RESOURCEHOLDER_HPP

#include <typeindex>
#include <map>

namespace resources {

	class ResourceHolder {
	public:
		template <typename T>
		void add(T& resource) {
			resources[std::type_index(typeid(T))] = &resource;
		}

		template <typename T>
		void add_ptr(T* resource) {
			resources[std::type_index(typeid(T))] = resource;
		}

		template <typename T>
		T& get() {
			return *static_cast<T*>(resources[std::type_index(typeid(T))]);
		}

		template <typename T>
		T* get_ptr() {
			return static_cast<T*>(resources[std::type_index(typeid(T))]);
		}

		template <typename T>
		bool has() {
			return resources.count(std::type_index(typeid(T))) != 0;
		}

		~ResourceHolder() {
			std::cout << "ResourceHolder destroyed" << std::endl;
		}

	private:
		std::unordered_map<std::type_index, void*> resources;
	};


} // resources

#endif //GAME_RESOURCEHOLDER_HPP
