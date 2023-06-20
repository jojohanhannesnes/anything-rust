struct Buffer {
    text: String,
}

struct Document {
    filename: String,
    modified: bool,
    buffer: Buffer,
}

impl Document {
    //TODO! modified no need to add new
    fn new(filename: String) -> Self {
        Document {
            filename,
            modified: false,
            buffer: Buffer {
                text: String::new(),
            },
        }
    }

    fn open(&mut self, filename: String) -> std::io::Result<()> {
        let file_contents = std::fs::read_to_string(&filename)?;
        self.buffer = Buffer {
            text: file_contents,
        };
        self.filename = filename;
        self.modified = false;

        Ok(())
    }

    fn save(&self) -> std::io::Result<()> {
        std::fs::write(&self.filename, &self.buffer.text)?;
        Ok(())
    }

    fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    fn is_modified(&self) -> bool {
        self.modified
    }
}

impl Buffer {
    fn create(&mut self, data: &str) {
        self.text = data.to_string();
    }

    fn insert(&mut self, data: &str) {
        self.text.push_str(data);
    }

    fn delete(&mut self) {
        let _ = self.text.remove(self.text.len() - 1);
    }

    fn replace(&mut self, old: &str, new: &str) {
        self.text = self.text.replace(old, new);
    }

    fn get(&self) -> &str {
        &self.text
    }
}

fn main() {
    // let mut text = Buffer {
    //     text: String::new(),
    // };

    // text.create("huhu xd");
    // text.insert("hihi");
    // text.delete();
    // text.replace("xd", "xp");
    // println!("{}", text.get());

    let mut document = Document::new(String::from("test.txt"));
    document.open(String::from("test.txt")).unwrap();
    document.buffer.insert("asdasdasd");
    document.set_modified(true);
    document.save().unwrap();
    println!("Document modified : {}", document.is_modified());
}
