# Funcy

An interpreted language that emulates an assembly (Kind of); Mostly just a fun language. 
User is required to use a stack, and can save values in memory. The challenge is to use 
a little memory as possible. Oh, do not forgot to pop the stack, it is not very big...

## Specification

- A stack of 8 elements is created which stores integers
- You get an infinitely growable chunk of memory, **you can never give memory back to the system**
- This is a safe language meaning in this case that instructions never pop from stack. Only possible with specific instruction

### Naming

- `id`: Index of element in memory
- `expr`: 32 bit integer
- `inst`: Index of instruction

### Data Instructions

- `Pop`: Pops the top of the stack
- `Store <id>`: Saves the value on top of stack in a memory register  
- `Push  <id|expr>`: Pushes a value of a memory register or a value to the top of stack
- `Print`: prints the value on the top of the stack in the terminal

### Logic Instructions

- `Equal`: Reads the top two elements of the stack, if they are not equal, you skip next instruction	
- `Greater`: Reads the top two elements of the stack, if left is not greater than right, you skip next instruction

### Flow Instructions

- `Jump <inst>` Jumps to index of the instruction specified

### Math Instructions

- `Add`: Reads the top elements on the stack, adds them together and pushes answer to stack
- `Min`: Reads the top elements on the stack, subtracts them and pushes answer to stack
- `Multiply`: Reads the top elements on the stack, multiplies them and pushes answer to stack
- `Divide`: Reads the top elements on the stack, divides them and pushes answer to stack

### Extra's

- `;`: Means a comments until end of line