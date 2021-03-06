#!/usr/bin/env python3
#pylint: disable=too-few-public-methods

import sys
import re
from collections import defaultdict
from functools import total_ordering

class Entry:
    GUARD_PATTERN = re.compile(r'Guard #(\d+) begins shift')
    ASLEEP_PATTERN = re.compile(r'falls asleep')
    WAKES_PATTERN = re.compile(r'wakes up')

    def __init__(self, entry):
        self.entry = entry
        self.asleep = bool(Entry.ASLEEP_PATTERN.match(self.entry))
        self.wakes = bool(Entry.WAKES_PATTERN.match(self.entry))
        self.guard = bool(Entry.GUARD_PATTERN.match(self.entry))
        self.guard_num = self.parse_guard_num() if self.guard else None

    def parse_guard_num(self):
        return Entry.GUARD_PATTERN.match(self.entry).groups()[0]

    def __str__(self):
        return self.entry

@total_ordering
class Record:
    def __init__(self, time, entry):
        self.time = time
        self.entry = Entry(entry)

    PATTERN = re.compile(r'\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)')
    @staticmethod
    def from_string(string):
        values = Record.PATTERN.match(string).groups()
        time, entry = values[:-1], values[-1]
        time = tuple(int(x) for x in time)
        return Record(time, entry)

    def __str__(self):
        return '[{:04d}-{:02d}-{:02d} {:02d}:{:02d}] {}'.format(*self.time,
                                                                self.entry)
    def __lt__(self, other):
        return self.time < other.time

    def __eq__(self, other):
        return self.time == other.time


def guard_sleep_times(records):
    records.sort()
    guard_num = records[0].entry.guard_num
    sleep_times = []
    for record in records:
        time, entry = record.time, record.entry
        if entry.guard:
            guard_num = entry.guard_num
        if entry.asleep:
            asleep = time[-1] # minute
        if entry.wakes:
            wakes = time[-1] # minute
            sleep_time = (guard_num, asleep, wakes)
            sleep_times.append(sleep_time)
    return sleep_times

def guard_sleep_amount(sleep_times):
    sleep_amount = defaultdict(int)
    # print(sleep_times)
    for guard, asleep, awake in sleep_times:
        sleep_amount[guard] += awake - asleep
    return sleep_amount

def sleepiest_guard(sleep_times):
    sleep_amounts = guard_sleep_amount(sleep_times)
    # print(sleep_amounts)
    sleepiest = max(sleep_amounts.items(), key=lambda tup: tup[1])
    # print(sleepiest)
    guard, _ = sleepiest
    return guard

def sleepiest_minute(sleep_times, guard):
    sleep_times = [(start, end) for guard_id, start, end in sleep_times
                   if guard_id == guard]
    minutes = defaultdict(int)
    for start, end in sleep_times:
        for minute in range(start, end):
            minutes[minute] += 1
    minute, _ = max(minutes.items(), key=lambda tup: tup[1])
    return minute

def solve(records):
    sleep_times = guard_sleep_times(records)
    guard = sleepiest_guard(sleep_times)
    minute = sleepiest_minute(sleep_times, guard)
    return int(guard) * minute

def get_input():
    return [Record.from_string(line.strip()) for line in sys.stdin.readlines()]

def main():
    records = get_input()
    print(solve(records))

if __name__ == '__main__':
    main()
