use std::sync::{ Arc, Mutex };

fn main() {
    let mut input = vec![];

    "1 + 2 + 2 + 4".split(" ").for_each(|c| (
        if c != " " {
            input.push(c);
        }
    ));

    let mut parent = None;
    // let mut minus_flag = false;
    for (i, &arg) in input.iter().enumerate() {
        if let None = parent {
            if let Ok(number) = arg.parse::<i32>() {
                let link = Node {
                    id: i,
                    value: NodeValue::Number(number),
                    parent: None,
                    child: None,
                };
                parent = Some(Arc::new(Mutex::new(link)));
            } else {
                if arg == "+" {
                }
            }
        } else if let Some(parent) = &mut parent {
            if let Ok(number) = arg.parse::<i32>() {
                let mut link = Node {
                    id: i,
                    value: NodeValue::Number(number),
                    parent: None,
                    child: None,
                };

                let mut lock = parent.lock().unwrap();
                if let NodeValue::Operation(_) = lock.value {
                    link.parent = Some(parent.clone());
                    let ref_link = Arc::new(Mutex::new(link));

                    lock.child.as_mut().unwrap().push(ref_link);
                }
            } else {
                if arg == "+" {
                    let link = Node {
                        id: i,
                        value: NodeValue::Operation("+".to_string()),
                        parent: None,
                        child: Some(vec![parent.clone()]),
                    };

                    let ref_link = Arc::new(Mutex::new(link));

                    parent.lock().unwrap().parent = Some(ref_link.clone());
                    *parent = ref_link;
                }
            }
        }
    }

    println!("{:?}", parent.unwrap().lock().unwrap().running())
}

#[derive(Debug)]
enum NodeValue {
    Number(i32),
    Operation(String),
}

#[derive(Debug)]
struct Node {
    id: usize,
    value: NodeValue,
    parent: Option<Arc<Mutex<Node>>>,
    child: Option<Vec<Arc<Mutex<Node>>>>,
    // childs: Option<(Option<Arc<Mutex<Node>>>, Option<Arc<Mutex<Node>>>)>,
}

impl Node {
    fn running(&self) -> i32 {
        if let NodeValue::Number(number) = self.value {
            return number;
        } else if let NodeValue::Operation(opcode) = &self.value {
            let childs = self.child.as_ref().unwrap();
            let a = childs[0].lock().unwrap().running();
            let b = childs[1].lock().unwrap().running();

            return a + b;
        } else {
            panic!("lah kok anjing")
        }
    }
}
