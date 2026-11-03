# Create CDK App CLI

A simple and opinionated CLI to create a new AWS CDK App in TypeScript.

Use Node.js 24 LTS and npm for generated projects. Node.js 22.18 or later
in the 22.x release line is also supported. The CDK entry point runs TypeScript
directly with Node.js, without a separate loader.

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

### Homebrew (macOS/Linux)

```bash
brew install dreamorosi/tap/create-cdk-app
```

### Shell installer

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dreamorosi/create-cdk-app-cli/releases/latest/download/create-cdk-app-installer.sh | sh
```

Prebuilt binaries for Apple Silicon macOS and x64/ARM64 Linux are also available on the [releases page](https://github.com/dreamorosi/create-cdk-app-cli/releases/latest).

### From source

Clone the repository and run the following commands:

```bash
cargo build --release
cp target/release/create-cdk-app ~/.local/bin
```

Note: Make sure `~/.local/bin` is in your PATH.

## Checking the scaffold

With Rust, Node.js, and npm installed, run:

```bash
bash scripts/check-scaffold.sh
```

This builds the CLI, creates a project in a temporary directory, installs its
locked dependencies, and runs build, lint, tests, and CDK synth. It also checks
that lint ignores synthesized output but catches errors in the infrastructure
source under `lib/` and that synth reports no unconfigured feature flags.
The temporary project is removed when the script exits.
No AWS credentials are needed, and no resources are deployed.

The same check runs in CI on Node.js 22.18 and 24, including on dependency-update
pull requests.
