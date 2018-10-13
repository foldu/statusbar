#[derive(Debug, Clone)]
pub struct OptionalString {
    cont: String,
    is_some: bool,
}

impl Default for OptionalString {
    fn default() -> Self {
        OptionalString {
            cont: "".into(),
            is_some: false,
        }
    }
}

impl OptionalString {
    pub fn get(&self) -> Option<&str> {
        if self.is_some {
            Some(&self.cont)
        } else {
            None
        }
    }

    pub fn copy_from<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        str_copy(&mut self.cont, s);
        self.is_some = true;
    }

    pub fn invalidate(&mut self) {
        self.is_some = false;
    }
}

pub fn str_copy<S>(dest: &mut String, target: S)
where
    S: AsRef<str>,
{
    dest.clear();
    dest.push_str(target.as_ref());
}
