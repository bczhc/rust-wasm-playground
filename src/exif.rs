use std::collections::HashMap;
use std::ptr::null_mut;
use rexif::{ExifData, ExifEntry, ExifResult, ExifTag, TagValue};
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::errors::AnyhowExt;

#[wasm_bindgen]
pub struct Exif {
    data: ExifData,
    map: HashMap<ExifTag, ExifEntry>,
    
    pub addr: *mut Exif,
}

#[wasm_bindgen(js_class = "Exif")]
impl Exif {
    /// Returns the raw object address
    pub fn read(buffer: &[u8]) -> crate::Result<*mut Exif> {
        let result: anyhow::Result<_> = try {
            let mut map = HashMap::new();
            let data = rexif::parse_buffer(buffer)?;
            for x in &data.entries {
                map.insert(x.tag, x.clone());
            }
            let exif = Exif {
                data,
                map,
                addr: null_mut(),
            };
            Box::into_raw(Box::new(exif))
        };
        result.map_err_string()
    }
    
    pub fn gps(exif: *mut Exif) -> Option<Vec<f64>> {
        unsafe {
            let coord = read_gps(&(*exif).map)?;
            Some(vec![coord.0, coord.1])
        }
    }
    
    pub fn free(exif: *mut Exif) {
        unsafe {
            drop(Box::from_raw(exif));
        }
    }
}

fn read_gps(map: &HashMap<ExifTag, ExifEntry>) -> Option<(f64, f64)> {
    let get_decimal = |value: &TagValue| {
        if let TagValue::URational(r) = value {
            Some(r[0].value() + r[1].value() / 60.0 + r[2].value() / 3600.0)
        } else {
            None
        }
    };
    let get_ref_ascii = |value: &TagValue| {
        if let TagValue::Ascii(x) = value {
            Some(x.clone())
        } else {
            None
        }
    };
    let mut latitude = get_decimal(&map.get(&ExifTag::GPSLatitude)?.value)?;
    let mut longitude = get_decimal(&map.get(&ExifTag::GPSLongitude)?.value)?;

    latitude *= match get_ref_ascii(&map.get(&ExifTag::GPSLatitudeRef)?.value)?.as_str() {
        "N" => 1.0,
        "S" => -1.0,
        _ => 1.0,
    };
    longitude *= match get_ref_ascii(&map.get(&ExifTag::GPSLongitudeRef)?.value)?.as_str() {
        "E" => 1.0,
        "W" => -1.0,
        _ => 1.0,
    };

    Some((latitude, longitude))
}