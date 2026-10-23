use gb_core::cpu::Cpu;
use gb_core::io::Buttons;
use gb_core::utils::{SCREEN_HEIGHT, SCREEN_WIDTH};
use js_sys::{Float32Array, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, KeyboardEvent};

#[wasm_bindgen]
pub struct GB {
    cpu: Cpu,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl GB {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GB, JsValue> {
        let cpu = Cpu::new();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let gb = GB { cpu, ctx };
        Ok(gb)
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, data: Uint8Array) {
        let mut rom: Vec<u8> = Vec::new();

        for i in 0..data.byte_length() {
            rom.push(data.get_index(i));
        }
        self.cpu.load_rom(&rom);
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> bool {
        self.cpu.tick()
    }

    #[wasm_bindgen]
    pub fn draw_screen(&mut self) {
        let mut framebuffer = self.cpu.render();
        let img_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut framebuffer),
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .unwrap();
        self.ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
    }

    #[wasm_bindgen]
    pub fn press_button(&mut self, event: KeyboardEvent, pressed: bool) {
        let key = event.key();
        if let Some(button) = key2btn(&key) {
            self.cpu.press_button(button, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn press_btn(&mut self, btn: &str, pressed: bool) {
        let button = match btn {
            "a" | "A" => Some(Buttons::A),
            "b" | "B" => Some(Buttons::B),
            "start" | "Start" => Some(Buttons::Start),
            "select" | "Select" => Some(Buttons::Select),
            "up" | "Up" => Some(Buttons::Up),
            "down" | "Down" => Some(Buttons::Down),
            "left" | "Left" => Some(Buttons::Left),
            "right" | "Right" => Some(Buttons::Right),
            _ => None,
        };
        if let Some(b) = button {
            self.cpu.press_button(b, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn get_title(&self) -> String {
        self.cpu.get_title().to_string()
    }

    #[wasm_bindgen]
    pub fn get_audio_samples(&mut self) -> Float32Array {
        let buffer = &self.cpu.bus.apu.audio_buffer;
        let js_array = unsafe { Float32Array::view(buffer.as_slice()) };
        let copied_array = Float32Array::new(&js_array);
        self.cpu.bus.apu.audio_buffer.clear();
        copied_array
    }

    #[wasm_bindgen]
    pub fn has_battery(&self) -> bool {
        self.cpu.bus.has_battery()
    }

    #[wasm_bindgen]
    pub fn is_battery_dirty(&self) -> bool {
        self.cpu.is_battery_dirty()
    }

    #[wasm_bindgen]
    pub fn clean_battery(&mut self) {
        self.cpu.clean_battery();
    }

    #[wasm_bindgen]
    pub fn get_save_data(&self) -> Uint8Array {
        let data = self.cpu.get_battery_data();
        let js_array = unsafe { Uint8Array::view(data) };
        Uint8Array::new(&js_array)
    }

    #[wasm_bindgen]
    pub fn load_save_data(&mut self, data: Uint8Array) {
        let mut save = Vec::new();
        for i in 0..data.byte_length() {
            save.push(data.get_index(i));
        }
        self.cpu.set_battery_data(&save);
    }
}

fn key2btn(key: &str) -> Option<Buttons> {
    match key {
        "ArrowDown" | "s" | "S" => Some(Buttons::Down),
        "ArrowUp" | "w" | "W" => Some(Buttons::Up),
        "ArrowRight" | "d" | "D" => Some(Buttons::Right),
        "ArrowLeft" | "a" | "A" => Some(Buttons::Left),
        "Enter" => Some(Buttons::Start),
        "Backspace" | "Tab" | " " => Some(Buttons::Select),
        "x" | "X" | "k" | "K" => Some(Buttons::A),
        "z" | "Z" | "j" | "J" => Some(Buttons::B),
        _ => None,
    }
}