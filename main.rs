struct Problem {
    addends: Vec<Vec<(char, Box<u8>)>>,
    answer: Vec<(char, Box<u8>)>,
    legend: Vec<u8>,
}

impl Problem {
    fn word_problem(&self) -> String {
        format!(
            "{}{} = {}",
            self.addends[0].iter().map(|(c, _)| *c).collect::<String>(),
            self.addends[1..]
                .iter()
                .map(|v| {
                    let mut s = String::from(" + ");
                    s.push_str(&v.iter().map(|(c, _)| *c).collect::<String>());
                    s
                })
                .collect::<String>(),
            self.answer.iter().map(|(c, _)| *c).collect::<String>()
        )
    }

    fn number_problem(&self) -> String {
        format!(
            "{}{} = {}",
            self.addends[0]
                .iter()
                .map(|(_, n)| n.to_string())
                .collect::<String>(),
            self.addends[1..]
                .iter()
                .map(|v| {
                    let mut s = String::from(" + ");
                    s.push_str(
                        &v.iter()
                            .map(|(_, n)| n.to_string())
                            .collect::<String>(),
                    );
                    s
                })
                .collect::<String>(),
            self.answer
                .iter()
                .map(|(_, n)| n.to_string())
                .collect::<String>()
        )
    }

    fn new(expr: String) -> Problem {
        let mut track = Vec::<char>::new();

        let mut count = 0;

        let mut problem = Problem {
            addends: Vec::<Vec<(char, Box<u8>)>>::new(),
            answer: Vec::<(char, Box<u8>)>::new(),
            legend: Vec::<u8>::new(),
        };

        for c in expr.chars() {
            if c.is_alphabetic() {
                let c = c.to_uppercase().to_string().chars().nth(0).unwrap();

                if !track.contains(&c) {
                    problem.legend.push(count);
                    track.push(c);
                    count += 1;
                }
            }
        }

        let mut temp = Vec::<char>::new();

        for c in expr.chars() {
            if c.is_alphabetic() {
                temp.push(c);
            } else if c == '+' {
                let mut addend = Vec::<(char, Box<u8>)>::new();

                for i in temp {
                    let i = i.to_uppercase().to_string().chars().nth(0).unwrap();
                    addend.push((
                        i,
                        Box::new(problem.legend[track.iter().rposition(|&x| x == i).unwrap()]),
                    ));
                }

                temp = Vec::<char>::new();
                problem.addends.push(addend);
            } else if c == '=' {
                if problem.addends.len() == 0 {
                    for i in temp {
                        problem.answer.push((
                            i,
                            Box::new(
                                problem.legend[track.iter().rposition(|&x| x == i).unwrap()],
                            ),
                        ));
                    }

                    temp = Vec::<char>::new();
                } else {
                    let mut addend = Vec::<(char, Box<u8>)>::new();

                    for i in temp {
                        let i = i.to_uppercase().to_string().chars().nth(0).unwrap();
                        addend.push((
                            i,
                            Box::new(
                                problem.legend[track.iter().rposition(|&x| x == i).unwrap()],
                            ),
                        ));
                    }

                    temp = Vec::<char>::new();
                    problem.addends.push(addend);
                }
            }
        }

        if problem.answer.len() == 0 {
            for i in temp {
                let i = i.to_uppercase().to_string().chars().nth(0).unwrap();
                problem.answer.push((
                    i,
                    Box::new(problem.legend[track.iter().rposition(|&x| x == i).unwrap()]),
                ));
            }
        } else {
            let mut addend = Vec::<(char, Box<u8>)>::new();

            for i in temp {
                addend.push((
                    i,
                    Box::new(problem.legend[track.iter().rposition(|&x| x == i).unwrap()]),
                ));
            }

            problem.addends.push(addend);
        }

        problem
    }
    
    fn valid(&self) -> bool {
        let mut valid = true;
        
        for i in 0..self.addends.len() {
            valid = valid && (self.addends.iter().fold(0, |sum, vec| {
                sum + *vec.get(i).unwrap_or(&('a', Box::new(0))).1
            }) == *self.answer.get(i).unwrap_or(&('a', Box::new(0))).1);
        }
        
        valid
    }
    
    fn solve(&mut self) -> bool {
        for n in self.legend.len() as u8..10_u8 {
            self.legend.push(n);
        }
            
        for i in 0..self.legend.len() {
            for j in i + 1..self.legend.len() {
                if self.valid() {
                    return true;
                } else {
                    self.legend.swap(i, j);
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
        println!("{}", problem.number_problem());
    } else {
        println!("Unable to solve: {}", problem.word_problem());
    }
}
