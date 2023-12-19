import re

alphadigs = "zero,one,two,three,four,five,six,seven,eight,nine".split(",")
numdigs = [str(i) for i in range(10)]
token_patt = re.compile("(" + "|".join(alphadigs + numdigs) + ")")
print(token_patt)


def resolve(digit: str) -> str:
    if digit.isnumeric():
        return digit
    elif digit in alphadigs:
        return str(alphadigs.index(digit))
    else:
        raise Exception(f"${digit} not valid")


def get_first_token(line):
    match = token_patt.search(line)
    return resolve(match.group(0))


def get_last_token(line):
    match = None
    i = 0
    while match is None:
        i -= 1
        match = token_patt.search(line[i:])
    return resolve(match.group(0))


total = 0
with open("input") as infile:
    for line in infile:
        digits = re.findall(token_patt, line)
        first = get_first_token(line)
        last = get_last_token(line)
        total += int(first + last)


print(total)
