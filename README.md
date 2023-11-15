# Scope Exit utils

Will call closure when leave current scope.
Useful to release some resource got from FFI.

e.g. Call c file API

```rust
fn test() {
    let fp = fopen("test.txt");
    scope_exit!(|| fclose(fp););
    // will call `fclose` by RAII
}
```
