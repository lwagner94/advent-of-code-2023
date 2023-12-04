

def line_by_line(path, process_fn):
    f = open(path)

    sum = 0
    for i, line in enumerate(f):
        sum += process_fn(i, line.strip())

    return sum

def part01(i, line: str) -> int:
    winning_start = line.find(":") + 1
    winning_end = line.find("|")

    nums_start = winning_end + 2

    winning_str = line[winning_start:winning_end]
    number_str = line[nums_start:]

    winning = set()

    for num in winning_str.split():
        winning.add(num)

    points = 0
    for num in number_str.split():
        if num in winning:
            if points == 0:
                points = 1
            else:
                points *= 2

    return points

from collections import defaultdict

def part02(path):
    f = open(path)

    copies = defaultdict(lambda: 1)
    sum = 0

    for i, line in enumerate(f):
        winning_start = line.find(":") + 1
        winning_end = line.find("|")

        nums_start = winning_end + 2

        winning_str = line[winning_start:winning_end]
        number_str = line[nums_start:]

        winning = set()

        for num in winning_str.split():
            winning.add(num)

        points = 0
        for num in number_str.split():
            if num in winning:
                points += 1

        for next in range(i + 1, i + 1 + points):
            copies[next] += copies[i]
        
        sum += copies[i]
    return sum

print(line_by_line('day_04/input', part01))
print(part02('day_04/input'))