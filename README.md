Here is the text in a single, copyable format:

```markdown
# WSL and Development Environment Setup

## Install WSL
For detailed instructions, visit the [WSL Installation Guide](https://learn.microsoft.com/en-us/windows/wsl/install) and [WSL Basic Commands](https://learn.microsoft.com/en-us/windows/wsl/basic-commands).

```sh
wsl -l -v
wsl --install Ubuntu-22.04
wsl --set-default Ubuntu-22.04
wsl
```

## Install Node.js
```sh
curl -sL https://deb.nodesource.com/setup_20.x -o /tmp/nodesource_setup.sh
sudo bash /tmp/nodesource_setup.sh
sudo apt install nodejs
```

## Install Rust
```sh
sudo apt install build-essential 
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
exit
wsl --shutdown
wsl
```

## Install Zig
Follow the [Zig Getting Started Guide](https://ziglang.org/learn/getting-started/).

```sh
wget https://ziglang.org/builds/zig-linux-x86_64-0.14.0-dev.184+bf588f67d.tar.xz
tar -xf zig-linux-x86_64-0.14.0-dev.184+bf588f67d.tar.xz
sudo mv zig-linux-x86_64-0.14.0-dev.184+bf588f67d /usr/local/zig
echo 'export PATH=$PATH:/usr/local/zig' >> ~/.bashrc
source ~/.bashrc
zig version
```

## Additional Resources
- [Setting up Rust Development Environment on WSL2](https://harsimranmaan.medium.com/install-and-setup-rust-development-environment-on-wsl2-dccb4bf63700)
- [Rust-based AWS Lambda with AWS CDK Deployment](https://medium.com/techhappily/rust-based-aws-lambda-with-aws-cdk-deployment-14a9a8652d62)
- [AWS Lambda Rust Runtime on GitHub](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main)

## Install Cargo Lambda
```sh
cargo install cargo-lambda
cargo lambda build --release
cargo lambda build --release --target x86_64-unknown-linux-musl
```

## Install Node.js and NPM
```sh
sudo apt install nodejs npm
wsl --shutdown
which npm
```
```
