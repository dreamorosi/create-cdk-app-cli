# Create CDK App CLI

A simple and opinionated CLI to create a new AWS CDK App in TypeScript.

## Usage

```bash
create-cdk-app --help

Create a new CDK project scaffold 🚀

Usage: create-cdk-app [OPTIONS] <APP_NAME>

Arguments:
  <APP_NAME>  The name of the CDK project (i.e. my-cdk-project)

Options:
  -n, --no-install  Optional flag to skip npm install
  -h, --help        Print help
  -V, --version     Print version
```

## Installation

Clone the repository and run the following commands:

```bash
cargo build --release
cp target/release/create-cdk-app ~/.local/bin
```

Note: Make sure `~/.local/bin` is in your PATH.
