mod point;
mod renderer;
mod curve;

use wasm_bindgen::prelude::*;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Debug)]
pub enum State {
    Add(bool),
    Delete,
    Edit((usize, usize)),
}

#[wasm_bindgen]
pub struct App {
    window: web_sys::Window,
    canvas: web_sys::HtmlCanvasElement,
    buttons: Vec<web_sys::HtmlInputElement>,
    renderer: renderer::Renderer,
    curve: curve::Besier,
    state: State,
}

#[wasm_bindgen]
impl App {
    pub fn new(window: web_sys::Window, canvas: web_sys::HtmlCanvasElement, buttons: Vec<web_sys::HtmlInputElement>) -> App {
        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

        let renderer = renderer::Renderer::new(&canvas);

        let mut curve = curve::Besier::new(48);
        curve.add_point(100.0f64, 100.0f64, 150.0f64, 150.0f64);
        curve.add_point(300.0f64, 100.0f64, 350.0f64, 50.0f64);

        App{window, canvas, buttons, renderer, curve, state: State::Add(false)}
    }
    
    pub fn render(&self) {
        self.renderer.clear();

        let points = self.curve.points();
        let control_points = self.curve.control_points();

        self.renderer.stroke("#000");
        self.renderer.draw_curve(&points);

        for cp in control_points.iter() {
            self.renderer.stroke("#f00");
            self.renderer.draw_line(&cp[0], &cp[1]);
            self.renderer.draw_line(&cp[0], &cp[2]);

            self.renderer.stroke("#000");
            self.renderer.draw_circle(&cp[0], 5.0f64);
            self.renderer.draw_circle(&cp[1], 5.0f64);
            self.renderer.draw_circle(&cp[2], 5.0f64);
        }
    }

    pub fn mouse_down(&mut self, x: f64, y: f64) {
        self.set_state();
        match self.state {
            State::Add(false) => {        
                self.curve.add_point(x, y, x, y);
                self.state = State::Add(true)
            }
            State::Delete => {
                let cp = self.curve.control_points();
                'outer_loop: for i in 0..cp.len() {
                    let dist = cp[i][0].dist_coord(x, y);
                    if dist < 10.0 {
                        self.curve.remove_point(i);
                        break 'outer_loop;
                    }
                }
            },
            State::Edit((_i, _j)) => {
                let mut id1 = 0;
                let mut id2 = 9;
                let cp = self.curve.control_points();
                'outer_loop: for i in 0..cp.len() {
                    for j in 0..3 {
                        let dist = cp[i][j].dist_coord(x, y);
                        if dist < 10.0 {
                            id1 = i;
                            id2 = j;
                            break 'outer_loop;
                        }
                    }
                }
                self.state = State::Edit((id1, id2));
            },
            _ => {}
        }
        self.render();
    }

    pub fn mouse_move(&mut self, x: f64, y: f64) {
        match self.state {
            State::Add(true) => {
                self.curve.edit_last_control_point(x, y);
            }
            State::Edit((i, j)) => {
                match j  {
                    0 => self.curve.edit_point(i, x, y),
                    1 => self.curve.edit_control_point(i, x, y, false),
                    2 => self.curve.edit_control_point(i, x, y, true),
                    _ => {}
                }
            }
            _ => {}
        }
        self.render();
    }

    pub fn mouse_up(&mut self, _x: f64, _y: f64) {
        match self.state {
            State::Add(true) => {        
                self.state = State::Add(false)
            }
            State::Edit((_i, _j)) => {
                self.state = State::Edit((0, 9));
            },
            _ => {}
        }
        self.render();
    }

    pub fn resize(&mut self) {
        self.canvas.set_width(self.window.inner_width().unwrap().as_f64().unwrap() as u32);
        self.canvas.set_height(self.window.inner_height().unwrap().as_f64().unwrap() as u32);
    }

    fn set_state(&mut self) {
        for button in self.buttons.iter() {
            if button.checked() {
                self.state = match button.value().parse::<usize>().unwrap() as usize {
                    1 => State::Add(false),
                    2 => State::Delete,
                    _ => State::Edit((0, 9)),
                };
            }
        }
    }
}