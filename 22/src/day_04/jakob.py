def containsEachotherCount(r1, r2):
    f1 = r1[0]
    t1 = r1[1]
    f2 = r2[0]
    t2 = r2[1]
    if (f1 == f2 and t1 == t2 ): return 1 ## should I count identical ranges as overlapping?
    if (f2 <= f1 and t1 <= t2 ): return 1 # 2 contains 1
    if (f1 <= f2 and t2 <= t1 ): return 1 # 1 contains 2
    else: return 0 # none contains the other

inp = open('input/22_04_test', 'r').read().split('\n')
c = 0
for pair in inp:
    comma_split = pair.split(",")
    range1 = comma_split[0].split("-")
    range2 = comma_split[1].split("-")
    c += containsEachotherCount(range1, range2)

print("pairs fully containing eachother: ", c)
