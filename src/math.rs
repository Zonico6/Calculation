use self::super::Num;

pub fn fac(a: Num) -> Num {
    if a == 1f32 {
        a
    } else {
        a * fac(a - 1f32)
    }
}