# rust-deploy-tool
rust-deploy-tool

# rust 实现 github/gitlab 部署的项目工具

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
