use std::any::Any;
use std::ops::{Deref, Add};
use once_cell::sync::Lazy;
use stylist::yew::styled_component;
use yew::html::ImplicitClone;
use yew::prelude::*;
use yew::virtual_dom::VChild;

use crate::{prefers_dark_scheme, ProviderProps};

#[derive(Debug, Clone, PartialEq)]
pub enum ThemeKind {
    Dark,
    Light,
}

impl ImplicitClone for ThemeKind {}

impl ThemeKind {
    pub fn current(&self) -> &Theme {
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            kind: ThemeKind::Light,
            font_color: "black".to_string(),
            content_background_color: "rgb(237, 244, 255)".to_string(),
            default_background_color: "#D5D5D5".to_string(),
            toolbar_background_color: "rgb(225, 129, 17)".to_string(),
            icon_background_color: "#24F".to_string(),
            other_background_color: "white".to_string(),
            hover_color: "#444".to_string(),
            icon_name: "sun_icon".to_string(),
            link_color: "#5a2ab9".to_string(),
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            kind: ThemeKind::Dark,
            font_color: "white".to_string(),
            content_background_color: "#101014".to_string(),
            default_background_color: "#151515".to_string(),
            toolbar_background_color: "rgb(175, 100, 10)".to_string(),
            icon_background_color: "#235".to_string(),
            other_background_color: "rgb(50, 50, 50)".to_string(),
            hover_color: "#AAA".to_string(),
            icon_name: "moon_icon".to_string(),
            link_color: "#AB90FF".to_string(),
        });

        match self {
            ThemeKind::Dark => &DARK_THEME,
            ThemeKind::Light => &LIGHT_THEME,
        }
    }

    pub(crate) fn elements_vec() -> Vec<ThemeKind> {
        vec![ThemeKind::Dark, ThemeKind::Light]
    }

    /**
     * "class" -> "class class--theme"
     */
    pub fn css_class_themed(&self, class_name: &str) -> String {
        let mut themed_class: String = String::new();
        themed_class.push_str(class_name);
        themed_class.push(' ');
        themed_class.push_str(class_name);
        themed_class.push_str(match self {
            ThemeKind::Dark => "--dark",
            ThemeKind::Light => "--light",
        });
        themed_class
    }

}

#[derive(Debug, Clone)]
pub struct Theme {
    pub kind: ThemeKind,
    pub font_color: String,
    pub content_background_color: String,
    pub default_background_color: String,
    pub toolbar_background_color: String,
    pub icon_background_color: String,
    pub other_background_color: String,
    pub hover_color: String,
    pub icon_name: String,
    pub link_color: String,
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
 * The sun and moon button that cycles between themes.
 */
#[styled_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme: ThemeContext = use_theme();

    let icon_name = theme.icon_name.clone();
    let background_color = theme.icon_background_color.clone();

    let theme_vec: Vec<ThemeKind> = ThemeKind::elements_vec();
    let theme_ind: usize = theme_vec.iter().position(|t| t == &theme.kind()).unwrap();
    let switch_theme = Callback::from(move |_| theme.set(
        ThemeKind::elements_vec().remove((theme_ind + 1) % theme_vec.len())
    ));

    html! {
        <div class={css!( r#" display: inline; "#)}>
            <button class={css!(
             r#"
                height: 37px;
                width: 37px;
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
                top: -20px;
                right: 10px;
                vertical-align: middle;
                margin-right: 20px;
                &:hover {
                    transform: rotate(-10deg);
                    background-color: ${bg_c};
                }
            "#, i = icon_name, bg_c = background_color
        )} onclick={switch_theme}></button>
        </div>
    }
}
