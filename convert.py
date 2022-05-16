import json
import sys

x = sys.stdin.read()
y = json.loads(x)

board1 = {
  "dimensions": y["dimensions"],
  "player": y["player1"],
  "goal": y["goal1"],
}

board2 = {
  "dimensions": y["dimensions"],
  "player": y["player2"],
  "goal": y["goal2"],
}

blocks = []
for block in y["blocks"]:
  blocks.append(block)

res = {
  "boards": [board1, board2],
  "blocks": blocks,
}

print(json.dumps(res))
