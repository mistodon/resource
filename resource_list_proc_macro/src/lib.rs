use std::path::{Path, PathBuf};

use proc_macro::TokenStream;

use quote::quote;

fn read_path_argument(path: TokenStream) -> PathBuf {
    let path = path.to_string();
    let path = path.trim_start_matches('"');
    let path = path.trim_end_matches('"');
    PathBuf::from(path)
}

fn enumerate_files_paths(path: &Path) -> (Vec<String>, Vec<String>) {
    let mut files_paths = vec![];

    let entries = std::fs::read_dir(path)
        .unwrap_or_else(|e| panic!("Failed to read directory `{}`: {}", path.display(), e));

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let file_type = entry.file_type().unwrap_or_else(|e| {
            panic!(
                "Failed to read file type of `{}`: {}",
                entry.path().display(),
                e
            )
        });
        if file_type.is_file() {
            let file_name = entry.file_name();
            let mut path = path.to_owned();
            path.push(&file_name);

            let file_name = file_name.to_string_lossy().into_owned();
            if !file_name.starts_with('.') {
                files_paths.push((file_name, path.to_string_lossy().into_owned()));
            }
        }
    }

    files_paths.sort();

    let (files, paths) = files_paths.into_iter().unzip();

    (files, paths)
}

#[proc_macro_hack::proc_macro_hack]
pub fn resource_list(path: TokenStream) -> TokenStream {
    let path = read_path_argument(path);
    let (files, paths) = enumerate_files_paths(&path);

    (quote! {
        [
            #((#files, resource!(#paths)),)*
        ]
    })
    .into()
}

#[proc_macro_hack::proc_macro_hack]
pub fn resource_str_list(path: TokenStream) -> TokenStream {
    let path = read_path_argument(path);
    let (files, paths) = enumerate_files_paths(&path);

    (quote! {
        [
            #((#files, resource_str!(#paths)),)*
        ]
    })
    .into()
}
