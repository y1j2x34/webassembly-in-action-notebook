# Meet WebAssembly

## 认识 asm.js

asm.js 是 Mozilla 提出的一个基于 JavaScript 的语法标准，也就是属于 JavaScript 的严格子集，其中的代码采用具有手动内存管理的静态类型语言（C/C++）编写, 代码使用[转译器](https://en.wikipedia.org/wiki/Source-to-source_compiler)编译（例如基于LLVM的Emscripten）。通过语言特性限制在适合提前优化和其他性能改进的范围内，提高性能。

asm.js 的目标并不是提高手写 Javascript 代码的性能，也不实现增强性能以外的其他功能，通常是作为一种通过编译器生成的中间语言。

### 代码生成

例如这个C语言代码

```c
double add(double a, int b) {
    return a + b;
}
```

Emscripten 将输出下列Javascript代码（已简化）：

```javascript
function add(a, b) {
  a = +a;
  b = b | 0;
  return +(a + +(b | 0));
}
```

实际上 asm.js 只提供两种数据类型：

1. *32位带符号整数*
2. *64位带符号浮点数*

对于其他类型都是以数值形式存在，保存在内存中，通过 TypedArray 调用。

asm.js 的变量类型声明有固定写法：

```javascript
function add(a, b) {
    a = +a; // a 是32位整数
    b = b | 0; // b 是64位浮点数
    return +(a + +(b | 0)); // 返回64位浮点数
}
```

asm.js 可以根据这种标记确定变量类型，不需要进行类型推断，浏览器可以根据这些标记进行激进的JIT优化，甚至 AOT 编译， 大幅提升性能。

### asm.js 带来了这些优点

- asm.js 作为一个 Javascript 子集，完全兼容 Javascript 语法，所有支持Javascript的浏览器都可以运行 asm.js 代码
- asm.js 有着更高效的运行速度，
- asm.js 不是用于提高手写 Javascript 的性能，而是由其他语言转译过来的中间语言，这让众多编程语言可以将现有的应用编译成 asm.js, 运行在浏览器中。

### 当然，它也有缺点

1. 类型提示让文件体积变得非常大
2. 代码编写仍然受 Javascript 语法限制，如果要新增功能，将不得不修改 Javascript 语言本身；
3. asm.js文件还是一个javascript 文件，所以它仍然需要被 Javascript 引擎解析，这在移动设备上会减慢加载速度和比较耗电。

## MVP

浏览器产商在研究如何改进 asm.js 时提出的一个 WebAssembly 前身: MVP，它不仅具有 asm.js 的优点，同时也解决了 asm.js 的缺点。
从 2017 年开始，四大浏览器产商都开始支持 MVP， 即 WebAssembly。

### MVP 解决了哪些问题？

1. WebAssembly 代码可以从一开始就被编译成机器代码，从而实现性能改进；
2. 当代码被编译成WebAssembly时，生成的字节码以二进制格式表示，而不是文本格式，这样可以减晓文件大小，从而快速传输和下载；
3. 二进制文件的设计方式使得模块验证可以一次性完成，并且该文件解构还允许并行编译文件的不同部分；
4. 能够在浏览器中运行 JavaScript 以外的语言，让开发技术和运行时技术解耦，极大促进代码复用；
5. 由于不依赖于 JavaScript， 因此可以轻松地对技术进行改造，这种独立性能够更快地改进 WebAssembly。

>
> *流式编译*： 指浏览器下载和接收文件时将 WebAssembly 代码编译成机器代码的过程， 流式编译允许 WebAssembly 模块在完成下载后立即初始化，这大大加快了启动时间。
>

## WebAssembly 模块结构

WebAssembly 目前支持四种数据类型：

- 32位整数
- 64位整数
- 32位浮点数
- 64位浮点数

布尔值则使用32位整数表示，0表示`false`，非0表示`true`。其他类型，包括字符串都需要在模块的线性内存中表示。

WebAssembly 程序（二进制版本和浏览器中的编译版本）主要单元被称为*模块*，下图是一个 WebAssembly 文件的基本结构：

[WebAssembly 文件基本结构](../assets/webassembly-file-structure.drawio ':include :type=code')

[//]: # (![WebAssembly 文件基本结构]&#40;../assets/webassembly-file-structure.png&#41;)

### Preamble

Preamble 包含一个魔数（`0x00 0x61 0x73 0x6D`, 即`\0asm`）, 用于区分 WebAssembly 模块和ES6 模块，这个魔数之后是一个版本号（`0x01 0x00 0x00 0x00`, 即 `1`）,它指示了WebAssembly二进制格式的版本。

在 preamble 之后的模块由几个可选分区区组成，因此可能存在一个不包含任何分区的空模块。

分区有两种： Known分区和自定义分区。

### Known 分区

Known 分区只能存在一个，并且必须以特定的顺序出现，每个分区都有特定用途，定义明确，并且在模块实例化时被校验。

### Cusom 分区

Custom 分区提供了一种在模块内包含数据的方法，用于不适用 Known 分区的用途。Custom 分区可以在模块中的任何位置（在Known 分区之前、中间或之后）出现任意次数, 并且允许多个 Custom 分区重名。

Custom 分区和 Known 分区不同的是，如果 Custom 分区内容错误，则不会触发校验错误。框架还可以延迟加载自定义部分，这表示他们包含的数据可能要到模块初始化后的某个时间点才可用。

## WebAssembly 文本格式

为了让人类可以阅读和编辑 WebAssembly， wasm 二进制格式提供了相应的文本表示。这是一种用来在文本编辑器、浏览器开发者工具等工具中显示的中间形式。

### S-表达式

不论是二进制还是文本格式，WebAssembly代码中的基本单元是一个模块。在文本格式中，一个模块被表示为一个大的S-表达式。

S-表达式是一个非常古老和非常简单的用来表示树的文本格式。因此，我们可以把一个模块想象为一棵由描述了模块结构和代码的节点组成的树。不过，与一门编程语言的抽象语法树不同的是，WebAssembly的树是相当平的，也就是大部分包含了指令列表。

如果我们在浏览器开发者工具查看 wasm 源码时，浏览器开发者工具会使用 WebAssembly 文本格式展示源码：

![WebAssembly source code](../assets/chrome-devtools-s-expression.png)

另外，如果一个空wasm值包含魔数（`0061 736d`）和版本号（`0100 0000`），使用开发者工具查看源码时可以看到一个空模块：

```wasm
(module)
```

## WebAssembly 安全性

WebAssembly 是第一种共享 JavaScript VM 的语言，它在一个沙盒中运行，在这个沙盒中，除了初始化程序时提供给它的ArrayBuffer(WebAssembly 将此 ArrayBuffer 用作线性内存,WebAssembly 会检查以确保代码在这个线性内存上运行)，而无法访问主机的内存和数据。
在 C++ 中，执行堆栈和线性内存一起位于内存中，尽管 C++ 代码不应该修改执行堆栈，但可以使用指针来这么做。WebAssembly 的执行堆栈与线性内存是分开的，代码无法访问。
另外， WebAssembly 也遵守和 JavaScript 相同的安全策略，包括 同源策略 等。

## 哪些语言支持转译为 WebAssembly

- C 和 C++。
- Rust 的目标是成为 WebAssembly 的首选编程语言。
- AssemblyScript 是一个新的编译器，它采用 TypeScript 并将其转换为 WebAssembly。转换 TypeScript 是有意义的，考虑到它是类型化的并且已经转译为 JavaScript。
- TeaVM 是一种将 Java 转换为 JavaScript 的工具，但现在也可以生成 WebAssembly。
- Go 1.11 向 WebAssembly 添加了一个实验性端口，其中包括一个垃圾收集器作为已编译 WebAssembly 模块的一部分。
- Pyodide 是 Python 的一个端口，其中包括 Python 科学堆栈的核心包：Numpy、Pandas 和 matplotlib。
- Blazor 是 Microsoft 将 C# 引入 WebAssembly 的一项实验性工作。

这个 [Github 仓库](https://github.com/appcypher/awesome-wasm-langs) 维护了一个可以编译为 WebAssembly 或将其 VM 包含在 WebAssembly 中的语言列表，并且指出每个语言支持程度：

## 使用场景

2017 开始，所有现代浏览器产商都发布了支持WebAssembly MVP 的浏览器版本， 包括： Chrome, Edge, Firefox, Opera 和 Safari。
一些移动 Web 浏览器也支持 WebAssembly, 包括 Chrome, Firefox for Android 和 Safari。

WebAssembly 在设计时考虑了移植性，因此可以在许多地方使用，而不仅限于浏览器。可以参考这篇关于 [WASI](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/) （WebAssembly Standard Interface）新标准的文章

## 引用

1. [Emscripten 安装](https://emscripten.org/docs/getting_started/downloads.html)
1. [WebAssembly](https://webassembly.org/)
1. [Why marketers should care about mobile page speed](https://www.thinkwithgoogle.com/marketing-strategies/app-and-mobile/mobile-page-speed-load-time/)
1. [理解WebAssembly文本格式](https://developer.mozilla.org/zh-CN/docs/WebAssembly/Understanding_the_text_format)