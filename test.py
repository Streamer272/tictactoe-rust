from dataclasses import dataclass
from typing import List

board: List[str] = [" ", " ", " ", " ", " ", " ", " ", " ", " "]


@dataclass(init=True)
class Posibility:
    board: List[str]
    outcome: str


def available(board: List[str]) -> List[int]:
    found: List[int] = []
    for (index, box) in enumerate(board):
        if box == " ":
            found += [index]
    return found


def find(board: List[str], current: str) -> List[Posibility]:
    is_full = True
    for box in board:
        if box == " ":
            is_full = False
            break

    is_won = False
    outcome = " "
    if board[0] == board[1] == board[2] != " ":
        is_won = True
        outcome = board[0]
    elif board[3] == board[4] == board[5] != " ":
        is_won = True
        outcome = board[3]
    elif board[6] == board[7] == board[8] != " ":
        is_won = True
        outcome = board[6]
    elif board[0] == board[3] == board[6] != " ":
        is_won = True
        outcome = board[0]
    elif board[1] == board[4] == board[7] != " ":
        is_won = True
        outcome = board[1]
    elif board[2] == board[5] == board[8] != " ":
        is_won = True
        outcome = board[2]
    elif board[0] == board[4] == board[8] != " ":
        is_won = True
        outcome = board[0]
    elif board[2] == board[4] == board[6] != " ":
        is_won = True
        outcome = board[2]

    if is_full or is_won:
        return [Posibility(board, outcome)]

    posibilities: List[Posibility] = []
    for each in available(board):
        copy = board.copy()
        copy[each] = current
        future = find(board, "X" if current == "O" else "O")
        posibilities.append(*future)

    return posibilities

future = find(board, "X")
print(future)
