use std::ffi::OsString;
use winreg::{enums::*, RegKey};

/// copy
///     "HKEY_CLASSES_ROOT\blender.<X.Y>\DefaultIcon"
/// to
///     "HKEY_CLASSES_ROOT\Applications\open_blender_file.exe\DefaultIcon"
pub fn set_icon() -> std::io::Result<()> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let blend_file_key = hkcr.open_subkey(".blend")?;
    dbg!(&blend_file_key);
    // blender.4.1
    let assoc = blend_file_key.get_value::<OsString, &str>("");
    dbg!(&assoc);
    let assoc_key = hkcr.open_subkey(assoc?)?;
    dbg!(&assoc_key);

    let src = assoc_key.open_subkey_with_flags("DefaultIcon", KEY_READ)?;
    dbg!(&src);
    // dst
    let application_key = hkcr.open_subkey("Applications\\open_blender_file.exe")?;
    dbg!(&application_key);
    let (dst, _) = application_key.create_subkey("DefaultIcon")?;
    src.copy_tree("", &dst)?;

    Ok(())
}
