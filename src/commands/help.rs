pub fn execute() -> Result<String,String> {
    Ok(r#"Im jinnee give you three wishes and help(available commands - use them to navigate threw filesystem):
ls - to print list of files in the directory
cd <dir> - to move threw the filesystem defining directory
cat <file> - to print data from file 
help - to summon jinnee
"#
    .to_string())
}
