# vsolve

This is a port of some PHP code to Rust to encode/decode Vigenère as well as a brute-force exploration of all keys 
of a given length.

## Example usage

```sh
$ cargo build --release

# get usage info
$ target/release/vsolve
Usage: target/release/vsolve [options] [key] input-string

Options:
    -d, --decode        decode input
    -b, --brute [LEN]   bruteforce key [optional len]

# encode
$ target/release/vsolve key "For centuries, people thought that this cipher was completely unbreakble."
PSP MILDYPSIQ, ZIMZPC DLMEKFD XFKX RRMQ MMNRIP GEQ MSKZPCDIJI YLLVCKOZVI.
chi: 637.8382

# decode (-d)
$ target/release/vsolve -d key "PSP MILDYPSIQ, ZIMZPC DLMEKFD XFKX RRMQ MMNRIP GEQ MSKZPCDIJI YLLVCKOZVI."
FOR CENTURIES, PEOPLE THOUGHT THAT THIS CIPHER WAS COMPLETELY UNBREAKBLE.
chi: 18.876003

# brute force, specifying key length (-b3)
$ target/release/vsolve -b3 "PSP MILDYPSIQ, ZIMZPC DLMEKFD XFKX RRMQ MMNRIP GEQ MSKZPCDIJI YLLVCKOZVI."
SEB
18.876003 [KEY] FOR CENTURIES, PEOPLE THOUGHT THAT THIS CIPHER WAS COMPLETELY UNBREAKBLE.
```

## Todo / future development

In order to achieve parity with PHP-based tools I've written for Vigenère cipher:

- non-brute-force deciphering using Kasiski's method and other staticistical methods
- dictionary mode brute force
- keep and show the top N results (rather than just top 1); keep in priority queue (max heap) for efficiency

Other todos:

- compile functionality into a library, adding other useful crypto functions
- build a PHP-extension to use this library
