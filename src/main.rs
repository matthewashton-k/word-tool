mod consts;
use orbtk::prelude::*;

pub use self::{main_state::*, main_view::*};
mod main_state;


mod main_view;

fn main(){
    Application::from_name("Word Search")
        .window(move |ctx| {
            Window::new()
                .title("Word Search")
                .position((100.0, 100.0))
                .size(372.0, 380.0)
                .resizeable(true)
                .child(MainView::new().title("Word Search").build(ctx))
                .build(ctx)
        })
        .run();
}
