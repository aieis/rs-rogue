pub struct Canvas {
    pub data: String,
    pub width: usize,
    pub height: usize,
}

pub fn norm(x: usize) -> usize {
    (x*3 + 1)/2
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
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
