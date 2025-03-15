use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, &'buf str>,
}

impl<'buf> QueryString<'buf> {}
