use poise::serenity_prelude as serenity;

use crate::utils;

pub fn interaction_create(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    interaction: &serenity::Interaction,
) {
    match interaction.kind() {
        serenity::InteractionType::Autocomplete => {
            utils::logger::info("Autocomplete Interaction created");
        }
        serenity::InteractionType::Command => {
            if let serenity::Interaction::Command(command) = interaction {
                let command_name = &command.data.name;
                utils::logger::info(
                    format!("Command Interaction created with name: {}", command_name).as_str(),
                );
            }
        }
        serenity::InteractionType::Modal => {
            if let serenity::Interaction::Modal(modal) = interaction {
                let custom_id = &modal.data.custom_id;
                utils::logger::info(
                    format!("Modal Interaction created with custom ID: {}", custom_id).as_str(),
                );
            }
        }
        // serenity::InteractionType::Ping => "Ping",
        _ => {}
    };
}
