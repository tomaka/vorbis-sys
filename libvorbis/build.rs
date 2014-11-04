extern crate gcc;

fn main() {
    let root = Path::new(std::os::getenv("CARGO_MANIFEST_DIR").unwrap())
        .join("libvorbis");

    let config = gcc::Config {
        include_directories: vec![
            root.join("include"),
            root.join("lib"),
            Path::new(std::os::getenv("DEP_OGG_INCLUDE").unwrap()),
        ],
        definitions: vec![
            ("_USRDLL".to_string(), None),
            ("LIBVORBIS_EXPORTS".to_string(), None)
        ],
        .. std::default::Default::default()
    };

    println!("cargo:include={}", root.join("include").as_str().unwrap());

    gcc::compile_library("libvorbis.a", &config, &[
        "libvorbis/lib/analysis.c",
        "libvorbis/lib/bitrate.c",
        "libvorbis/lib/block.c",
        "libvorbis/lib/codebook.c",
        "libvorbis/lib/envelope.c",
        "libvorbis/lib/floor0.c",
        "libvorbis/lib/floor1.c",
        "libvorbis/lib/info.c",
        "libvorbis/lib/lookup.c",
        "libvorbis/lib/lpc.c",
        "libvorbis/lib/lsp.c",
        "libvorbis/lib/mapping0.c",
        "libvorbis/lib/mdct.c",
        "libvorbis/lib/psy.c",
        "libvorbis/lib/registry.c",
        "libvorbis/lib/res0.c",
        "libvorbis/lib/sharedbook.c",
        "libvorbis/lib/smallft.c",
        "libvorbis/lib/synthesis.c",
        "libvorbis/lib/vorbisenc.c",
        "libvorbis/lib/window.c",
    ]);
}
