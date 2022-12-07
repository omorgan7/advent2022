use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

#[derive(Debug)]
struct File {
    name: String,
    sz: i64,
}

#[derive(Debug)]
struct Directory {
    parent: Option<Rc<RefCell<Directory>>>,
    name: String,
    files: Vec<File>,
    children: Vec<Rc<RefCell<Directory>>>,
}

fn directory_size(ent: Rc<RefCell<Directory>>) -> i64 {
    ent.borrow().files.iter().fold(0, |acc, file| acc + file.sz)
        + ent
            .borrow()
            .children
            .iter()
            .fold(0, |acc, child| acc + directory_size(Rc::clone(child)))
}

fn part1(ent: Rc<RefCell<Directory>>) -> i64 {
    let self_sz = directory_size(Rc::clone(&ent));
    ent.borrow()
        .children
        .iter()
        .fold(0, |acc, child| acc + part1(Rc::clone(child)))
        + if self_sz > 100000 { 0 } else { self_sz }
}

fn part2(ent: Rc<RefCell<Directory>>, free_space: i64) -> i64 {
    let self_sz = directory_size(Rc::clone(&ent));
    if free_space + self_sz > 30000000 {
        // println!("Viable! {} {} {}", ent.borrow().name, self_sz, free_space + self_sz);
        std::cmp::min(
            self_sz,
            ent.borrow()
                .children
                .iter()
                .map(|child| part2(Rc::clone(child), free_space))
                .min()
                .unwrap_or(i64::MAX),
        )
    } else {
        i64::MAX
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Directory {
        parent: None,
        name: "/".to_string(),
        files: vec![],
        children: vec![],
    }));
    // let mut root_tree = DirectoryTree { parent: None, children: vec![], entry: &root};
    let mut cwd = Rc::clone(&root);

    let input = include_str!("../input.txt");

    let mut line_it = input.lines().peekable();
    while let Some(line) = line_it.next() {
        let mut cmd = line.split(' ').skip(1);

        match cmd.next().unwrap() {
            "cd" => {
                let target = cmd.next().unwrap();
                if target == "/" {
                    // IGNORE
                } else if target == ".." {
                    let new_cwd = Rc::clone(cwd.borrow().parent.as_ref().unwrap());
                    cwd = new_cwd;
                } else {
                    // println!("{:?}", cwd.borrow().children);
                    let new_cwd = Rc::clone(
                        cwd.borrow()
                            .children
                            .iter()
                            .find(|child| child.borrow().name == target)
                            .unwrap(),
                    );
                    cwd = new_cwd;
                }
            }
            "ls" => {
                while let Some(newline) = line_it.peek() {
                    if &newline[0..3] == "dir" {
                        let entry = Rc::new(RefCell::new(Directory {
                            parent: Some(Rc::clone(&cwd)),
                            name: newline[4..].to_string(),
                            files: vec![],
                            children: vec![],
                        }));
                        cwd.borrow_mut().children.push(entry);
                    } else if newline.starts_with('$') {
                        break;
                    } else {
                        let mut words = newline.split(' ');
                        let file = File {
                            sz: words.next().unwrap().parse::<i64>().unwrap(),
                            name: words.next().unwrap().to_string(),
                        };
                        cwd.borrow_mut().files.push(file);
                    }
                    line_it.next();
                }
            }
            _ => {}
        }
    }

    println!("{:?}", part1(Rc::clone(&root)));
    println!(
        "{:?}",
        part2(
            Rc::clone(&root),
            70000000 - directory_size(Rc::clone(&root))
        )
    );
}
