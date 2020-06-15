pub fn is_call(mn: &str) -> bool {
    mn.starts_with("jsr ") || mn.starts_with("bsr ")
}

pub fn is_return(mn: &str) -> bool {
    mn == "rts"
}
