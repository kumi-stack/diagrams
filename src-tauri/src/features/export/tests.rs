use super::{
    model::{PngBackground, PngMetadata, PngOptions},
    renderer,
};

const SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="10">
  <rect x="1" y="1" width="18" height="8" fill="#008577"/>
</svg>"##;
const TEXT_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="30">
  <text x="2" y="20" font-family="JetBrains Mono Variable" font-size="16">Diagram</text>
</svg>"#;

#[test]
fn inspects_scaled_dimensions() {
    assert_eq!(
        renderer::inspect(
            SVG,
            PngOptions {
                scale: 3,
                background: PngBackground::Transparent,
            },
        )
        .unwrap(),
        PngMetadata {
            width: 60,
            height: 30
        }
    );
}

#[test]
fn renders_transparent_and_white_backgrounds() {
    let transparent = renderer::render(
        SVG,
        PngOptions {
            scale: 1,
            background: PngBackground::Transparent,
        },
    )
    .unwrap();
    let white = renderer::render(
        SVG,
        PngOptions {
            scale: 1,
            background: PngBackground::White,
        },
    )
    .unwrap();

    assert_eq!(transparent.rgba[3], 0);
    assert_eq!(&white.rgba[..4], &[255, 255, 255, 255]);
    assert!(transparent.encoded.starts_with(b"\x89PNG\r\n\x1a\n"));
}

#[test]
fn rejects_invalid_scale() {
    let error = renderer::inspect(
        SVG,
        PngOptions {
            scale: 4,
            background: PngBackground::Transparent,
        },
    )
    .unwrap_err();

    assert_eq!(error.code, "invalid_scale");
}

#[test]
fn renders_text_with_a_monospace_fallback() {
    let rendered = renderer::render(
        TEXT_SVG,
        PngOptions {
            scale: 1,
            background: PngBackground::Transparent,
        },
    )
    .unwrap();

    assert!(rendered.rgba.chunks_exact(4).any(|pixel| pixel[3] > 0));
}
