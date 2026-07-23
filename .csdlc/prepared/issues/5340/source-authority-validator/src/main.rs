use std::{env, fs, path::Path};

use syn::{
    visit::{self, Visit},
    Attribute, ExprAsync, ExprAwait, ExprClosure, ExprUnsafe, File, ItemExternCrate,
    ItemForeignMod, ItemImpl, ItemTrait, Macro, Signature, TypeBareFn, UseTree,
};

const FORBIDDEN_NAMESPACES: &[&str] = &[
    "env", "fs", "future", "io", "net", "os", "path", "process", "sync", "task", "thread", "time",
];
const ALLOWED_ATTRIBUTES: &[&str] = &[
    "allow",
    "deny",
    "deprecated",
    "derive",
    "doc",
    "expect",
    "forbid",
    "inline",
    "must_use",
    "non_exhaustive",
    "serde",
    "warn",
];
#[derive(Default)]
struct AuthorityVisitor {
    findings: Vec<&'static str>,
}

impl AuthorityVisitor {
    fn inspect_path(&mut self, path: &[String]) {
        let bounded_memory_writer = path.len() == 3
            && path[0] == "std"
            && path[1] == "io"
            && matches!(path[2].as_str(), "Error" | "Result" | "Write");
        if path.len() > 1
            && matches!(path[0].as_str(), "std" | "core")
            && FORBIDDEN_NAMESPACES.contains(&path[1].as_str())
            && !bounded_memory_writer
        {
            self.findings.push("forbidden std/core authority path");
        }
    }

    fn inspect_use(&mut self, tree: &UseTree, prefix: &mut Vec<String>) {
        match tree {
            UseTree::Path(node) => {
                prefix.push(node.ident.to_string());
                self.inspect_use(&node.tree, prefix);
                prefix.pop();
            }
            UseTree::Name(node) => {
                prefix.push(node.ident.to_string());
                self.inspect_path(prefix);
                if node.ident == "self"
                    && matches!(prefix.first().map(String::as_str), Some("std" | "core"))
                {
                    self.findings.push("aliased std/core root import");
                }
                prefix.pop();
            }
            UseTree::Rename(node) => {
                prefix.push(node.ident.to_string());
                self.inspect_path(prefix);
                if (node.ident == "self" || prefix.len() == 1)
                    && matches!(prefix.first().map(String::as_str), Some("std" | "core"))
                {
                    self.findings.push("aliased std/core root import");
                }
                prefix.pop();
            }
            UseTree::Glob(_) => {
                self.inspect_path(prefix);
                if matches!(prefix.first().map(String::as_str), Some("std" | "core")) {
                    self.findings.push("glob import from std/core");
                }
            }
            UseTree::Group(group) => {
                for item in &group.items {
                    self.inspect_use(item, prefix);
                }
            }
        }
    }
}

impl<'ast> Visit<'ast> for AuthorityVisitor {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        self.inspect_use(&node.tree, &mut Vec::new());
        visit::visit_item_use(self, node);
    }

    fn visit_path(&mut self, node: &'ast syn::Path) {
        self.inspect_path(
            &node
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>(),
        );
        visit::visit_path(self, node);
    }

    fn visit_signature(&mut self, node: &'ast Signature) {
        if node.asyncness.is_some() {
            self.findings.push("async function");
        }
        if node.unsafety.is_some() {
            self.findings.push("unsafe function");
        }
        if node.abi.is_some() {
            self.findings.push("native ABI function");
        }
        visit::visit_signature(self, node);
    }

    fn visit_type_bare_fn(&mut self, node: &'ast TypeBareFn) {
        if node.unsafety.is_some() || node.abi.is_some() {
            self.findings.push("unsafe/native function pointer");
        }
        visit::visit_type_bare_fn(self, node);
    }

    fn visit_expr_async(&mut self, node: &'ast ExprAsync) {
        self.findings.push("async block");
        visit::visit_expr_async(self, node);
    }

    fn visit_expr_await(&mut self, node: &'ast ExprAwait) {
        self.findings.push("await expression");
        visit::visit_expr_await(self, node);
    }

    fn visit_expr_closure(&mut self, node: &'ast ExprClosure) {
        if node.asyncness.is_some() {
            self.findings.push("async closure");
        }
        visit::visit_expr_closure(self, node);
    }

    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) {
        self.findings.push("unsafe block");
        visit::visit_expr_unsafe(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        if node.unsafety.is_some() {
            self.findings.push("unsafe impl");
        }
        visit::visit_item_impl(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        if node.unsafety.is_some() {
            self.findings.push("unsafe trait");
        }
        visit::visit_item_trait(self, node);
    }

    fn visit_item_foreign_mod(&mut self, node: &'ast ItemForeignMod) {
        self.findings.push("native ABI block");
        visit::visit_item_foreign_mod(self, node);
    }

    fn visit_item_extern_crate(&mut self, node: &'ast ItemExternCrate) {
        if matches!(node.ident.to_string().as_str(), "std" | "core") {
            self.findings.push("aliased std/core crate");
        }
        visit::visit_item_extern_crate(self, node);
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        // syn intentionally treats macro bodies as opaque token streams. A
        // blanket product-source macro ban is smaller and fail-closed: derive
        // and serde attributes remain available, while authority cannot hide
        // inside an unparsed macro body.
        self.findings.push("macro invocation or definition");
        visit::visit_macro(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast Attribute) {
        let allowed = node
            .path()
            .get_ident()
            .is_some_and(|ident| ALLOWED_ATTRIBUTES.contains(&ident.to_string().as_str()));
        if !allowed {
            self.findings.push("non-allowlisted attribute");
        }
        visit::visit_attribute(self, node);
    }
}

fn inspect(path: &Path) -> Result<Vec<&'static str>, String> {
    let source = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let syntax: File = syn::parse_file(&source).map_err(|error| error.to_string())?;
    let mut visitor = AuthorityVisitor::default();
    visitor.visit_file(&syntax);
    visitor.findings.sort_unstable();
    visitor.findings.dedup();
    Ok(visitor.findings)
}

fn main() {
    let mut rejected = Vec::new();
    let files = env::args().skip(1).collect::<Vec<_>>();
    if files.is_empty() {
        eprintln!("no Rust source paths supplied");
        std::process::exit(64);
    }
    for file in &files {
        match inspect(Path::new(file)) {
            Ok(findings) if findings.is_empty() => {}
            Ok(findings) => rejected.push(format!("{file}:{}", findings.join(","))),
            Err(error) => rejected.push(format!("{file}:parse-error:{error}")),
        }
    }
    if !rejected.is_empty() {
        eprintln!("forbidden product-source authority: {}", rejected.join(";"));
        std::process::exit(1);
    }
    println!(
        "{{\"schema\":\"adl.wp06.source-authority-proof.v3\",\"parser\":\"syn-2.0.118\",\"scanned_files\":{},\"outcome\":\"passed\"}}",
        files.len()
    );
}
