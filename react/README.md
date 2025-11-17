# general Rust concepts

[section on expression vs statement, no return keyword, no semicolon]

There are no classes in Rust! Instead we have types and traits.
- A type can be primitive (`uint64`), provided by rust (`Vec<T>`) or custom defined.
- But there is something similar to class methods, called 'inherent functions' of a type where you do `impl Typename { ... }`
- A trait is simply a collection of function, and you do `impl Traitname for Typename { ... }`

A function in a trait can be only the function signature, or provide a implementation, which will be defaulted to if 
no Typename-specific implementation of that trait function is provided in the impl block.
`T` in `Vec<T>` is a generic type, and generic types can also be constrained to implement one or more traits.

[section on anonymous functions and `dyn Fn(...) -> ...`, and also Rc<> and .clone()]

`'static` and stuff starting from `'` like `'a` are lifetimes
Ask ai / google for the deets

list of syntactic sugar (search if dont understand):
- `impl` inside function signature
- `Self` type

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

```rust
pub trait _Component {
    fn id(&self) -> usize;
    fn create_element(&mut self) -> (bool, Box<dyn Element>);
    fn on_message(&mut self, event: &Message);
}
```

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

**Widget<State>** is the main type to work with, and has inherent methods for `State`s that satisfy different trait bounds (constraints).
Moreover, it implements `_Component`, so it classifies as a `Component` type too.

<details>
<summary>
Below is an annotated widget.rs with code mostly omitted:
</summary>

```rust
pub struct Widget<State> {
    id: usize,
    pub state: State,
    prev: Option<Component>,
    needs_rebuild: bool,
    builder: Box<dyn Fn(&State) -> Component>,
    on_message: Rc<dyn Fn(&mut Self, &Message)>,
    create_element: Rc<dyn Fn(&mut Self) -> (bool, Box<dyn Element>)>,
}

impl<State> Widget<State> where State: 'static {
    // public convenience 'constructor' for stateful components
    pub fn stateful(
        state: State,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&State) -> Component + 'static,
    ) -> Component { /*...code...*/ }

    // public convenience 'constructor' for elemental components
    pub fn elemental(
        state: State,
        on_message: impl Fn(&mut Self, &Message) + 'static,
        create_element: impl Fn(&mut Self) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component { /*...code...*/ }

    // private helper function for code in this file
    fn _build(&mut self) -> (bool, Component) {
        if !self.needs_rebuild && let Some(prev) = &self.prev {
            (false, prev.clone())
        } else {
            /*...code...*/
            (true, new_widget)
        }
    }

    // set the state!
    pub fn set_state(&mut self, f: impl FnOnce(&mut State)) {
        f(&mut self.state);
        self.needs_rebuild = true;
    }
}

// in below, 'State' is the tuple type (JoinHandle<T>, Option<T>)
impl<T: 'static + Send + Sync> Widget<(JoinHandle<T>, Option<T>)> {
    pub fn future(
        task: impl Future<Output = T> + Send + Sync + 'static,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&Option<T>) -> Component + 'static,
    ) -> Component { /*...code...*/ }
}

// private helper function for code above
fn create_child<T: 'static>(this: &mut Widget<T>) -> (bool, Box<dyn Element>) { /*...code...*/ }

// public helper function, a common case for `on_message` input of `elemental`, see row.rs or column.rs
pub fn propagate(this: &mut Widget<Vec<Component>>, msg: &Message) { /*...code...*/ }
```
</details>

To be most clear one should write `Widget<State>::elemental(...)` but rust compiler can infer the `State` type so can write 
`Widget::elemental(...)`, will compilation error if you use this shorthand and there is ambiguity in what type `State` is

### Explanation for this separation

Widget is a more concrete, renderloop-related Component with funcs and data 
- incorporates data like `id`, `needs_rebuild` bool, etc 

Component is more abstract, basically a trait (technically `_Component` is the underlying trait) for creating Elements and handling Messages

Element is a multithread-friendly  trait for creating Frames 

Frame is a `Vec<String>` with custom functions added on for convenience

Most questionable: are separation to Widget, Component, Element truly necesary? TODO: ask Kendrew for the concrete philosophy behind each 

## Message passing (WIP)

propagate vs intercept 

In
Tokio
, the UnboundedSender and an "unbounded channel" refer to a multi-producer, single-consumer (mpsc) channel that has no limit on the number of messages it can store.

## Render loop (WIP)

## Typical file structure (WIP)

of .rs in widgets and .rs in elements

for 'elemental'-type widgets, the State is simply the children widgets

### Components as Functions 
In many modern Rust UI frameworks, especially those in the Reactive Retained Mode category (like Dioxus and Sycamore)
and all Immediate Mode frameworks (egui), a component is often just a function.

How it works:

- The function takes input data (props) and some context (Scope or Ui).
- The function describes the UI structure for that single frame or state.
- The framework's renderer interprets this description (or calls the rendering primitives directly) to update the screen.

Summary of Architectures
Structure
	Pattern Name	State Management	Example Frameworks
Function	Immediate Mode	Managed by caller	egui
Function	Reactive/VDOM	Managed by framework (hooks/state)	Dioxus, Sycamore
Struct + Trait	Retained Mode	Managed by component struct	Iced, GTK-RS, Slint
