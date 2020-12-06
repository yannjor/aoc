from functools import reduce


def sumCount1(allAnswers):
    noDuplicates = ["".join(set(line)) for line in allAnswers]
    return reduce(lambda acc, a: acc + len(a), noDuplicates, 0)


def sumCount2(groups):
    def helper(count, group):
        return count + len(set(group[0]).intersection(*group))
    return reduce(helper, groups, 0)


def main():
    with open("06.txt") as f:
        content = f.read()
        allAnswers = [line.replace("\n", "") for line in content.split("\n\n")]
        groups = [line.strip().split("\n") for line in content.split("\n\n")]
        print("Count 1:", sumCount1(allAnswers))
        print("Count 2:", sumCount2(groups))


if __name__ == "__main__":
    main()
