fn main() {
    let append_exclamation = |word: &str| -> String {
        let mut new_word = String::from(word);
        new_word.push('!');
        return new_word;
    };
    let num = |word: String| -> usize { word.len() };
    let add_one = |num: usize| -> usize { num + 1 };

    let composed = pipe().to(append_exclamation).to(num).to(add_one);

    println!("{}", composed.call("Hi")); // prints 4
}

fn pipe() -> PipeMaker {
    PipeMaker::new()
}

struct PipeMaker {}

impl PipeMaker {
    fn new() -> PipeMaker {
        PipeMaker {}
    }

    fn to<'a, F, T, U>(self, wrapped_function: F) -> Pipe<'a, T, U>
    where
        F: 'a + Fn(T) -> U,
        T: 'a,
        U: 'a,
    {
        Pipe {
            wrapped_function: Box::new(wrapped_function),
        }
    }
}

struct Pipe<'a, T, U> {
    wrapped_function: Box<dyn Fn(T) -> U + 'a>,
}

impl<'a, T, U> Pipe<'a, T, U> {
    fn to<F, V>(self, input: F) -> Pipe<'a, T, V>
    where
        F: 'a + Fn(U) -> V,
        T: 'a,
        U: 'a,
        V: 'a,
    {
        Pipe {
            wrapped_function: Box::new(move |arg: T| -> V { input((self.wrapped_function)(arg)) }),
        }
    }

    fn call(self, arg: T) -> U {
        (self.wrapped_function)(arg)
    }
}
