# tok4j
tokenizers for java
You will need to manually compile
```sh
cd src/tok4jbinding
cargo buil --release
cp target/release/libtok4jbindings.dylib ../main/resources/
```

Then you should be able to invoke this with
```sh
gradle run
```
