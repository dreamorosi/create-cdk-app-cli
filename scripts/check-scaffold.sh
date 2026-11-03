#!/usr/bin/env bash
set -euo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch_dir="$(mktemp -d "${TMPDIR:-/tmp}/create-cdk-app.XXXXXX")"
trap 'rm -rf "$scratch_dir"' EXIT

cd "$scratch_dir"
cargo run --quiet --locked --manifest-path "$repo_dir/Cargo.toml" -- scaffold-check
cd scaffold-check

npm run build
npm run lint
npm test
npm run cdk -- synth --quiet 2>&1 | tee "$scratch_dir/synth.log"
if grep -q 'feature flags are not configured' "$scratch_dir/synth.log"; then
  echo "The scaffold has unconfigured CDK feature flags" >&2
  exit 1
fi

# Generated CDK output must not cause lint failures.
npm run lint

# Make sure lint checks the infrastructure source, not just the Lambda handler.
printf '\ndebugger;\n' >> lib/scaffoldcheck-stack.ts
if npm run lint > "$scratch_dir/lint-negative.log" 2>&1; then
  echo "Lint failed to detect an error in lib/scaffoldcheck-stack.ts" >&2
  exit 1
fi
grep -q 'lint/suspicious/noDebugger' "$scratch_dir/lint-negative.log"
