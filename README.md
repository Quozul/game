## Dependencies

### OpenSSL

```shell
sudo dnf install openssl-devel
```

### raylib

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

### Box2D

```shell
git clone https://github.com/erincatto/box2d.git
cd box2d
git checkout v2.4.1
mkdir build && cd build
cmake -DBUILD_SHARED_LIBS=ON -DBOX2D_BUILD_UNIT_TESTS=OFF ..
make -j $(nproc)
sudo make install -j $(nproc)
```

### Raygui

```shell
git clone https://github.com/raysan5/raygui.git
git checkout 3.2
mv src/raygui.h src/raygui.c
gcc -o raygui.so src/raygui.c -shared -fpic -DRAYGUI_IMPLEMENTATION -lraylib -lGL -lm -lpthread -ldl -lrt -lX11
sudo cp src/raygui.c /usr/include/raygui.h
sudo cp raygui.so /usr/lib/raygui.so
```
