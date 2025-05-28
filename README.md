# GnuPG Decryption and Encryption Tool

GnuPG Decryption and Encryption Tool is a simple wrapper around GnuPG to make it more user-friendly.

# Usage
`gpgde -d|-e filename`
- `-d`, `--decrypt`  Decrypt file specified
- `-e`, `--encrypt`  Encrypt file specified
- `filename`         Name of file

# Requirements
- `rustc`
- `make`
- `gpg`

# Building and Installing
1. Run the following to build:
```
./configure
make -j$(nproc)
```
2. Install with `make install` to install to the default prefix of `/usr/local`. Alternatively, you can install to a specific prefix by running `make install PREFIX={yourprefix}` where {yourprefix} is your custom prefix. Use sudo when needed.