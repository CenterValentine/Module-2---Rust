use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


struct File_Identity {

}

impl File_Identity{
    fn new(path: String,  ) -> Self {
        self { path, }
    }


}