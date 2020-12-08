def part1(instructions):
    acc = 0
    seen = []
    i = 0
    infLoop = False
    while i < len(instructions):
        # If instruction already ran
        if i in seen:
            infLoop = True
            break
        seen.append(i)
        ins = instructions[i][:3]
        num = int(instructions[i][4:])
        if ins == "nop":
            i += 1
        if ins == "acc":
            acc += num
            i += 1
        if ins == "jmp":
            i += num
    return acc, infLoop


def part2(instructions):
    for i in range(len(instructions)):
        if instructions[i][:3] == "acc":
            continue
        # Try changing jmp to nop or vice versa
        newOp = "nop" if instructions[i][:3] == "jmp" else "jmp"
        newInstruction = newOp + instructions[i][3:]
        newProgram = instructions[:i] + [newInstruction] + instructions[i+1:]
        acc, infLoop = part1(newProgram)
        if not infLoop:
            return acc
    return 0


def main():
    with open("08.txt", "r") as f:
        instructions = [ins.strip() for ins in f.readlines()]
        print("Part 1:", part1(instructions)[0])
        print("Part 2:", part2(instructions))


if __name__ == "__main__":
    main()
