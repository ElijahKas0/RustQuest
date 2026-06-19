use crate::filesystem::FileSystem;
use crate::session::Session;

pub fn execute(
    session: &mut Session,
    fs: &FileSystem,
    path: &str,
) -> Result<String,String> {
    let result:Option<String>=fs.cd(session, path);
    match result {
        Some(res)=>{
            Ok(res)
        }
        None=>{
            Err("Naaaah no such path dumbass".to_string())
        }
    }
}
