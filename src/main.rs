use core::num;
use std::sync::{ Arc, Mutex };

fn main() {
    let raw_arguments = " - 32 - 1 + 200 + 3";
    let mut args: Vec<String> = vec![];

    // check
    // 0: num
    // 1: operator
    let mut counter = 0;
    let mut index = 0;

    // flag
    let mut negatif = false;
    let mut negatif_between = false;
    raw_arguments
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            println!("{:?}", c);
            if counter == 0 && (c == '/' || c == '*') {
                let space = " ".repeat(i);
                panic!(
                    "\nkalkuloc panic in order {}\n\n{}\n{}^\n{}unexpected syntax\n",
                    i,
                    raw_arguments,
                    space,
                    space
                );
            }

            if let Ok(_) = c.to_string().parse::<f32>() {
                if counter == 1 {
                    index += 1;
                    counter = 0;
                }
            } else if c == '-' {
                negatif = !negatif;
                if index != 0 {
                    negatif_between = true;
                }
            } else {
                if c != ' ' {
                    index += 1;
                    counter = 1;
                }
            }

            if c != ' ' && c != '-' {
                if let Some(item) = args.get_mut(index) {
                    if negatif {
                        index += 1;

                        args.push("+".to_string());
                        args.push(format!("-{}", c.to_string()));

                        index += 1;
                        negatif = false;
                    } else {
                        item.push(c);
                    }
                } else {
                    if counter == 0 && negatif {
                        args.push(format!("-{}", c.to_string()));
                        negatif = false;
                    } else {
                        args.push(c.to_string());
                    }
                }
            }
        });

    println!("{:?}", args);

    let mut parent = None;
    let mut minus_flag = false;
    let mut multiple_div_flag = false;
    for (i, arg) in args.iter().enumerate() {
        if let None = parent {
            if let Ok(number) = arg.parse::<f32>() {
                let link = Node {
                    id: i,
                    value: NodeValue::Number(
                        if !minus_flag {
                            number
                        } else {
                            minus_flag = false;
                            -number
                        }
                    ),
                    parent: None,
                    childs: None,
                };
                parent = Some(Arc::new(Mutex::new(link)));
            } else {
                if arg == "-" {
                    minus_flag = true;
                }
            }
        } else if let Some(parent) = &mut parent {
            if let Ok(number) = arg.parse::<f32>() {
                let mut link = Node {
                    id: i,
                    value: NodeValue::Number(
                        if !minus_flag {
                            number
                        } else {
                            minus_flag = false;
                            -number
                        }
                    ),
                    parent: None,
                    childs: None,
                };

                let mut lock = parent.lock().unwrap();
                if !multiple_div_flag {
                    if let NodeValue::Operation(_, _) = lock.value {
                        link.parent = Some(parent.clone());

                        let ref_link = Arc::new(Mutex::new(link));
                        if let None = lock.childs.as_ref().unwrap().0 {
                            lock.childs.as_mut().unwrap().0 = Some(ref_link);
                        } else {
                            lock.childs.as_mut().unwrap().1 = Some(ref_link);
                        }
                    }
                } else {
                    multiple_div_flag = false;
                    let ref_link = Arc::new(Mutex::new(link));
                    lock.childs
                        .as_mut()
                        .unwrap()
                        .0.as_mut()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .childs.as_mut()
                        .unwrap().1 = Some(ref_link);
                }
            } else {
                if arg == "+" {
                    let link = Node {
                        id: i,
                        value: NodeValue::Operation("+".to_string(), 0),
                        parent: None,
                        // child: Some(vec![parent.clone()]),
                        childs: if let NodeValue::Number(_) = parent.lock().unwrap().value {
                            Some((Some(parent.clone()), None))
                        } else {
                            Some((None, Some(parent.clone())))
                        },
                    };

                    let ref_link = Arc::new(Mutex::new(link));
                    parent.lock().unwrap().parent = Some(ref_link.clone());
                    *parent = ref_link;
                } else if arg == "-" {
                    let link = Node {
                        id: i,
                        value: NodeValue::Operation("+".to_string(), 0),
                        parent: None,
                        childs: if let NodeValue::Number(_) = parent.lock().unwrap().value {
                            Some((Some(parent.clone()), None))
                        } else {
                            Some((None, Some(parent.clone())))
                        },
                    };
                    let ref_link = Arc::new(Mutex::new(link));

                    {
                        let mut lock = parent.lock().unwrap();

                        lock.parent = Some(ref_link.clone());
                    }
                    *parent = ref_link;
                    minus_flag = true;
                } else if arg == "*" {
                    let mut lock = parent.lock().unwrap();
                    if let NodeValue::Operation(_, power) = lock.value {
                        if power == 1 {
                            let link = Node {
                                id: i,
                                value: NodeValue::Operation("*".to_string(), 1),
                                parent: None,
                                childs: if let NodeValue::Number(_) = lock.value {
                                    Some((Some(parent.clone()), None))
                                } else {
                                    Some((None, Some(parent.clone())))
                                },
                            };
                            let ref_link = Arc::new(Mutex::new(link));
                            lock.parent = Some(ref_link.clone());
                            drop(lock);
                            *parent = ref_link;
                        } else if power < 1 {
                            let parent_childs = lock.childs.as_mut().unwrap();
                            if
                                let NodeValue::Operation(_, _) = parent_childs.1
                                    .as_ref()
                                    .unwrap()
                                    .lock()
                                    .unwrap().value
                            {
                                let left_parent_child = parent_childs.0.as_ref().unwrap().clone();
                                let link = Node {
                                    id: i,
                                    value: NodeValue::Operation("*".to_string(), 1),
                                    parent: None,
                                    childs: Some((Some(left_parent_child), None)),
                                };
                                let ref_link = Some(Arc::new(Mutex::new(link)));
                                parent_childs.0 = ref_link;
                                multiple_div_flag = true;
                            } else {
                                // let left = parent_childs.0.as_mut().unwrap();

                                let right = parent_childs.1.as_mut().unwrap();
                                let link = Node {
                                    id: i,
                                    value: NodeValue::Operation("*".to_string(), 1),
                                    parent: None,
                                    childs: Some((Some(right.clone()), None)),
                                };

                                *parent_childs.1.as_mut().unwrap() = parent_childs.0
                                    .as_mut()
                                    .unwrap()
                                    .clone();

                                let ref_link = Some(Arc::new(Mutex::new(link)));
                                parent_childs.0 = ref_link;
                                multiple_div_flag = true;
                            }
                        }
                    } else {
                        let link = Node {
                            id: i,
                            value: NodeValue::Operation("*".to_string(), 1),
                            parent: None,
                            childs: if let NodeValue::Number(_) = lock.value {
                                Some((Some(parent.clone()), None))
                            } else {
                                Some((None, Some(parent.clone())))
                            },
                        };
                        let ref_link = Arc::new(Mutex::new(link));
                        lock.parent = Some(ref_link.clone());
                        drop(lock);
                        *parent = ref_link;
                    }
                } else if arg == "/" {
                    let mut lock = parent.lock().unwrap();
                    if let NodeValue::Operation(_, power) = lock.value {
                        if power == 1 {
                            let link = Node {
                                id: i,
                                value: NodeValue::Operation("/".to_string(), 1),
                                parent: None,
                                childs: if let NodeValue::Number(_) = lock.value {
                                    Some((Some(parent.clone()), None))
                                } else {
                                    Some((None, Some(parent.clone())))
                                },
                            };
                            let ref_link = Arc::new(Mutex::new(link));
                            lock.parent = Some(ref_link.clone());
                            drop(lock);
                            *parent = ref_link;
                        } else if power < 1 {
                            let parent_childs = lock.childs.as_mut().unwrap();
                            if
                                let NodeValue::Operation(_, _) = parent_childs.1
                                    .as_ref()
                                    .unwrap()
                                    .lock()
                                    .unwrap().value
                            {
                                let left_parent_child = parent_childs.0.as_ref().unwrap().clone();
                                let link = Node {
                                    id: i,
                                    value: NodeValue::Operation("/".to_string(), 1),
                                    parent: None,
                                    childs: Some((Some(left_parent_child), None)),
                                };
                                let ref_link = Some(Arc::new(Mutex::new(link)));
                                parent_childs.0 = ref_link;
                                multiple_div_flag = true;
                            } else {
                                // let left = parent_childs.0.as_mut().unwrap();

                                let right = parent_childs.1.as_mut().unwrap();
                                let link = Node {
                                    id: i,
                                    value: NodeValue::Operation("/".to_string(), 1),
                                    parent: None,
                                    childs: Some((Some(right.clone()), None)),
                                };

                                *parent_childs.1.as_mut().unwrap() = parent_childs.0
                                    .as_mut()
                                    .unwrap()
                                    .clone();

                                let ref_link = Some(Arc::new(Mutex::new(link)));
                                parent_childs.0 = ref_link;
                                multiple_div_flag = true;
                            }
                        }
                    } else {
                        let link = Node {
                            id: i,
                            value: NodeValue::Operation("/".to_string(), 1),
                            parent: None,
                            childs: if let NodeValue::Number(_) = lock.value {
                                Some((Some(parent.clone()), None))
                            } else {
                                Some((None, Some(parent.clone())))
                            },
                        };
                        let ref_link = Arc::new(Mutex::new(link));
                        lock.parent = Some(ref_link.clone());
                        drop(lock);
                        *parent = ref_link;
                    }
                }
            }
        }
    }

    println!("{:?}", parent.unwrap().lock().unwrap().running())
}

#[derive(Debug)]
enum NodeValue {
    Number(f32),
    Operation(String, usize),
}

#[derive(Debug)]
struct Node {
    id: usize,
    value: NodeValue,
    parent: Option<Arc<Mutex<Node>>>,
    childs: Option<(Option<Arc<Mutex<Node>>>, Option<Arc<Mutex<Node>>>)>, //(value, ops/value)
}

impl Node {
    fn running(&self) -> f32 {
        if let NodeValue::Number(number) = self.value {
            return number;
        } else if let NodeValue::Operation(opcode, _) = &self.value {
            let childs = self.childs.as_ref().unwrap();
            let a = childs.0.as_ref().unwrap().lock().unwrap().running();
            let b = childs.1.as_ref().unwrap().lock().unwrap().running();

            if opcode == "+" {
                return a + b;
            } else if opcode == "*" {
                return a * b;
            } else if opcode == "/" {
                return a / b;
            }

            0.0
        } else {
            panic!("lah kok anjing")
        }
    }
}
