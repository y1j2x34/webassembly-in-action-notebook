# WebAssembly 模块与 JavaScript 互操作（Rust语言）

关于如何使用rust创建WebAssembly， 可以参考这篇 [`wasm_bindgen` 入门](./getting-started-with-wasm-bindgen.md)。

## JavaScript 调用 Rust 函数

首先要在 Rust 中导出函数：

`src/lib.rs`

```rust
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}
```

编译代码：

```bash
wasm-pack build --target nodejs
```
当然， target 也可以指定为 `web`, 表示生成物要在浏览器中运行。

在 JavaScript 中使用:

```js
import init, { add } from './pkg/js_call_rust.js';  // js_call_rust 是 cargo.toml 配置的 Package name

(async () => {
    await init();
    console.log(add(1, 1)) // 2
})();
```

## Rust 调用 JavaScript

[//]: # (`#[wasm_bindgen]` 宏可以加到 `extern "C" {}` 上来导入 JavaScript 中的函数。 这也是 `js-sys` 和 `web-sys` 这两个库的原理。)

首先，我们先创建一个 js 文件， js 文件使用 `export` 语句导出方法和类型：

```js
// js-module.js
export function reload() {
    console.log('reload method')
}
export class User {
    constructor(id) {
        this._id = id;
    }
    get id() {
        return this._id;
    }
    set id(i) {
        this._id = i;
    }
    say() {
        console.trace('Hello')
    }
}
```

然后在 Rust 中引用 js 文件：

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js-module.js")]
extern "C" {
    
}
```

映射 js 方法和类以及类成员

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js-module.js")]
extern "C" {
    fn reload() -> JsValue;

    type User; // class User

    #[wasm_bindgen(constructor)]
    fn new(id: String) -> User;

    #[wasm_bindgen(method, getter)]
    fn id(this: &User) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_id(this: &User, id: String) -> User;

    #[wasm_bindgen(method)]
    fn say(this: &User);
}

#[wasm_bindgen(start)]
pub fn run() {
    let u = User::new(String::from("456789"));
    u.set_id(String::from("123asd"));
    u.say();
    reload();
}
```

编译：

```bash
wasm-pack build --target web
```

HTML 页面：

```html
<script type="module">
    import init from './pkg/import_js.js';
    init().then(mod => {
        console.log(mod);
    })
</script>
```

控制台输出：

![Rust 中调用 js](../assets/rust-call-js.png)
