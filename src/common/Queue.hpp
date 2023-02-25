// Source: https://juanchopanzacpp.wordpress.com/2013/02/26/concurrent-queue-c11/

#ifndef GAME_QUEUE_HPP
#define GAME_QUEUE_HPP

#include <queue>
#include <thread>
#include <mutex>
#include <condition_variable>

template<typename T>
class Queue {
public:
	T pop() {
		std::unique_lock<std::mutex> mlock(mutex);
		while (queue.empty()) {
			cond.wait(mlock);
		}
		auto item = queue.front();
		queue.pop();
		return item;
	}

	void push(const T &item) {
		queue.push(item);
		cond.notify_one();
	}

	int size() {
		return queue.size();
	}

private:
	std::queue<T> queue;
	std::mutex mutex;
	std::condition_variable cond;
};

#endif //GAME_QUEUE_HPP