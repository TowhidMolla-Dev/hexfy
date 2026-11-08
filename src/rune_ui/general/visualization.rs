//! Dictation HUD visualization controls in General.

use rune::El;

use super::search::Rows;
use super::{PANEL, pick_row};
use crate::app_settings::{IndicatorPosition, IndicatorVisualization};
use crate::desktop::i18n::tr;
use crate::rune_ui::Ui;
use crate::rune_ui::controls::Paint;

pub(super) fn rows(ui: &Ui, p: Paint, layer: &mut Vec<El>, rows: &mut Rows<'_>) {
    let settings = &ui.settings;
    if settings.labs.edge_glow {
        rows.push(
            tr("Visualization"),
            pick_row(
                ui,
                p,
                "indicator-visualization",
                &PANEL,
                tr("Visualization"),
                IndicatorVisualization::ALL
                    .into_iter()
                    .map(|visualization| tr(visualization.label()))
                    .collect(),
                settings.indicator_visualization as usize,
                |ui, index| {
                    let visualization = IndicatorVisualization::ALL[index];
                    if ui.settings.indicator_visualization != visualization {
                        ui.update(|settings| {
                            settings.indicator_visualization = visualization;
                            if visualization == IndicatorVisualization::Pill
                                && settings.indicator_position == IndicatorPosition::Hidden
                            {
                                settings.indicator_position = IndicatorPosition::Top;
                            }
                        });
                    }
                },
                layer,
            ),
        );
        if settings.indicator_visualization == IndicatorVisualization::Pill {
            rows.push(
                tr("Pill position"),
                pick_row(
                    ui,
                    p,
                    "indicator-pill-position",
                    &PANEL,
                    tr("Pill position"),
                    vec![tr("Top"), tr("Bottom")],
                    usize::from(settings.indicator_position == IndicatorPosition::Bottom),
                    |ui, index| {
                        let position = if index == 0 {
                            IndicatorPosition::Top
                        } else {
                            IndicatorPosition::Bottom
                        };
                        if ui.settings.indicator_position != position {
                            ui.update(|settings| settings.indicator_position = position);
                        }
                    },
                    layer,
                ),
            );
        }
    } else {
        rows.push(
            tr("Dictation HUD"),
            pick_row(
                ui,
                p,
                "indicator",
                &PANEL,
                tr("Dictation HUD"),
                vec![tr("Top"), tr("Bottom"), tr("Off")],
                match settings.indicator_position {
                    IndicatorPosition::Top => 0,
                    IndicatorPosition::Bottom => 1,
                    IndicatorPosition::Hidden => 2,
                },
                |ui, index| {
                    let position = [
                        IndicatorPosition::Top,
                        IndicatorPosition::Bottom,
                        IndicatorPosition::Hidden,
                    ][index];
                    if ui.settings.indicator_position != position {
                        ui.update(|settings| settings.indicator_position = position);
                    }
                },
                layer,
            ),
        );
    }
}
