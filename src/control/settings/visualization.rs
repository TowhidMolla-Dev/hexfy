//! Validation for the Labs-gated dictation HUD visualization setting.

use super::{ControlError, invalid};
use crate::app_settings::{AppSettings, IndicatorVisualization};

pub(super) fn validate(name: &str, settings: &AppSettings) -> Result<(), ControlError> {
    if name == "indicator_visualization"
        && settings.indicator_visualization == IndicatorVisualization::EdgeGlow
        && !settings.labs.edge_glow
    {
        return Err(invalid(
            name,
            "Edge glow requires Labs › Edge glow: hex labs on edge-glow",
        ));
    }
    Ok(())
}
