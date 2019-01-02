#![feature(plugin)]
#![plugin(complexity_lint)]


fn fn_const() {
    return ()
}

fn fn_loop(){
    for i in 0..10 {
        fn_const()
    }
}

enum LinkedList<A> {
    Nil,
    Cons(A, Box<LinkedList<A>>)
}

fn len<A>(input: &LinkedList<A>) -> u32 {
    use self::LinkedList::*;

    match input {
        Nil => 0,
        Cons(_, ref rest) => 1 + len(rest)
    }
}