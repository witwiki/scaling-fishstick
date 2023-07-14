# seahorse_calculator
# Built with Seahorse v0.2.7
# https://seahorse-lang.org/docs/your-first-seahorse-program

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

class Calculator(Account):
    owner: Pubkey
    display: i64

@instruction
def init_calculator(owner: Signer, calculator: Empty[Calculator]):
    # Initialize the calculator and set the owner
    calculator = calculator.init(
        payer= owner,
        seeds= ['Calculator', owner]
    )
    calculator.owner = owner.key()

@instruction
def reset_calculator(owner: Signer, calculator: Calculator):
    print(owner.key(), ' is resetting ', calculator.key())

    # Verify owner
    assert owner.key() == calculator.owner, 'This is not your calculator!'

    calculator.display = 0

class Operator(Enum):
    ADD = 0
    SUB = 1
    MUL = 2
    DIV = 3

@instruction
def do_operation(owner: Signer, calculator: Calculator, op: Operator, num:i64):
    # Verify owner
    assert owner.key() == calculator.owner, 'This is not your calculator!'

    if op == Operator.ADD:
        calculator.display += num
    elif op == Operator.SUB:
        calculator.display -= num
    elif op == Operator.MUL:
        calculator.display *= num
    elif op == Operator.DIV:
        calculator.display //= num



