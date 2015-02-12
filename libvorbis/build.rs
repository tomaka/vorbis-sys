extern crate "pkg-config" as pkg_config;
extern crate gcc;

fn main() {
    let root = Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                    .join("libvorbis");

    println!("cargo:include={}", root.join("include").as_str().unwrap());
    println!("cargo:src={}", root.join("lib").as_str().unwrap());

    match pkg_config::find_library("vorbis") {
        Ok(_) => return,
        Err(..) => {}
    };

    gcc::Config::new()
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
                .include(root.join("include"))
                .include(root.join("lib"))
                .include(Path::new(std::env::var("DEP_OGG_INCLUDE").unwrap()))
                .compile("libvorbis.a");
}
