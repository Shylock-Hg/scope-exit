use std::ops::Drop;

struct ScopeExit<'a> {
    call_: Box<dyn FnMut() + 'a>,
}

impl<'a> ScopeExit<'a> {
    pub fn new<F>(call: F) -> Self
    where
        F: FnMut() + 'a,
    {
        ScopeExit {
            call_: Box::<F>::new(call),
        }
    }
}

impl<'a> Drop for ScopeExit<'a> {
    fn drop(&mut self) {
        (self.call_)()
    }
}

macro_rules! scope_exit {
    ($call: expr) => {
        let _scope_exit = ScopeExit::new($call);
    };
}

#[cfg(test)]
mod test {
    // modify local variable

    use crate::ScopeExit;
    #[test]
    fn modify_local_variable_test() {
        let mut i = 0;
        {
            let call = || {
                i = 2;
            };
            let _scope_exit = ScopeExit::new(Box::new(call));
        }
        assert_eq!(i, 2);
    }

    #[test]
    fn modify_local_variable_by_macro_test() {
        let mut i = 0;
        let mut j = 0;
        {
            scope_exit!(|| {
                i = 1;
            });

            scope_exit!(|| {
                j = 2;
            });
        }
        assert_eq!(i, 1);
        assert_eq!(j, 2);
    }
}
