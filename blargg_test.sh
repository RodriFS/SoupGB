#!/bin/sh
cargo run test_rom/cpu_instrs/cpu_instrs.gb &
PID=$!
sleep 70
kill $PID

cargo run test_rom/instr_timing/instr_timing.gb &
PID=$!
sleep 4
kill $PID

cargo run test_rom/mem_timing/mem_timing.gb &
PID=$!
sleep 4
kill $PID

cargo run test_rom/mem_timing-2/mem_timing.gb &
PID=$!
sleep 4
kill $PID

