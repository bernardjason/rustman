
fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();

    if target_arch == "unix"  {
        println!("cargo:rustc-link-lib=static=SDL2_image");
        println!("cargo:rustc-link-lib=static=SDL2_gfx");
        println!("cargo:rustc-link-lib=static=SDL2_ttf");
        println!("cargo:rustc-link-lib=static=freetype");
        println!("cargo:rustc-link-lib=static=jpeg");
        println!("cargo:rustc-link-lib=static=png");
        println!("cargo:rustc-link-lib=static=z");
        println!("cargo:rustc-link-lib=static=tiff");
        println!("cargo:rustc-link-lib=static=lzma");
        println!("cargo:rustc-link-lib=static=jbig");
        println!("cargo:rustc-link-lib=static=zstd");
        println!("cargo:rustc-link-lib=static=webp");
        println!("cargo:rustc-link-search=linux/libwebp-0.4.1/lib");
        println!("cargo:rustc-link-search=linux/zstd_v1.4.4/lib");
    } else {
        println!("cargo:rustc-link-lib=static=SDL2_image");
        println!("cargo:rustc-link-lib=static=SDL2_gfx");
        println!("cargo:rustc-link-lib=static=SDL2_ttf");
        println!("cargo:rustc-link-lib=static=freetype");
        println!("cargo:rustc-link-lib=static=bz2");
        println!("cargo:rustc-link-lib=static=jpeg");
        println!("cargo:rustc-link-lib=static=png16");
        println!("cargo:rustc-link-lib=static=zzz");
        println!("cargo:rustc-link-lib=static=tiff");
        println!("cargo:rustc-link-lib=static=lzma");
        println!("cargo:rustc-link-lib=static=zstd");
        println!("cargo:rustc-link-lib=static=webp");
        println!("cargo:rustc-link-lib=static=glib-2.0");

        println!("cargo:rustc-link-lib=dylib=intl");
        println!("cargo:rustc-link-lib=dylib=pcre");
        println!("cargo:rustc-link-lib=dylib=graphite2");
        println!("cargo:rustc-link-lib=dylib=brotlicommon");
        println!("cargo:rustc-link-lib=dylib=brotlidec");
        println!("cargo:rustc-link-lib=dylib=harfbuzz");

        println!(r"cargo:rustc-link-search=mingw-w64-x86_64");
        println!(r"cargo:rustc-link-search=windows");

    }
}
