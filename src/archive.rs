use std::{
    ffi::OsStr,
    fs::File,
    io,
    path::{Path, PathBuf},
};

use binfmt::ar::Archive;
use rustc_codegen_ssa::back::archive::ArchiveBuilder;
use rustc_codegen_ssa::back::archive::ArchiveBuilderBuilder;
use rustc_session::cstore::DllImport;
use rustc_session::Session;

#[derive(Default)]
pub struct XlangArchiveBuilder {
    archive: Archive,
}

impl ArchiveBuilder<'_> for XlangArchiveBuilder {
    fn add_file(&mut self, file: &Path) {
        let new_member = self.archive.new_member();
        io::copy(&mut File::open(file).unwrap(), new_member)
            .expect("error writing to archive file");
        new_member.set_name(
            file.file_name()
                .and_then(OsStr::to_str)
                .expect("couldn't get file name for archive file"),
        );
    }

    fn add_archive(
        &mut self,
        _: &std::path::Path,
        _: std::boxed::Box<(dyn for<'a> std::ops::FnMut(&'a str) -> bool + 'static)>,
    ) -> std::result::Result<(), std::io::Error> {
        todo!()
    }

    fn build(mut self: std::boxed::Box<Self>, output: &std::path::Path) -> bool {
        self.archive.ranlib();
        self.archive.write(File::create(output).unwrap()).unwrap();
        false // Return value is unused
    }
}

#[derive(Default)]
pub struct XlangArchiveBuilderBuilder {}

impl ArchiveBuilderBuilder for XlangArchiveBuilderBuilder {
    fn new_archive_builder<'a>(
        &self,
        _: &'a Session,
    ) -> std::boxed::Box<(dyn ArchiveBuilder<'a> + 'a)> {
        Box::<XlangArchiveBuilder>::default()
    }

    fn create_dll_import_lib(
        &self,
        _: &Session,
        _: &str,
        _: &[DllImport],
        _: &std::path::Path,
        _: bool,
    ) -> PathBuf {
        todo!()
    }
}
