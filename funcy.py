import sys
import time
from enum import Enum

file = ""


class InstructionType(Enum):
    Pop = -1
    Store = 1
    Push = 2
    Print = 3
    Equal = 4
    Greater = 5
    Jump = 6
    Add = 7
    Min = 8
    Multiply = 9
    Divide = 10


class Instruction:
    iType: InstructionType
    argument: str  # Optional
    file: str
    line: int
    column: int

    def __init__(self, iType, argument, filename, line, ):
        self.iType = iType
        self.argument = argument
        self.file = filename
        self.line = line

    def __str__(self):
        if self.argument != "":
            return self.file + "({})".format(self.line) + " -> {} {}".format(self.iType, self.argument)
        return self.file + "({})".format(self.line) + " -> {}".format(self.iType)


def GetStatements(str):
    return str.split("\n")


def ParseInstructions(statements):
    instructions = []
    line = 1
    for stat in statements:
        if len(stat) > 0 and stat[0] != ';':
            x = stat.split()
            match x[0]:
                case "Pop":
                    instructions.append(Instruction(InstructionType.Pop, "", file, line))
                case "Store":
                    if not x[1].isnumeric():
                        print("Syntax Error {}: Store needs to be and index\n   {}".format(line, stat))
                        exit(1)
                    instructions.append(Instruction(InstructionType.Store, x[1], file, line))
                case "Push":
                    if not x[1][0] == "i" and not x[1].isnumeric():
                        print(
                            "Syntax Error {}: Push needs to be either an integer or an index\n   {}".format(line, stat))
                        exit(1)
                    instructions.append(Instruction(InstructionType.Push, x[1], file, line))
                case "Print":
                    instructions.append(Instruction(InstructionType.Print, "", file, line))
                case "Equal":
                    instructions.append(Instruction(InstructionType.Equal, "", file, line))
                case "Greater":
                    instructions.append(Instruction(InstructionType.Greater, "", file, line))
                case "Jump":
                    if not x[1].isnumeric():
                        print("Syntax Error {}: Jump needs to be an instruction index\n   {}".format(line, stat))
                        exit(1)
                    instructions.append(Instruction(InstructionType.Jump, x[1], file, line))
                case "Add":
                    instructions.append(Instruction(InstructionType.Add, "", file, line))
                case "Min":
                    instructions.append(Instruction(InstructionType.Min, "", file, line))
                case "Multiply":
                    instructions.append(Instruction(InstructionType.Multiply, "", file, line))
                case "Divide":
                    instructions.append(Instruction(InstructionType.Divide, "", file, line))
                case _:
                    print("Syntax Error {}: '{}' is not an instruction\n   {}".format(line, x[0], stat))
                    exit(1)
        line += 1
    return instructions, line


STACK_SIZE = 8
MEMORY_SIZE = 1024


class Interpreter:
    stack: [int]  # 8 elements
    memory: [int]  # 1024 elements
    instructions: [Instruction]  # Program instructions
    cursor: int  # Current instruction
    lines: int

    def __init__(self, instructions, lines):
        self.instructions = instructions
        self.lines = lines
        self.cursor = 0

        self.memory = []
        self.stack = []

    def Evaluate(self):
        while self.cursor < len(self.instructions):
            instruction = self.instructions[self.cursor]
            match instruction.iType:
                case InstructionType.Pop:
                    self.Pop(instruction)
                case InstructionType.Store:
                    self.Store(instruction)
                case InstructionType.Push:
                    self.Push(instruction)
                case InstructionType.Print:
                    if len(self.stack) == 0:
                        print("RunTime Error (line: {}):\n\t Stack had size of 0, but program popped".format(
                            instruction.line))
                        exit(1)
                    print(self.stack[-1])
                case InstructionType.Equal:
                    if len(self.stack) < 2:
                        print(
                            "RunTime Error (line: {}):\n\t Equal instruction needs two values, but the stack has: {}".format(
                                instruction.line, len(self.stack)))
                        exit(1)
                    left = self.stack[-1]
                    right = self.stack[-2]
                    if left != right:
                        self.cursor += 1
                case InstructionType.Greater:
                    if len(self.stack) < 2:
                        print("RunTime Error (line: {}):\n\t Greater instruction needs two values, but the stack has: "
                              "{}".format(instruction.line, len(self.stack)))
                        exit(1)
                    left = self.stack[-1]  # Top is stack is left
                    right = self.stack[-2]  # Second of stack is right
                    if left > right:
                        self.cursor += 1
                case InstructionType.Jump:
                    if len(self.instructions) < int(instruction.argument):
                        print("RunTime Error (line: {}):\n\t Jump instruction jumps to line that does not exists, "
                              "there are: {} lines".format(instruction.line, self.lines))
                        exit(1)

                    # There is a -1, because every loop is + 1. AKA compensation
                    self.cursor = int(instruction.argument) - 2
                case InstructionType.Add:
                    if len(self.stack) < 2:
                        print(
                            "RunTime Error (line: {}):\n\t Add instruction needs to values, but the stack has: {}".format(
                                instruction.line, len(self.stack)))
                        exit(1)
                    left = self.stack[-1]
                    right = self.stack[-2]
                    value = "i" + str(left + right)
                    instruction.argument = value
                    self.Push(instruction)
                case InstructionType.Min:
                    if len(self.stack) < 2:
                        print(
                            "RunTime Error (line: {}):\n\t Min instruction needs to values, but the stack has: {}".format(
                                instruction.line, len(self.stack)))
                        exit(1)
                    left = self.stack[-1]
                    right = self.stack[-2]
                    value = "i" + str(left - right)
                    instruction.argument = value
                    self.Push(instruction)
                case InstructionType.Multiply:
                    if len(self.stack) < 2:
                        print(
                            "RunTime Error (line: {}):\n\t Multiply instruction needs to values, but the stack has: {}".format(
                                instruction.line, len(self.stack)))
                        exit(1)
                    left = self.stack[-1]
                    right = self.stack[-2]
                    value = "i" + str(left * right)
                    instruction.argument = value
                    self.Push(instruction)
                case InstructionType.Divide:
                    if len(self.stack) < 2:
                        print(
                            "RunTime Error (line: {}):\n\t Divide instruction needs to values, but the stack has: {}".format(
                                instruction.line, len(self.stack)))
                        exit(1)
                    left = self.stack[-1]
                    right = self.stack[-2]
                    value = "i" + str(round(left / right))
                    instruction.argument = value
                    self.Push(instruction)

            # Next instruction
            self.cursor += 1

    def Pop(self, instruction: Instruction):
        if len(self.stack) == 0:
            print("RunTime Error (line: {}):\n\t Stack had size of 0, but program popped".format(instruction.line))
            exit(1)

        # Pops the top of stack
        self.stack.pop(len(self.stack) - 1)

    def Push(self, instruction: Instruction):
        if len(self.stack) == STACK_SIZE:
            print("RunTime Error (line: {}):\n\t Stack was full (max 8 values), but program pushed".format(
                instruction.line))
            exit(1)

        # Pushes value of memory register or expression to top of stack
        if instruction.argument[0] == "i":
            self.stack.append(int(instruction.argument[1:]))
        else:
            self.stack.append(self.memory[int(instruction.argument)-1])

    def Store(self, instruction: Instruction):
        idx = int(instruction.argument)
        if idx > 1024 or idx < 0:
            print("RunTime Error (line: {}):\n\t Memory location needs to be between 0 and 1024, you had: '{}'"
                  .format(instruction.line, instruction.argument))
            exit(1)
        if len(self.stack) == 0:
            print("RunTime Error (line: {}): Stack had size of 0".format(instruction.line))

        # Puts the value of the top of the stack in a memory register
        x = idx-len(self.memory)
        for _ in range(x):
            self.memory.append(None)
        self.memory[idx - 1] = self.stack[-1]


debug = False

if __name__ == "__main__":
    start = time.time()
    args = sys.argv[1:]
    if len(args) == 0:
        print("Please provide a file to execute")
        exit(1)
    elif len(args) == 1:
        file = args[0]
    elif len(args) == 2:
        file = args[-1]
        if args[0] == "-d":
            debug = True
        else:
            print("Unknown flag: {}. Existing flags: -d".format(args[0]))
            exit(1)
    f = open(file, "r")
    statements = GetStatements(f.read())
    instructions, lines = ParseInstructions(statements)
    interpreter = Interpreter(instructions, lines)
    interpreter.Evaluate()
    end = time.time()
    if debug:
        print("\nDEBUG INFO:")
        print("    INSTRUCTIONS:")
        for i in instructions:
            print("        {}".format(i))
        print("    MEMORY USAGE: {}".format(len(interpreter.memory)))
        print("    EXECUTION TIME: {} sec".format(end-start))
