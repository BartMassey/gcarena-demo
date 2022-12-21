use gc_arena::*;

#[derive(Collect)]
#[collect(no_drop)]
struct MyRoot<'gc>(GcCell<'gc, Vec<List<'gc>>>);

make_arena!(MyArena, MyRoot);

#[derive(Collect)]
#[collect(no_drop)]
enum ConsCell<'gc> {
    Nil,
    Cons { car: u32, cdr: List<'gc> },
}
use ConsCell::*;

#[derive(Collect)]
#[collect(no_drop)]
struct List<'gc>(GcCell<'gc, ConsCell<'gc>>);

impl<'gc> List<'gc> {
    fn new(mc: MutationContext<'gc, '_>) -> Self {
        List(GcCell::allocate(mc, Nil))
    }

    fn cons(self, mc: MutationContext<'gc, '_>, car: u32) -> Self {
        List(GcCell::allocate(mc, Cons { car, cdr: self }))
    }

    fn car(&self) -> u32 {
        if let Cons { car, .. } = *self.0.read() {
            car
        } else {
            panic!("car of nil")
        }
    }

    fn cdr(&self) -> Self {
        if let Cons { cdr, .. } = &*self.0.read() {
            cdr.clone()
        } else {
            panic!("cdr of nil")
        }
    }

    fn is_nil(&self) -> bool {
        matches!(&*self.0.read(), Nil)
    }

    fn print(&self) {
        let mut cur = self.clone();
        let mut sep = "";
        while !cur.is_nil() {
            print!("{sep}");
            print!("{}", cur.car());
            cur = cur.cdr();
            sep = " ";
        }
        println!();
    }
}

impl Clone for List<'_> {
    fn clone(&self) -> Self {
        List(self.0)
    }
}

fn main() {
    let params = ArenaParameters::default();
    let mut arena = MyArena::new(params, |mc| MyRoot(GcCell::allocate(mc, vec![])));

    arena.mutate(|mc, root| {
        let l1 = List::new(mc).cons(mc, 1);
        let l2 = l1.clone().cons(mc, 2);
        root.0.write(mc).push(l1);
        root.0.write(mc).push(l2);
    });

    arena.mutate(|_, root| {
        let roots: &Vec<List<'_>> = &root.0.read();
        roots[0].print();
        roots[1].print();
    });
}
