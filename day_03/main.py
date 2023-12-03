
def is_symbol(c: str) -> bool:
    return (not c.isdigit()) and c != "."

def count_symbols(line, i):
    symbols = 0
    if i > 0:
        symbols += is_symbol(line[i-1])
    if i < len(line) - 1:
        symbols += is_symbol(line[i+1])
    symbols += is_symbol(line[i])

    return symbols

def part1(last, current, next) -> int:
    current_sum = 0
    current_num = 0
    symbols = 0

    for (i, c) in enumerate(current):
        if c.isdigit():
            current_num *= 10
            current_num += int(c)

            symbols += count_symbols(current, i)
            if last:
                symbols += count_symbols(last, i)
            if next:
                symbols += count_symbols(next, i)

        else:
            if symbols:
                current_sum += current_num
            current_num = 0
            symbols = 0
    if symbols:
        current_sum += current_num

    return current_sum


def line_by_line(file, process_fn) -> int:
    f = open(file)

    total_sum = 0
    
    last_line = None
    penultimate_line = None

    for line in f.readlines():
        line = line.strip()
        if last_line:
            total_sum += process_fn(penultimate_line, last_line, line)

        penultimate_line = last_line
        last_line = line

    total_sum += process_fn(penultimate_line, last_line, None)

    return total_sum


def parse_number(line, start):
    left = start
    right = start
    
    if not line[start].isdigit():
        return (None, None)

    while left > 0:
        if line[left-1].isdigit():
            left -= 1
        else:
            break

    while right < len(line) - 1:
        if line[right+1].isdigit():
            right += 1
        else:
            break

    substr = line[left:left+right-left+1]
    return int(substr), (left, right)



def part2(last, current, next):
    current_sum = 0

    for (i, c) in enumerate(current):
        nums = 0
        prod = 1

        if c == "*":
            (nums, prod) = check_line(i, current, nums, prod)

            if last:
                (nums, prod) = check_line(i, last, nums, prod)
            if next:
                (nums, prod) = check_line(i, next, nums, prod)

        if nums == 2:
            current_sum += prod

    return current_sum

def check_line(i, line, nums, prod):
    spans = set()
    if i > 0:
        (nums, prod) = find_and_multiply_number(i-1, line, nums, prod, spans)
    if i < len(line) - 1:
        (nums, prod) = find_and_multiply_number(i+1, line, nums, prod, spans)

    (nums, prod) = find_and_multiply_number(i, line, nums, prod, spans)

    return nums, prod

def find_and_multiply_number(i, line, nums, prod, spans):
    (num, span) = parse_number(line, i)
    if num is not None and span not in spans:
        nums += 1
        prod *= num
        spans.add(span)
    return (nums, prod)


print(line_by_line('day_03/input', process_fn=part1))
print(line_by_line('day_03/input', process_fn=part2))