use std::f32::consts::PI;
use bevy::prelude::*;

mod darts;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (system, update_config))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // text
    commands.spawn(TextBundle::from_section(
        "Hold 'Left' or 'Right' to change the line width",
        TextStyle {
            font: asset_server.load("fonts/JetBrainsMonoNerdFontMono-Bold.ttf"),
            font_size: 24.,
            color: Color::WHITE,
        },
    ));

}

fn draw_dart_board(gizmos: &mut Gizmos){
    gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
    gizmos.circle_2d(Vec2::ZERO, 20., Color::BLACK);



    let mut angle: f32 = 0.;


    for _number in 0..20 {
        let x = angle.cos() * 120.;
        let y = angle.sin() * 120.;
        gizmos.line_2d(Vec2::ZERO, Vec2::new(x, y), Color::BLACK);

        angle += PI / 10.;
    }

}

fn system(mut gizmos: Gizmos, _time: Res<Time>) {
    draw_dart_board(&mut gizmos);
}

fn update_config(mut config: ResMut<GizmoConfig>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    if keyboard.pressed(KeyCode::Right) {
        config.line_width += 5. * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        config.line_width -= 5. * time.delta_seconds();
    }
}
