use orbtk::prelude::*;

use crate::MainViewState;
use crate::main_state::Action;

type List = Vec<String>;

widget!(
    MainView<MainViewState> {
        title: String,
        list: List
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").title("word search")
        .child(
            Stack::new()
                .margin(8)
                .id("MAIN_STACK")
                .child(
                    TextBox::new()
                        .id("SEARCH_QUERY")
                        .water_mark("search query")
                    .build(ctx)
                )
                // .child(
                //     TextBox::new()
                //         .id("PATH")
                //         .water_mark("path to wordlist")
                //     .build(ctx)
                // )
                .child(
                    Button::new()
                        .text("Search")
                        .on_click(
                            move |states, _| {
                                state(id, states).action(Action::Search);
                                true
                            }
                        )
                        .id("SEARCH_BUTTON")
                    .build(ctx)
                )
            .build(ctx)
        )
    }
}
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
