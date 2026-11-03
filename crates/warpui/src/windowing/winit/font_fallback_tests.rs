use super::*;
use crate::elements::DEFAULT_UI_LINE_HEIGHT_RATIO;
use crate::fonts::{Properties, init_fonts};
use crate::platform::FontDB as _;
use crate::text_layout::{DEFAULT_TOP_BOTTOM_RATIO, TextStyle};

#[test]
fn windows_chinese_ui_text_uses_registered_fallback_glyphs() -> Result<()> {
    let (font_db, latin_family) = init_fonts();
    let text = "设置 字体 主题 隐私 English";
    let line = font_db.text_layout_system().layout_line(
        text,
        LineStyle {
            font_size: 16.,
            line_height_ratio: DEFAULT_UI_LINE_HEIGHT_RATIO,
            baseline_ratio: DEFAULT_TOP_BOTTOM_RATIO,
            fixed_width_tab_size: None,
        },
        &[(
            0..text.chars().count(),
            StyleAndFont::new(latin_family, Properties::default(), TextStyle::new()),
        )],
        f32::MAX,
        ClipConfig::default(),
    );

    assert!(line.width > 0.);
    assert!(
        line.chars_with_missing_glyphs.is_empty(),
        "Chinese UI glyphs are missing: {:?}",
        line.chars_with_missing_glyphs
    );
    assert!(
        line.runs
            .iter()
            .flat_map(|run| &run.glyphs)
            .all(|glyph| glyph.id != 0)
    );

    let characters: Vec<_> = text.chars().collect();
    for run in &line.runs {
        for glyph in &run.glyphs {
            if characters
                .get(glyph.index)
                .is_some_and(|character| character.is_whitespace())
            {
                continue;
            }
            font_db.glyph_raster_bounds(
                run.font_id,
                16.,
                glyph.id,
                vec2f(1., 1.),
                &GlyphConfig::default(),
            )?;
        }
    }
    Ok(())
}
