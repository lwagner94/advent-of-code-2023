
import itertools
import math

file = 'day_06/input'


def part1():
    f = open(file)
    times = []
    distances = []

    line = f.readline()
    for num in itertools.islice(line.split(), 1, None):
        times.append(int(num))

    line = f.readline()
    for num in itertools.islice(line.split(), 1, None):
        distances.append(int(num))


    assert len(times) == len(distances)

    product = 1

    for time, distance in zip(times, distances):
        a = time / 2
        b = math.sqrt((time ** 2 / 4) - distance)

        t1 = (a - b)
        t2 = (a + b)

        t1 = math.floor(t1 + 1)
        t2 = math.ceil(t2 - 1)
        nums = t2 - t1 + 1
        product *= nums


    print(product)


def part2():
    f = open(file)
    times = []
    distances = []

    line = f.readline()
    for num in itertools.islice(line.split(), 1, None):
        times.append(num)

    line = f.readline()
    for num in itertools.islice(line.split(), 1, None):
        distances.append(num)

    distance = int("".join(distances))
    time = int("".join(times))

    a = time / 2
    b = math.sqrt((time ** 2 / 4) - distance)

    t1 = (a - b)
    t2 = (a + b)

    t1 = math.floor(t1 + 1)
    t2 = math.ceil(t2 - 1)
    nums = t2 - t1 + 1

    print(nums)


part1()
part2()