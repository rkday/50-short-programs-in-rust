import time

class GoodHashable:
    def __init__(self, x):
        self.x = x

    def __hash__(self):
        return self.x

    def __eq__(self, other):
        return other.x == self.x

class BadHashable:
    def __init__(self, x):
        self.x = x

    def __hash__(self):
        return 8

    def __eq__(self, other):
        return other.x == self.x

def time_it(cls, entries):

    start = time.time()

    myset = set()

    for x in range(entries):
        myset.add(cls(x))

    return time.time() - start

good = time_it(GoodHashable, 1000)
bad = time_it(BadHashable, 1000)
bad2 = time_it(BadHashable, 4000)

good2 = time_it(GoodHashable, 4000)
print("Normal case took {} ms, worst case {} ms, {}x difference".format(good * 1000, bad * 1000, bad/good))
print("Bad case took {}x longer with 4x more input".format(bad2/bad))
print("Normal case took {}x longer with 4x more input".format(good2/good))
