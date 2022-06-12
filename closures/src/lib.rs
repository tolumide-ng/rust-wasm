use js_sys::{Array, Date};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window};


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    // One of the first interesting thinfs we can do with closures is simply access stack data in Rust
    let array = Array::new();
    array.push(&"Hello".into());
    array.push(&1.into());
    
    let mut first_item = None;
    array.for_each(&mut |obj, idx, _arr| match idx {
        0 => {
            assert_eq!(obj, "Hello");
            first_item = obj.as_string();
        }
        1 => assert_eq!(obj, 1),
        _ => panic!("unknown index: {}", idx),
    });
    assert_eq!(first_item, Some("Hello".to_string()));

    // Below are some more advanced usages of the `Closure` type for closures
    // that need to live beyonf our function call

    setup_clock(&window, &document)?;
    setup_clicker(&document);

    document.get_element_by_id("loading")
        .expect("should have #loading on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style()
        .set_property("display", "none")?;

    Ok(())
}


// set up a clock on our page and update it each secons to ensure it's got an accurate date.
// 
// Note the usage of `Closure` here becayse the closure is "long lived",
// vasically meaning it has to persist beyond the call to this function.
// Also of note here is the `.as_ref().unchecked_ref()` chain, which is how
// you can extract &`Function`, what `web-sys` expectes, from a `Closure`
// which only hands you `&JsValue` visa `AsRef`.
fn setup_clock(window: &Window, document: &Document) -> Result<(), JsValue> {
    let current_time = document.get_element_by_id("current-time")
        .expect("should have #current-time on the page");
    
        update_time(&current_time);
        let a = Closure::wrap(Box::new(move || update_time(&current_time)) as Box<dyn Fn()>);

        window.set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;

        fn update_time(current_time: &Element) {
            current_time.set_inner_html(&String::from(
                Date::new_0().to_locale_string("en-GB", &JsValue::undefined()),
            ));
        }

        a.forget();

        Ok(())
}

fn setup_clicker(document: &Document) {
    let num_clicks = document.get_element_by_id("num-clicks")
        .expect("should have #num-clcicks on the page");

    let mut clicks = 0;
    let a = Closure::wrap(Box::new(move || {
        clicks += 1;
        num_clicks.set_inner_html(&clicks.to_string());
    }) as Box<dyn FnMut()>);

    document.get_element_by_id("green-square")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));

    a.forget();
}