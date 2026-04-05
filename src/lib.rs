use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use serde::{Deserialize, Serialize};
use rand::prelude::*;
use rand_pcg::Pcg64;
use js_sys::Map;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WeatherData {
    pub temperature: f32,
    pub rain: f32,
    pub snow: f32,
    pub clouds: f32,
    pub wind_speed: f32,
    pub wind_deg: f32,
    pub pressure: f32,
    pub time: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Forecast {
    pub hourly: Vec<WeatherData>,
    pub current: WeatherData,
    pub tmin: f32,
    pub tmax: f32,
}

#[wasm_bindgen]
pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    rng: Pcg64,
    draw_xstep: f32,
    draw_ystep: f32,
    draw_xflat: f32,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Renderer, JsValue> {
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        let width = canvas.width();
        let height = canvas.height();
        let rng = Pcg64::seed_from_u64(42);

        Ok(Renderer { 
            ctx, 
            width, 
            height, 
            rng, 
            draw_xstep: 30.0, 
            draw_ystep: 60.0, 
            draw_xflat: 10.0 
        })
    }

    pub fn draw_landscape(&mut self, forecast_val: JsValue, sprites: Map) -> Result<(), JsValue> {
        let forecast: Forecast = serde_wasm_bindgen::from_value(forecast_val)?;
        
        self.ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        
        let tline = self.calculate_tline(&forecast);
        
        // Render soil
        self.draw_soil(&tline)?;

        // Render House and Smoke
        self.draw_house_and_smoke(&forecast, &tline, &sprites)?;

        // Render weather effects
        self.draw_weather_effects(&forecast, &tline, &sprites)?;

        // Render Wind (Trees)
        self.draw_wind_effects(&forecast, &tline, &sprites)?;

        // Render Sun/Moon/Flowers
        self.draw_celestial_and_plants(&forecast, &tline, &sprites)?;
        
        Ok(())
    }

    fn calculate_tline(&self, forecast: &Forecast) -> Vec<f32> {
        let xstart = 30.0;
        let mut tline = vec![0.0; self.width as usize];
        
        let temprange = forecast.tmax - forecast.tmin;
        let degreeperpixel = if temprange < self.draw_ystep { 1.0 } else { temprange / self.draw_ystep };

        let deg_to_pix = |t: f32| -> f32 {
            let n = (t - forecast.tmin) / degreeperpixel;
            self.height as f32 - (self.draw_ystep + n)
        };

        let mut oldy = deg_to_pix(forecast.current.temperature);
        for i in 0..(xstart as usize) {
            if i < tline.len() {
                tline[i] = oldy;
            }
        }

        let mut xpos = xstart;
        let n = (self.draw_xstep - self.draw_xflat) / 2.0;

        for f in &forecast.hourly {
            let newy = deg_to_pix(f.temperature);
            
            // Bezier segment
            for i in 0..(n as usize) {
                let dx = (xpos + i as f32) as usize;
                if dx < self.width as usize {
                    tline[dx] = self.mybezier(dx as f32, xpos, oldy, xpos + n, newy);
                }
            }

            // Flat segment + bridge gaps
            for i in (n as usize)..(self.draw_xstep as usize) {
                let dx = (xpos + i as f32) as usize;
                if dx < self.width as usize {
                    tline[dx] = newy;
                }
            }
            
            xpos += self.draw_xstep;
            oldy = newy;
        }

        // Fill remaining
        for i in (xpos as usize)..tline.len() {
            tline[i] = oldy;
        }

        tline
    }

    fn mybezier(&self, x: f32, xa: f32, ya: f32, xb: f32, yb: f32) -> f32 {
        let d = xb - xa;
        let t = (x - xa) / d;
        (1.0 - t).powi(3) * ya + 3.0 * (1.0 - t).powi(2) * t * ya + 3.0 * (1.0 - t) * t.powi(2) * yb + t.powi(3) * yb
    }

    fn draw_soil(&self, tline: &[f32]) -> Result<(), JsValue> {
        self.ctx.begin_path();
        self.ctx.set_stroke_style(&JsValue::from_str("black"));
        self.ctx.set_line_width(2.0);
        if let Some(&first_y) = tline.first() {
            self.ctx.move_to(0.0, first_y as f64);
            for (x, &y) in tline.iter().enumerate().skip(1) {
                self.ctx.line_to(x as f64, y as f64);
            }
        }
        self.ctx.stroke();
        Ok(())
    }

    fn draw_house_and_smoke(&mut self, forecast: &Forecast, tline: &[f32], sprites: &Map) -> Result<(), JsValue> {
        let xpos = 0.0;
        let ypos = tline[0];
        
        self.draw_sprite("house", 0, xpos, ypos, false, sprites)?;

        let pressure_min = 950.0;
        let pressure_max = 1050.0;
        let smokeangle_deg = ((forecast.current.pressure - pressure_min) / (pressure_max - pressure_min)) * 85.0 + 5.0;
        let smokeangle_deg = smokeangle_deg.clamp(0.0, 90.0);
        self.draw_smoke(xpos + 21.0, ypos - 23.0, smokeangle_deg)?;

        Ok(())
    }

    fn draw_sprite(&self, name: &str, index: u32, x: f32, y: f32, mirror: bool, sprites: &Map) -> Result<(), JsValue> {
        let key = format!("{}_{:02}", name, index);
        let img_val = sprites.get(&JsValue::from_str(&key));
        
        if !img_val.is_undefined() && !img_val.is_null() {
            self.ctx.save();
            
            let (w, h) = if let Some(img) = img_val.dyn_ref::<HtmlImageElement>() {
                let w = img.width() as f64;
                let h = img.height() as f64;
                if mirror {
                    self.ctx.translate(x as f64 + w, 0.0)?;
                    self.ctx.scale(-1.0, 1.0)?;
                    self.ctx.draw_image_with_html_image_element(img, 0.0, (y - h as f32) as f64)?;
                } else {
                    self.ctx.draw_image_with_html_image_element(img, x as f64, (y - h as f32) as f64)?;
                }
                (w, h)
            } else if let Some(canvas) = img_val.dyn_ref::<HtmlCanvasElement>() {
                let w = canvas.width() as f64;
                let h = canvas.height() as f64;
                if mirror {
                    self.ctx.translate(x as f64 + w, 0.0)?;
                    self.ctx.scale(-1.0, 1.0)?;
                    self.ctx.draw_image_with_html_canvas_element(canvas, 0.0, (y - h as f32) as f64)?;
                } else {
                    self.ctx.draw_image_with_html_canvas_element(canvas, x as f64, (y - h as f32) as f64)?;
                }
                (w, h)
            } else {
                return Err(JsValue::from_str("Sprite is neither Image nor Canvas"));
            };

            self.ctx.restore();
        }
        Ok(())
    }

    fn draw_smoke(&mut self, x0: f32, y0: f32, angle_deg: f32) -> Result<(), JsValue> {
        let a = (std::f32::consts::PI * angle_deg) / 180.0;
        let r_px = 30.0;
        let smoke_size = 60.0;
        let k = r_px * a.sin() / (r_px * a.cos()).sqrt();

        self.ctx.set_fill_style(&JsValue::from_str("black"));
        
        for x in 0..100 {
            let y_smoke = k * (x as f32).sqrt();
            let r = ((x * x) as f32 + y_smoke * y_smoke).sqrt();
            if r > smoke_size { break; }
            
            if self.rng.r#gen::<f32>() * 1.3 > (r / smoke_size) {
                let (dx, dy) = if self.rng.r#gen::<f32>() * 1.2 < (r / smoke_size) {
                    (self.rng.r#gen_range(-1.0..1.0), self.rng.r#gen_range(-1.0..1.0))
                } else {
                    (0.0, 0.0)
                };
                self.ctx.fill_rect((x0 + x as f32 + dx) as f64, (y0 - y_smoke + dy) as f64, 1.0, 1.0);
            }
        }
        Ok(())
    }

    fn draw_weather_effects(&mut self, forecast: &Forecast, tline: &[f32], sprites: &Map) -> Result<(), JsValue> {
        let mut xpos = 30.0;
        
        self.draw_rain(forecast.current.rain, 0, 30, tline)?;
        self.draw_snow(forecast.current.snow, 0, 30, tline)?;
        self.draw_clouds(forecast.current.clouds, 0.0, 30.0, sprites)?;

        for f in &forecast.hourly {
            self.draw_rain(f.rain, xpos as usize, self.draw_xstep as usize, tline)?;
            self.draw_snow(f.snow, xpos as usize, self.draw_xstep as usize, tline)?;
            self.draw_clouds(f.clouds, xpos, self.draw_xstep, sprites)?;
            xpos += self.draw_xstep;
            if xpos > self.width as f32 { break; }
        }
        Ok(())
    }

    fn draw_rain(&mut self, value: f32, xpos: usize, width: usize, tline: &[f32]) -> Result<(), JsValue> {
        if value <= 0.0 { return Ok(()); }
        let heavy_rain = 5.0;
        let rain_factor = 20.0;
        let r = 1.0 - (value / heavy_rain) / rain_factor;

        self.ctx.set_fill_style(&JsValue::from_str("black"));
        for x in xpos..(xpos + width) {
            if x >= self.width as usize { break; }
            let y_limit = tline[x] as usize;
            for y in (0..y_limit).step_by(2) {
                if self.rng.r#gen::<f32>() > r {
                    self.ctx.fill_rect(x as f64, y as f64, 1.0, 2.0);
                }
            }
        }
        Ok(())
    }

    fn draw_snow(&mut self, value: f32, xpos: usize, width: usize, tline: &[f32]) -> Result<(), JsValue> {
        if value <= 0.0 { return Ok(()); }
        let heavy_snow = 5.0;
        let snow_factor = 10.0;
        let r = 1.0 - (value / heavy_snow) / snow_factor;

        self.ctx.set_fill_style(&JsValue::from_str("black"));
        for x in xpos..(xpos + width) {
            if x >= self.width as usize { break; }
            let y_limit = tline[x] as usize;
            for y in (0..y_limit).step_by(2) {
                if self.rng.r#gen::<f32>() > r {
                    self.ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
                }
            }
        }
        Ok(())
    }

    fn draw_clouds(&mut self, percent: f32, xpos: f32, width: f32, sprites: &Map) -> Result<(), JsValue> {
        if percent < 2.0 { return Ok(()); }
        let cloudset = if percent < 5.0 { vec![2] }
            else if percent < 10.0 { vec![3, 2] }
            else if percent < 20.0 { vec![5, 3, 2] }
            else if percent < 30.0 { vec![10, 5] }
            else if percent < 40.0 { vec![10, 10] }
            else if percent < 50.0 { vec![10, 10, 5] }
            else if percent < 60.0 { vec![30, 5] }
            else if percent < 70.0 { vec![30, 10] }
            else if percent < 80.0 { vec![30, 10, 5, 5] }
            else if percent < 90.0 { vec![30, 10, 10] }
            else { vec![50, 30, 10, 10, 5] };

        let y_clouds = self.height as f32 / 4.0;
        for c in cloudset {
            let dx = self.rng.r#gen_range(0.0..width);
            self.draw_sprite("cloud", c, xpos + dx, y_clouds, false, sprites)?;
        }
        Ok(())
    }

    fn draw_wind_effects(&mut self, forecast: &Forecast, tline: &[f32], sprites: &Map) -> Result<(), JsValue> {
        let mut xpos = 30.0;
        for f in &forecast.hourly {
            self.draw_wind(f.wind_speed, f.wind_deg, xpos, tline, sprites)?;
            xpos += self.draw_xstep;
            if xpos > self.width as f32 { break; }
        }
        Ok(())
    }

    fn draw_wind(&mut self, speed: f32, direction: f32, xpos: f32, tline: &[f32], sprites: &Map) -> Result<(), JsValue> {
        let mut tree_types = Vec::new();
        let deg_dist = |d1: f32, d2: f32| {
            let d = (d1 - d2).abs();
            if d > 180.0 { 360.0 - d } else { d }
        };

        let add_type = |dir: f32, target: f32, name: &str, items: &mut Vec<String>| {
            let dist = deg_dist(dir, target);
            let n = (dist / 11.25) as usize;
            let counts = vec![4, 3, 3, 2, 2, 1, 1];
            if n < counts.len() {
                for _ in 0..counts[n] {
                    items.push(name.to_string());
                }
            }
        };

        add_type(direction, 0.0, "pine", &mut tree_types);
        add_type(direction, 90.0, "east", &mut tree_types);
        add_type(direction, 180.0, "palm", &mut tree_types);
        add_type(direction, 270.0, "tree", &mut tree_types);

        if tree_types.is_empty() { return Ok(()); }

        let wind_indices = if speed <= 0.4 { vec![] }
            else if speed <= 0.7 { vec![0] }
            else if speed <= 1.7 { vec![1, 0, 0] }
            else if speed <= 3.3 { vec![1, 1, 0, 0] }
            else if speed <= 5.2 { vec![1, 2, 0, 0] }
            else if speed <= 7.4 { vec![1, 2, 2, 0] }
            else if speed <= 9.8 { vec![1, 2, 3, 0] }
            else if speed <= 12.4 { vec![2, 2, 3, 0] }
            else { vec![3, 3, 3, 3] };

        for &idx in &wind_indices {
            let tree_name = &tree_types[self.rng.r#gen_range(0..tree_types.len())];
            let xx = xpos + self.rng.r#gen_range(-1.0..1.0);
            let mirror = self.rng.r#gen_bool(0.5);
            let offset = (xx as usize).min(tline.len() - 1);
            self.draw_sprite(tree_name, idx, xx, tline[offset] + 1.0, mirror, sprites)?;
        }

        Ok(())
    }

    fn draw_celestial_and_plants(&mut self, forecast: &Forecast, tline: &[f32], sprites: &Map) -> Result<(), JsValue> {
        let y_celestial = self.height as f32 / 6.0;
        self.draw_sprite("sun", 0, self.width as f32 * 0.2, y_celestial, false, sprites)?;
        
        let flower_x = (self.width as f32 * 0.7).min(tline.len() as f32 - 1.0);
        let flower_idx = (forecast.current.time / 43200) % 2; 
        if let Some(&fy) = tline.get(flower_x as usize) {
            self.draw_sprite("flower", flower_idx, flower_x, fy + 1.0, false, sprites)?;
        }
        
        Ok(())
    }

    pub fn draw_temperature_text(&self, n: f32, x: f32, y: f32, sprites: &Map) -> Result<(), JsValue> {
        let n_round = n.round().abs() as u32;
        let digits = vec![
            n_round / 100,
            (n_round % 100) / 10,
            n_round % 10,
        ];
        
        let mut dx = 0.0;
        if n < 0.0 {
            dx += self.draw_digit(11, x + dx, y, sprites)? + 1.0;
        } else {
            dx += self.draw_digit(10, x + dx, y, sprites)? + 1.0;
        }

        let mut started = false;
        for (i, &d) in digits.iter().enumerate() {
            if d != 0 || i == 2 || started {
                dx += self.draw_digit(d, x + dx, y, sprites)?;
                started = true;
            }
        }
        Ok(())
    }

    fn draw_digit(&self, id: u32, x: f32, y: f32, sprites: &Map) -> Result<f32, JsValue> {
        let key = format!("digit_{:02}", id);
        let img_val = sprites.get(&JsValue::from_str(&key));
        if !img_val.is_undefined() && !img_val.is_null() {
            let img = img_val.dyn_ref::<HtmlImageElement>().ok_or("Failed to convert digit to HtmlImageElement")?;
            let w = img.width() as f64;
            let h = img.height() as f64;
            self.ctx.draw_image_with_html_image_element(img, x as f64, (y - h as f32) as f64)?;
            return Ok(w as f32);
        }
        Ok(0.0)
    }
}
