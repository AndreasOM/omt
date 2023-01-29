#!/bin/sh

#cargo run --release --bin omt-atlas -- combine --output Data/test-atlas --maximum-size 64 -s 1024 -b 32 -i Data/64x64_green.png Data/64x64_red.png Data/64x64_blue.png -r Data/
#cargo run --release --bin omt-atlas -- combine --output Data/test-atlas-%d --maximum-size 64 -b 32 -i Data/64x64_green.png Data/64x64_red.png Data/64x64_blue.png -r Data/
cargo run --release --bin omt-atlas -- combine --output Data/test-atlas-%d --maximum-size 256 -b 8 -i Data/64x64_green.png Data/64x64_red.png Data/64x64_blue.png -r Data/
cargo run --release --bin omt-atlas -- info -i Data/test-atlas-0
for o in Data/*omtr
do
	n=$(basename ${o} .omtr)
	a=$(cat ${o})
	echo ${n} "->" ${a}
done
cargo run --release --bin omt-atlas -- preview -i Data/test-atlas-0

