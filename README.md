# Tinycom
**Tiny COM/Serial interation program**

# Intallation

```bash
cargo install --git https://github.com/sy1ntexx/tinycom
# By default cargo installs to ~/.cargo/bin/
# Make sure this directory in $PATH otherwise you won't be able to launch the program
```

# Usage
```bash
# By default `tinycom` will try to connect to `/dev/ttyACM0`. You can change it by passing `-d` argument.
[user@desktop ~]$ sudo tinycom -d /dev/ttyACM1
```
