import re
from time import time


def getInvalidValues(ticketData):
    sections = ticketData.split("\n\n")
    validValues = getValidValues(sections[0])
    nearbyTicketValues = [int(t) for t in re.findall(r"\d+", sections[2])]
    invalidValues = [ticket for ticket in nearbyTicketValues if
                     ticket not in validValues]
    return invalidValues


def getValidValues(ticketData):
    ruleRanges = re.findall(r"\d+-\d+", ticketData)
    validValues = []
    for ranges in ruleRanges:
        start, end = ranges.split("-")
        validValues.extend(range(int(start), int(end)+1))
    return validValues


def part1(invalidValues):
    return sum(invalidValues)


def filterValidTickets(tickets, invalidValues):
    validTickets = []
    for ticket in tickets:
        values = [int(val) for val in ticket.split(",")]
        intersection = [val for val in values if val in invalidValues]
        if len(intersection) == 0:
            validTickets.append(ticket)
    return validTickets


def getFieldIndexes(rules, ticketCols):
    def findLoner(fieldIdxs):
        toDelete = ""
        for field, candidates in fieldIdxs.items():
            if len(candidates) == 1:
                loner = candidates[0]
                toDelete = field
                removeIdx(fieldIdxs, loner)
                fieldIndexes[field] = loner
                break
        del fieldIdxs[toDelete]

    def removeIdx(fieldIdxs, idx):
        for _, candidates in fieldIdxs.items():
            if idx in candidates:
                candidates.remove(idx)

    fieldIndexes = {}
    fieldIndexCandidates = {}
    for field, validNums in rules.items():
        for i, col in ticketCols.items():
            intersection = [n for n in col if n in validNums]
            if intersection == col:
                if field in fieldIndexCandidates:
                    fieldIndexCandidates[field].append(i)
                else:
                    fieldIndexCandidates[field] = [i]

    while fieldIndexCandidates != {}:
        findLoner(fieldIndexCandidates)

    return fieldIndexes


def part2(ticketData, invalidValues):
    sections = ticketData.split("\n\n")
    nearbyTickets = sections[2].split("\n")[1:]
    cols = {}
    for ticket in filterValidTickets(nearbyTickets, invalidValues):
        fields = ticket.split(",")
        for i, field in enumerate(fields):
            if i in cols:
                cols[i].append(int(field))
            else:
                cols[i] = [int(field)]
    rules = {}
    for rule in sections[0].split("\n"):
        fieldName = rule.split(":")[0]
        validValues = getValidValues(rule)
        rules[fieldName] = validValues

    fieldIndexes = getFieldIndexes(rules, cols)
    myTicket = [int(val) for val in re.findall(r"\d+", sections[1])]
    product = 1
    for field, idx in fieldIndexes.items():
        if "departure" in field:
            product *= myTicket[idx]
    return product


def main():
    with open("16.txt", "r") as f:
        content = f.read()
        start = time()
        invalidValues = getInvalidValues(content)
        print("Part 1:", part1(invalidValues))
        print("Part 2:", part2(content, invalidValues))
        end = time()
        print(f"Took {end - start} seconds")


if __name__ == "__main__":
    main()
