#![allow(unused)]

use bevy::prelude::*;
use std::str::FromStr;
use bevy_ui::prelude::*;

use crate::components::button::{InteractiveState, ButtonStyle};


/* -------- DISPLAY -------- */

pub struct ColorResources { 
    pub bg_primary: Color, 
    pub bg_secondary: Color,

    pub outline_primary: Color, 
    pub outline_secondary: Color,
    pub outline_tint: Color,

    pub text_heading: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    
    pub status_success: Color,
    pub status_warning: Color,
    pub status_danger: Color,
}

impl Default for ColorResources {
    fn default() -> Self {
        ColorResources {
            bg_primary: Colors::tapa().shade1000,
            bg_secondary: Colors::tapa().shade950,
            
            outline_primary: Colors::tapa().shade0,
            outline_secondary: Colors::tapa().shade700,
            outline_tint: Colors::transparent().shade700,
            
            text_heading: Colors::tapa().shade0,
            text_primary: Colors::tapa().shade100,
            text_secondary: Colors::tapa().shade300,

            status_success: Colors::malachite().shade500,
            status_warning: Colors::lightning().shade500,
            status_danger: Colors::torch_red().shade500,
        }
    }
}


/* -------- ICONS -------- */

#[derive(Copy, Clone)]
pub struct IconColor {
    pub color: Color,
}

impl IconColor {
    pub fn new(state: InteractiveState) -> Self {
        match state {
            InteractiveState::Default => IconColor {
                color: Colors::tapa().shade0,
            },
            InteractiveState::Disabled => IconColor {
                color: Colors::tapa().shade700,
            },
            InteractiveState::Hover => IconColor {
                color: Colors::tapa().shade200,
            },
            InteractiveState::Selected => IconColor {
                color: Colors::tapa().shade0,
            },
        }
    }
}

/* -------- INTERACTIVE -------- */

#[derive(Copy, Clone)]
pub struct ButtonColor {
    pub background: Color,
    pub label: Color,
    pub outline: Color,
}

impl ButtonColor {
    pub fn new(style: ButtonStyle, state: InteractiveState) -> Self {
        match (style, state) {
            // Primary button styles
            (ButtonStyle::Primary, InteractiveState::Default) => ButtonColor {
                background: Colors::torch_red().shade500,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            (ButtonStyle::Primary, InteractiveState::Hover) => ButtonColor {
                background: Colors::torch_red().shade600,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            (ButtonStyle::Primary, InteractiveState::Selected) => ButtonColor {
                background: Colors::torch_red().shade700,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            (ButtonStyle::Primary, InteractiveState::Disabled) => ButtonColor {
                background: Colors::tapa().shade500,
                label: Colors::tapa().shade1000,
                outline: Colors::transparent().shade0,
            },

            // Secondary button styles
            (ButtonStyle::Secondary, InteractiveState::Default) => ButtonColor {
                background: Colors::transparent().shade0,
                label: Colors::tapa().shade0,
                outline: Colors::tapa().shade700,
            },
            (ButtonStyle::Secondary, InteractiveState::Hover) => ButtonColor {
                background: Colors::tapa().shade950,
                label: Colors::tapa().shade0,
                outline: Colors::tapa().shade700,
            },
            (ButtonStyle::Secondary, InteractiveState::Selected) => ButtonColor {
                background: Colors::transparent().shade0,
                label: Colors::tapa().shade0,
                outline: Colors::tapa().shade700,
            },
            (ButtonStyle::Secondary, InteractiveState::Disabled) => ButtonColor {
                background: Colors::tapa().shade500,
                label: Colors::tapa().shade1000,
                outline: Colors::tapa().shade700,
            },

            // Ghost button styles
            (ButtonStyle::Ghost, InteractiveState::Default) => ButtonColor {
                background: Colors::transparent().shade0,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            (ButtonStyle::Ghost, InteractiveState::Hover) => ButtonColor {
                background: Colors::tapa().shade950,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            (ButtonStyle::Ghost, InteractiveState::Selected) => ButtonColor {
                background: Colors::tapa().shade950,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            (ButtonStyle::Ghost, InteractiveState::Disabled) => ButtonColor {
                background: Colors::transparent().shade0,
                label: Colors::tapa().shade500,
                outline: Colors::transparent().shade0,
            },
        }
    }
}

/* -------- SHADES -------- */

pub struct Colors {
    pub shade0: Color,
    pub shade50: Color,
    pub shade100: Color,
    pub shade200: Color,
    pub shade300: Color,
    pub shade400: Color,
    pub shade500: Color,
    pub shade600: Color,
    pub shade700: Color,
    pub shade800: Color,
    pub shade900: Color,
    pub shade950: Color,
    pub shade1000: Color,
}

impl Colors {
    pub fn tapa() -> Self {
        Colors {
            shade0: hex("ffffff"),
            shade50: hex("f4f3f2"),
            shade100: hex("e2e1df"),
            shade200: hex("c7c4c1"),
            shade300: hex("a7a29d"),
            shade400: hex("8e8781"),
            shade500: hex("78716c"),
            shade600: hex("6d6561"),
            shade700: hex("585250"),
            shade800: hex("4d4846"),
            shade900: hex("443f3f"),
            shade950: hex("262322"),
            shade1000: hex("000000"),
        }
    }

    pub fn torch_red() -> Self {
        Colors {
            shade0: hex("ffffff"),
            shade50: hex("fef2f2"),
            shade100: hex("fee2e3"),
            shade200: hex("fdcbcd"),
            shade300: hex("fba6a9"),
            shade400: hex("f67377"),
            shade500: hex("eb343a"),
            shade600: hex("da282e"),
            shade700: hex("b71e23"),
            shade800: hex("971d21"),
            shade900: hex("7e1e21"),
            shade950: hex("440b0d"),
            shade1000: hex("000000"),
        }
    }

    pub fn malachite() -> Self {
        Colors {
            shade0: hex("ffffff"),
            shade50: hex("f1fcf2"),
            shade100: hex("dff9e4"),
            shade200: hex("c0f2ca"),
            shade300: hex("8fe6a1"),
            shade400: hex("57d171"),
            shade500: hex("3ccb5a"),
            shade600: hex("239631"),
            shade700: hex("1f7631"),
            shade800: hex("1d5e2c"),
            shade900: hex("1a4d26"),
            shade950: hex("092a12"),
            shade1000: hex("000000"),
        }
    }

    pub fn lightning() -> Self {
        Colors {
            shade0: hex("ffffff"),
            shade50: hex("fffdeb"),
            shade100: hex("fefac7"),
            shade200: hex("fdf48a"),
            shade300: hex("fce94d"),
            shade400: hex("fbd924"),
            shade500: hex("f5bd14"),
            shade600: hex("d99106"),
            shade700: hex("b46809"),
            shade800: hex("92500e"),
            shade900: hex("78420f"),
            shade950: hex("452203"),
            shade1000: hex("000000"),
        }
    }

    pub fn transparent() -> Self {
        Colors {
            shade0: hex_transparent("ffffff", 0.),
            shade50: hex_transparent("ffffff", 0.),
            shade100: hex_transparent("ffffff", 25.),
            shade200: hex_transparent("ffffff", 50.),
            shade300: hex_transparent("ffffff", 75.),
            shade400: hex_transparent("ffffff", 100.),
            shade500: hex_transparent("ffffff", 125.),
            shade600: hex_transparent("ffffff", 150.),
            shade700: hex_transparent("ffffff", 175.),
            shade800: hex_transparent("ffffff", 200.),
            shade900: hex_transparent("ffffff", 225.),
            shade950: hex_transparent("ffffff", 225.),
            shade1000: hex_transparent("ffffff", 225.),
        }
    }

}

pub fn hex(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

pub fn hex_transparent(hex: &str, a: f32) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = f32::from_str(&hex[0..2]).unwrap_or(0.);
    let g = f32::from_str(&hex[2..4]).unwrap_or(0.);
    let b = f32::from_str(&hex[4..6]).unwrap_or(0.);
    Color::srgba(r, g, b, a)
}
