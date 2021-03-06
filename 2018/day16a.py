#!/usr/bin/env python3
#pylint: disable=invalid-name

import re
import copy

################################################################################
# Run
################################################################################
def solve(examples):
    num_matching_funs = [num_funs_matching_example(e) for e in examples]
    return count(num_matching_funs, lambda n: n >= 3)

def num_funs_matching_example(example):
    before, instruction, after = example
    num_matching = 0
    for fun in FUNCTIONS:
        result = run_example_fun(before, instruction, fun)
        if result == after:
            num_matching += 1
    return num_matching

def run_example_fun(registers, instruction, fun):
    registers = copy.deepcopy(registers)
    _, a, b, c = instruction
    fun(a, b, c, registers)
    return registers

def count(enumerable, fun):
    return sum(1 for val in enumerable if fun(val))

################################################################################
# Instructions
################################################################################
def instruction_addr(a, b, c, registers):
    registers[c] = registers[a] + registers[b]

def instruction_addi(a, b, c, registers):
    registers[c] = registers[a] + b

def instruction_mulr(a, b, c, registers):
    registers[c] = registers[a] * registers[b]

def instruction_muli(a, b, c, registers):
    registers[c] = registers[a] * b

def instruction_banr(a, b, c, registers):
    registers[c] = registers[a] & registers[b]

def instruction_bani(a, b, c, registers):
    registers[c] = registers[a] & b

def instruction_borr(a, b, c, registers):
    registers[c] = registers[a] | registers[b]

def instruction_bori(a, b, c, registers):
    registers[c] = registers[a] | b

def instruction_setr(a, _b, c, registers):
    registers[c] = registers[a]

def instruction_seti(a, _b, c, registers):
    registers[c] = a

def instruction_gtir(a, b, c, registers):
    registers[c] = 1 if a > registers[b] else 0

def instruction_gtri(a, b, c, registers):
    registers[c] = 1 if registers[a] > b else 0

def instruction_gtrr(a, b, c, registers):
    registers[c] = 1 if registers[a] > registers[b] else 0

def instruction_eqir(a, b, c, registers):
    registers[c] = 1 if a == registers[b] else 0

def instruction_eqri(a, b, c, registers):
    registers[c] = 1 if registers[a] == b else 0

def instruction_eqrr(a, b, c, registers):
    registers[c] = 1 if registers[a] == registers[b] else 0

FUNCTIONS = [instruction_addr,
             instruction_addi,
             instruction_mulr,
             instruction_muli,
             instruction_banr,
             instruction_bani,
             instruction_borr,
             instruction_bori,
             instruction_setr,
             instruction_seti,
             instruction_gtir,
             instruction_gtri,
             instruction_gtrr,
             instruction_eqir,
             instruction_eqri,
             instruction_eqrr]

################################################################################
# Input
################################################################################

BEFORE_PATTERN = re.compile(r'Before: *\[(.*)\]')
AFTER_PATTERN = re.compile(r'After: *\[(.*)\]')
def get_input():
    examples = list(get_all_examples())
    # get instructions...
    return examples

def get_all_examples():
    while True:
        before_line = input().strip()
        instruction_line = input().strip()
        # after examlpes, there are two blank lines
        if not instruction_line:
            return
        after_line = input().strip()
        input() # consume blank line
        before = get_ints(BEFORE_PATTERN.match(before_line).groups()[0], ', ')
        instruction = tuple(get_ints(instruction_line, ' '))
        after = get_ints(AFTER_PATTERN.match(after_line).groups()[0], ', ')
        yield before, instruction, after

def get_ints(string, sep=' '):
    return to_ints(string.split(sep))

def to_ints(strings):
    return [int(s) for s in strings]

def main():
    examples = get_input()
    print(solve(examples))

if __name__ == '__main__':
    main()
