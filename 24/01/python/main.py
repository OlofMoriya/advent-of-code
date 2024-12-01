left = []
right = []
with open('../../01/01') as file:
    content = file.read()
    print(content)
    lines = content.splitlines()
    for line in lines:
        parts = line.split()
        left.append(int(parts[0]))
        right.append(int(parts[1]))

result = [right.count(l) for l in left]
print(result)
sum = 0
for l,c in zip(left, result):
    print(l, c)
    sum = sum + (l * c)
print(sum)


