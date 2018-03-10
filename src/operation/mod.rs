pub struct Operation<T>
where
    T: Fn() -> (),
{
    pub about: &'static str,
    pub name: &'static str,
    execute: T,
}

impl<T> Operation<T>
where
    T: Fn() -> (),
{
    pub fn new(
        name: &'static str,
        about: &'static str,
        execute: T,
    ) -> Operation<T> {
        Operation {
            name: name,
            about: about,
            execute: execute,
        }
    }

    pub fn perform(&self) {
        (self.execute)()
    }
}
