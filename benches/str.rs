use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_string_replace::{FastReplacePatterns as fsr, StdReplacePatterns as ssr};

fn criterion_benchmark(c: &mut Criterion) {
    // hello world
    let mut group = c.benchmark_group("hello world");
    let patterns = &["greet"];
    let values = &["world".to_owned()];
    let input = "Hello %{greet}!";
    group.bench_function("fsr", |b| {
        b.iter(|| criterion::black_box(fsr::replace_patterns(&input, patterns, values)));
    });
    // let patterns = &["%{greet}"];
    group.bench_function("std", |b| {
        b.iter(|| criterion::black_box(ssr::replace_patterns(&input, patterns, values)));
    });
    group.finish();

    // long text
    let mut group = c.benchmark_group("long text");
    let patterns = &["macro", "what"];
    let values = &["format!".to_owned(), "string".to_owned()];
    let input = r#"The first argument %{macro} receives is a format %{what}. 
        This must be a %{what} literal. 
        The power of the formatting %{what} is in the {}s contained. 
        Additional parameters passed to %{macro} replace the {}s within the formatting %{what} in the order given unless named or positional parameters are used.
        "#;
    group.bench_function("fsr", |b| {
        b.iter(|| criterion::black_box(fsr::replace_patterns(&input, patterns, values)))
    });
    // let patterns = &["%{macro}", "%{what}"];
    group.bench_function("std", |b| {
        b.iter(|| criterion::black_box(ssr::replace_patterns(&input, patterns, values)))
    });
    group.finish();

    // many variables
    let mut group = c.benchmark_group("many variables");
    let patterns = &["id", "name", "surname", "email", "city", "zip", "website"];
    let values = &[
        "3".to_owned(),
        "Marion".to_owned(),
        "Christiansen".to_owned(),
        "Marion_Christiansen83@hotmail.com".to_owned(),
        "Litteltown".to_owned(),
        "8408".to_owned(),
        "https://snoopy-napkin.name".to_owned(),
    ];
    let input = r#"Hello %{name} %{surname}, your account id is %{id}, email address is %{email}. 
        You live in %{city} %{zip}. 
        Your website is %{website}."#;
    group.bench_function("fsr", |b| {
        b.iter(|| criterion::black_box(fsr::replace_patterns(&input, patterns, values)))
    });
    // let patterns = &[
    //     "%{id}",
    //     "%{name}",
    //     "%{surname}",
    //     "%{email}",
    //     "%{city}",
    //     "%{zip}",
    //     "%{website}",
    // ];
    group.bench_function("std", |b| {
        b.iter(|| criterion::black_box(ssr::replace_patterns(&input, patterns, values)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
