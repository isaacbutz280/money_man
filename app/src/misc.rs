

pub fn money_to_float(s: &str) -> Result<f32, std::num::ParseFloatError> {
    let mut rv = String::new();
    for c in s.chars() {
        if c != '$' && c != ' ' && c != '+' && c != ',' {
            rv.push(c);
        }
    }

    rv.parse()
}

pub fn round(num: f32, decimals: u32) -> f32 {
    let precison = 10i32.pow(decimals) as f32;
    (num * precison).round() / precison
}
