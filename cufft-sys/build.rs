extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let whitelist_functions = [
        "cufftPlan1d", "cufftPlan2d", "cufftPlan3d", "cufftPlanMany",
        "cufftCreate",
        "cufftMakePlan1d", "cufftMakePlan2d", "cufftMakePlan3d", "cufftMakePlanMany", "cufftMakePlanMany64", "cufftXtMakePlanMany", 
        "cufftEstimate1d", "cufftEstimate2d", "cufftEstimate3d", "cufftEstimateMany", 
        "cufftGetSize1d", "cufftGetSize2d", "cufftGetSize3d", "cufftGetSizeMany", "cufftGetSizeMany64", "cufftXtGetSizeMany",
        "cufftGetSize", 
        "cufftSetAutoAllocation", "cufftSetWorkArea", "cufftXtSetWorkAreaPolicy",
        "cufftDestroy",
        "cufftExecC2C", "cufftExecZ2Z",
        "cufftExecR2C", "cufftExecD2Z",
        "cufftExecC2R", "cufftExecZ2D",
        "cufftXtExec", "cufftXtExecDescriptor",
        "cufftXtSetGPUs", "cufftXtSetWorkArea", 
        "cufftXtExecDescriptorC2C", "cufftXtExecDescriptorZ2Z",
        "cufftXtMalloc", "cufftXtFree", "cufftXtMemcpy",
        "cufftXtSetCallback", "cufftXtClearCallback", "cufftXtSetCallbackSharedSize",
        "cufftSetStream", "cufftGetVersion", "cufftGetProperty"
    ];

    
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cufft");
    
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/local/cuda/include");
    
    for func in whitelist_functions.iter() {
        builder = builder.whitelist_function(func);
    }
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    builder.generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}