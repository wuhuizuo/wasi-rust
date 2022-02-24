wasm-wasi rust 语言生态验证
===

调研运行时:
- wasmtime
- wasmedge

## 文件IO

> 代码目录: `io/`

结论: 都运行ok.

- wasmtime: `wasmtime --dir=. target/wasm32-wasi/debug/io.wasm <args>`
- wasmedge: `wasmedge --dir=.:. target/wasm32-wasi/debug/io.wasm <args>`

## 网络请求： http请求

> 代码目录: `http-request`

结论: 目前标准wasi没有针对这块的能力，但wasmedge针对进行了自己的socket扩展。在wasi app中需要使用wasmedge自己的生态http请求库能成功。

- wasmtime: 不支持
- wasmedge: 仅支持ip地址形式的http请求，域名形式和https都不支持。

## lib 模式:

> 这里只体现编写的方式.
> 代码目录: `lib-demo`

不同运行时运行方式:

- wasmtime: 
  - `wasmtime target/wasm32-wasi/debug/lib_demo.wasm --invoke greet 2`
  - `wasmtime --invoke greet target/wasm32-wasi/debug/lib_demo.wasm 2`
- wasmedge: 
  - `wasmedge --reactor target/wasm32-wasi/debug/lib_demo.wasm greet -100`
- wasmer:
  - `wasmer target/wasm32-wasi/debug/lib_demo.wasm -i greet1 1`
    > 运行失败: 
    > [root@VM-77-32-centos lib-demo]# wasmer target/wasm32-wasi/debug/lib_demo.wasm -i greet1
    > wasmer: /lib64/libtinfo.so.5: no version information available (required by wasmer)
    > error: failed to run `target/wasm32-wasi/debug/lib_demo.wasm`
    > ╰─▶ 1: Error while importing "wasi_snapshot_preview1"."fd_write": unknown import. Expected Function(FunctionType { params: [I32, I32, I32, I32], results: [I32] })
    > [root@VM-77-32-centos lib-demo]# wasmer --version
    > wasmer: /lib64/libtinfo.so.5: no version information available (required by wasmer)
    > wasmer 2.1.1

### 关于wasm-bindgen

使用 wasm-bindgen 运行时会报找不到 bindgen相关模块的问题。
wasm-bindgen是给js浏览器运行时使用的，wasmtime及wasmedge运行时并不兼容这种编译产物。

