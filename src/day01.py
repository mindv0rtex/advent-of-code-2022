import fileinput

counts = sorted(
    [sum(map(int, x.split())) for x in "".join(fileinput.input()).split("\n\n")],
    reverse=True,
)

print(f"Part 1: {counts[0]}, Part 2: {sum(counts[:3])}")
