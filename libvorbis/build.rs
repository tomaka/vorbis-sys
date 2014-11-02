extern crate gcc;

fn main() {
    let root = Path::new(std::os::getenv("CARGO_MANIFEST_DIR").unwrap()
        .join("libvorbis").join("lib");

    let config = gcc::Config {
        include_directories: vec![
            root.join("include"),
            std::os::getenv("DEP_OGG_INCLUDE").unwrap(),
        ],
        definitions: vec![
            ("_USRDLL".to_string(), None),
            ("LIBVORBIS_EXPORTS".to_string(), None)
        ],
        .. std::default::Default::default()
    }

    println!("cargo:include={}", root.join("include"));

    let libfiles_root = root.join("lib");

    if std::os::args().find(|s| s.as_slice() == "vorbis").is_some() {
        gcc::compile_library("libvorbis.a", &[
            libfiles_root.join("analysis.c"),
            libfiles_root.join("bitrate.c"),
            libfiles_root.join("block.c"),
            libfiles_root.join("codebook.c"),
            libfiles_root.join("envelope.c"),
            libfiles_root.join("floor0.c"),
            libfiles_root.join("floor1.c"),
            libfiles_root.join("info.c"),
            libfiles_root.join("lookup.c"),
            libfiles_root.join("lpc.c"),
            libfiles_root.join("lsp.c"),
            libfiles_root.join("mapping0.c"),
            libfiles_root.join("mdct.c"),
            libfiles_root.join("psy.c"),
            libfiles_root.join("registry.c"),
            libfiles_root.join("res0.c"),
            libfiles_root.join("sharedbook.c"),
            libfiles_root.join("smallft.c"),
            libfiles_root.join("synthesis.c"),
            libfiles_root.join("vorbisenc.c"),
            libfiles_root.join("window.c"),
        ]);
    }

    if std::os::args().find(|s| s.as_slice() == "vorbisfile").is_some() {
        gcc::compile_library("libvorbisfile.a", &[
            libfiles_root.join("vorbisfile.c"),
        ]);
    }
}
