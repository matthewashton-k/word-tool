use orbtk::prelude::*;
use std::error::Error;

use grep::matcher::Matcher;
use grep::regex::RegexMatcher;
use grep::searcher::{Searcher, sinks::UTF8};

pub const MAIN_STACK: &'static str = "MAIN_STACK";
pub const PATH:&'static str = "PATH";
pub const SEARCH_QUERY:&'static str = "SEARCH_QUERY";

use crate::main_view;

#[derive(Default, AsAny)]
pub struct MainViewState {
    pub action: Option<Action>,
    popup_action: Option<PopupAction>,
    popup: Option<Entity>,
    entities: MainStateEntities
}
#[derive(Debug, Copy, Clone)]
pub enum Action {
    Search
}

#[derive(Debug, Clone,)]
pub enum PopupAction {
    ShowError(String),
    ShowMessage(String),
    ShowFindings(Findings),
    Close,
}
impl Default for PopupAction {
    fn default() -> PopupAction {
        PopupAction::Close
    }
}

#[derive(Default)]
pub struct MainStateEntities {
    main_stack: Option<Entity>,
    query: Option<Entity>
}

impl MainViewState {
    pub fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
    pub fn popup_action(&mut self, action: impl Into<Option<PopupAction>>) {
        self.popup_action = action.into();
    }
}

impl State for MainViewState {
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        loop{
            if let Some(action) = &self.action{
                match action {
                    Action::Search => {
                        use crate::consts::WORDLIST;
                        let query = ctx.get_widget(self.entities.query.unwrap()).clone::<String16>("text");
                        let matches = match search(&query.to_string()[..], WORDLIST) {
                            Ok(m) => {
                                m
                            }
                            Err(e) =>{
                                self.popup_action(PopupAction::ShowError("No matches!".to_string()));
                                break;
                            }
                        };
                        let count = &matches.len();
                        let mut findings = Findings {
                            count: *count as u128,
                            words: vec![]
                        };
                        for m in matches {
                            findings.words.push(m.1);
                        }
                        self.popup_action(PopupAction::ShowFindings(findings));
                        self.action = None;
                        break;
                    }
                }
            }

        break;}

        ////////////popup actions ////////////
        if let Some(action) = &self.popup_action {
            match action {
                PopupAction::ShowError(s) => {
                    println!("[WARN] Showing an error message.");
                    let tab = ctx.entity_of_child(MAIN_STACK).unwrap();
                    let current_entity = ctx.entity;
                    let build_context = &mut ctx.build_context();

                    let popup = create_popup(current_entity, &s, build_context, WidgetState::MainViewState);
                    build_context.append_child(tab, popup);
                    self.popup = Some(popup);
                    self.popup_action = None;
                    self.action = None;
                }
                PopupAction::ShowMessage(s) => {

                    let container = ctx.entity_of_child(MAIN_STACK).unwrap();
                    let current_entity = ctx.entity;
                    let build_context = &mut ctx.build_context();

                    let popup = create_popup(current_entity, &s, build_context, WidgetState::MainViewState);
                    build_context.append_child(container, popup);
                    self.popup = Some(popup);
                    self.popup_action = None;
                    self.action = None;
                }
                PopupAction::ShowFindings(findings) => {

                    {
                        for x in findings.clone().words {
                            main_view(ctx.widget()).list_mut()
                            .push(x);
                        }
                    }
                    let container = ctx.entity_of_child(MAIN_STACK).unwrap();
                    let current_entity = ctx.entity;
                    let build_context = &mut ctx.build_context();


                    let popup = show_findings(current_entity, &findings, build_context, WidgetState::MainViewState);
                    build_context.append_child(container, popup);
                    self.popup = Some(popup);
                    self.popup_action = None;
                    self.action = None;
                }

                PopupAction::Close => {
                    let container = ctx.entity_of_child(MAIN_STACK).unwrap();
                    ctx.entity = container;
                    ctx.remove_child(self.popup.unwrap());
                    self.popup_action = None;
                }

            }
            self.popup_action = None;
        }
    }

    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.entities.main_stack = ctx.entity_of_child(MAIN_STACK);
        ctx.entity = self.entities.main_stack.unwrap();

        self.entities.query = ctx.entity_of_child("SEARCH_QUERY");
    }

    fn cleanup(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    fn update_post_layout(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}
}

//the different states that you could use to open a popup
pub enum WidgetState {
    MainViewState,
}

#[derive(Clone, Debug)]
pub struct Findings {
    count: u128,
    words: Vec<String>
}
///////// mostly used for error popups ///////////
////creates a popup with a textbox and a button to close it//////
pub fn create_popup(target: Entity, text: &str, build_context: &mut BuildContext, state: WidgetState) -> Entity {
    Popup::new()
        .target(target)
        .open(true)
        .h_align("center")
        .v_align("bottom")
        .width(400.0)
        .height(100.0)
        .child(
            Stack::new()
                .spacing(4.0)
                .child(
                    TextBlock::new()
                        .h_align("center")
                        .clip(false)
                        .max_height(90.0)
                        .v_align("top")
                        //.element("h1")
                        .foreground("#000000")
                        //.element("p")
                        .text(text)
                        .build(build_context),
                )
                .child(
                    TextBlock::new()
                        .h_align("center")
                        .text(".........................................")
                    .build(build_context)
                )
                .child(
                    Button::new()
                        .text("Close")
                        .v_align("bottom")
                        .h_align("center")
                        .max_width(75.0)
                        .on_click(
                            move |states, _| {
                                match state {
                                    WidgetState::MainViewState => {
                                        states.get_mut::<MainViewState>(target).popup_action(PopupAction::Close);
                                    }
                                }
                                true
                            }
                        )
                    .build(build_context)
                )
            .build(build_context),
        )
    .build(build_context)
}

pub fn show_findings(target: Entity, findings: &Findings, ctx: &mut BuildContext, state: WidgetState) -> Entity {
    Popup::new()
        .target(target)
        .open(true)
        .h_align("center")
        .v_align("top")
        // .width(300.0)
        // .height(300.0)
        .child(
            Stack::new()
                .spacing(8.0)
                .child(
                    ItemsWidget::new()
                        .count(findings.words.len())
                        .height(200.0)
                        .width(300.0)
                        .margin(8.0)
                        .h_align("center")
                        .items_builder(
                            move |bc, index| {
                                let fetched = &bc.get_widget(target).clone::<Vec<String>>("list")[index];
                                TextBlock::new()
                                    .text(format!("{}: {}", index, fetched))
                                .build(bc)
                            }
                        )
                    .build(ctx)
                )
                .child(
                    Button::new()
                    .text("Close")
                    .v_align("bottom")
                    .h_align("center")
                    .max_width(80.0)
                    .on_click(
                        move |states, _| {
                            states.get_mut::<MainViewState>(target).popup_action(PopupAction::Close);
                            true
                        }
                    )
                    .build(ctx)
                )
            .build(ctx)
        )
    .build(ctx)
}


fn search(pattern: &str, wordlist: &'static [u8]) -> Result<Vec<(u64, String)>, Box<Error>> {
    let matcher = RegexMatcher::new_line_matcher(pattern)?;
    let mut matches: Vec<(u64, String)> = vec![];
    Searcher::new().search_slice(&matcher, wordlist, UTF8(|lnum, line| {
        // We are guaranteed to find a match, so the unwrap is OK.
        let mymatch = matcher.find(line.as_bytes())?.unwrap();
        matches.push((lnum, line[mymatch].to_string()));
        Ok(true)
    }))?;
    println!("matches:{:?}", matches);
    Ok(matches)
}
