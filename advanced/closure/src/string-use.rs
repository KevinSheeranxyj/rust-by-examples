

impl String {
    pub fn contains(&self, p: impl Pattern) -> bool {
        p.is_contained_in(self)
    }
}


