use super::point::Point;

#[derive(Debug, Clone)]
pub struct Besier {
    p: Vec<Point>,
    c: Vec<Point>,
    cp: Vec<Vec<Point>>,
    b: Vec<Vec<f64>>,
}

impl Besier {
    pub fn new(div: usize) -> Self {
        let p: Vec<Point> = Vec::new();
        let c: Vec<Point> = Vec::new();
        let cp: Vec<Vec<Point>> = Vec::new();

        let mut b = vec![vec![0.0f64; 4]; div+1];
        for i in 0..=div {
            let t: f64 = (i as f64) / (div as f64);
            b[i][0] = (1.0 - t).powf(3.0);
            b[i][1] = 3.0 * (1.0 - t).powf(2.0) * t;
            b[i][2] = 3.0 * (1.0 - t) * t.powf(2.0);
            b[i][3] = t.powf(3.0);
        }
        Self{p, c, cp, b}
    }

    pub fn add_point(&mut self, px: f64, py: f64, cx: f64, cy: f64) {
        self.p.push(Point{x: px, y: py});
        self.c.push(Point{x: cx - px, y: cy - py});
        let l = self.p.len();
        let list: Vec<Point> = vec![
            self.p[l-1],
            self.p[l-1] + self.c[l-1],
            self.p[l-1] - self.c[l-1],
        ];
        self.cp.push(list);
    }

    pub fn remove_point(&mut self, id: usize) {
        self.p.remove(id);
        self.c.remove(id);
        self.cp.remove(id);
    }

    pub fn edit_point(&mut self, id: usize, px: f64, py: f64) {
        self.p[id].set(px, py);
        self.cp[id][0] = self.p[id];
        self.cp[id][1] = self.p[id] + self.c[id];
        self.cp[id][2] = self.p[id] - self.c[id];
    }

    pub fn edit_control_point(&mut self, id: usize, cx: f64, cy: f64, inv: bool) {
        if !inv {
            self.c[id].set(cx - self.p[id].x, cy - self.p[id].y);
        } else {
            self.c[id].set(- cx + self.p[id].x, - cy + self.p[id].y);
        }
        
        self.cp[id][1] = self.p[id] + self.c[id];
        self.cp[id][2] = self.p[id] - self.c[id];
    }

    pub fn edit_last_control_point(&mut self, cx: f64, cy: f64) {
        self.edit_control_point(self.p.len() - 1, cx, cy, false);
    }

    pub fn points(&self) -> Vec<Point> {
        let mut output = vec![];
        for i in 0..self.cp.len()-1 {
            for j in 0..self.b.len() {
                let p = self.b[j][0] * self.cp[i][0] + self.b[j][1] * self.cp[i][1] + self.b[j][2] * self.cp[i+1][2] + self.b[j][3] * self.cp[i+1][0];
                output.push(p);
            }
        }
        output
    }

    pub fn control_points(&self) -> Vec<Vec<Point>> {
        self.cp.clone()
    }
}