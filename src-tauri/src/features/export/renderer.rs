use super::{
    error::ExportError,
    model::{PngBackground, PngMetadata, PngOptions},
};
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::{self, fontdb},
};

const MAX_DIMENSION: u32 = 16_384;
const MAX_PIXELS: u64 = 64_000_000;

pub struct RenderedPng {
    pub metadata: PngMetadata,
    pub rgba: Vec<u8>,
    pub encoded: Vec<u8>,
}

pub fn inspect(svg: &str, options: PngOptions) -> Result<PngMetadata, ExportError> {
    let tree = parse_svg(svg)?;
    dimensions(&tree, options.scale)
}

pub fn render(svg: &str, options: PngOptions) -> Result<RenderedPng, ExportError> {
    let tree = parse_svg(svg)?;
    let metadata = dimensions(&tree, options.scale)?;
    let mut pixmap = Pixmap::new(metadata.width, metadata.height).ok_or_else(|| {
        ExportError::new(
            "image_allocation_failed",
            "Could not allocate the PNG image.",
        )
    })?;

    if matches!(options.background, PngBackground::White) {
        pixmap.fill(resvg::tiny_skia::Color::WHITE);
    }

    resvg::render(
        &tree,
        Transform::from_scale(options.scale as f32, options.scale as f32),
        &mut pixmap.as_mut(),
    );

    let rgba = pixmap.data().to_vec();
    let encoded = pixmap.encode_png().map_err(|error| {
        ExportError::new(
            "png_encode_failed",
            format!("Could not encode PNG: {error}"),
        )
    })?;

    Ok(RenderedPng {
        metadata,
        rgba,
        encoded,
    })
}

fn parse_svg(svg: &str) -> Result<usvg::Tree, ExportError> {
    if svg.trim().is_empty() {
        return Err(ExportError::new(
            "empty_svg",
            "There is no rendered SVG to export.",
        ));
    }

    let mut font_database = fontdb::Database::new();
    font_database.load_system_fonts();
    let monospace_family = font_database
        .faces()
        .find(|face| face.monospaced)
        .and_then(|face| face.families.first())
        .map(|(family, _)| family.clone())
        .unwrap_or_else(|| "monospace".to_owned());
    font_database.set_monospace_family(&monospace_family);

    let normalized_svg = svg
        .replace("JetBrains Mono Variable", &monospace_family)
        .replace("JetBrains Mono", &monospace_family);

    let options = usvg::Options {
        font_family: monospace_family,
        fontdb: std::sync::Arc::new(font_database),
        ..Default::default()
    };

    usvg::Tree::from_str(&normalized_svg, &options).map_err(|error| {
        ExportError::new(
            "invalid_svg",
            format!("Could not parse rendered SVG: {error}"),
        )
    })
}

fn dimensions(tree: &usvg::Tree, scale: u32) -> Result<PngMetadata, ExportError> {
    if !(1..=3).contains(&scale) {
        return Err(ExportError::new(
            "invalid_scale",
            "PNG scale must be 1, 2, or 3.",
        ));
    }

    let size = tree.size();
    let width = (size.width() * scale as f32).ceil() as u32;
    let height = (size.height() * scale as f32).ceil() as u32;
    let pixels = u64::from(width) * u64::from(height);

    if width == 0 || height == 0 {
        return Err(ExportError::new(
            "invalid_dimensions",
            "The rendered SVG has invalid dimensions.",
        ));
    }

    if width > MAX_DIMENSION || height > MAX_DIMENSION || pixels > MAX_PIXELS {
        return Err(ExportError::new(
            "image_too_large",
            format!("The requested PNG is too large ({width} x {height}). Choose a smaller scale."),
        ));
    }

    Ok(PngMetadata { width, height })
}
