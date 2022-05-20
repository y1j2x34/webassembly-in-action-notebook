# WebAssembly 模块与 JavaScript 互操作（Rust语言）

## JavaScript 调用 Rust 函数

在使用 rust 创建 WebAssembly 模块前，先安装工具：

```bash
cargo install wasm-pack
```

创建项目

```bash
cargo new --lib hello
```

`Cargo.toml`:

```toml
[package]
name = "hello-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.80"
```

`src/lib.rs`

```rust
use wasm_bindgen::prelude::*;

// 导出 add 函数
#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}
```

编译代码：

```bash
wasm-pack build --target nodejs
```

在 JavaScript 中使用:

```js
const rust = import('./pkg/hello.js');

rust
    .then(m => m.add(100, 100))
    .then(result => console.log(result));
```

运行 `node ./index.js` 将顺利输出 200。

## Rust 调用 JavaScript

`#[wasm_bindgen]` 宏可以加到 `extern "C" {}` 上来导入 JavaScript 中的函数。 这也是 `js-sys` 和 `web-sys` 这两个库的原理。
我们也可以在自己的代码中使用这一特性。

首先，我们先创建一个 js 文件， 提供给 Rust 调用。

```js
// def.js
export function reload() {
    location.reload();
}
export class User {
    constructor(id) {
        this.id = id;
    }
    get id() {
        return id;
    }
    set id(i) {
        this.id = i;
    }
    say() {
        console.trace('Hello')
    }
}
```

然后在 Rust 中导入：

```rust
#[wasm_bindgen(module = "path/to/def.js")]
extern "C" {
    fn reload() -> ();
    
    type User;
    
    #[wasm_bingen(constructor)]
    fn new() -> User;
    
    #[wasm_binden(method, getter)]
    fn id(this: &User) -> u32;
    
    #[wasm_bindgen(method, setter)]
    fn set_id(this: &User, id: u32) -> User;
    
    #[wasm_bindgen(method)]
    fn say();
}
#[wasm_bindgen(start)]
pub fn run() {
    let u = User::new();
    u.set_id("123asd");
    u.say();
    reload();
}
```



## JavaScript 如何与 Rust 交换数据
