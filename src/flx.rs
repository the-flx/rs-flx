include!("./bindings.rs");

use std::ffi::CString;

pub struct FlxResult {
    pub score: i32,
    pub tail: i32,
}

impl FlxResult {
    pub fn new(_score: i32, _tail: i32) -> Self {
        Self {
            score: _score,
            tail: _tail,
        }
    }
}

pub fn score(it_str: &str, query: &str) -> FlxResult {
    let str = CString::new(it_str).expect("CString::new failed with `it_str`");
    let query = CString::new(query).expect("CString::new failed with `query`");

    let ret_result: FlxResult;

    unsafe {
        let result = flx_score(str.as_ptr(), query.as_ptr());

        ret_result = FlxResult::new((*result).score, (*result).tail);
    }

    return ret_result;
}
