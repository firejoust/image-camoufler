<div align="center">
<h1>Camoufler</h1>
<p><i>An image fingerprinting circumvention tool</i></p>
</div>

## Installation
- User needs to have [`Cargo`](https://www.rust-lang.org/tools/install) and [`git`](https://git-scm.com/downloads) installed.
```bash
git clone https://github.com/firejoust/image-camoufler.git
cd ./image-camoufler
cargo build --release
cd ./target/release
```

## Usage
`camoufler <input_image> <output_folder> [<arguments>]`, where `input_image` is the path to an encoded image.
#### Arguments:
- `--smudge-weight | -w <1-255>` The maximum range of the randomised value
- `--smudge-shade | -s <true|false>` Changes the sign of the randomised value (true: -, false: +)

## Preview
- Randomisation towards `#ffffff` (Shading disabled)
```bash
camoufler ./assets/weetbix1 ./assets --smudge-weight X # Replace X with: 1, 5, 10, 15.
```
<table>
    <tr>
        <td><img src="assets/weetbix1.png"></td>
        <td><img src="assets/weetbix2.png"></td>
        <td><img src="assets/weetbix3.png"></td>
        <td><img src="assets/weetbix4.png"></td>
    </tr>
</table>

- Randomisation towards `#000000` (Shading enabled)
```bash
camoufler ./assets/weetbix1 ./assets --smudge-shade true --smudge-weight X # Replace X with: 1, 5, 10, 15.
```
<table>
    <tr>
        <td><img src="assets/weetbix5.png"></td>
        <td><img src="assets/weetbix6.png"></td>
        <td><img src="assets/weetbix7.png"></td>
        <td><img src="assets/weetbix8.png"></td>
    </tr>
</table>

## Disclaimer
- Creator not responsible for misuse.
