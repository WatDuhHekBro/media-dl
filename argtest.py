import sys, re
from argparse import ArgumentParser

parser = ArgumentParser(description='Process some integers.')
parser.add_argument('integers', metavar='N', type=str, nargs='+', help='an integer for the accumulator')

args = sys.argv[1:]

if len(args) == 0:
	args = re.sub(r" +", r" ", input('Flags ("-h" for help): ')).strip().split(" ")

print(parser.parse_args(args))