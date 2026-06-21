use crate::session::Session;

pub struct File {
    pub name: String,
    pub content: String,
}

pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub directories: Vec<Directory>,
}

pub struct FileSystem {
    pub root: Directory,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: Directory {
                name: "/".to_string(),

                files: vec![
                    File {
                        name: "readme.txt".to_string(),
                        content: "Brooooo not this way".to_string(),
                    },
                ],

                directories: vec![
                    Directory {
                        name: "hint1".to_string(),
                        files: vec![
                            File {
                                name: "hint1.txt".to_string(),
                                content: "hint2".to_string(),
                            },
                        ],           
                        directories: vec![
                            Directory {
                                name: "hint2".to_string(),
                                files: vec![
                                    File {
                                        name: "hint2.txt".to_string(),
                                        content: "cd .., then cd .secret".to_string(),
                                    },
                                ],
                                directories: vec![],
                            },
                        ],
                    },
                    Directory {
                        name: ".secret".to_string(),
                        files: vec![
                            File {
                                name: "reward.txt".to_string(),
                                content: "
                                           ███   ███  ████  ████  █   █ 
                                          █     █   █ █   █ █   █  █ █  
                                          █     █   █ █   █ █   █   █   
                                          █     █   █ █   █ █   █   █   
                                           ███   ███  ████  ████    █
                                           ".to_string(),
                            },
                        ],
                        directories: vec![],
                    },
                ],
            },
        }
    }
}

impl FileSystem {
    pub fn ls(&self, session:&Session) -> Option<Vec<&str>> {
        
        if session.current_dir == "/".to_string() {
            let mut vecc:Vec<&str>=self.root.directories.iter().filter(|file| !file.name.starts_with('.')).map(|curdir| curdir.name.as_str()).collect();
            let extended:Vec<&str> = self.root.files.iter().filter(|file| !file.name.starts_with('.')).map(|curfile| curfile.name.as_str()).collect();
            vecc.extend(extended);
            return Some(vecc);
        }
        

        let whole_path_split:Vec<&str> = session.current_dir.split('/').filter(|curdir| !curdir.is_empty()).collect();
        let mut current = &self.root;

        for path_split in whole_path_split {
            let current_inbetween = current.directories.
            iter().
            find(|curpath| curpath.name==path_split);
        
            if let Some(res) = current_inbetween {
                current=res;
            } else {
                return None;
            }
        }

        let mut resultat:Vec<&str> = current.files.iter().filter(|file| !file.name.starts_with('.')).map(|curfile| curfile.name.as_str()).collect();
        let extended_res:Vec<&str> = current.directories.iter().filter(|file| !file.name.starts_with('.')).map(|curdir| curdir.name.as_str()).collect();
        resultat.extend(extended_res);
        Some(resultat)
    }

    pub fn cat(&self,session:&Session,filename:&str) -> Option<String> {
        let mut res:Option<String>=None;

        let mut current = &self.root;

        for path in session.current_dir.split('/').filter(|curdir| !curdir.is_empty()) {
            let inbetween_current = current.directories
            .iter()
            .find(|dir| dir.name==path);

            if let Some(res) = inbetween_current{
                current = res;
            }
        }

        let proper_file = current.files.iter().find(|file| file.name==filename);

        if let Some(result) = proper_file{
            res=Some(result.content.clone());
        }

        res
    }

    pub fn cd(&self,session:&mut Session,path:&str) -> Option<String> {
        match path {
            ".."=>{
                if session.current_dir=="/" {
                    return Some("/".to_string());
                }
                let mut curpath:Vec<&str> = session.current_dir.split('/').filter(|curdir| !curdir.is_empty()).collect();
                curpath.pop();

                let curpath_joined = if curpath.is_empty() {
                    "/".to_string()
                } else {
                    format!("/{}", curpath.join("/"))
                };
                session.current_dir=curpath_joined.clone();


                return Some(curpath_joined);
            } 
            _=>{
                let mut current = &self.root;
                let parts:Vec<&str> = session.current_dir
                .split("/")
                .filter(|curdir| !curdir.is_empty())
                .collect();

                for part in parts {
                    let current_inbetween = current.directories
                    .iter()
                    .find(|dir| dir.name==part);
                    if let Some(res) = current_inbetween {
                        current=res;
                    }
                }

                if let Some(_) = current.directories.iter().find(|d| d.name == path) {
                    if session.current_dir == "/" {
                        session.current_dir = format!("/{}", path);
                    } else {
                        session.current_dir = format!("{}/{}", session.current_dir, path);
                    }

                    return Some(session.current_dir.clone());
                }

                None
                
            }
        }
    }
}