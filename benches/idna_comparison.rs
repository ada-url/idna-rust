use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn benchmark_to_ascii_ada_vs_idna(c: &mut Criterion) {
    let test_domains = vec![
        // ASCII domains (should be fast for both)
        "example.com",
        "test.org",
        "simple.net",
        "sub.domain.example.com",
        // Unicode domains requiring punycode conversion
        "straße.de",
        "café.example.com",
        "münchen.de",
        "правда.com",
        "ドメイン名例.jp",
        "test.中国",
        "bücher.example.org",
        "naïve.français.fr",
        "español.com",
        "niño.es",
        // Complex Unicode scripts
        "مثال.شبكة",
        "اختبار.com",
        "例子.中国",
        "测试.com",
        "例え.日本",
        "テスト.com",
        "예시.한국",
        "테스트.com",
        "ตัวอย่าง.ไทย",
        "ทดสอบ.com",
        // Mixed scripts
        "test-café.com",
        "münchen-test.de",
        "test-例子.com",
        "café-тест.org",
        "example-مثال.net",
    ];

    let mut group = c.benchmark_group("to_ascii");

    // Benchmark ada-idna (our implementation)
    group.bench_function("ada_idna", |b| {
        b.iter(|| {
            for domain in &test_domains {
                let _ = black_box(ada_idna::domain::to_ascii(black_box(domain)));
            }
        })
    });

    // Benchmark standard idna crate
    group.bench_function("idna_crate", |b| {
        b.iter(|| {
            for domain in &test_domains {
                let _ = black_box(idna::domain_to_ascii(black_box(domain)));
            }
        })
    });

    group.finish();
}

fn benchmark_to_unicode_ada_vs_idna(c: &mut Criterion) {
    let test_domains = vec![
        // ASCII domains (should pass through)
        "example.com",
        "test.org",
        "simple.net",
        // Punycode domains to decode
        "xn--strae-oqa.de",
        "xn--caf-dma.example",
        "xn--mnchen-3ya.de",
        "xn--80aafi6cg.com",
        "xn--eckwd4c7c.xn--wgv71a119e.jp",
        "test.xn--fiqz9s",
        "xn--bcher-kva.example",
        "xn--nave-6pa.xn--franais-xka.fr",
        "xn--espaol-zwa.com",
        "xn--nio-5qa.es",
        // Complex punycode
        "xn--mgbh0fb.xn--ngbc5azd",
        "xn--kgbechtv.com",
        "xn--fsq.xn--fiqs8s",
        "xn--0zwm56d.com",
        "xn--r8jz45g.xn--wgv71a119e",
        "xn--zckzah.com",
        "xn--9n2bp8q.xn--3e0b707e",
        "xn--o39a.com",
        "xn--12c1fe0br.xn--o3cw4h",
        "xn--12co0c3b4eva.com",
        // Mixed domains
        "simple.xn--caf-dma.com",
        "xn--caf-dma.simple.com",
        "test.xn--strae-oqa.example",
    ];

    let mut group = c.benchmark_group("to_unicode");

    // Benchmark ada-idna (our implementation)
    group.bench_function("ada_idna", |b| {
        b.iter(|| {
            for domain in &test_domains {
                let _ = black_box(ada_idna::domain::to_unicode(black_box(domain)));
            }
        })
    });

    // Benchmark standard idna crate
    group.bench_function("idna_crate", |b| {
        b.iter(|| {
            for domain in &test_domains {
                let _ = black_box(idna::domain_to_unicode(black_box(domain)));
            }
        })
    });

    group.finish();
}

fn benchmark_punycode_encoding(c: &mut Criterion) {
    let test_strings = vec![
        "straße",
        "café",
        "münchen",
        "правда",
        "ドメイン名例",
        "中国",
        "bücher",
        "español",
        "niño",
        "مثال",
        "اختبار",
        "例子",
        "测试",
        "예시",
        "테스트",
        "ตัวอย่าง",
        "ทดสอบ",
        "αβγδε",
        "Ελληνικά",
    ];

    let mut group = c.benchmark_group("punycode_encode");

    // Benchmark ada-idna punycode
    group.bench_function("ada_idna", |b| {
        b.iter(|| {
            for s in &test_strings {
                // Convert to UTF-32 first, then encode
                let utf32_chars = ada_idna::unicode::utf8_to_utf32(black_box(s.as_bytes()));
                let _ = black_box(ada_idna::punycode::utf32_to_punycode(&utf32_chars));
            }
        })
    });

    group.finish();
}

fn benchmark_punycode_decoding(c: &mut Criterion) {
    let test_punycode = vec![
        "strae-oqa",
        "caf-dma",
        "mnchen-3ya",
        "80aafi6cg",
        "eckwd4c7c",
        "fiqz9s",
        "bcher-kva",
        "espaol-zwa",
        "nio-5qa",
        "mgbh0fb",
        "kgbechtv",
        "fsq",
        "0zwm56d",
        "r8jz45g",
        "zckzah",
        "9n2bp8q",
        "o39a",
        "12c1fe0br",
        "12co0c3b4eva",
        "mxacd",
        "nxagarb2b",
    ];

    let mut group = c.benchmark_group("punycode_decode");

    // Benchmark ada-idna punycode
    group.bench_function("ada_idna", |b| {
        b.iter(|| {
            for s in &test_punycode {
                let _ = black_box(ada_idna::punycode::punycode_to_utf32(black_box(s)));
            }
        })
    });

    group.finish();
}

fn benchmark_unicode_normalization(c: &mut Criterion) {
    let test_strings = vec![
        "café",         // Composed é
        "cafe\u{0301}", // Decomposed e + combining acute
        "weiß",         // German sharp s
        "naïve",        // Multiple accents
        "español",      // Spanish characters
        "αβγδε",        // Greek
        "правда",       // Cyrillic
        "北京",         // Chinese
        "東京",         // Japanese
        "한국",         // Korean
        "ไทย",          // Thai
        "العربية",      // Arabic
        "עברית",        // Hebrew
        // Complex normalization cases
        "ﬁle",   // fi ligature
        "ﬀ",     // ff ligature
        "²test", // Superscript 2
        "test³", // Superscript 3
    ];

    let mut group = c.benchmark_group("unicode_normalization");

    // Benchmark ada-idna normalization
    group.bench_function("ada_idna", |b| {
        b.iter(|| {
            for s in &test_strings {
                let _ = black_box(ada_idna::normalization::normalize(black_box(s)));
            }
        })
    });

    group.finish();
}

fn benchmark_single_domain_performance(c: &mut Criterion) {
    // Test performance on individual domains to see per-domain overhead
    let domains = [
        ("ascii", "example.com"),
        ("simple_unicode", "café.com"),
        ("complex_unicode", "münchen-straße.de"),
        ("non_latin", "правда.com"),
        ("cjk", "ドメイン名例.jp"),
        ("mixed_script", "test-例子.com"),
    ];

    for (name, domain) in domains.iter() {
        let mut group = c.benchmark_group(format!("single_domain_{}", name));

        group.bench_function("ada_idna_to_ascii", |b| {
            b.iter(|| ada_idna::domain::to_ascii(black_box(domain)))
        });

        group.bench_function("idna_crate_to_ascii", |b| {
            b.iter(|| idna::domain_to_ascii(black_box(domain)))
        });

        group.finish();
    }
}

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(150)
        .noise_threshold(0.005)
        .significance_level(0.01)
        .confidence_level(0.99)
}

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets =
        benchmark_to_ascii_ada_vs_idna,
        benchmark_to_unicode_ada_vs_idna,
        benchmark_punycode_encoding,
        benchmark_punycode_decoding,
        benchmark_unicode_normalization,
        benchmark_single_domain_performance,
}
criterion_main!(benches);
