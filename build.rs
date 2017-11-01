extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=deezer");

    let bindings = bindgen::Builder::default()
        .header("libdeezer/wrapper.h")
        // naming for connects
        .bitfield_enum("dz_streaming_mode_t")
        .bitfield_enum("dz_connect_cache_event_t")
        .bitfield_enum("dz_connect_event_t")

        // naming for player
        .bitfield_enum("dz_player_event_t")
        .bitfield_enum("dz_player_play_command_t")
        .bitfield_enum("dz_queuelist_repeat_mode_t")

        // naming for object
        .bitfield_enum("dz_error_t")

        // naming for track
        .bitfield_enum("dz_track_metadata_t")
        .bitfield_enum("dz_track_quality_t")
        .bitfield_enum("dz_media_format_t")

        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}