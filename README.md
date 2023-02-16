## Dependencies

### ENet

```shell
git clone https://github.com/lsalzman/enet.git enet 
cd enet
git checkout v1.3.17
sudo dnf install autoconf automake libtool
autoreconf -vfi
./configure 
make 
sudo make install
```

### Raylib

```shell
git clone https://github.com/raysan5/raylib.git raylib
cd raylib
git checkout 4.2.0
mkdir build && cd build
cmake -DBUILD_SHARED_LIBS=ON ..
make
sudo make install
```

### EnTT

```shell
git clone git@github.com:skypjack/entt.git entt
cd entt
git checkout v3.11.1
mkdir build && cd build
cmake -DBUILD_SHARED_LIBS=ON ..
make
sudo make install
```