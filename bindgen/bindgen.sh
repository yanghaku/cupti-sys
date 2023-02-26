#!/bin/bash

set -ex

if [ "${CUDA_HOME}" == "" ]; then
    # default cuda home
    CUDA_HOME="/usr/local/cuda"
fi

DIRNAME=$(dirname "$0")

# todo: support cuptyActivity*

bindgen \
    --allowlist-type="^CUpti.*" \
    --allowlist-var="^CUpti.*" \
    --allowlist-function="^cupti.*" \
    --allowlist-type=".*_params$" \
    --blocklist-file="${CUDA_HOME}/include/cuda.h" \
    --blocklist-type="^CUpti_Activity.*" \
    --blocklist-function="cuptiGetAutoBoostState" \
    --blocklist-function="cuptiSetThreadIdType" \
    --blocklist-function="cuptiGetThreadIdType" \
    --blocklist-function="^cuptiActivity.*" \
    --default-enum-style=rust \
    --no-doc-comments \
    --with-derive-default \
    --with-derive-eq \
    --with-derive-hash \
    --with-derive-ord \
    ${DIRNAME}/wrapper.h -- -I${CUDA_HOME}/include -I${CUDA_HOME}/extras/CUPTI/include \
    >${DIRNAME}/../src/cupti.rs
