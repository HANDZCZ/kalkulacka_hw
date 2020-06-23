#[cfg(target_os = "windows")]
use winapi::um::winnt;

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico")
        .set_language(winnt::MAKELANGID(
            winnt::LANG_CZECH,
            winnt::SUBLANG_ENGLISH_US,
        ));
    res.compile().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {}
