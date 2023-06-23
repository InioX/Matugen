use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matugen::template::parser::parse;

const DATA: &str = r#"
@define-color onPrimary_shifted2 @{on_primary.rgb(20 30 -20)};
@define-color primaryContainer_shifted2 @{primary_container.strip(20 30 -20)};
@define-color onPrimaryContainer_shifted2 @{on_primary_container(20 30 -20)};
@define-color secondaryContainer @{secondary_container.rgba};
@define-color onSecondaryContainer @{on_secondary_container};
@define-color tertiary @{tertiary};
@define-color errorContainer_shifted2 @{error_container.rgba(20 30 -20)};
@define-color onErrorContainer_shifted2 @{on_error_container(20 30 -20)};
@define-color background_shifted2 @{background(20 30 -20)};
@define-color onSurface_shifted2 @{on_surface(20 30 -20)};
@define-color surfaceVariant_shifted2 @{surface_variant(20 30 -20)};
@define-color onSurfaceVariant_shifted2 @{on_surface_variant(20 30 -20)};
@define-color outline_shifted2 @{outline.rgba(20 30 -20)};
@define-color shadow_shifted2 @{shadow(20 30 -20)};
@define-color inverseOnSurface_shifted2 @{inverse_on_surface(20 30 -20)};
@define-color inversePrimary_shifted2 @{inverse_primary.strip(20 30 -20)};
"#;

fn criterion_benchmark(c: &mut Criterion) {
  let data = DATA.repeat(16);
  c.bench_function("parse 512 lines", |b| b.iter(|| parse(black_box(&data))));

  let data = DATA.repeat(256);
  c.bench_function("parse 4096 lines", |b| b.iter(|| parse(black_box(&data))));

  let data = DATA.repeat(2048);
  c.bench_function("parse 32768 lines", |b| b.iter(|| parse(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
