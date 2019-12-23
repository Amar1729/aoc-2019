from itertools import permutations

import pexpect


class Amp():

    def __init__(self, phase):
        self.phase = phase

        self.proc = pexpect.spawn("./target/debug/day-7")

        # self.proc.logfile = sys.stderr.buffer

        self.proc.sendline(str(phase))

    def write(self, inp):
        self.proc.sendline(str(inp))
        self.proc.expect('>')
        return self.proc.buffer.decode().strip()


def create_and_run_amps(phases):
    amps = [Amp(p) for p in phases]

    curr = 0
    output = 0
    while True:
        try:
            output = amps[curr].write(output)
        except OSError:
            # catch the amplifiers halting:
            break
        curr = (curr+1) % 5

    return int(output)


def main():
    _max = 0
    for phases in permutations(range(5, 10)):
        output = create_and_run_amps(phases)

        if output > _max:
            _max = output
            print(_max)
            print(str(phases))

    print("\nMax found: {}".format(_max))


if __name__ == "__main__":
    # memory should be written in file: mem.txt
    main()
