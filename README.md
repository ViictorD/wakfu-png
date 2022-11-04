# Wakfu
Wakfu map exporter as png

![Imgur](https://i.imgur.com/MeFbkLQ.jpg)
![Imgur](https://i.imgur.com/LC9Pwnn.jpg)
![Imgur](https://i.imgur.com/8OA5IoI.png)


# Building

This project requires [cargo](https://crates.io) to build.

Once everything is in place:
```bash
cargo build --release
```

# Using


```bash
wakfu_png --path /path/to/game/Ankama/Wakfu --map 527|paper [--recursive] [--indoor]
```

`--map` is the map id you want to export or use `paper` to extract all global paper maps.

`--recursive` allows to recursively extract all maps that are linked (through teleporter) to the first specified map.

`--indoor` allows to render inside houses. This flag is passed to children if `recursive` is used.

After the program is done, you can find the result in the `output` folder.

## Note

This project does not include **any** authored assets. In order to run it, you may get such assets by obtaining a copy of the game Wakfu, created by Ankama Games.