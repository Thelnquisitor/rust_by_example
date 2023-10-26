// Without this, we cannot reference Cons, Nil without the List:: prefix.
// Wondering why a method being attached to the enum doesn't have this kind of 
// implicit referencing already though.
use crate::List::*;

enum List {
    // Tuple struct that wraps an element and a pointer to the next node
    // Box is used to allocate values on the heap. BOX is a smart pointer to a heap allocated
    // value of type T (in this case List) 
    Cons(u32, Box<List>),
    // A node that signifies the end of the linked list
    Nil,
}

// Attach methods to the enum
impl List {
    fn new() -> List {
        Nil
    }

    // Consume a list and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // Return length of list
    fn len(&self) -> u32 {
        
        // Need to match on self, since it could either be Cons or Nil
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())  
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // list will be of type List::Nil
    let mut list = List::new();

    // We take the existing Nil variant and add it to the Cons.Box pointer then return it.
    // The list should look like this now: 1->Nil
    list = list.prepend(1);

    // Now it looks like: 2->1->Nil
    list = list.prepend(2);

    // 3->2->1->Nil
    list = list.prepend(3);

    // We call len on the pointer for each Con node, which looks like this:
    // at node 3 so 1 + tail.len (count: 1)
    // at node 2 so 1 + tail.len (count: 2)
    // at node 1 so 1 + tail.len (count: 3)
    // at node Nil so 0          (count: 3)
    println!("linked list has length: {}", list.len());

    // Similar story here. We call stringify on every Con node until we reach a Nil node
    // at node 3 so 3, ->NextNode
    // at node 2 so 3, 2, ->NextNode
    // at node 1 so 3, 2, 1, ->NextNode
    // at node Nil so 3, 2, 1, Nil
    println!("{}", list.stringify());
}