use egui::{CentralPanel, Context, DragValue};

use crate::{algs::QuickFindUF, AppData};

pub fn show(ctx: &Context, app_data: &mut AppData) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Quick Find");
        ui.horizontal(|ui| {
            ui.label("Number of elements:");
            ui.add(DragValue::new(&mut app_data.n).speed(0.1));
        });

        if ui.button("Go").clicked() {
            app_data.uf = Some(QuickFindUF::new(app_data.n));
        }

        if let Some(uf) = &mut app_data.uf {
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("p:");
                ui.add(DragValue::new(&mut app_data.p_union).speed(0.1));

                ui.label("q:");
                ui.add(DragValue::new(&mut app_data.q_union).speed(0.1));

                if ui.button("Union").clicked() {
                    uf.union(app_data.p_union, app_data.q_union);
                };

                ui.separator();

                ui.label("p:");
                ui.add(DragValue::new(&mut app_data.p_connected).speed(0.1));

                ui.label("q:");
                ui.add(DragValue::new(&mut app_data.q_connected).speed(0.1));

                if ui.button("Connected").clicked() {
                    match uf.connected(app_data.p_connected, app_data.q_connected) {
                        true => app_data.connected_text = Some("Yes!".to_owned()),
                        false => app_data.connected_text = Some("No!".to_owned()),
                    };
                };

                if let Some(text) = &app_data.connected_text {
                    ui.label(text);
                }
            });

            ui.separator();
            for (i, id) in uf.ids.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(i.to_string());
                    ui.label(id.to_string());
                });
            }
        }
    });
}
