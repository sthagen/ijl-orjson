#!/usr/bin/env bash

set -eou pipefail

export PIP_DISABLE_PIP_VERSION_CHECK=1

mkdir -p .cargo && cp ci/config.toml .cargo/config.toml

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain "${RUST_TOOLCHAIN}" --target "${TARGET}" --profile minimal --component rust-src -y

source $HOME/.cargo/env

cargo fetch --target "${TARGET}" &

${PYTHON} -m pip install maturin auditwheel -r test/requirements.txt

${PYTHON} -m maturin build \
  --release \
  --strip \
  --features="${ORJSON_FEATURES}" \
  --compatibility="${ORJSON_COMPATIBILITY}" \
  --interpreter="${PYTHON}" \
  --target="${TARGET}"

${PYTHON} -m auditwheel repair --plat "${ORJSON_TAG}" --only-plat target/wheels/orjson*
rm -r target/wheels/*
mv wheelhouse/* target/wheels

if [[ "${ORJSON_TAG}" != *"manylinux_2_39"* ]]; then
  ${PYTHON} -m pip install target/wheels/orjson*
  PYTHONMALLOC="debug" ${PYTHON} -m pytest -vv test
fi
