## Summary
I needed a simple tool for removing whitespace from the bottom of images... and converting them to webp... with high performance... no error handling required.. etc.

Very specific requirements. Probably noone else will need this, but if you do, or need to do domething simular, feel free to use this tool or reuse the code!

## How to use
Compile the binary or run (if you have Rust installed):
```
cargo install --git https://github.com/sverro2/white-remover.git
```

Then run the command like:
```bash
white-remover input.png output.webp
```

## So what does it actually do?
It opens the image, checks from the bottom, per line, if any of the pixels are full white. It keeps doing that until it founds a line without any white, crops the image and saves it as webp.
