#!/bin/bash
cargo build --release
chmod +x ./target/release/yacd
sudo cp ./target/release/yacd /usr/local/bin
echo "Installation completed. You can now use 'yacd' in the terminal."