#[derive(Clone, Debug, PartialEq)]
pub struct NodeList {
    inner: web_sys::NodeList,
}

impl IntoIterator for NodeList {
    type Item = web_sys::Node;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inner: self.inner,
            index: 0,
        }
    }
}

pub struct IntoIter {
    inner: web_sys::NodeList,
    index: u32,
}

impl Iterator for IntoIter {
    type Item = web_sys::Node;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.inner.item(self.index);
        self.index += 1;
        ret
    }
}

impl From<web_sys::NodeList> for NodeList {
    fn from(node_list: web_sys::NodeList) -> Self {
        Self { inner: node_list }
    }
}
