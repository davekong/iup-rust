use iup_sys;
use std::path::{Path, PathBuf};

use Element;

pub struct FileDlg(*mut iup_sys::Ihandle);

impl FileDlg {
    pub fn new() -> FileDlg {
        unsafe { FileDlg::from_raw(iup_sys::IupFileDlg()) }
    }

    pub fn new_open() -> FileDlg {
        FileDlg::new().set_attrib_data("DIALOGTYPE", cstr!("OPEN") as *const _)
    }

    pub fn new_save() -> FileDlg {
        FileDlg::new().set_attrib_data("DIALOGTYPE", cstr!("SAVE") as *const _)
    }

    pub fn new_dir() -> FileDlg {
        FileDlg::new().set_attrib_data("DIALOGTYPE", cstr!("DIR") as *const _)
    }

    pub fn files(&self) -> Option<Vec<PathBuf>> {
        self.attrib("VALUE").and_then(|value| {
            let values =  value.split_terminator('|').collect::<Vec<&str>>();
            match values.len() {
                0 => None,
                1 => Some([PathBuf::from(values[0])].to_vec()),
                _ => {
                    // When multiple files are selected, the first value is the path the files are
                    // contained in and then the filenames, let's build paths based on this.
                    let path = Path::new(values[0]);
                    Some(values.into_iter().skip(1)
                               .map(|file| path.join(file))
                               .collect())
                },
            }
        })
    }

    pub fn path(&self) -> Option<PathBuf> {
        self.attrib("VALUE").map(|value| value.into())
    } 

}

impl_dialog!(FileDlg, "filedlg");
impl ::callback::HelpCb for FileDlg {}
