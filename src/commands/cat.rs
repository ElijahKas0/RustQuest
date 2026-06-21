use crate::filesystem::FileSystem;
use crate::session::Session;

pub fn execute(
    session: &Session,
    fs: &FileSystem,
    file: &str,
) -> Result<String,String> {
    let result:Option<String>=fs.cat(session, file);
    match result {
        Some(res)=>{
            Ok(res)
        }
        None=>{
            Err("No such file in the derictory dumbass".to_string())
        }
    }
}
