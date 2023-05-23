#! /bin/bash
assert() {
    expected="$1"
    input="$2"

    # 短路运算，仅在左边为0(即成功)时exit
    cargo run -q -- "$input" >tmp.s || exit
    riscv64-linux-gnu-gcc -static tmp.s -o tmp
    qemu-riscv64 -L "$RISCV"/sysroot ./tmp
    actual=$?

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

assert 0 0
assert 42 42
assert 34 "12-34+56"
assert 41 "  12 + 34 - 5"
echo "---PASS---"
