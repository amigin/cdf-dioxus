pub fn is_email(src: &str) -> bool {
    let at_index = src.find("@");

    if at_index.is_none() {
        return false;
    }

    let at_index = at_index.unwrap();

    if at_index == 0 {
        return false;
    }

    if at_index == src.len() - 1 {
        return false;
    }

    let dot_index = src.find(".");

    if dot_index.is_none() {
        return false;
    }

    let dot_index = dot_index.unwrap();

    if dot_index == src.len() - 1 {
        return false;
    }

    dot_index > at_index
}
