use std::any::Any;
use std::ops::Deref;

use once_cell::sync::Lazy;
use stylist::yew::styled_component;
use yew::html::ImplicitClone;
use yew::prelude::*;
use yew::virtual_dom::VChild;
use crate::{prefers_dark_scheme, ProviderProps};
// use strum_macros::EnumIter; // 0.17.1 TODO

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ThemeKind {
    Dark,
    Light,
}

impl ImplicitClone for ThemeKind {}

impl ThemeKind {
    pub fn current(&self) -> &Theme {
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "black".to_string(),
            content_background_color: "rgb(237, 244, 255)".to_string(),
            default_background_color: "#D5D5D5".to_string(),
            toolbar_background_color: "rgb(180, 190, 200)".to_string(),
            icon_background_color: "#235".to_string(),
            other_background_color: "white".to_string(),
            hover_color: "#444".to_string(),
            icon_name: "moon_icon".to_string(),
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "white".to_string(),
            content_background_color: "#101014".to_string(),
            default_background_color: "#151515".to_string(),
            toolbar_background_color: "rgb(80, 90, 100)".to_string(),
            icon_background_color: "#24F".to_string(),
            other_background_color: "rgb(50, 50, 50)".to_string(),
            hover_color: "#AAA".to_string(),
            icon_name: "sun_icon".to_string(),
        });

        match self {
            ThemeKind::Dark => &DARK_THEME,
            ThemeKind::Light => &LIGHT_THEME,
        }
    }

    pub(crate) fn elements_vec() -> Vec<ThemeKind> {
        vec![ThemeKind::Dark, ThemeKind::Light]
    }

}

#[derive(Debug, Clone)]
pub(crate) struct Theme {
    pub font_color: String,
    pub content_background_color: String,
    pub default_background_color: String,
    pub toolbar_background_color: String,
    pub icon_background_color: String,
    pub other_background_color: String,
    pub hover_color: String,
    pub icon_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeContext {
    inner: UseStateHandle<ThemeKind>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
        Self { inner }
    }

    pub fn set(&self, kind: ThemeKind) {
        self.inner.set(kind)
    }

    pub fn kind(&self) -> ThemeKind {
        (*self.inner).clone()
    }

}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        &*self.inner.current()
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[styled_component(ThemeProvider)]
pub(crate) fn theme_provider(props: &ProviderProps) -> Html {
    let theme_context = ThemeContext::new(use_state(|| if prefers_dark_scheme() {ThemeKind::Dark} else {ThemeKind::Light}));

    html! {
        <ContextProvider<ThemeContext> context={theme_context}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub(crate) fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}


// struct UpdatedProps {
//     // child: VChild<dyn Component<Message=dyn Any, Properties=dyn UpdateResponder>>,
//     // z: ChildrenWithProps<dyn Component>,
// }

// /**
//  * to be wrapped around component to update when theme changes
//  */
// #[styled_component(ThemeUpdated)]
// pub(crate) fn theme_updater(props: &UpdatedProps) -> Html {
//     // let theme_context: ThemeContext = use_context();
//     // props.z.into_iter().map(|p: Component| { p.; p });
//     // props.child.props.as_ref().theme_update(theme_context.inner);
//     html! {
//         <>
//             // props.child.clone()
//         </>
//     }
// }

/**
 * the sun and moon button that cycles between themes.
 */
#[styled_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme: ThemeContext = use_theme();

    let icon_name = theme.icon_name.clone();
    let background_color = theme.icon_background_color.clone();
    // TODO??? let rotate = if theme.kind().clone() == ThemeKind::Dark {"20".to_string()} else {"0".to_string()};

    let theme_vec: Vec<ThemeKind> = ThemeKind::elements_vec();
    let theme_ind: usize = theme_vec.iter().position(|t| t == &theme.kind()).unwrap();
    let switch_theme = Callback::from(move |_| theme.set(
        ThemeKind::elements_vec().remove((theme_ind + 1) % theme_vec.len())
    ));

    // TODO transition: color .2s ease-in-out, box-shadow .3s ease-in-out, rotate;
    // , r = rotate
    html! {
        <div>
            <button class={css!(
             r#"
                height: 40px;
                width: 40px;
                background-size: 30px;
                background-repeat: no-repeat;
                border-radius: 50%;
                background-position: center;
                border: dashed lightslategray 0px;
                background-image: url("/assets/${i}.png");
                transition: all .25s ease-in-out;
                transform: rotate(20deg);
                color: transparent;
                background-color: transparent;
                position: absolute;
                top: 0px;
                right: 10px;
                &:hover {
                    transform: rotate(-10deg);
                    background-color: ${bg_c};
                }
            "#, i = icon_name, bg_c = background_color
        )} onclick={switch_theme}></button>
        </div>
    }
}
