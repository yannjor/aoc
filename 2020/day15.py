from time import time


def getNthNumber(nums, n):
    lastOccurences = dict([(num, i) for i, num in enumerate(nums[:-1])])
    lastNum = nums[-1]
    i = len(nums)
    while i != n:
        if lastNum in lastOccurences:
            nextNum = i - lastOccurences[lastNum] - 1
        else:
            nextNum = 0
        lastOccurences[lastNum] = i - 1
        lastNum = nextNum
        i += 1
    return lastNum


def main():
    myInput = [19, 0, 5, 1, 10, 13]
    start = time()
    print("Part 1:", getNthNumber(myInput, 2020))
    print("Part 2:", getNthNumber(myInput, 30000000))
    end = time()
    print(f"Took {round(end - start)} seconds")
    # Took 9s on my machine


if __name__ == "__main__":
    main()
