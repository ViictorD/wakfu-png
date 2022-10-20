# Wakfu
Wakfu map exporter as png

![Imgur](https://i.imgur.com/KKEgEtE.jpg)


# Building

This project requires [cargo](https://crates.io) to build.

Once everything is in place:
```bash
cargo build --release
```

# Using


```bash
wakfu_png --path /path/to/game/Ankama/Wakfu --map 527 [--recursive]
```

`--recursive` flag is optional. It allows to recursively extract all maps that are linked (through teleporter) to the first specified map.

After the program is done, you can find the result in the `output` folder.

Note that this project does not include **any** authored assets. In order to run it, you may get such assets by obtaining a copy of the game Wakfu, created by Ankama Games.

# Credit

Based initially on @jac3km4 `vakfu` projet.