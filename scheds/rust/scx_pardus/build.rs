// Copyright (c) Andrea Righi <andrea.righi@linux.dev>
//
// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

fn main() {
//println!("cargo:rustc-link-search=native=/home/user2/build_on/MinSizeRel/_deps/abseil_cpp-build/absl/container");

    
  //  println!("cargo:rustc-link-search=native=/home/user2/build_on/MinSizeRel");

      /*println!("cargo:rustc-link-lib=static=onnxruntime_session");
    println!("cargo:rustc-link-lib=static=onnxruntime_framework");
    println!("cargo:rustc-link-lib=static=onnxruntime_common");
    println!("cargo:rustc-link-lib=static=onnxruntime_test_utils");*/




    println!("cargo:rustc-link-lib=static=absl_hashtablez_sampler");

/*
    println!("cargo:rustc-link-lib=static=absl_raw_hash_set");
    println!("cargo:rustc-link-lib=static=absl_hash");
    println!("cargo:rustc-link-lib=static=absl_city");
    println!("cargo:rustc-link-lib=static=absl_low_level_hash");
    println!("cargo:rustc-link-lib=static=absl_base");
    println!("cargo:rustc-link-lib=static=absl_spinlock_wait");
    println!("cargo:rustc-link-lib=static=absl_malloc_internal");

   
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=rt");*/
    scx_cargo::BpfBuilder::new()
        .unwrap()
        .enable_intf("src/bpf/intf.h", "bpf_intf.rs")
        .enable_skel("src/bpf/main.bpf.c", "bpf")
        .build()
        .unwrap();
}
