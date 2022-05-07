use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

fn main() {
    dioxus::web::launch(app)
}

pub fn app(cx: Scope) -> Element {
    let date = use_state(&cx, || js_sys::Date::new_0());

    use_future(&cx, date, |_| {
        let date = date.to_owned();
        async move {
            let _ = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |accept, _| {
                let f = Rc::new(RefCell::new(None));
                let g = f.clone();

                *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                    let _ = accept.call0(web_sys::window().unwrap().as_ref());
                    let _ = f.borrow_mut().take();
                }) as Box<dyn FnMut()>));

                let _ = web_sys::window()
                    .expect("Failed to get window object")
                    .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref());
            }))
            .await;
            date.set(js_sys::Date::new_0());
        }
    });

    let second_deg = 360.0 * date.get_seconds() as f32 / 60.0;
    let minute_deg = 360.0 * date.get_minutes() as f32 / 60.0 + 6.0 * second_deg / 360.0;
    let hour_deg = 360.0 * date.get_hours() as f32 / 12.0 + 30.0 * minute_deg / 360.0;

    cx.render(rsx!(
        svg {
            version: "1.1",
            baseProfile: "full",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100vmin",
            height: "100vmin",
            view_box: "0 0 100 100",

            circle {
                cx: "50",
                cy: "50",
                r: "45",
                fill: "#ffffff",
                stroke: "#000000",
                stroke_width: "2",
                filter: "drop-shadow(1.3 0.9 1 rgb(120, 120, 120))",
            }

            line {
                id: "second_bar",
                x1: "50",
                y1: "50",
                x2: "50",
                y2: "7",
                stroke: "#000000",
                stroke_width: "0.4",
                transform: "rotate({second_deg}, 50, 50)",
            }

            line {
                id: "minute_bar",
                x1: "50",
                y1: "50",
                x2: "50",
                y2: "7",
                stroke: "#000000",
                stroke_width: "0.8",
                transform: "rotate({minute_deg}, 50, 50)",
            }

            line {
                id: "hour_bar",
                x1: "50",
                y1: "50",
                x2: "50",
                y2: "20",
                stroke: "#000000",
                stroke_width: "1.0",
                transform: "rotate({hour_deg}, 50, 50)",
            }

            (0..=59).map(|i| {
                let deg = i * 360/60;
                let length = if i % 5 == 0 { 3 } else { 2 };
                let y1 = 5 + length;
                rsx! {
                    line {
                        x1: "50",
                        y1: "{y1}",
                        x2: "50",
                        y2: "5",
                        stroke: "#000000",
                        stroke_width: "0.3",
                        transform: "rotate({deg}, 50, 50)",
                    }
                }
            }),

            (1..=12).map(|i| {
                use std::f32::consts::PI;
                let (x_1, y_1) = (0.0, 39.0);

                let theta = -i as f32 * PI / 6.0;
                let x = 50.0 + x_1 * f32::cos(theta) - y_1 * f32::sin(theta);
                let y = 53.0 + x_1 * f32::sin(theta) - y_1 * f32::cos(theta);

                rsx! {
                    text {
                        x: "{x}",
                        y: "{y}",
                        text_anchor: "middle",
                        font_size: "7.5",
                        fill: "#000000",
                        "{i}"
                    }
                }
            }),
        }
    ))
}
