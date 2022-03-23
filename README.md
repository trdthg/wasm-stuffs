# Wasm

## 📚 简介

### 内存排布
js的`Object`, `Array`, `DOM Nodes`等对象都存储在GC Heap上
wasm的内存是和js分离的，线型排布，rust的数据就存在其中

### js与rust通讯
wasm目前无法直接访问js的GC Heap(这点可能会改变, wasm提案正尝试为wasm加入高级数据类型), 而js可以直接访问wasm的内存数据，虽然需要把数据转换为固定大小的buf array(u8, i32, f64 ...). wasm函数也只能接受返回标量值. 上面的内容构成了js和wasm通讯的基本模块

### wasm-bindgen
该工具包装了rust数据结构，并能够将指针返回给js, 封装了js对象，直接调用js-api

但是依然需要考虑如何设计数据结构以适配wasm的需要

### 优化方向
- 最小化copy数据, 在js和wasm之间拷贝数据会带来不必要的开销
    如果js能够使用指针直接操作wasm数据，就能大幅减少开销

- 最小化序列化

一些大型的，长期存在的数据结构应该将指针暴露给js

## 🎉 展示

## ⭐ idea

- 字符画网页