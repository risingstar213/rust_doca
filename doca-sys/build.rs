use std::env::{self, consts};
use std::path::{Path, PathBuf};

fn main() {
    let arch = consts::ARCH;
    println!(
        "cargo:rustc-link-search=native=/opt/mellanox/doca/lib/{}-linux-gnu",
        arch
    );
    println!("cargo:rustc-link-lib=doca_dma");
    println!("cargo:rustc-link-lib=doca_common");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Check whether doca is available in this machine or not
    assert!(
        Path::new("/opt/mellanox/doca").is_dir(),
        "doca is not available in this machine"
    );

    // First we build a `util.a` for function `parse_pci_addr` to use
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // generate bindings based on the wrapper header
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/opt/mellanox/doca/include")
        .generate_comments(false)
        .allowlist_function("doca_dev_.*")
        .allowlist_function("doca_devinfo_.*")
        // DOCA_DEV part
        .allowlist_type("doca_dev")
        .allowlist_type("doca_devinfo")
        // DOCA_MMAP part
        .allowlist_function("doca_mmap_.*")
        .allowlist_type("doca_mmap")
        .allowlist_type("doca_access_flags")
        // DOCA_BUF_INVENTORY part
        .allowlist_type("doca_buf_inventory")
        .allowlist_function("doca_buf_inventory_.*")
        // DOCA_CTX part
        .allowlist_type("doca_event")
        .allowlist_type("doca_ctx")
        .allowlist_type("doca_workq_.*")
        .allowlist_type("doca_job_.*")
        .allowlist_function("doca_workq_.*")
        .allowlist_function("doca_ctx_.*")
        // DOCA_BUF part
        .allowlist_type("doca_buf")
        .allowlist_function("doca_buf_.*")
        // DOCA_DMA part
        .allowlist_type("doca_dma_.*")
        .allowlist_function("doca_dma_.*")
        .allowlist_type("doca_pci_bdf")
        .derive_default(true)
        .derive_debug(true)
        .prepend_enum_name(false)
        .size_t_is_usize(true)
        // .constified_enum_module("doca_error")
        .rustified_enum("doca_error")
        .bitfield_enum("doca_access_flags")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
