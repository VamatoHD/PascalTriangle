def PascalTriangle(line: int):
    if line == 1:
        return [1]
    cur = [1, 1]

    for _ in range(2, line):
        for i in range(len(cur) - 1, 0, -1):
            cur[i] += cur[i - 1]
        cur += [1]
    return cur


def main():
    while True:
        n = int(input("Line: "))
        line = PascalTriangle(n)
        print(line)


if __name__ == "__main__":
    main()
