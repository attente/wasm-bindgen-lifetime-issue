pub struct S {
}

impl S {
    pub fn f() -> Result<(), &'static str> {
        Err("error")
    }
}
