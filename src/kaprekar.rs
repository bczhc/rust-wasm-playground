use std::sync::Mutex;
use rayon::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct Kaprekar;

#[inline(always)]
fn check_split(n: u128) -> Option<(u128, u128)> {
    let mut i = 1_u32;
    loop {
        let m = 10_u128.pow(i);
        let part1 = n / m;
        let part2 = n % m;
        if (part1 + part2).checked_mul(part1 + part2) == Some(n) {
            return Some((part1, part2));
        }
        if part1 < 10 {
            break None;
        }
        i += 1;
    }
}

type NumTuple3 = (u128, u128, u128);

/// TODO: rayon multithreading on WASM
fn run(start: u128, end: u128) -> impl IntoParallelIterator<Item=Option<NumTuple3>> {
    let base_max = end.isqrt();
    let base_min = start.isqrt();
    let base_range = base_min..=base_max;
    base_range
        .into_par_iter()
        .map(|x| x * x)
        .map(|x| {
            check_split(x).map(|r| (x, r.0, r.1))
        })
}

struct SendWrapper<T>(T);

unsafe impl<T> Send for SendWrapper<T> {}

#[wasm_bindgen]
impl Kaprekar {
    pub fn kaprekar(start: u64, end: u64, callback: &js_sys::Function) {
        // SAFETY: no async/multithreading is involved here. Make the outer JS context `Send`.
        let callback = Mutex::new(SendWrapper((JsValue::null(), callback)));
        let results = run(start as u128, end as u128);
        results.into_par_iter().filter(|x| x.is_some()).for_each(|x| {
            let x = x.unwrap();
            let guard = callback.lock().unwrap();
            let guard = &guard.0;
            let _ = guard.1.call3(&guard.0,  &x.0.into(), &x.1.into(), &x.2.into());
        });
    }
}
