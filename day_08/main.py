import itertools
import math
import re

regex = re.compile(r'(\w{3}) = \((\w{3}), (\w{3})\)')


def part1(file):
    directions = None

    map = dict()

    with open(file) as f:
        for line in f:
            if not len(line.strip()):
                continue
            if directions == None:
                directions = line.strip()
                continue

            m = regex.match(line)

            map[m.group(1)] = (m.group(2), m.group(3))


    current = 'AAA'

    for step, decision in enumerate(itertools.cycle(directions)):
        if decision == 'L':
            current = map[current][0]
        else: 
            current = map[current][1]

        if current == 'ZZZ':
            return step + 1

def part2(file):
    directions = None

    map = dict()
    states = list()

    with open(file) as f:
        for line in f:
            if not len(line.strip()):
                continue
            if directions == None:
                directions = line.strip()
                continue

            m = regex.match(line)
            node = m.group(1)

            map[node] = (m.group(2), m.group(3))
            if node.endswith('A'):
                states.append(node)

    steps = dict()
    for i in range(len(states)):
        current = states[i]

        for step, decision in enumerate(itertools.cycle(directions)):
            if decision == 'L':
                current = map[current][0]
            else: 
                current = map[current][1]

            if current.endswith('Z'):
                steps[i] = step + 1
                break

    return math.lcm(*steps.values())


print('Part 1: ', part1('input'))
print('Part 2: ', part2('input'))