mod slides;

use slides::{md_to_slides, Slides};

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlFormElement, HtmlInputElement};

const SAMPLE_TEXT: &str = "# Example slideshow

Hello world

## Second slide

A paragraph

Another paragraph

## Third slide

A paragraph

Another paragraph

## Fourth slide

A paragraph

Another paragraph

## Fifth slide

This is great.";

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct PresentationTool {
    inner: Rc<RefCell<PresentationToolInner>>,
}

pub struct PresentationToolInner {
    form: HtmlFormElement,
    text_area: HtmlInputElement,
    presentation: HtmlElement,
    slides: Slides,
}

#[wasm_bindgen]
impl PresentationTool {
    pub fn new(
        form: HtmlFormElement,
        text_area: HtmlInputElement,
        presentation: HtmlElement,
    ) -> Self {
        text_area.set_value(SAMPLE_TEXT);
        Self {
            inner: Rc::new(RefCell::new(PresentationToolInner {
                form,
                text_area,
                presentation,
                slides: Slides::new(),
            })),
        }
    }

    pub fn start_presentation(&mut self) {
        let txt = self.inner.borrow().text_area.value();

        self.inner.borrow_mut().slides = md_to_slides(txt);
        self.inner.borrow_mut().hide_input_form();
        self.add_keyboard_listener();
        self.render_next_slide();
    }

    // TODO: Fix bug where kb listener is added repeatedly without removing the old listener when
    // pressing Escape and starting a new presentation.
    fn add_keyboard_listener(&mut self) {
        let inner = self.inner.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            match event.code().as_ref() {
                "ArrowLeft" | "ArrowUp" | "KeyA" => {
                    inner.borrow_mut().render_previous_slide();
                }
                "Space" | "ArrowRight" | "ArrowDown" | "KeyD" => {
                    inner.borrow_mut().render_next_slide();
                }
                "Escape" => inner.borrow_mut().show_input_form(),
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();
    }

    fn render_next_slide(&self) {
        self.inner.borrow_mut().render_next_slide();
    }
}

impl PresentationToolInner {
    fn hide_input_form(&mut self) {
        self.form.set_class_name("hidden");
        self.presentation.set_class_name("");
    }

    fn show_input_form(&mut self) {
        self.form.set_class_name("");
        self.presentation.set_class_name("hidden");
    }

    fn render_next_slide(&mut self) {
        if let Some(slide) = self.slides.next() {
            let html = slide.to_html();
            self.presentation.set_inner_html(&html);
        }
    }

    fn render_previous_slide(&mut self) {
        if let Some(slide) = self.slides.previous() {
            let html = slide.to_html();
            self.presentation.set_inner_html(&html);
        }
    }
}
