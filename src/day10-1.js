// Solution made in js for testing purpose: it's easier to implement graphics on js than rust
{
  class Point{
    constructor(x, y, vx, vy) {
      this.x = x;
      this.y = y;
      this.vx = vx;
      this.vy = vy;
      this.time = 0;
    }
    
    move(speed = 1) {
      this.x -= -1 * speed * this.vx;
      this.y -= -1 * speed * this.vy;
      this.time += speed;
    }
    
    draw(ctx) {
      if (this.x >= 0 && this.y >= 0 && this.x < 1280 && this.y < 720) {
        ctx.fillRect(this.x, this.y, 2, 2);
      }
    }
  }
  
  function getInput() {
    let points = [];
    let reg = /(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)/g;
    let rawInput = document.body.innerText;
    let capture;
    
    while ((capture = reg.exec(rawInput)) !== null) {
      points.push(new Point(Number(capture[1]), Number(capture[2]), Number(capture[3]), Number(capture[4])));
    }
    
    return points;
  }
  
  function getCanvas() {
    let canvas = document.getElementById('canvas');
    if(!canvas) {
      canvas = document.createElement('canvas');
      canvas.setAttribute('id', 'canvas');
      canvas.setAttribute('tabindex', '1');
      canvas.width = 1280;
      canvas.height = 720;
      document.body.appendChild(canvas);
    }
    return canvas;
  }
  
  function fastForward(points) {
    let point = points[0];
    while(point.x < 0 || point.y < 0 || point.x >= 1280 || point.y >= 720) {
      for(pt of points) {
        pt.move();
      }
    }
  }
  
  function run(points, ctx, speed = 1) {
    requestAnimationFrame(() => {
      ctx.clearRect(0, 0, 1280, 720);
      for (point of points) {
        point.move(speed);
        point.draw(ctx);
      }
    });
  }
  
  function onCanvasKeyDown(event, time) {
    let speed = 1;
    switch(event.key) {
      case 'ArrowDown':
        speed = -10;
        break;
      case 'ArrowUp':
        speed = 10;
        break;
      case 'ArrowLeft':
        speed = -1;
        break;
      case 'ArrowRight':
        speed = 1;
        break;
      case 'Enter': 
        console.info(`Estimated time: ${time}`);
      default:
        speed = 0;
    }
    event.preventDefault();
    return speed;
  }
  
  function main() {
    let points = getInput();
    let canvas = getCanvas();
    let ctx = canvas.getContext('2d');
    ctx.fillColor = 'orange';
    fastForward(points);
    canvas.addEventListener('keydown', evt => run(points, ctx, onCanvasKeyDown(evt, points[0].time)));
  }
  
  main();
}