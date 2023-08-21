mod utils;

use espresso_common::EsTarget;
use espresso_compiler::Compiler;
use espresso_package::Package;
use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::read_file;

mod plugin_output {
    use super::*;

    #[tokio::test]
    async fn adds_mjs_ext_to_imports_exports() {
        let sandbox = create_sandbox("imports-exports");
        let package = Package::new(sandbox.path()).unwrap();
        let compiler = Compiler::new(&package).unwrap();
        let out_dir = compiler.compile(EsTarget::Es2020).await.unwrap();

        assert_snapshot!(read_file(out_dir.join("index.mjs")));
    }
}
