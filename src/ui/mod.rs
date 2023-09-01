use egui::{CentralPanel, Context, DragValue};

use crate::{
    algs::{UnionFind, UnionFindType},
    AppData,
};

pub fn show(ctx: &Context, app_data: &mut AppData) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Quick Find");
        let mut selected_type = app_data.union_find_type.clone();
        ui.horizontal(|ui| {
            if ui
                .selectable_value(
                    &mut selected_type,
                    UnionFindType::QuickFind,
                    UnionFindType::QuickFind.name(),
                )
                .clicked()
                || ui
                    .selectable_value(
                        &mut selected_type,
                        UnionFindType::QuickUnion,
                        UnionFindType::QuickUnion.name(),
                    )
                    .clicked()
            {
                app_data.union_find_type = selected_type;
                app_data.uf = None;
            };
        });

        ui.horizontal(|ui| {
            ui.label("Number of elements:");
            ui.add(DragValue::new(&mut app_data.n).speed(0.1));
        });

        if ui.button("Go").clicked() {
            app_data.uf = Some(app_data.union_find_type.get_alg(app_data.n));
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
            for (i, id) in uf.ids().iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(i.to_string());
                    ui.label(id.to_string());
                });
            }
        }
    });
}
