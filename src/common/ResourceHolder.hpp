#ifndef GAME_RESOURCEHOLDER_HPP
#define GAME_RESOURCEHOLDER_HPP

#include <typeindex>
#include <map>

namespace resources {

	class ResourceHolder {
	public:
		template<typename ResourceType>
		void add(ResourceType& resource) {
			resources_[std::type_index(typeid(ResourceType))] = &resource;
		}

		template<typename ResourceType>
		ResourceType& get() {
			return *static_cast<ResourceType*>(resources_[std::type_index(typeid(ResourceType))]);
		}

		template<typename ResourceType>
		bool has() const {
			auto it = resources_.find(std::type_index(typeid(ResourceType)));
			return it != resources_.end() && it->second != nullptr;
		}

	private:
		std::unordered_map<std::type_index, void*> resources_;
	};

} // resources

#endif //GAME_RESOURCEHOLDER_HPP
