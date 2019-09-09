use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

use rand::{Rng, thread_rng};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::UnwrapThrowExt;
use web_sys;
use web_sys::CanvasRenderingContext2d;

use polycons::config::WorldConfig;

use crate::polycons::{Line, Node, World};

mod polycons;
mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type InternalType = f64;

type JsResult = Result<(), JsValue>; // Could use T instead of ()

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn initWorld(
    numNodes: usize,
    dimX: InternalType,
    dimY: InternalType,
    maxStrength: InternalType,
    lineThreshold: InternalType,
    minV: InternalType,
    maxV: InternalType,
    minR: InternalType,
    maxR: InternalType,
) -> *mut World<InternalType> {
    let mut rng = thread_rng();
    let world = World::random(
        &mut rng,
        numNodes,
        (dimX, dimY),
        WorldConfig {
            max_strength: maxStrength,
            line_threshold: lineThreshold,
        },
        (minV, maxV),
        (minR, maxR),
    );
    Box::into_raw(Box::new(world))
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn generateNodes(world: *mut World<InternalType>, deltaTime: f64) {
    unsafe {world.as_mut().expect_throw("World object was null").step_nodes(deltaTime)}
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn drawNodes(canvas: &CanvasRenderingContext2d, world: *const World<InternalType>) -> JsResult {
    // TODO: should this go inside the loop
    let nodes = unsafe {
        world.as_ref().expect_throw("World object was null").nodes()
    };
    for node in nodes.iter()
        {
            draw_node(canvas, node)?;
        }
    Ok(())
}

fn draw_node(canvas: &CanvasRenderingContext2d, node: &Node<InternalType>) -> JsResult
    where
        InternalType: Into<f64>,
{
    canvas.begin_path();
    const FULL_CIRCLE: f64 = 2.0 * PI;
    canvas.arc(
        node.x().into(),
        node.y().into(),
        node.get_radius().into(),
        0.0,
        FULL_CIRCLE,
    )?;

    canvas.fill();
    Ok(())
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn generateLines(world: *const World<InternalType>) -> *const Vec<Line<InternalType>> {
    let lines = unsafe { world.as_ref().expect_throw("World object was null").calculate_lines()};
    Box::into_raw(Box::new(lines))
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn drawLines(canvas: &CanvasRenderingContext2d, lines: *const Vec<Line<InternalType>>) {
    for line in unsafe { lines.as_ref().expect_throw("Lines array was null").iter() } {
        draw_line(canvas, line)
    }
}

fn draw_line(canvas: &CanvasRenderingContext2d, line: &Line<InternalType>) {
    let (start, end) = line.endpoints();
    canvas.begin_path(); // TODO: should this go inside the loop
    canvas.set_stroke_style(&JsValue::from(format!("rgba(0, 0, 0, {})", line.get_strength().get() as f64 / 255.0)));
    canvas.move_to(start[0] as f64, start[1] as f64);
    canvas.line_to(end[0] as f64, end[1] as f64);
    canvas.stroke();
}
