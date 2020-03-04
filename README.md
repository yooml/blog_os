# blog_os

这是一个用rust写的个人os，学习自 https://os.phil-opp.com/

#### 如何构建：
1.先要安装rust最新的nightly版本和qemu
```bash
rustup default nightly
rustup toolchain list
```

2.然后安装xbuild用于交叉编译：
```bash
cargo install cargo-xbuild

#其他依赖：
cargo install bootimage --version "^0.7.7"
rustup component add llvm-tools-preview
rustup component add rust-src
```

3.构建：
```bash
cargo xrun
#or
cargo xbuild
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
```

4.测试：
```bash
cargo xtest
cargo xtest --lib
cargo xtest --test should_panic
```
