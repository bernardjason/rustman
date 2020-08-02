# rustman

first rust program. A pacman clone

on linux to target windows
```
export PKG_CONFIG_ALLOW_CROSS=1
cargo build --target x86_64-pc-windows-gnu
```

or to switch off sound, wine should then work
```
cargo build --target x86_64-pc-windows-gnu  --features soundoff
```

or from linux
cargo build

the windows target should have al the dependencies from mingw in the code base. I've tried on a normal Windows PC
For linux maybe but I to do a package installs during dev. These maybe enough
```
sudo apt-get install libsdl2-gfx-dev libsdl2-image-dev libsdl2-ttf-dev
```

to deliver you need to zip up in the debug directory
rustman
artifacts/
any dlls

```
cd target/x86_64-pc-windows-gnu/debug
zip rustman_windows.zip rustman.exe artifacts/* *dll
```

or
```
cd target/debug
zip rustman_linux.zip rustman artifacts/*
```

To run release download either
https://github.com/bernardjason/rustman/releases/download/0.2/rustman_linux.zip

or

https://github.com/bernardjason/rustman/releases/download/0.2/rustman_windows.zip

##### TO DO still maybe..
do something when lives ended.
