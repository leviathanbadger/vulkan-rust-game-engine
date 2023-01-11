# vulkan-rust-game-engine

Game engine created using the Rust programming language. Learning to use Rust, but
also learning to use Vulkan and several related graphics technologies that I've
never played with in my prior experience with OpenGL.

More details about this project can be found [here][vulkan-rust-game-engine].

## Local Development

To build the engine locally:

1. Install [Rust][rust]
2. Install [LLVM][llvm] (required for interoperability with external SDKs such as
   DLSS)
3. Download the [NVidia DLSS SDK][dlss-sdk]. Copy the header files from the "include"
   directory to "crates/dlss-sys/dlss/include". Copy the dev nvngx_dlss.dll and
   nvsdk_ngx_s.lib into the "crates/dlss-sys/dlss/lib" directory.
4. Run `cargo build` from the project root to build all crates.

[vulkan-rust-game-engine]: https://brandonslade.me/projects/vulkan-rust-game-engine
[rust]: https://www.rust-lang.org/
[llvm]: https://releases.llvm.org/download.html
[dlss-sdk]: https://developer.nvidia.com/rtx/dlss/get-started
