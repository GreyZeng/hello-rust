# Rust学习笔记

## Rust优势

- 高性能媲美C/C++，可以做嵌入式开发
- 没有GC，同时也不需要手工管理内存(所有权机制)
- 没有野指针
- 并发安全
- 集所有语言之大成者


Rust中的所有权的转移
v2= v1，发生所有权转移
变量默认read-only，如需修改，需要显示声明mut