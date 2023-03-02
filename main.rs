use std::collections::{BTreeSet, HashMap};

struct Problem {
    addends: Vec<String>,
    answer: String,
    legend: HashMap<char, u8>,
    unique: Vec<char>,
    largest: usize,
}

impl Problem {
    fn word_problem(&self) -> String {
        format!(
            "{}{} = {}",
            self.addends[0],
            self.addends[1..]
                .iter()
                .map(|s| {
                    let mut s = s.clone();
                    s.insert_str(0, " + ");
                    s
                })
                .collect::<String>(),
            self.answer
        )
    }

    fn number_problem(&self) -> String {
        format!(
            "{}{} = {}",
            self.addends[0]
                .chars()
                .map(|c| self.legend.get(&c).unwrap().to_string())
                .collect::<String>(),
            if self.addends.len() > 0 {
                self.addends[1..]
                    .iter()
                    .map(|s| {
                        let mut s = s
                            .chars()
                            .map(|c| self.legend.get(&c).unwrap().to_string())
                            .collect::<String>();
                        s.insert_str(0, " + ");
                        s
                    })
                    .collect::<String>()
            } else {
                "".to_string()
            },
            self.answer
                .chars()
                .map(|c| self.legend.get(&c).unwrap().to_string())
                .collect::<String>(),
        )
    }

    fn new(expr: String) -> Problem {
        let mut count = 0;

        let mut problem = Problem {
            addends: Vec::<String>::new(),
            answer: String::new(),
            legend: HashMap::<char, u8>::new(),
            unique: Vec::<char>::new(),
            largest: 0,
        };

        for c in expr.chars() {
            if c.is_alphabetic() {
                let c = c.to_uppercase().to_string().chars().nth(0).unwrap();

                if !problem.unique.contains(&c) {
                    problem.legend.insert(c, count);
                    problem.unique.push(c);
                    count += 1;
                }
            }
        }

        let mut temp = String::new();

        for c in expr.chars() {
            let c = c.to_uppercase().to_string().chars().nth(0).unwrap();

            if c.is_alphabetic() {
                temp.push(c);
            } else if c == '+' {
                problem.addends.push(temp);
                temp = String::new();
            } else if c == '=' {
                if problem.addends.len() == 0 {
                    problem.answer = temp;
                    temp = String::new();
                } else {
                    problem.addends.push(temp);
                    temp = String::new();
                }
            }
        }

        if problem.answer.len() == 0 {
            problem.answer = temp;
        } else {
            problem.addends.push(temp);
        }

        problem.largest = problem.addends.iter().fold(0, |l, a| if a.len() < l { l } else { a.len() } );
        
        problem
    }

    fn valid(&self) -> bool {
        let mut valid = true;
        println!("{:?}", self.legend);

        for i in 0..self.largest {
            let num = self.addends.iter().map(|s| *self.legend.get(&s.chars().nth(i).unwrap()).unwrap()).sum::<u8>();
            let carry = num / 10;
            valid = valid
                && (carry + (num % 10) == *self.legend.get(&self.answer.chars().nth(i).unwrap()).unwrap());
        }

        valid
    }

    fn solve(&mut self) -> bool {
        // loop through each unique letter
        for i in 0..self.unique.len() {
            // try each number for the letters
            for j in 0..10 {
                println!("----------------");
                println!("{:?}", self.unique);
                let mut nums = BTreeSet::new();
                for x in 0..10 { nums.insert(x); }
                self.legend.insert(self.unique[i], j);
                nums.remove(&j);
                
                // loop through all the other letters
                for k in 0..self.unique.len() {
                    if self.valid() {
                        return true;
                    } else {
                        if i != j as usize {
                            // find unique numbers
                            for x in 0..10 {
                                println!("Testing: {x}");
                                if nums.remove(&x) {
                                    println!("Setting {} to {x}", self.unique[k]);
                                    self.legend.insert(self.unique[k], x);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }
}

fn main() {
    //let mut problem = Problem::new(String::from("send + more = money"));
    let mut problem = Problem::new(String::from("a + b = c"));

    println!("{}", problem.word_problem());

    if problem.solve() {
        println!("Solved: {}", problem.number_problem());
    } else {
        println!("Unable to solve: {}", problem.word_problem());
    }
}