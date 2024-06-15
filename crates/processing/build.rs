extern crate cmake;

#[cfg(feature = "gpu")]
use cmake::Config;

fn main()
{
    #[cfg(feature = "gpu")]
    {
        let dst = Config::new("csrc")
            .define("CL_TARGET_OPENCL_VERSION", "300")
            .build();

        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-link-lib=static=picturify-processing-opencl");
        println!("cargo:rustc-link-lib=OpenCL");
    }
}