# rust_learning
记录学习rust过程中的exercise以及练习

目标: 作为初步的技能掌握，为后面的 rCore 等训练打一个基础。
## 第一阶段 —— 阶段目标: 熟练掌握 rust 语言语法
第一阶段的参考资料:
Rust语言圣经(Rust Course)

https://course.rs/about-book.html

第一阶段配套的参考习题:

https://github.com/sunface/rust-by-practice

参照了 rCore 中第一阶段验证完成度的部分，110 道 rustling 习题

https://github.com/LearningOS/rust-rustlings-2024-spring-fi-dev-tang

目前已经安装到本地，现在的计划是 Rust语言圣经 or Rustlings 练习作为每天的日常。

## 2024/6/27 摸索 rustlings 的执行方式
目前的经验是, 在 rust-rustlings-2024-spring-fi-dev-tang 目录下执行
```bash
rustlings watch
```
显示待解决的问题

```bash
rustlings list
```
可以显示当前待测的练习

对于编译情况，这里并没有按照 cargo 项目的格式进行打包，测试运行可以直接按照
```bash
rustc intro1.rs
./intro1
```
得到结果。

![rustling_into1](picture/rustlings_intro1.png)

现在分成两个学习教程，以 Rust 语言圣经为主，同时兼顾 Rust-by-example 中的完整代码程序。