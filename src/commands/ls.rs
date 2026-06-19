use crate::filesystem::FileSystem;
use crate::session::Session;

pub fn execute(
    session: &Session,
    fs: &FileSystem,
) -> Result<String,String> {
    let result:Option<Vec<&str>>=fs.ls(session);
    match result {
        Some(res)=>Ok(res.join("\n").to_string()),
        None=>Err("Broooo not like this. Empty directory".to_string())
    }
}
