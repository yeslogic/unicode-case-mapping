#![feature(test)]

extern crate test;

use test::Bencher;

const INPUT: &str = "Font shaping is the process of laying out the glyphs of a font in order to represent some input text. Rasterisation of the glyphs is a separate process. Font shaping for Latin text is quite simple. For some scripts, like those used by Indic languages, it is quite complex and requires reordering and substituting the glyphs in each syllable to produce the final output. There are only three main font shaping engines in use today: DirectWrite on Windows, CoreText on macOS and iOS, and HarfBuzz on open-source operating systems and some web-browsers. Of these, only HarfBuzz is open source.

Unfortunately there is no complete specification of (how) to <perform> font shaping for complex scripts, so determining the desired behaviour often comes down to observing what the other font shaping engines do and implementing that. In the hope of improving this situation we have been supporting Nathan Willis in an effort to document the OpenType shaping behaviour.

Prince is mostly written in the Mercury logic programming [language] but when it came to building the new font parsing and shaping engine we chose Rust. The reasons include a growing community and ecosystem, good interoperability with C (Mercury compiles to C), strong memory safety guarantees, high performance, minimal runtime, and cross-platform support (we build Prince binaries for FreeBSD, Linux, macOS, and Windows).";

#[bench]
fn bench_std(b: &mut Bencher) {
    b.iter(|| {
        let input = test::black_box(INPUT);
        input.chars().for_each(|ch| {
            let mut res = [' '; 3];
            ch.to_uppercase().enumerate().for_each(|(i, c)| res[i] = c);
        })
    });
}

#[bench]
fn bench_this_crate(b: &mut Bencher) {
    b.iter(|| {
        let input = test::black_box(INPUT);
        input.chars().for_each(|ch| {
            unicode_case_mapping::to_uppercase(ch);
        })
    });
}
