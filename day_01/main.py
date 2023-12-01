#!/usr/bin/env python3

def part1():
    file = open('input')

    sum = 0

    for line in file.readlines():
        first = None
        last = None

        for c in line: 
            if c.isdigit():
                num = int(c)
                if not first:
                    first = num
                last = num

        if first is not None and last is not None:
            sum += 10 * first + last

    print(f"Part 1: {sum}")

import re

lookup = {
    1: re.compile("1|one"),
    2: re.compile("2|two"),
    3: re.compile("3|three"),
    4: re.compile("4|four"),
    5: re.compile("5|five"),
    6: re.compile("6|six"),
    7: re.compile("7|seven"),
    8: re.compile("8|eight"),
    9: re.compile("9|nine"),
}

def part2():
    file = open('input')

    sum = 0

    for line in file.readlines():
        first_index = len(line)
        first_val = 0

        last_index = -1
        last_val = 0

        for num, regex in lookup.items():
            indices = [m.start() for m in regex.finditer(line)]

            if len(indices) > 0:
                min_index = min(indices)
                max_index = max(indices)

                if min_index < first_index:
                    first_val = num
                    first_index = min_index
                if max_index > last_index:
                    last_val = num
                    last_index = max_index

        val = 10 * first_val + last_val
        sum += val

    print(f"Part 2: {sum}")

part1()
part2()


