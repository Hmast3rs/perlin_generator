use std::f64::consts::PI;
use std::collections::HashMap;

pub struct PerlinGen {
    pub grad_cache: HashMap<(u32, u32), (f64, f64)>,
}

impl PerlinGen {
    pub fn new() -> PerlinGen {
        PerlinGen {
            grad_cache: HashMap::new()
        }
    }

    pub fn get_at(&mut self, x: f64, y: f64) -> f64 {
        let x0 = x.floor();
        let y0 = y.floor();
        let x1 = x0 + 1.0;
        let y1 = y0 + 1.0;

        let dx0 = x - x0;
        let dy0 = y - y0;
        let dx1 = x - x1;
        let dy1 = y - y1;

        let g1 = self.get_gradient_at((x0 as u32, y0 as u32));
        let g2 = self.get_gradient_at((x1 as u32, y0 as u32));
        let g3 = self.get_gradient_at((x0 as u32, y1 as u32));
        let g4 = self.get_gradient_at((x1 as u32, y1 as u32));

        let d1 = PerlinGen::dot(g1, (dx0, dy0));
        let d2 = PerlinGen::dot(g2, (dx1, dy0));
        let d3 = PerlinGen::dot(g3, (dx0, dy1));
        let d4 = PerlinGen::dot(g4, (dx1, dy1));
        
        PerlinGen::interpolate(
            PerlinGen::interpolate(d1, d2, dx0),
            PerlinGen::interpolate(d3, d4, dx0),
            dy0
        )
    }

    pub fn get_gradient_at(&mut self, p: (u32, u32)) -> (f64, f64) {
        // Hack deterministic randomness by caching gradients
        if let Some(grad) = self.grad_cache.get(&p) {
            return *grad;
        }
        let theta = 2.0 * PI * rand::random::<f64>();
        let grad = (theta.cos(), theta.sin());
        self.grad_cache.insert(p, grad);
        grad
    }

    fn dot((x1,y1): (f64, f64), (x2,y2): (f64, f64)) -> f64 {
        x1*x2 + y1*y2
    }

    fn interpolate(a0: f64, a1:f64, a: f64) -> f64 {
        // Linear interpolation
        // a0 + (a1 - a0) * a
        // Cubic interpolation [[Smoothstep]]
        // a0 + (a1 - a0) * (3.0 - a * 2.0) * a * a
        // Quintic interpolation [[Smootherstep]]
        a0 + (a1 - a0) * ((a * (a * 6.0 - 15.0) + 10.0) * a * a * a)
    }
}
