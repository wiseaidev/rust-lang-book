use mathlikeanim_rs::{
    animations::{create::create, draw_stroke_then_fill::draw_stroke_then_fill},
    objects::{
        plotting::axes::{axes, plot_in_axes, riemann_rectangles_for_plot},
        svg_to_vector::svg_to_vector,
        vector_object::{VectorFeatures, VectorObject},
    },
    scene::Scene,
    utils::{log, smooth},
};
use once_cell::sync::Lazy;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use reqwest;
use std::error::Error;
use wasm_bindgen::prelude::*;

static mut SCENE: Lazy<Scene> = Lazy::new(|| Scene::new(1920, 1080, 60, ""));

async fn tex2svg(tex: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "http://localhost:8080/tex2svg?from={}",
        percent_encode(tex.as_bytes(), NON_ALPHANUMERIC)
    );

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let svg = response.text().await?;
        Ok(svg)
    } else {
        Err(format!("Error Occurred: {}", response.status()).into())
    }
}

pub async fn tex_to_vector(latex: String) -> VectorFeatures {
    let svg = tex2svg(latex.as_str()).await;
    return svg_to_vector(svg.unwrap().as_str()).set_stroke_color((1.0, 1.0, 1.0, 1.0), true);
}

#[wasm_bindgen(start)]
pub async fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    log("Loaded document");
    let canvas = document.get_element_by_id("canvas").unwrap();
    log("Loaded canvas");
    let context = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    log("Got context");
    let sn = unsafe { &mut SCENE };
    sn.init_context(context);
    log("Initialized context");
    let axes = axes(
        0.0,
        10.0,
        1.0,
        0.0,
        10.0,
        1.0,
        (960.0, 540.0),
        Some(800.0),
        Some(800.0),
        Some((1.0, 1.0, 1.0, 1.0)),
        Some(4.0),
        Some("butt"),
        Some("miter"),
        Some(0),
        Some(true),
        Some(true),
        Some(20.0),
        Some(20.0),
        Some(true),
        Some(true),
        None,
    );
    sn.add(axes.clone());
    sn.play(vec![draw_stroke_then_fill], vec![0], 60, |t| {
        smooth(t, 10.0)
    })
    .await;
    log("Added axes");
    let plot = plot_in_axes(
        |t| t.powi(2) / 10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.005,
        &axes.clone(),
        Some((249.0 / 255.0, 105.0 / 255.0, 14.0 / 255.0, 1.0)),
        Some(4.0),
        Some("butt"),
        Some("miter"),
        Some(1),
        None,
    );
    sn.add(plot.clone());
    sn.play(vec![create], vec![1], 60, |t| smooth(t, 10.0))
        .await;
    log("Added plot");
    let riemann_rectangles = riemann_rectangles_for_plot(
        |t| t.powi(2) / 10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        1.0,
        0.0,
        10.0,
        10,
        &axes,
        Some((0.0, 0.0, 0.0, 1.0)),
        Some((249.0 / 255.0, 105.0 / 255.0, 14.0 / 255.0, 0.75)),
        Some(2.0),
        Some("butt"),
        Some("miter"),
        Some(2),
        None,
    );
    log(format!(
        "Riemann rectangles: {:?}",
        riemann_rectangles.subobjects.len()
    )
    .as_str());
    sn.add(riemann_rectangles);
    sn.add(axes);
    sn.play(vec![draw_stroke_then_fill], vec![2], 60, |t| {
        smooth(t, 10.0)
    })
    .await;
    let mut func_tex = tex_to_vector("$$f(x) = \\frac{x^2}{10}$$".to_string()).await;
    func_tex = func_tex.scale(150.0 / func_tex.get_height(), true);
    func_tex = func_tex.next_to_point((0.0, 0.0), (1.0, 1.0), 20.0, (0.0, 0.0), true);
    func_tex = func_tex.set_stroke_color((1.0, 1.0, 1.0, 1.0), true);
    func_tex.index = 3;
    sn.add(func_tex);
    sn.play(vec![draw_stroke_then_fill], vec![3], 60, |t| {
        smooth(t, 10.0)
    })
    .await;
    sn.update();
}

#[wasm_bindgen(js_name = changeNRects)]
pub async fn change_n_rects(n_rects: usize) {
    let sn = unsafe { &mut SCENE };
    let axes = sn.get_objects_from_indices(vec![0])[&0].clone();
    let riemann_rectangles = riemann_rectangles_for_plot(
        |t| t.powi(2) / 10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        1.0,
        0.0,
        10.0,
        n_rects,
        &axes,
        Some((0.0, 0.0, 0.0, 1.0)),
        Some((249.0 / 255.0, 105.0 / 255.0, 14.0 / 255.0, 0.75)),
        Some(2.0),
        Some("butt"),
        Some("miter"),
        Some(2),
        None,
    );
    sn.add(riemann_rectangles);
    sn.add(axes);
    sn.update();
}
