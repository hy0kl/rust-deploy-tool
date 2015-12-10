# rust-deploy-tool
rust-deploy-tool

# rust 实现 github/gitlab 部署项目的小工具

* 并行化部署项目,防止机器多时出现部署代码不一致的情况;
* Terminal 彩色化输出,有趣味,更实用.

```
1. 安装 Rust
2. clone 项目
3. cargo build --release
4. 参考 ./conf/demo.json 写项目部署配置文件
5. 使用例子:
$ ./target/release/deploy
-----USAGE----
./target/release/deploy project deploy             deploy project with latest <head>.
./target/release/deploy project rollback <head>    rollback with <head>.

注: 编译生成的工具是目录无关的.
```
