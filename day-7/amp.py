import subprocess
from itertools import permutations


def amp(mem, phase, inp):
    proc = subprocess.Popen(
        ["cargo", "run"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    proc.stdin.write("{}\n{}\n{}".format(mem, phase, inp))
    return int(proc.communicate()[0].decode())


def main():
    print("Memory: ")
    mem = ','.join([str(i) for i in input()])

    _max = 0
    for phases in permutations(range(0, 5)):
        output = 0
        for p in phases:
            output = amp(mem, p, output)

        if output > _max:
            _max = output
            print(_max)
            print(str(phases))

    print("\nMax found: {}".format(_max))


if __name__ == "__main__":
    main()
