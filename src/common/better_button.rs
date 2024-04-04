use bevy::prelude::*;

const DEFAULT_NORMAL_BUTTON: Color = Color::WHITE;
const DEFAULT_HOVERED_BUTTON: Color = Color::rgb(0.8,0.8,0.8);
const DEFAULT_PRESSED_BUTTON: Color = Color::rgb(0.6,0.6,0.6);

pub struct BetterButtonPlugin;

impl Plugin for BetterButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            update_color_buttons,
            (update_release_buttons, update_release_buttons_with_force_key).chain()
        ));
    }
}

#[derive(Component)]
pub struct ReleaseButton {
    pub just_released: bool,
    previous_interaction: Interaction,
}

#[derive(Component)]
pub struct ReleaseButtonForceKey {
    pub key_code: KeyCode,
}

#[derive(Component)]
pub struct ColorButton {
    pub normal_color: Color,
    pub hovered_color: Color,
    pub pressed_color: Color,
}

impl Default for ReleaseButton {
    fn default() -> Self {
        ReleaseButton {
            just_released: false,
            previous_interaction: Interaction::None
        }
    }
}

impl ReleaseButtonForceKey {
    pub fn new(key_code: KeyCode) -> Self {
        Self {
            key_code
        }
    }
}

impl Default for ColorButton {
    fn default() -> Self {
        ColorButton {
            normal_color: DEFAULT_NORMAL_BUTTON,
            hovered_color: DEFAULT_HOVERED_BUTTON,
            pressed_color: DEFAULT_PRESSED_BUTTON,
        }
    }
}

fn update_release_buttons(
    mut query: Query<(&Interaction, &mut ReleaseButton), With<Button>>
)
{
    for (interaction, mut release_button) in query.iter_mut() {
        if release_button.just_released {
            release_button.just_released = false
        }
        else {
            if let Interaction::Hovered = interaction {
                if let Interaction::Pressed = release_button.previous_interaction {
                    release_button.just_released = true;
                }
            }
        }

        release_button.previous_interaction = *interaction;
    }
}

fn update_release_buttons_with_force_key(
    mut button_query: Query<(&mut ReleaseButton, &ReleaseButtonForceKey)>,
    input: Res<ButtonInput<KeyCode>>,
)
{
    for (mut button, key) in button_query.iter_mut() {
        if input.just_pressed(key.key_code) {
            button.just_released = true;
        }
    }
}

fn update_color_buttons(
    mut interaction_query: Query<
        (
            &ColorButton,
            &Interaction,
            &mut BackgroundColor
        ),
        (Changed<Interaction>, With<Button>)
    >,
)
{
    for (color_button, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = color_button.pressed_color.into();
            }
            Interaction::Hovered => {
                *color = color_button.hovered_color.into();
            }
            Interaction::None => {
                *color = color_button.normal_color.into();
            }
        }
    }
}