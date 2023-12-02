

import re
from collections import defaultdict

color_regex = re.compile(r"(\d+) (\w+)")
game_regex = re.compile(r"Game (\d+):")

def part1():
    f = open('day_02/input')

    possible_games = 0

    for line in f.readlines():
        game = int(game_regex.match(line).group(1))

        possible = True

        for line in line.split(';'):
            d = defaultdict(lambda: 0)
            for match in color_regex.finditer(line):
                count = int(match.group(1))
                color = match.group(2)

                d[color] += count


            if d['red'] > 12 or d['green'] > 13 or d['blue'] > 14:
                possible = False
                break

        if possible:
            possible_games += game

    print(possible_games)


from functools import reduce
import operator

def part2():
    f = open('day_02/input')

    power_sum = 0

    for line in f.readlines():
        game = int(game_regex.match(line).group(1))

        d = defaultdict(lambda: 0)

        for line in line.split(';'):
            for match in color_regex.finditer(line):
                count = int(match.group(1))
                color = match.group(2)
                d[color] = max(d[color], count)

        power_sum += reduce(operator.mul, d.values(), 1)

    print(power_sum)




part1()
part2()


