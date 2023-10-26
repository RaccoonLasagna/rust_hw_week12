trait Text {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Text>;
}
impl Clone for Box<dyn Text> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
struct PlainText {
    chars: String,
}

#[derive(Debug, Clone)]
struct RepeatedText {
    chars: String,
    repeat: usize,
}

#[derive(Clone)]
struct JoinedText {
    chars_vec: Vec<Box<dyn Text>>,
    join: PlainText,
}

impl From<&str> for PlainText {
    fn from(text: &str) -> PlainText {
        PlainText {
            chars: text.to_string(),
        }
    }
}
impl RepeatedText {
    fn with_parts(text: &dyn Text, repeat: usize) -> RepeatedText {
        RepeatedText {
            chars: text.value(),
            repeat,
        }
    }
}
impl JoinedText {
    fn with_parts(chars_vec: &Vec<Box<dyn Text>>, join: &PlainText) -> JoinedText {
        JoinedText {
            chars_vec: chars_vec.to_vec(),
            join: join.clone(),
        }
    }
}

impl Text for PlainText {
    fn value(&self) -> String {
        self.chars.clone()
    }
    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl Text for RepeatedText {
    fn value(&self) -> String {
        self.chars.repeat(self.repeat)
    }
    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl Text for JoinedText {
    fn value(&self) -> String {
        let mut final_string = self.chars_vec[0].value().to_string();
        for strings in self.chars_vec[1..].iter() {
            final_string += &self.join.value();
            final_string += &strings.value();
        }
        final_string
    }
    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl AsRef<dyn Text> for PlainText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

fn main() {}

#[test]
fn test_text_composition() {
    let t1 = PlainText::from("x|x");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    let mut tvec: Vec<Box<dyn Text>> = Vec::new();
    tvec.push(t1.clone_box());
    tvec.push(t2.clone_box());
    tvec.push(t3.clone_box());
    tvec.push(t4.clone_box());
    let t5 = PlainText::from("--");
    let t6 = JoinedText::with_parts(&tvec, &t5);
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}
