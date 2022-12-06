import sys
from pathlib import Path
import numpy as np


input_filename = Path(sys.argv[1])
with input_filename.open("r", encoding="utf-8") as input_file:
    input = [
        sum([int(calories) for calories in elf_food.split("\n")])
        for elf_food in input_file.read().split("\n\n")
    ]
input.sort()
print(f"P1: {input[-1]}")
print(f"P2: {sum(input[-3:])}")
