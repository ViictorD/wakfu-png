WAKFU_GAME_PATH=$1

cargo build --release

./target/release/wakfu_png --path $WAKFU_GAME_PATH --map paper

for f in $WAKFU_GAME_PATH/contents/maps/gfx/*.jar
do
	map_id=$(basename $f .jar)
	./target/release/wakfu_png --path $WAKFU_GAME_PATH --map $map_id
	./target/release/wakfu_png --path $WAKFU_GAME_PATH --map $map_id --indoor
done