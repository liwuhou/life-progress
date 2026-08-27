use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use life_progress_core::{
    get_birthday_time, get_progress_info, load_profile, profile_path, save_profile, Profile,
    ProgressInfo,
};
use serde::{Deserialize, Serialize};

const MIN_ICON_WIDTH_PT: u32 = 22;
const MAX_ICON_WIDTH_PT: u32 = 200;
const ICON_SIZE: u32 = 44;
const ICON_HEIGHT: u32 = 48;
const MONO_FILL: [u8; 4] = [35, 35, 33, 255];
const MONO_TRACK: [u8; 4] = [149, 148, 143, 255];
const THRESHOLD_TRACK: [u8; 4] = [100, 100, 100, 255];
const RED: [u8; 4] = [255, 59, 48, 255];
const ORANGE: [u8; 4] = [255, 149, 0, 255];
const GREEN: [u8; 4] = [76, 217, 100, 255];
const DIMMED: [u8; 4] = [142, 142, 147, 160];

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LifeMetric {
    Remaining,
    Elapsed,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisplayStyle {
    Percent,
    ShortText,
    Custom,
    RingProgress,
    ProgressBar,
    ThresholdBar,
    RainbowThreshold,
    BarChart,
}

impl DisplayStyle {
    pub fn is_graphic(&self) -> bool {
        matches!(
            self,
            Self::RingProgress
                | Self::ProgressBar
                | Self::ThresholdBar
                | Self::RainbowThreshold
                | Self::BarChart
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    Monochrome,
    Threshold,
}

impl Default for ColorMode {
    fn default() -> Self {
        Self::Monochrome
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ThresholdBoundaries {
    #[serde(default = "default_threshold_lower")]
    pub lower: u32,
    #[serde(default = "default_threshold_upper")]
    pub upper: u32,
}

fn default_threshold_lower() -> u32 {
    30
}

fn default_threshold_upper() -> u32 {
    70
}

impl Default for ThresholdBoundaries {
    fn default() -> Self {
        Self {
            lower: default_threshold_lower(),
            upper: default_threshold_upper(),
        }
    }
}

impl ThresholdBoundaries {
    pub fn clamped(&self) -> Self {
        let lower = self.lower.min(99);
        let upper = self.upper.clamp(1, 100);
        if lower < upper {
            return Self { lower, upper };
        }
        let upper = upper.max((lower + 1).min(100));
        Self {
            lower: lower.min(upper - 1),
            upper,
        }
    }
}

fn default_icon_width() -> u32 {
    60
}

fn default_border_radius() -> u32 {
    20
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DesktopPreferences {
    pub metric: LifeMetric,
    pub style: DisplayStyle,
    pub show_title: bool,
    pub title_template: String,
    #[serde(default = "default_icon_width")]
    pub icon_width: u32,
    #[serde(default)]
    pub color_mode: ColorMode,
    #[serde(default)]
    pub threshold_boundaries: ThresholdBoundaries,
    #[serde(default = "default_border_radius")]
    pub border_radius: u32,
}

impl Default for DesktopPreferences {
    fn default() -> Self {
        Self {
            metric: LifeMetric::Remaining,
            style: DisplayStyle::ProgressBar,
            show_title: true,
            title_template: "{mode} {percent}% · {days}天".into(),
            icon_width: default_icon_width(),
            color_mode: ColorMode::Monochrome,
            threshold_boundaries: ThresholdBoundaries::default(),
            border_radius: default_border_radius(),
        }
    }
}
impl DesktopPreferences {
    pub fn normalized(mut self) -> Self {
        self.icon_width = self.icon_width.clamp(MIN_ICON_WIDTH_PT, MAX_ICON_WIDTH_PT);
        self.threshold_boundaries = self.threshold_boundaries.clamped();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DisplayState {
    pub percent: u32,
    pub days: u64,
    pub elapsed_days: u64,
    pub remaining_days: u64,
    pub title: String,
}
fn selected_display_values(
    metric: &LifeMetric,
    progress: &ProgressInfo,
) -> (f64, u64, &'static str) {
    match metric {
        LifeMetric::Remaining => (progress.rest_progress, progress.rest, "余生"),
        LifeMetric::Elapsed => (progress.progress, progress.spent, "已过"),
    }
}

pub fn display_state(profile: &Profile, preferences: &DesktopPreferences) -> Result<DisplayState> {
    let birthday: NaiveDate = get_birthday_time(&profile.birthday)?;
    let progress = get_progress_info(birthday, profile.gender.clone(), Some(&profile.nation))?;
    let (percent, days, mode) = selected_display_values(&preferences.metric, &progress);
    let percent = percent.clamp(0.0, 100.0).round() as u32;
    let title = if preferences.show_title {
        preferences
            .title_template
            .replace("{mode}", mode)
            .replace("{percent}", &percent.to_string())
            .replace("{days}", &days.to_string())
    } else {
        String::new()
    };
    Ok(DisplayState {
        percent,
        days,
        elapsed_days: progress.spent,
        remaining_days: progress.rest,
        title,
    })
}

pub fn render_progress_icon(
    percent: u32,
    style: &DisplayStyle,
    icon_width: u32,
    color_mode: &ColorMode,
    boundaries: &ThresholdBoundaries,
    border_radius: u32,
) -> (Vec<u8>, u32, u32) {
    let percent = percent.min(100);
    let boundaries = boundaries.clamped();
    match style {
        DisplayStyle::RingProgress => {
            let mut rgba = vec![0; (ICON_SIZE * ICON_SIZE * 4) as usize];
            draw_ring(&mut rgba, percent, color_mode, &boundaries);
            (rgba, ICON_SIZE, ICON_SIZE)
        }
        DisplayStyle::BarChart => {
            let mut rgba = vec![0; (ICON_SIZE * ICON_SIZE * 4) as usize];
            draw_bar_chart(&mut rgba, percent, color_mode, &boundaries);
            (rgba, ICON_SIZE, ICON_SIZE)
        }
        DisplayStyle::ProgressBar | DisplayStyle::ThresholdBar | DisplayStyle::RainbowThreshold => {
            let width = icon_width.clamp(MIN_ICON_WIDTH_PT, MAX_ICON_WIDTH_PT) * 2;
            let mut rgba = vec![0; (width * ICON_HEIGHT * 4) as usize];
            draw_horizontal_bar(
                &mut rgba,
                width,
                percent,
                style,
                color_mode,
                &boundaries,
                border_radius,
            );
            (rgba, width, ICON_HEIGHT)
        }
        DisplayStyle::Percent | DisplayStyle::ShortText | DisplayStyle::Custom => {
            (vec![0, 0, 0, 0], 1, 1)
        }
    }
}

fn pixel_offset(width: u32, x: u32, y: u32) -> usize {
    ((y * width + x) * 4) as usize
}

fn put_pixel(rgba: &mut [u8], width: u32, x: u32, y: u32, color: [u8; 4]) {
    let offset = pixel_offset(width, x, y);
    rgba[offset..offset + 4].copy_from_slice(&color);
}

fn rounded_contains(
    x: u32,
    y: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
    radius: u32,
) -> bool {
    if width == 0 || height == 0 || x < left || y < top || x >= left + width || y >= top + height {
        return false;
    }
    let radius = radius.min(width / 2).min(height / 2);
    if radius == 0 {
        return true;
    }
    let right = left + width - 1;
    let bottom = top + height - 1;
    let corner = if x < left + radius && y < top + radius {
        Some((left + radius - 1, top + radius - 1))
    } else if x > right - radius && y < top + radius {
        Some((right - radius + 1, top + radius - 1))
    } else if x < left + radius && y > bottom - radius {
        Some((left + radius - 1, bottom - radius + 1))
    } else if x > right - radius && y > bottom - radius {
        Some((right - radius + 1, bottom - radius + 1))
    } else {
        None
    };
    corner.is_none_or(|(cx, cy)| {
        let dx = x as i64 - cx as i64;
        let dy = y as i64 - cy as i64;
        dx * dx + dy * dy <= (radius as i64) * (radius as i64)
    })
}

fn fill_color(percent: u32, mode: &ColorMode, boundaries: &ThresholdBoundaries) -> [u8; 4] {
    match mode {
        ColorMode::Monochrome => MONO_FILL,
        ColorMode::Threshold => threshold_color(percent, boundaries),
    }
}

fn track_color(mode: &ColorMode) -> [u8; 4] {
    match mode {
        ColorMode::Monochrome => MONO_TRACK,
        ColorMode::Threshold => THRESHOLD_TRACK,
    }
}

fn threshold_color(percent: u32, boundaries: &ThresholdBoundaries) -> [u8; 4] {
    if percent < boundaries.lower {
        RED
    } else if percent <= boundaries.upper {
        ORANGE
    } else {
        GREEN
    }
}

fn rainbow_color(
    relative_x: u32,
    width: u32,
    mode: &ColorMode,
    boundaries: &ThresholdBoundaries,
) -> [u8; 4] {
    if *mode == ColorMode::Monochrome {
        if relative_x < width / 3 {
            MONO_FILL
        } else if relative_x < width * 2 / 3 {
            [80, 80, 78, 220]
        } else {
            MONO_FILL
        }
    } else {
        let red_end = (width * boundaries.lower / 100).min(width);
        let orange_end = (width * boundaries.upper / 100).min(width);
        if relative_x < red_end {
            RED
        } else if relative_x < orange_end {
            ORANGE
        } else {
            GREEN
        }
    }
}

fn draw_horizontal_bar(
    rgba: &mut [u8],
    width: u32,
    percent: u32,
    style: &DisplayStyle,
    mode: &ColorMode,
    boundaries: &ThresholdBoundaries,
    border_radius: u32,
) {
    let bar_x = 2;
    let bar_y = 4;
    let bar_width = width - 4;
    let bar_height = ICON_HEIGHT - 8;
    let fill_width = ((percent as f32 / 100.0) * bar_width as f32).round() as u32;
    let radius = border_radius.min(bar_height / 2);
    let track = track_color(mode);

    for y in bar_y..bar_y + bar_height {
        for x in bar_x..bar_x + bar_width {
            if !rounded_contains(x, y, bar_x, bar_y, bar_width, bar_height, radius) {
                continue;
            }
            let relative_x = x - bar_x;
            let color = if *style == DisplayStyle::RainbowThreshold {
                if relative_x < fill_width {
                    rainbow_color(relative_x, bar_width, mode, boundaries)
                } else {
                    DIMMED
                }
            } else if relative_x < fill_width {
                fill_color(percent, mode, boundaries)
            } else {
                track
            };
            put_pixel(rgba, width, x, y, color);
        }
    }
}

fn draw_ring(rgba: &mut [u8], percent: u32, mode: &ColorMode, boundaries: &ThresholdBoundaries) {
    let cx = ICON_SIZE as f32 / 2.0;
    let cy = ICON_SIZE as f32 / 2.0;
    let outer_r = 20.0;
    let inner_r = 14.0;
    let fill_angle = percent as f32 / 100.0 * std::f32::consts::TAU;
    let fill = fill_color(percent, mode, boundaries);
    let track = track_color(mode);

    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let distance = (dx * dx + dy * dy).sqrt();
            if !(inner_r..=outer_r).contains(&distance) {
                continue;
            }
            let angle = dx.atan2(-dy);
            let normalized = if angle < -std::f32::consts::FRAC_PI_2 {
                angle + std::f32::consts::TAU
            } else {
                angle
            };
            let relative = normalized + std::f32::consts::FRAC_PI_2;
            put_pixel(
                rgba,
                ICON_SIZE,
                x,
                y,
                if relative < fill_angle { fill } else { track },
            );
        }
    }
}

fn draw_bar_chart(
    rgba: &mut [u8],
    percent: u32,
    mode: &ColorMode,
    boundaries: &ThresholdBoundaries,
) {
    let chart_x = 14;
    let chart_width = 16;
    let top_y = 4;
    let baseline_y = ICON_SIZE - 4;
    let chart_height = baseline_y - top_y;
    let fill_top = baseline_y - ((percent as f32 / 100.0) * chart_height as f32).round() as u32;
    let fill = fill_color(percent, mode, boundaries);
    let track = track_color(mode);

    for y in top_y..baseline_y {
        for x in chart_x..chart_x + chart_width {
            put_pixel(
                rgba,
                ICON_SIZE,
                x,
                y,
                if y >= fill_top { fill } else { track },
            );
        }
    }
}

fn preferences_path() -> Result<PathBuf> {
    Ok(profile_path()?
        .parent()
        .context("profile path has no parent")?
        .join("desktop.toml"))
}

pub fn load_preferences() -> Result<DesktopPreferences> {
    let path = preferences_path()?;
    match fs::read_to_string(path) {
        Ok(text) => Ok(toml::from_str::<DesktopPreferences>(&text)?.normalized()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(DesktopPreferences::default()),
        Err(e) => Err(e.into()),
    }
}

pub fn save_preferences(preferences: &DesktopPreferences) -> Result<()> {
    let path = preferences_path()?;
    let preferences = preferences.clone().normalized();
    fs::create_dir_all(path.parent().context("preferences path has no parent")?)?;
    fs::write(path, toml::to_string_pretty(&preferences)?)?;
    Ok(())
}

pub fn load_configured_profile() -> Result<Option<Profile>> {
    load_profile()
}

pub fn is_init_done() -> Result<bool> {
    Ok(load_configured_profile()?.is_some())
}

pub fn is_profile_configured_at(path: &Path) -> Result<bool> {
    Ok(Profile::load_from_path(path)?.is_some())
}

pub fn get_settings() -> Result<(Option<Profile>, DesktopPreferences), String> {
    Ok((
        load_configured_profile().map_err(|e| e.to_string())?,
        load_preferences().map_err(|e| e.to_string())?,
    ))
}

pub fn save_settings(
    birthday: String,
    gender: Option<life_progress_core::Gender>,
    nation: String,
    preferences: DesktopPreferences,
) -> Result<Profile, String> {
    let profile = Profile::new(&birthday, gender, &nation).map_err(|e| e.to_string())?;
    save_profile(&profile).map_err(|e| e.to_string())?;
    save_preferences(&preferences).map_err(|e| e.to_string())?;
    Ok(profile)
}

pub fn search_nations(query: String) -> Result<Vec<String>, String> {
    life_progress_core::search_nation(&query)
        .map(|items| items.into_iter().map(|((name, _), _)| name).collect())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_preferences_get_new_defaults() {
        let prefs: DesktopPreferences = toml::from_str(
            r#"metric = "remaining"
style = "progress_bar"
show_title = true
title_template = "{percent}%"
icon_width = 60
"#,
        )
        .expect("old preferences should remain readable");
        assert_eq!(prefs.color_mode, ColorMode::Monochrome);
        assert_eq!(prefs.threshold_boundaries, ThresholdBoundaries::default());
        assert_eq!(prefs.border_radius, 20);
    }

    #[test]
    fn threshold_boundaries_are_ordered() {
        assert_eq!(
            ThresholdBoundaries {
                lower: 90,
                upper: 10
            }
            .clamped(),
            ThresholdBoundaries {
                lower: 90,
                upper: 91
            }
        );
        assert_eq!(
            ThresholdBoundaries { lower: 0, upper: 0 }.clamped(),
            ThresholdBoundaries { lower: 0, upper: 1 }
        );
    }

    #[test]
    fn rainbow_style_round_trips() {
        let prefs = DesktopPreferences {
            style: DisplayStyle::RainbowThreshold,
            ..DesktopPreferences::default()
        };
        let encoded = toml::to_string(&prefs).expect("preferences should serialize");
        let decoded: DesktopPreferences =
            toml::from_str(&encoded).expect("preferences should deserialize");
        assert_eq!(decoded.style, DisplayStyle::RainbowThreshold);
    }

    #[test]
    fn physical_width_and_transparent_margins_are_stable() {
        let (rgba, width, height) = render_progress_icon(
            50,
            &DisplayStyle::ProgressBar,
            22,
            &ColorMode::Monochrome,
            &ThresholdBoundaries::default(),
            20,
        );
        assert_eq!((width, height), (44, 48));
        assert_eq!(rgba.len(), (44 * 48 * 4) as usize);
        assert_eq!(
            &rgba[pixel_offset(width, 0, 0)..pixel_offset(width, 0, 0) + 4],
            &[0, 0, 0, 0]
        );
        assert_eq!(
            &rgba[pixel_offset(width, 2, 24)..pixel_offset(width, 2, 24) + 4],
            &MONO_FILL
        );
    }

    #[test]
    fn threshold_colors_use_displayed_percent_for_both_metrics() {
        let boundaries = ThresholdBoundaries::default();
        for percent in [29, 30, 70, 71] {
            let expected = match percent {
                29 => RED,
                30 | 70 => ORANGE,
                _ => GREEN,
            };
            assert_eq!(threshold_color(percent, &boundaries), expected);
        }
        for metric in [LifeMetric::Remaining, LifeMetric::Elapsed] {
            let displayed_percent = match metric {
                LifeMetric::Remaining => 86,
                LifeMetric::Elapsed => 86,
            };
            assert_eq!(
                fill_color(displayed_percent, &ColorMode::Threshold, &boundaries),
                GREEN
            );
        }
    }

    #[test]
    fn rainbow_threshold_contains_all_default_bands() {
        let (rgba, width, _) = render_progress_icon(
            100,
            &DisplayStyle::RainbowThreshold,
            60,
            &ColorMode::Threshold,
            &ThresholdBoundaries::default(),
            20,
        );
        assert!(rgba.chunks_exact(4).any(|pixel| pixel == RED));
        assert!(rgba.chunks_exact(4).any(|pixel| pixel == ORANGE));
        assert!(rgba.chunks_exact(4).any(|pixel| pixel == GREEN));
        assert_eq!(width, 120);
    }

    #[test]
    fn invalid_persisted_preferences_are_normalized() {
        let prefs: DesktopPreferences = toml::from_str::<DesktopPreferences>(
            r#"metric = "remaining"
style = "progress_bar"
show_title = true
title_template = "{percent}%"
icon_width = 999

[threshold_boundaries]
lower = 90
upper = 10
"#,
        )
        .expect("invalid persisted preferences should still parse")
        .normalized();
        assert_eq!(prefs.icon_width, 200);
        assert_eq!(
            prefs.threshold_boundaries,
            ThresholdBoundaries {
                lower: 90,
                upper: 91
            }
        );
    }
    #[test]
    fn selected_metric_uses_matching_percent_and_days() {
        let progress = ProgressInfo {
            spent: 12_345,
            progress: 42.0,
            rest: 17_890,
            rest_progress: 58.0,
        };
        assert_eq!(
            selected_display_values(&LifeMetric::Remaining, &progress),
            (58.0, 17_890, "余生")
        );
        assert_eq!(
            selected_display_values(&LifeMetric::Elapsed, &progress),
            (42.0, 12_345, "已过")
        );
    }
}
