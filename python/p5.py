import copy
import re
import sys
from pathlib import Path

INSTRUCTION_REGEX = re.compile(r"move (\d+) from (\d+) to (\d+)")


class Instruction:
    def __init__(self, source_stack_index, destination_stack_index, number_of_crates):
        self.source_stack_index = source_stack_index - 1
        self.destination_stack_index = destination_stack_index - 1
        self.number_of_crates = number_of_crates

    def __repr__(self):
        return f"Moving {self.number_of_crates} from {self.source_stack_index} to {self.destination_stack_index}"


input_filename = Path(sys.argv[1])
instructions: list(Instruction) = []
stacks: list[list] = []
# Parse problem input
with input_filename.open("r", encoding="utf-8") as input_file:
    initial_stack, rearrangement_procedure = input_file.read().split("\n\n")
    initial_stack = initial_stack.split("\n")
    initial_stack.reverse()
    number_of_stacks, *crates_stacks = initial_stack
    number_of_stacks = list(filter(None, number_of_stacks.split(" ")))
    [stacks.append([]) for _ in range(len(number_of_stacks))]
    for crates_stack in crates_stacks:
        string_size = len(crates_stack)
        for i in range(len(stacks)):
            crate_index = 1 + i * 4
            if (
                string_size > crate_index
                and (crate := crates_stack[crate_index]) != " "
            ):
                stacks[i].append(crate)
    for instruction in rearrangement_procedure.split("\n"):
        match = INSTRUCTION_REGEX.match(instruction)
        if match:
            instructions.append(
                Instruction(
                    source_stack_index=int(match.group(2)),
                    destination_stack_index=int(match.group(3)),
                    number_of_crates=int(match.group(1)),
                )
            )


def apply_instruction(instruction, stacks, reverse):
    source_stack = stacks[instruction.source_stack_index]
    stacks[instruction.source_stack_index], moved_crates = (
        source_stack[: -instruction.number_of_crates],
        source_stack[-instruction.number_of_crates :],
    )
    if reverse:
        moved_crates.reverse()
    stacks[instruction.destination_stack_index].extend(moved_crates)


p1_stacks = copy.deepcopy(stacks)
for instruction in instructions:
    apply_instruction(instruction, p1_stacks, reverse=True)

print("P1: " + "".join([stack[-1] for stack in p1_stacks]))

p2_stacks = copy.deepcopy(stacks)
for instruction in instructions:
    apply_instruction(instruction, p2_stacks, reverse=False)

print("P2: " + "".join([stack[-1] for stack in p2_stacks]))
