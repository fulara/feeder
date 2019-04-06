# feeder

in order to build for armv6 on ubuntu, use this:
instruction borrowed from: <https://github.com/tiziano88/rust-raspberry-pi/blob/master/Dockerfile>
```
RASPBERRY_PI_TOOLS_COMMIT_ID=5caa7046982f0539cf5380f94da04b31129ed521
wget https://github.com/raspberrypi/tools/archive/$RASPBERRY_PI_TOOLS_COMMIT_ID.zip -O pi-tools.zip
unzip pi-tools.zip

arm-bcm2708/arm-linux-gnueabihf/bin:$PATH
```
now  have this in path whnever you want to cross compile anything: sth like:
```
export PATH=pi-tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH
```

building:
```
rustc_target=arm-unknown-linux-gnueabihf
cargo build --target=$rustc_target && scp target/arm-unknown-linux-gnueabihf/debug/feeder $RASPB:
```
