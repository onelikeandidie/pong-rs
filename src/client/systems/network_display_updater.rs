use bevy::prelude::*;

use crate::{shared::network::udp::ClientSocket, client::components::network_displays::{PingDisplay, ConnectionDisplay, PacketsDisplay}};

pub struct NetworkDisplayPlugin;

impl Plugin for NetworkDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(network_ui_setup)
            .add_system(ping_display_updater)
            .add_system(connection_display_updater)
            .add_system(packets_display_updater)
        ;
    }
}

pub fn packets_display_updater(socket: Res<ClientSocket>, mut query: Query<&mut Text, With<PacketsDisplay>>) {
    for mut display in query.iter_mut() {
        let packets_sent = socket.network.packets.sent;
        let packets_recieved = socket.network.packets.recieved;
        let last_packets = socket.network.last_packets.sent;
        display.sections[1].value = packets_sent.to_string();
        display.sections[3].value = packets_recieved.to_string();
        if last_packets.0 != 0 && last_packets.1 > last_packets.0 {
            let rate_fixed = (socket.network.rates.sent * 100.0).ceil() / 100.0;
            display.sections[5].value = rate_fixed.to_string();
            let rate_fixed = (socket.network.rates.recieved * 100.0).ceil() / 100.0;
            display.sections[7].value = rate_fixed.to_string();
        } else {
            display.sections[5].value = "no packets".to_string();
            display.sections[7].value = "no packets".to_string();
        }
    }
}

pub fn ping_display_updater(socket: Res<ClientSocket>, mut query: Query<&mut Text, With<PingDisplay>>) {
    if socket.network.last_pong >= socket.network.last_ping {
        let ping: u128 = socket.network.last_pong - socket.network.last_ping;
        for mut display in query.iter_mut() {
            display.sections[0].value = ping.to_string();
        }
    };
}

pub fn connection_display_updater(socket: Res<ClientSocket>, mut query: Query<&mut Text, With<ConnectionDisplay>>) {
    for mut display in query.iter_mut() {
        display.sections[0].value = if socket.network.connected { "o" } else { "x" }.to_string();
    }
}

pub fn network_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(32.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        }).with_children(|parent| {
            let styles = TextStyle {
                font_size: 30.0,
                font: asset_server.load("fonts/JetBrainsMono-Medium.ttf"),
                color: Color::WHITE,
                ..default()
            };
            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "ping_placeholder".to_string(),
                        style: styles.clone()
                    }, TextSection {
                        value: "ms".to_string(),
                        style: styles
                    }],
                    ..default()
                },
                ..default()
            }).insert(PingDisplay { ping: 0 });
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(32.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(32.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        }).with_children(|parent| {
            let styles = TextStyle {
                font_size: 30.0,
                font: asset_server.load("fonts/JetBrainsMono-Medium.ttf"),
                color: Color::WHITE,
                ..default()
            };
            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "connection_placeholder".to_string(),
                        style: styles
                    }],
                    ..default()
                },
                ..default()
            }).insert(ConnectionDisplay);
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(32.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(64.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        }).with_children(|parent| {
            let styles = TextStyle {
                font_size: 22.0,
                font: asset_server.load("fonts/JetBrainsMono-Medium.ttf"),
                color: Color::WHITE,
                ..default()
            };
            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "packets: ".to_string(),
                        style: styles.clone()
                    },TextSection {
                        value: "packet_placeholder".to_string(),
                        style: styles.clone()
                    }, TextSection {
                        value: "s".to_string(),
                        style: styles.clone()
                    }, TextSection {
                        value: "packet_placeholder".to_string(),
                        style: styles.clone()
                    }, TextSection {
                        value: "r ".to_string(),
                        style: styles.clone()
                    }, TextSection {
                        value: "packet_rate_placeholder".to_string(),
                        style: styles.clone()
                    }, TextSection {
                        value: "/s".to_string(),
                        style: styles.clone()
                    }, TextSection {
                        value: "packet_rate_placeholder".to_string(),
                        style: styles.clone()
                    }, TextSection {
                        value: "/r".to_string(),
                        style: styles.clone()
                    }, TextSection {
                        value: "/s".to_string(),
                        style: styles
                    }],
                    ..default()
                },
                ..default()
            }).insert(PacketsDisplay);
        });
    });
}