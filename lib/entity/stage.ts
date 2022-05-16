import { z } from "zod";

import board from "./board";
import block from "./block";

const schema = z.object({
  boards: z.array(board.schema),
  blocks: z.array(block.schema),
});

const render = (ctx: CanvasRenderingContext2D) => {
};

export default { schema, render };
