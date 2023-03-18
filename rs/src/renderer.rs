use super::point::Point;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub struct Renderer {
    context: web_sys::CanvasRenderingContext2d,
    width: f64,
    height: f64,
}

impl Renderer {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Renderer {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        
        Renderer{context, width, height}
    }

    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.width, self.height);
    }

    pub fn stroke(&self, color: &str) {
        self.context.set_stroke_style(&JsValue::from_str(color));
    }

    pub fn draw_line(&self, p1: &Point, p2: &Point) {
        self.context.begin_path();
        self.context.move_to(p1.x, p1.y);
        self.context.line_to(p2.x, p2.y);
        self.context.stroke();
    }

    pub fn draw_curve(&self, p: &Vec<Point>) {
        self.context.begin_path();
        self.context.move_to(p[0].x, p[0].y);
        for i in 1..p.len() {
            self.context.line_to(p[i].x, p[i].y);
        }
        self.context.stroke();
    }

    pub fn draw_circle(&self, p: &Point, r: f64) {
        self.context.begin_path();
        self.context.arc(p.x, p.y, r, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        self.context.stroke();
    }
}