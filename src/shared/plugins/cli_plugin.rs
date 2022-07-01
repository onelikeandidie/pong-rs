use bevy::prelude::Plugin;

use crate::shared::resources::cli_args::CliArgs;

pub struct CliPlugin;

impl Plugin for CliPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let cli_args = CliArgs::extract_args();
        println!("{:?}", cli_args);
        app.insert_resource(cli_args);
    }
}