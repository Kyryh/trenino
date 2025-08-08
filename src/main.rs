use std::{mem, ops::Deref, rc::Rc};

use dioxus::{logger::tracing, prelude::*};

const TRAIN_SPRITE: Asset = asset!("/assets/train.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn Vagon(img: Option<String>) -> Element {
    match img {
        Some(img) => rsx! {
            img {
                src: img,
                max_width: "100%",
                max_height: "100%",
                display: "block",
                margin_left: "auto",
                margin_right: "auto",
            }
        },
        None => rsx! {},
    }
}

#[derive(Clone, Debug)]
enum TrainState {
    Still,
    Going,
    Returning,
}

impl TrainState {
    fn get_transform(&self) -> &'static str {
        match self {
            Self::Still => "",
            Self::Going => "scaleX(-1)",
            Self::Returning => "",
        }
    }

    fn get_left_offset(&self) -> &'static str {
        match self {
            Self::Still => "0",
            Self::Going => "150vw",
            Self::Returning => "0",
        }
    }

    fn button_enabled(&self) -> bool {
        matches!(self, Self::Still)
    }
}

#[component]
fn Train() -> Element {
    let mut vagon_img = use_signal(|| {
        Some("https://images.unsplash.com/photo-1526779259212-939e64788e3c".to_owned())
    });

    let mut train_state = use_signal(|| TrainState::Still);
    rsx! {
        "{train_state:?}",
        div {
            display: "inline-block",
            position: "relative",
            ontransitionend: move |evt| {
                // hacky as fuck but dioxus doesn't offer ANY way to get
                // the data of an event so whatever

                // SAFETY: TransitionData has a single field, so it has the
                // same layout of its field of type Box<dyn HasTransitionData>
                let evt = unsafe {
                    mem::transmute::<_, Box<dyn HasTransitionData>>(Rc::into_inner(evt.data).unwrap())
                };

                if evt.property_name() == "left" {
                    match train_state() {
                        TrainState::Going => train_state.set(TrainState::Returning),
                        TrainState::Returning => train_state.set(TrainState::Still),
                        TrainState::Still => unreachable!(),
                    }
                }

            },
            transform: train_state().get_transform(),
            left: train_state().get_left_offset(),
            transition_property: "transform, left",
            transition_duration: "0.5s, 5s",
            img {
                src: TRAIN_SPRITE,
                max_width: "100%",
            }
            div {
                position: "absolute",
                background_color: "yellow",
                width: "50%",
                height: "70%",
                top: "0px",
                left: "45%",
                z_index: -1,
                Vagon { img: vagon_img() }
            }
        }
        button {
            font_size: "24px",
            padding: "20px",
            margin_top: "20px",
            onclick: move |_| train_state.set(TrainState::Going),
            disabled: !train_state().button_enabled(),
            "🚂 Parti",
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            overflow: "hidden",
            Train {  }

        }
    }
}
