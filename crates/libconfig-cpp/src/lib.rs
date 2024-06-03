use std::path::Path;

pub fn libconfig_path() -> &'static Path
{
    Path::new(env!("LIBCONFIG_PATH"))
}
