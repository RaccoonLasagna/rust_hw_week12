#[derive(Debug, Clone)]
enum Text {
    Plain(String),
    Repeated(String, usize),
    Joined(Vec<Box<Text>>, String),
}

impl Text {
    fn value(&self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(t, n) => t.clone().repeat(*n),
            Text::Joined(v, t) => {
                let mut final_string = v[0].value().to_string();
                for strings in v[1..].iter() {
                    final_string += t;
                    final_string += &strings.value();
                }
                final_string
            }
        }
    }
}
impl From<&Text> for Box<Text> {
    fn from(t: &Text) -> Box<Text> {
        Box::new(t.clone())
    }
}
impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text {
        &self
    }
}

impl Into<String> for &Text {
    fn into(self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(t, n) => (*t.repeat(*n)).to_string(),
            Text::Joined(v, t) => {
                let mut final_string = v[0].value().to_string();
                for strings in v[1..].iter() {
                    final_string += t;
                    final_string += &strings.value();
                }
                final_string
            }
        }
    }
}

fn main() {
    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let mut tvec: Vec<Box<Text>> = Vec::new();
    tvec.push(t1.into());
    tvec.push(t2.into());
    tvec.push(t3.into());
    tvec.push(t4.into());
    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, (&t5).into());
    println!("{}", t6.value());
}

#[test]
fn test_text_composition() {
    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let mut tvec: Vec<Box<Text>> = Vec::new();
    tvec.push(t1.into());
    tvec.push(t2.into());
    tvec.push(t3.into());
    tvec.push(t4.into());
    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, (&t5).into());
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}
