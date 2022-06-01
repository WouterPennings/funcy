# Funcy

An interpreted language that emulates an assembly (Kind of); Mostly just a fun language. 
User is required to use a stack, and can save values in memory. The challenge is to use 
a little memory as possible. Oh, do not forgot to pop the stack, it is not very big...

## How to run

1. Clone the repository
2. Create `main.funcy`, and write a program
3. Make sure you have Python3.10 installed
4. Type this in the terminal: `python3.10 funcy.py main.funcy`

Put `-d` before your `.funcy` file if you want debug information.

## Specification

- A stack of 4 elements is created which stores integers
- You get an infinitely growable chunk of memory, **you can never give memory back to the system**
- This is a safe language meaning in this case that instructions never pop from stack. Only possible with specific instruction

### Naming

- `id`: Index of element in memory
- `expr`: 32-bit integer, you have to put an 'i' before the number (`i1234`)
- `inst`: Index of instruction

### Data Instructions

- `Pop`: Pops the top of the stack
- `Store <id|null>`: Saves the value on top of stack in a memory register
  - When `Store` has no arguments, it will use the top of the stack as the value and the second to top as memory location. Will free enough memory when there is not enough
- `Push  <id|expr>`: Pushes a value of a memory register or a value to the top of stack
- `Swap`: Swaps the top 2 elements on the stack
- `Rot`: Rotates the top 3 elements on the stack: `0, 1, 2` -> `1, 2, 0`

### IO Instructions

- `Print`: Prints the value on the top of the stack in the terminal
- `Write`: Prints the value of the top of the stack as ASCII character (Decimal)

### Logic Instructions

- `Equal`: Reads the top two elements of the stack, if they are not equal, you skip next instruction	
- `Greater`: Reads the top two elements of the stack, if left is not greater than right, you skip next instruction

### Flow Instructions

- `Jump <inst>` Jumps to index of the instruction specified

### Math Instructions

- `Add`: Reads the top 2 elements on the stack, adds them together and pushes answer to stack
- `Min`: Reads the top 2 elements on the stack, subtracts them and pushes answer to stack
- `Multiply`: Reads the top 2 elements on the stack, multiplies them and pushes answer to stack
- `Divide`: Reads the top 2 elements on the stack, divides them and pushes answer to stack
- `Modulo`: Reads the top 2 elements on the stack, pushes the remainder to the stack

### Extra's

- `;`: Means a comments until end of line