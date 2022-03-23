# wasm

## 连接rust和js

js的`Object`, `Array` 还有`DOM Nodes`对象都分配在垃圾回收堆(GC-heap)上，它与wasm的线性内存空间是分开的，rust中的变量就被分配在这里.
wasm当前并没有访问js内存堆的能力(但未来可能会改变 [Interface Types Proposal - wasm的高级数据类型提案](https://github.com/WebAssembly/interface-types/blob/main/proposals/interface-types/Explainer.md))，但是js能够访问到wasm的内存空间，并进行读写操作, 但是数据类型被限制为固定大小的缓冲区`ArrayBuffer of scalar values(u8, i32, f64, etc...)`
上面那些构成了js和wasm通信的基本模块

`wasm-bindgen`定义了js和wasm如何交互更复杂的数据类型. 包括:
- 装箱(Box)rust结构, 把指针封装进js类中以提高可用性
- 从rust索引到js对象表(table of JavaScript objects)

`wasm-bindgen`只是一个工具，需要我们考虑如何设计数据类型, 那些数据和结构会在js和wasm之间传递

设计接口是需要从下面的一些角度进行优化
- 最小化在js和wasm之间copy数据. 不必要的copy会再来更多的开销
- 最小化序列化和反序列化. 通过尽量使用指针去直接操作wasm线型内存能够减少性能开销

##