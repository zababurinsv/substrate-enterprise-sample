
				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/home/zb/Desktop/rust/substrate-enterprise-sample/chain/target/release/build/enterprise-sample-runtime-cc7cc2ef42c11535/out/wasm_binary.rs",
						"/home/zb/Desktop/rust/substrate-enterprise-sample/chain/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base -C link-arg=--import-memory ",
					)
				}
			