extern crate cc;
extern crate pkg_config;

use std::path::Path;

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&root).join("libvorbis");

    println!(
        "cargo:include={}",
        root.join("include").into_os_string().into_string().unwrap()
    );
    println!(
        "cargo:src={}",
        root.join("lib").into_os_string().into_string().unwrap()
    );

    match pkg_config::find_library("vorbis") {
        Ok(_) => return,
        Err(..) => {}
    };

    let ogg_inc = std::env::var("DEP_OGG_INCLUDE").unwrap();
    let ogg_inc = Path::new(&ogg_inc);

    cc::Build::new()
        .file("libvorbis/lib/analysis.c")
        .file("libvorbis/lib/bitrate.c")
        .file("libvorbis/lib/block.c")
        .file("libvorbis/lib/codebook.c")
        .file("libvorbis/lib/envelope.c")
        .file("libvorbis/lib/floor0.c")
        .file("libvorbis/lib/floor1.c")
        .file("libvorbis/lib/info.c")
        .file("libvorbis/lib/lookup.c")
        .file("libvorbis/lib/lpc.c")
        .file("libvorbis/lib/lsp.c")
        .file("libvorbis/lib/mapping0.c")
        .file("libvorbis/lib/mdct.c")
        .file("libvorbis/lib/psy.c")
        .file("libvorbis/lib/registry.c")
        .file("libvorbis/lib/res0.c")
        .file("libvorbis/lib/sharedbook.c")
        .file("libvorbis/lib/smallft.c")
        .file("libvorbis/lib/synthesis.c")
        .file("libvorbis/lib/vorbisenc.c")
        .file("libvorbis/lib/window.c")
        .define("_USRDLL", None)
        .define("LIBVORBIS_EXPORTS", None)
        .include(&root.join("include"))
        .include(&root.join("lib"))
        .include(&ogg_inc)
        .compile("libvorbis.a");
}
