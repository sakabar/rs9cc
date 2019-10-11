#!/bin/bash
set -u

try() {
    expected="$1"
    input="$2"

    tmp_dir=$(mktemp -d)
    ./target/debug/rs9cc "${input}" > ${tmp_dir}/tmp.s
    if [[ $? -ne 0 ]]; then
        echo "Rustによるコンパイルが失敗しました: input = ${input}"
        exit 1
    fi
    gcc -o ${tmp_dir}/tmp ${tmp_dir}/tmp.s
    if [[ $? -ne 0 ]]; then
        echo "アセンブリに失敗しました: input = ${input}"
        exit 1
    fi

    ${tmp_dir}/tmp
    actual="$?"

    if [[ "${actual}" = "${expected}" ]]; then
        echo "${input} => ${actual}"
        rm -rf ${tmp_dir}
    else
        echo "${input} => ${expected} expected, but got ${actual}"
        rm -rf ${tmp_dir}
        exit 1
    fi
}

try 0 0
try 42 42
try 21 "5+20-4"

echo "OK"
