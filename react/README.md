# general Rust concepts

There are no classes in Rust! Instead we have types and traits.
A type can be primitive (`uint64`) provided by rust (`Vec<T>`) or custom defined.
A trait is simply a collection of function, and you do `impl Traitname for Typename {}`

A function in a trait can be only the function signature, or provide a implementation, which will be defaulted to if 
no Typename-specific implementation of that trait function is provided in the impl block.
`T` in `Vec<T>` is a generic type, and generic types can also be constrained to implement one or more traits.

`'static` and stuff starting from `'` like `'a` are lifetimes
Ask ai / google for the deets

# rough explanation of this 'react'

_Description of version b552f64 (2025-11-16)_ 

## Important types / traits

### Frame 

**Frame** is a `Vec<String>` type that implements `FrameExt` trait. 
Therefore you can manipulate it like a normal `Vec<String>` but also use the following additional functions:

```rust
pub trait FrameExt {
    fn height(&self) -> usize;
    fn first_width(&self) -> usize;
    fn max_width(&self) -> usize;
    fn align_width(&mut self);
    fn expand_to_height(&mut self, target: usize);
}
```

### Element

**Element** is a trait with a single function `fn draw(&self) -> Frame;`.
A type that implements **Element** must also implement **Send**.
(a rust marker trait for marking a type can be safely transferred across thread boundaries)

Note that `Box<dyn Element>` is a pointer to a type that implements **Element** trait.
(Recommend to rename this trait to sound more like an adjective)

### Component 

**Component** is a dynamic trait type (trait object) for the `_Component` trait.

Cloning a component is very cheap because it is a pointer.

<details>
<summary>dyn example</summary>
A dyn Trait is a dynamically sized type, meaning the compiler doesn't know its exact size at compile time because it could hold a Circle, a Square, or any other type implementing the trait.

Because their size is unknown at compile time, trait objects must always be used behind a pointer (e.g., a reference &, a mutable reference &mut, or a smart pointer like Box<T>, Rc<T>, or Arc<T>). The pointer itself has a fixed, known size.

Dynamic trait types are commonly used when building systems that need to manage collections of different, yet related, objects, such as a Graphical User Interface (GUI) library with various drawable components.

```rust
trait Draw {
    fn draw(&self);
}

struct Button;
impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a Button");
    }
}

struct TextField;
impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing a TextField");
    }
}

fn main() {
    // We can store different types in the same vector using trait objects
    let screen_components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button),
        Box::new(TextField),
        Box::new(Button),
    ];

    for component in screen_components {
        // Dynamic dispatch: the correct draw method is called at runtime
        component.draw(); 
    }
}
```
</details>

### Widget

## Message passing

propagate vs intercept 

## Render loop

## typical file structure

of .rs in widgets and .rs in elements
