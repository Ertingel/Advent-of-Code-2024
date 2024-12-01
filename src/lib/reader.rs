use std::rc::Rc;

struct Reader {
    source: Rc<Vec<char>>,
    head: usize,
}

impl Reader {
    pub fn new(source: &str) -> Reader {
        Reader {
            source: Rc::new(source.chars().collect()),
            head: 0,
        }
    }

    pub fn split(&self) -> Reader {
        Reader {
            source: self.source.clone(),
            head: self.head,
        }
    }

    pub fn merge(mut self, other: &Reader) -> Self {
        self.head = other.head;
        self
    }

    pub fn get_char(&self, relative_index: Option<usize>) -> Option<&char> {
        let index = self.head + relative_index.unwrap_or(0);
        self.source.get(index)
    }
}
