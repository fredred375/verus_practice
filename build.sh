set -xeu

# script dir is root of repo
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
cd "$DIR/src"

rv=$VERUS_DIR/source/target-verus/release/verus

# Build deps hack first, think of it as a library
# We need to build it first so that we can use the
# `--extern` flag to link it to the main program
cd deps_hack
cargo build

cd ..

# Build the main program, note that verus needs the --verify-module flag
# to verify the modules we are interested in
"$rv" -L dependency=deps_hack/target/debug/deps \
  --extern=deps_hack="deps_hack/target/debug/libdeps_hack.rlib" \
  --compile \
  --verify-module mystruct::exec::check \
  --verify-module mystruct::trusted::exec_types \
  --verify-module mystruct::trusted::spec_types \
  main.rs