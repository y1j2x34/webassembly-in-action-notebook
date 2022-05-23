# `wasm-bindgen` 入门

WebAssembly 规范只定义了四种类型：

- 32位整数
- 64位整数
- 32位浮点数
- 64位浮点数

但是，大多情况下， Js 和 Rust 开发人员会使用到更多更丰富的类型，例如最常用的字符串，以及 DOM/BOM 对象等。

`wasm-bindgen` 则提供了一个 JS 和 Rust 类型之间的桥梁，它可以让 JS 在调用 Rust API 时使用字符串， 或者使用 Rust 函数来捕获 JS 异常。方便实现 JS 和 Rust 互操作。

`wasm-bindgen` 功能：

- 在 Rust 中导入JS模块和接口，例如 DOM、console 以及 performance 等；
- 导出 Rust 结构体或函數等，并在 JS 中调用。
- 使用丰富的类型：字符串、数字、类、闭包和对象，而不是简单的四种WebAssembly规范类型；
- 为导出给 JS 调用的 Rust 代码生成 `d.ts`

## Hello World 项目

### 开始准备环境

1. 安装 [`wasm-pack`](https://github.com/rustwasm/wasm-pack) 插件

    ```bash
    cargo install wasm-pack
    ```

2. 创建项目

    ```bash
    cargo new --lib hello-wasm
    ````

3. 添加配置
    
    ```toml
    [lib]
    crate-type = ["cdylib"]
    
    [dependencies]
    wasm-bindgen = "0.2.80"
    ```

    > 为什么是 `cdylib` ?
     `cdylib` 是一种为其他语言提供的动态库，它会静态链接 rust 的标准库，以及递归的静态链接该 crate 的所有依赖。
     这样输出的动态库只会暴露所声明的符号， 不会依赖标准库的符号。`cdylib` 最终会作为其他语言编写的可执行程序的动态库依赖，或者被其他语言以 `dlopenn` 等动态加载方式加载。
     参考： <https://doc.rust-lang.org/reference/linkage.html> 

4. 编写代码：
 
   ```rust
    // src/lib.rs
    // 使用一个外部库
    extern crate wasm_bindgen;
   
    use wasm_bindgen::prelude::*;
   
    #[wasm_bindgen]
    extern {
        // 调用 JavaScript 函数（window.alert）
        pub fn alert(s: &str);
    }
   // 定义一个可以被 JavaScript 调用的函数
    #[wasm_bindgen]
    pub fn say(what: &str) {
        alert(&format!("Hello, {}!", name));
    }
    ```

5. 构建

    ```bash
    wasm-pack build --target web
    ```
   
    构建过程：
   1. 将 Rust 编译为 WebAssembly;
   2. 在编译好的 WebAssembly 代码基础上运行 `wasm-bindgen`, 生成一个 js 文件，这个 js 会导入 wasm 文件，并包装成模块；
   3. 创建一个 `pkg` 文件夹，生成的 wasm 文件和 js 文件都会放入其中;
   4. 读取 `cargo.toml` 生成相应的 `package.json`;
   5. 如果有 `README.md` ，会被复制到 `pkg` 目录中。
