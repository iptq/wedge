import Game from "./lib/game";

const canvas = <HTMLCanvasElement>document.getElementById("canvas");
const ctx = canvas.getContext("2d");

let x = 0;
let game = new Game();

function main() {
  ctx.clearRect(0, 0, 854, 480);
  ctx.fillStyle = "blue";
  x += 1;
  ctx.fillRect(x, 100, 20, 20);

  game.render(ctx);

  requestAnimationFrame(main);
}

document.addEventListener("DOMContentLoaded", main);
