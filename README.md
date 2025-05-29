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
1. Get the src code. You can either do this by cloning the repo to get the latest dev code (`git clone https://github.com/matthewyang204/GnuPG-Decryption-Encryption-Tool.git`) or downloading the latest ZIP at its release and decompressing it.
2. Run the following to build:
```
./configure
make -j$(nproc)
```
3. Install with `make install` to install to the default prefix of `/usr/local`. Alternatively, you can install to a specific prefix by running `make install PREFIX={yourprefix}` where {yourprefix} is your custom prefix. Use `sudo` when needed.
