# Ri18n

## Usage
Using Ri18n to generate i18n codes, you can run command:
```shell
ri18n -s path/to/your/i18nfile_{iso639}.[yaml, json, properties, toml] -t src/ -n name_of_code_file java
```

## Build
To compile cross platform, we should install [cross](https://github.com/cross-rs/cross) by using command:
`cargo install cross --git https://github.com/cross-rs/cross`

After installing `cross` application, we should pull and run docker images to guarantee that `Ri18n` can be compiled cross difference system:
* `docker pull freeznet/aarch64-apple-darwin-cross:11.3.1`
* `docker pull freeznet/x86_64-unknown-linux-musl-cross:11.3.1`

and also need to install rustup targets:
* `rustup target add aarch64-apple-darwin`
* `rustup target add x86_64-apple-darwin-cross:11.3.1`
* `rustup target add x86_64-unknown-linux-musl`

If everything goes well, then we can build `ri18n` cross platform, for instance:
`cross build --release --target x86_64-apple-darwin`