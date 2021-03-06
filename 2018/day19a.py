#!/usr/bin/env python3
#pylint: disable=invalid-name

import sys
import re
from collections import defaultdict

################################################################################
# Run
################################################################################
def solve(ip_reg, instructions):
    registers = run(ip_reg, instructions)
    return registers[0]

def run(ip_reg, instructions):
    registers = defaultdict(int)
    ip = 0
    # while 0 <= ip < len(instructions):
    #     # import copy
    #     # old_registers = copy.deepcopy(registers)
    #     registers[ip_reg] = ip
    #     instruction = instructions[ip]
    #     run_instruction(registers, instruction)
    #     ip = registers[ip_reg]
    #     # print(cycle_string(old_registers, registers, ip, instruction))
    #     ip += 1
    debug_index = 0
    # while 0 <= ip < len(instructions):
    while 0 <= ip < len(instructions):
        import copy
        old_registers = copy.deepcopy(registers)
        old_ip = ip
        registers[ip_reg] = ip
        instruction = instructions[ip]
        run_instruction(registers, instruction)
        ip = registers[ip_reg]
        print('{: 3d}: '.format(debug_index), end='')
        print(cycle_string(old_registers, registers, old_ip, instruction))
        ip += 1
        debug_index += 1
        if debug_index >= 40:
            exit()
    return registers

def run_instruction(registers, instruction):
    opname, a, b, c = instruction
    fun = OPNAME_TO_FUN[opname]
    fun(a, b, c, registers)

def cycle_string(old_registers, registers, ip, instruction):
    old_regs_str = registers_string(old_registers)
    new_regs_str = registers_string(registers)
    instruction_str = ' '.join(str(instr) for instr in  instruction)
    return 'ip={} {} {} {}'.format(ip, old_regs_str, instruction_str,
                                   new_regs_str)

NUM_REGS = 6
def registers_string(registers):
    return '[{}]'.format(', '.join(str(registers[i])
                                   for i in range(NUM_REGS)))

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

OPNAME_TO_FUN = {
    'addr': instruction_addr,
    'addi': instruction_addi,
    'mulr': instruction_mulr,
    'muli': instruction_muli,
    'banr': instruction_banr,
    'bani': instruction_bani,
    'borr': instruction_borr,
    'bori': instruction_bori,
    'setr': instruction_setr,
    'seti': instruction_seti,
    'gtir': instruction_gtir,
    'gtri': instruction_gtri,
    'gtrr': instruction_gtrr,
    'eqir': instruction_eqir,
    'eqri': instruction_eqri,
    'eqrr': instruction_eqrr,
}

################################################################################
# Input
################################################################################

def get_input():
    ip_reg = get_ip_reg()
    instructions = get_all_instructions()
    return ip_reg, instructions

PATTERN = re.compile(r'#ip (\d+)')
def get_ip_reg():
    first_line = input()
    match = PATTERN.match(first_line)
    groups = match.groups()
    reg = groups[0]
    return int(reg)

def get_all_instructions():
    return [parse_instruction(line.strip()) for line in sys.stdin.readlines()]

def parse_instruction(line):
    words = line.split(' ')
    opname, rest = words[0], words[1:]
    lst = [opname] + [int(num) for num in rest]
    return tuple(lst)

def get_ints(string, sep=' '):
    return to_ints(string.split(sep))

def to_ints(strings):
    return [int(s) for s in strings]

def main():
    ip_reg, instructions = get_input()
    print(solve(ip_reg, instructions))

if __name__ == '__main__':
    main()
