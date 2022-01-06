pub enum PlugResult<Ctx> {
    Continue,
    Rerun,
    Terminate,
    Replace(Vec<Box<dyn Plug<Ctx>>>),
    Err(anyhow::Error),
}

pub trait Plug<Ctx> {
    fn execute(&mut self, ctx: &mut Ctx) -> PlugResult<Ctx>;
}

impl<Ctx, F> Plug<Ctx> for F
where
    F: FnMut(&mut Ctx) -> PlugResult<Ctx>,
{
    fn execute(&mut self, ctx: &mut Ctx) -> PlugResult<Ctx> {
        self(ctx)
    }
}

pub struct Chain<Ctx> {
    plugs: Vec<Box<dyn Plug<Ctx>>>,
    idx: usize,
    rerun: usize,
}

impl<Ctx> Chain<Ctx> {
    pub fn new(plugs: Vec<Box<dyn Plug<Ctx>>>) -> Self {
        Self {
            plugs,
            idx: 0,
            rerun: 0,
        }
    }

    pub fn execute(&mut self, ctx: &mut Ctx) -> Result<(), anyhow::Error> {
        while self.idx < self.plugs.len() {
            let plug = &mut self.plugs[self.idx];
            match plug.execute(ctx) {
                PlugResult::Continue => {
                    self.idx += 1;
                    self.rerun = 0;
                }
                PlugResult::Rerun => {
                    if self.rerun == 5 {
                        return Err(anyhow::anyhow!("two many reruns"));
                    }
                    self.rerun += 1;
                }
                PlugResult::Terminate => {
                    return Ok(());
                }
                PlugResult::Replace(plugs) => {
                    self.plugs = plugs;
                    self.idx = 0;
                    self.rerun = 0;
                }
                PlugResult::Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn basic_works() {
        let mut chain: Chain<i32> = Chain::new(vec![
            Box::new(|data: &mut i32| {
                *data += 1;
                PlugResult::Continue
            }),
            Box::new(|data: &mut i32| {
                *data += 1;
                PlugResult::Continue
            }),
        ]);

        let mut data = 1;
        assert!(chain.execute(&mut data).is_ok());
        assert_eq!(data, 3);
    }

    #[test]
    fn rerun_works() {
        let mut number = 1;
        let mut chain: Chain<i32> = Chain::new(vec![
            Box::new(move |data: &mut i32| {
                if number == 1 {
                    number += 1;
                    return PlugResult::Rerun;
                }
                *data += 1;
                PlugResult::Continue
            }),
            Box::new(|data: &mut i32| {
                *data += 1;
                PlugResult::Continue
            }),
        ]);

        let mut data = 1;
        assert!(chain.execute(&mut data).is_ok());
        assert_eq!(data, 3);
    }

    #[test]
    fn replace_works() {
        let mut number = 1;
        let mut chain: Chain<i32> = Chain::new(vec![
            Box::new(|data: &mut i32| {
                *data += 1;
                PlugResult::Continue
            }),
            Box::new(|_data: &mut i32| {
                PlugResult::Replace(vec![
                    Box::new(|data: &mut i32| {
                        *data += 2;
                        PlugResult::Continue
                    }),
                    Box::new(|data: &mut i32| {
                        *data += 2;
                        PlugResult::Continue
                    }),
                ])
            }),
        ]);

        let mut data = 1;
        assert!(chain.execute(&mut data).is_ok());
        assert_eq!(data, 6);
    }
}
