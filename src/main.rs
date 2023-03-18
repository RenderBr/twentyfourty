#![allow(non_snake_case)]
#![allow(unused_parens)]
use std::{iter::Map, borrow::Borrow};

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*, html::input_data::keyboard_types::Key};
use grid::*;
use rand::Rng;
struct GameGrid {
    grid: Grid<i32>,
    binding: Grid<i32>,
}
impl GameGrid {
    fn new() -> Self {
        Self {
            grid: grid![[0,0,0,0]
            [0,0,0,0]
            [0,0,0,0]
            [0,0,0,0]],
            binding: grid![[0,0,0,0]
            [0,0,0,0]
            [0,0,0,0]
            [0,0,0,0]],
        }
    }

    fn getGrid(&self) -> Grid<i32> {
        self.grid.clone()
    }

    fn resetGame(&mut self){
        self.grid = grid![[0,0,0,0]
        [0,0,0,0]
        [0,0,0,0]
        [0,0,0,0]];
    }

    fn updateBinding(&mut self){
        self.binding = self.grid.clone();
    }

    fn getBinding<'a>(&'a mut self) -> &'a grid::Grid<i32>{
        self.binding.borrow()
    }

    fn placeRandom(&mut self){

        let mut pos1 = rand::thread_rng().gen_range(0..4);
        let mut pos2 = rand::thread_rng().gen_range(0..4);

        let mut index = 0;
        while(self.grid[pos1][pos2] != 0){
            index+=1;
            if(index > 16){
                break;
            }
            pos1 = rand::thread_rng().gen_range(0..4);
            pos2 = rand::thread_rng().gen_range(0..4);
        }

  

        self.grid[pos1][pos2] = 2;
        self.updateBinding();
    }

    fn parseInput(&mut self, evt: Key){
        match evt {
            Key::ArrowRight => {
                self.moveRight();
                println!("{:?}", self.grid);
            },
            Key::ArrowLeft => {
                println!("left");
            },
            Key::ArrowUp => {
                println!("up");
            },
            Key::ArrowDown => {
                println!("down");
            },
            _ => {
                println!("other");
            }
        }
        self.updateBinding();
    }

    fn startGame(&mut self) {
        self.resetGame();
        self.placeRandom();
    }

    fn moveRight(&mut self){
        self.placeRandom();

    }
    


}

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

fn sendInput(key: Key, game: &mut GameGrid){
    game.parseInput(key);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Props)]
struct RenderProps<'a> {
    game: &'a GameGrid
}
fn Render<'a>(cx:Scope<'a, RenderProps>) -> Element<'a>{
    let grid_rendered = cx.props.game.binding.iter().map(|row| {
        rsx!(div{
            class: "col col-xs",
            background_color:"gray",
            height:"3rem",
            key: "{row}",
            "{row}",
        })
    });   
    cx.render(rsx! { grid_rendered })

}


// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let mut game = GameGrid::new();
    game.startGame();

        cx.render(rsx! {
        div {
        width:"100%",
        height:"100%",
        tabindex: "0",
        onkeydown: move |evt| {
            sendInput(evt.key(), &mut game);           
        },
        link {
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css",
            rel: "stylesheet",
        }
        div{
            class: "container",
            width: "50%",
            div{
                class: "row row-cols-4 text-center",
                Render {
                    game: &game,
                }
            }
    
        }
    }
        
    })

    
    
   
}