use crate::Vec3;

pub struct Canvas {
    pub data: String,
    pub width: usize,
    pub height: usize,
}

pub fn norm(x: usize) -> usize {
    (x*3 + 1)/2
}

pub fn arcnorm(x: usize) -> usize {
    (x*2 - 1)/3
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let width = norm(width);
        let mut data_arr = Vec::new();
        data_arr.resize((width + 1) * height, ' ');

        for i in 0..height {
            data_arr[i * (width + 1) + width] = '\n';
        }

        let data = data_arr.iter().collect();
        
        Self {
            data,
            width,
            height
        }
    }

    pub fn w(&self) -> i32 {
        arcnorm(self.width) as i32
    }

    pub fn h(&self) -> i32 {
        self.height as i32
    }

    pub fn clear(&mut self) {
        let clear_row = std::iter::repeat(" ").take(self.width).collect::<String>();
        
        for i in 0..self.height {
            let idx = i * (self.width + 1);
            self.data.replace_range(idx..idx+self.width, &clear_row);
        }        
    }
}

pub fn clear_background() {
    print!("{}[2J", 27 as char);
}

pub fn draw_circle(canvas: &mut Canvas, r: usize, x: usize, y: usize) {
    let x = norm(x);
    for j in 0..=r {
        let x_max = norm(f64::sqrt((r*r - j*j) as f64) as usize);
        let tex = std::iter::repeat("#").take(x_max*2+1).collect::<String>();

        let idx = (y+j)*(canvas.width+1) + x-x_max;
        canvas.data.replace_range(idx..=idx+x_max*2, &tex);

        let idx = (y-j)*(canvas.width+1) + x-x_max;
        canvas.data.replace_range(idx..=idx+x_max*2, &tex);
    }    
}

pub fn draw_line(canvas: &mut Canvas, point1: (usize, usize), point2: (usize, usize)) {
    let point1 = (norm(point1.0), point1.1);
    let point2 = (norm(point2.0), point2.1);

    //TODO: Consider using the m = dx / dy instead of m = dy / dx
    if point2.0 == point1.0 {

        for y in point1.1..point2.1 {
            let idx = y * (canvas.width + 1) + point1.0;
            canvas.data.replace_range(idx..idx+1, "#");
        }

        return;
    }

    let m = (point2.1 as f32 - point1.1 as f32) / (point2.0 as f32 - point1.0 as f32);

    for x in point1.0..point2.0 {
        let y_f = m*(x as f32) + point1.0 as f32;
        let y1 = y_f.ceil() as usize;
        let y2 = y_f.floor() as usize;
        let idx = y1 * (canvas.width + 1) + x;
        println!("{idx} {y1} {x}");
        canvas.data.replace_range(idx..idx+1, "#");
        let idx = y2 * (canvas.width + 1) + x;
        canvas.data.replace_range(idx..idx+1, "#");        
    }
}

pub fn draw_triangle(canvas: &mut Canvas, points: [(usize, usize); 3]) {
    let points : [(usize, usize); 3] = [
        (norm(points[0].0), points[0].1),
        (norm(points[1].0), points[1].1),
        (norm(points[2].0), points[2].1),
    ];

    let mut mnx = points[0].0;
    let mut mny = points[0].1;
    let mut mxx = points[0].0;
    let mut mxy = points[0].1;

    let pvecs = [
        Vec3::new(points[0].0 as i32, points[0].1 as i32, 0),
        Vec3::new(points[1].0 as i32, points[1].1 as i32, 0),
        Vec3::new(points[2].0 as i32, points[2].1 as i32, 0)
    ];

    for (px, py) in points[1..].iter() {
        if *px < mnx {
            mnx = *px;
        }

        if *px > mxx {
            mxx = *px;
        }

        if *py < mny {
            mny = *py;
        }
        if *py > mxy {
            mxy = *py;
        }
    }

    println!("{mny} {mxy} {mnx} {mxx}");
    for py in mny..=mxy {
        for px in mnx..=mxx {
            let p = Vec3::new(px as i32, py as i32, 0);
            let idx = py * (canvas.width + 1) + px;
            if Vec3::cross_z(&pvecs[0], &pvecs[1], &p) < 0{
                //canvas.data.replace_range(idx..idx+1, ">");
                continue;
            }

            if Vec3::cross_z(&pvecs[1], &pvecs[2], &p) < 0{
                //canvas.data.replace_range(idx..idx+1, "<");
                continue;
            }

            if Vec3::cross_z(&pvecs[2], &pvecs[0], &p) < 0{
                //canvas.data.replace_range(idx..idx+1, "=");
                continue;
            }
            
            canvas.data.replace_range(idx..idx+1, "#");
        }
    }
}

pub fn draw_quad(canvas: &mut Canvas, points: [(usize, usize); 4]) {
    let tl = points[0];
    let tr = points[1];
    let br = points[2];
    let bl = points[3];

    println!("{:?}", points);

    for y in tl.1..=bl.1 {
        
        let pc = (y  as f32 - tl.1 as f32) / (bl.1 as f32 - tl.1 as f32);

        let x1 = ((bl.0 as f32 - tl.0 as f32) * pc + tl.0 as f32) as usize;
        let p1 = (x1, y);

        let y2 = ((br.1 as f32 - tr.1 as f32) * pc + tr.1 as f32) as usize;
        let x2 = ((br.0 as f32 - tr.0 as f32) * pc + tr.0 as f32) as usize;
        let p2 = (x2, y2);

        println!("{pc}: ({x1}, {y}) => ({x2}, {y2})");

        draw_line(canvas, p1, p2);
    }
}

pub fn draw_rectangle(canvas: &mut Canvas, width: usize, height: usize, x: usize, y: usize) {
    let x = norm(x);
    let width = norm(width);
    let tex = std::iter::repeat("#").take(width).collect::<String>();
    for j in 0..height {        
        let idx = (y+j) * (canvas.width + 1) + x;
        canvas.data.replace_range(idx..idx+width, &tex);
    }
}

pub fn draw_square(canvas: &mut Canvas, s: usize, x: usize, y: usize) {
    draw_rectangle(canvas, s, s, x, y);
}

pub fn draw_text(canvas: &mut Canvas, text: &str, x: usize, y: usize) {
    let x = norm(x);
    let idx = y * (canvas.width + 1) + x;
    canvas.data.replace_range(idx..idx+text.len(), text);
}

pub fn swap_buffer(canvas: &Canvas) {
    print!("{}", canvas.data);
}
