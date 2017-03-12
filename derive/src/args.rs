use quote::Tokens;

pub struct Args<'a> {
    input: &'a str,
    pub tokens: Tokens,
}

impl<'a> Args<'a> {
    pub fn new(input: &str) -> Args {
        Args {
            input: input.trim(),
            tokens: Tokens::new(),
        }
    }

    pub fn process(&mut self) -> &mut Self {
        if self.input.is_empty() {
            self.tokens = quote!(reql::Term::new());
            return self;
        }

        let args = format!("args!({})", self.input);
        let body = self.body();

        self.tokens = quote!({
            let mut args = reql::Args::new();
            args.set_string(#args);
            #body
            args
        });

        self
    }

    fn body(&self) -> Tokens {
        Tokens::new()
    }
}
