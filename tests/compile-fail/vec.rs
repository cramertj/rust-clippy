#![feature(plugin)]
#![plugin(clippy)]

#![deny(useless_vec)]

fn on_slice(_: &[u8]) {}
#[allow(ptr_arg)]
fn on_vec(_: &Vec<u8>) {}

struct Line {
    length: usize,
}

impl Line {
    fn length(&self) -> usize {
        self.length
    }
}

fn main() {
    on_slice(&vec![]);
    //~^ ERROR useless use of `vec!`
    //~| HELP you can use
    //~| SUGGESTION on_slice(&[])
    on_slice(&[]);

    on_slice(&vec![1, 2]);
    //~^ ERROR useless use of `vec!`
    //~| HELP you can use
    //~| SUGGESTION on_slice(&[1, 2])
    on_slice(&[1, 2]);

    on_slice(&vec ![1, 2]);
    //~^ ERROR useless use of `vec!`
    //~| HELP you can use
    //~| SUGGESTION on_slice(&[1, 2])
    on_slice(&[1, 2]);

    on_slice(&vec!(1, 2));
    //~^ ERROR useless use of `vec!`
    //~| HELP you can use
    //~| SUGGESTION on_slice(&[1, 2])
    on_slice(&[1, 2]);

    on_slice(&vec![1; 2]);
    //~^ ERROR useless use of `vec!`
    //~| HELP you can use
    //~| SUGGESTION on_slice(&[1; 2])
    on_slice(&[1; 2]);

    on_vec(&vec![]);
    on_vec(&vec![1, 2]);
    on_vec(&vec![1; 2]);

    // Now with non-constant expressions
    let line = Line { length: 2 };

    on_slice(&vec![2; line.length]);
    on_slice(&vec![2; line.length()]);

    for a in vec![1, 2, 3] {
        //~^ ERROR useless use of `vec!`
        //~| HELP you can use
        //~| SUGGESTION for a in &[1, 2, 3] {
        println!("{}", a);
    }
}
