use rand::Rng;
use std::collections::HashSet;
use std::fmt;

pub mod input {
    use std::io;

    pub fn str(tip: &str) -> String {
        println!("{}", tip);
        let mut s = String::new();
        match io::stdin().read_line(&mut s) {
            Ok(_) => s.trim().to_string(),
            Err(e) => {
                eprintln!("{}", e);
                str(tip)
            }
        }
    }

    fn uint(tip: &str) -> usize {
        match str(tip).parse::<usize>() {
            Ok(ui) => ui,
            Err(e) => {
                eprintln!("{}", e);
                uint(tip)
            }
        }
    }

    pub fn uint8(tip: &str) -> u8 {
        match str(tip).parse::<u8>() {
            Ok(ui) => ui,
            Err(e) => {
                eprintln!("{}", e);
                uint8(tip)
            }
        }
    }

    pub fn uint16(tip: &str) -> u16 {
        match str(tip).parse::<u16>() {
            Ok(ui) => ui,
            Err(e) => {
                eprintln!("{}", e);
                uint16(tip)
            }
        }
    }

    pub fn opt(tip: &str, opts: &Vec<&str>) -> usize {
        println!("{}", tip);
        for (i, opt) in opts.iter().enumerate() {
            println!("\t[{}] {}", i, opt);
        }
        let mut i = uint("Enter the number before the option you want:");
        while i >= opts.len() {
            i = uint("Out of Index");
        }
        i
    }
}

enum Tag {
    Default,
    Determined,
    Undetermined,
}

enum State {
    Covered(Tag),
    Discovered,
}

enum Object {
    Mine,
    Null(u8),
}

struct Cell {
    state: State,
    object: Object,
}

impl Cell {
    fn mine() -> Cell {
        Cell {
            state: State::Covered(Tag::Default),
            object: Object::Mine,
        }
    }

    fn null(n: u8) -> Cell {
        Cell {
            state: State::Covered(Tag::Default),
            object: Object::Null(n),
        }
    }

    fn click(&mut self) -> CellFeedback {
        if let State::Covered(tag) = &self.state {
            if let Tag::Determined = tag {
                CellFeedback::Protection
            } else {
                self.state = State::Discovered;
                match self.object {
                    Object::Mine => CellFeedback::Failure,
                    Object::Null(n) => match n {
                        0 => CellFeedback::ClickSurrounding,
                        _ => CellFeedback::Continue,
                    },
                }
            }
        } else {
            CellFeedback::Useless
        }
    }

    fn switch(&mut self) {
        if let State::Covered(tag) = &self.state {
            match tag {
                Tag::Default => self.state = State::Covered(Tag::Determined),
                Tag::Determined => self.state = State::Covered(Tag::Undetermined),
                Tag::Undetermined => self.state = State::Covered(Tag::Default),
            }
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.state {
                State::Covered(tag) => match tag {
                    Tag::Default => "*".to_string(),
                    Tag::Determined => "!".to_string(),
                    Tag::Undetermined => "?".to_string(),
                },
                State::Discovered => match self.object {
                    Object::Mine => "@".to_string(),
                    Object::Null(n) =>
                        if n == 0 {
                            " ".to_string()
                        } else {
                            n.to_string()
                        },
                },
            }
        )
    }
}

enum CellFeedback {
    Protection,
    Failure,
    ClickSurrounding,
    Continue,
    Useless,
}

struct Table {
    table: Vec<Vec<Cell>>,
    x: u8,
    y: u8,
}

impl Table {
    fn generate_random_points(num: u16, max_x: u8, max_y: u8) -> Option<HashSet<(u8, u8)>> {
        println!("Generating random points...");
        if num > (max_x as u16) * (max_y as u16) {
            return None;
        }
        let mut rng = rand::rng();
        let mut points = HashSet::new();
        while points.len() < num as usize {
            points.insert((rng.random_range(0..max_x), rng.random_range(0..max_y)));
        }
        Some(points)
    }

    fn generate_abstraction_by(points: HashSet<(u8, u8)>, x: u8, y: u8) -> Option<Vec<Vec<u8>>> {
        println!("Generating abstraction...");
        let mut abstraction = vec![vec![0; x as usize]; y as usize];
        for point in points {
            abstraction[point.1 as usize][point.0 as usize] = 1;
        }
        Some(abstraction)
    }

    fn calc_surrounding_mines(abstraction: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
        let mut surrounding_mines = 0;
        for delta_y in -1..=1 {
            let processed_y = y as isize + delta_y;
            for delta_x in -1..=1 {
                match abstraction.get(processed_y as usize) {
                    None => continue,
                    Some(row) => {
                        let processed_x = x as isize + delta_x;
                        match row.get(processed_x as usize) {
                            None => continue,
                            Some(status) => match *status {
                                0 => (),
                                1 => surrounding_mines += 1,
                                _ => unreachable!(),
                            },
                        }
                    }
                }
            }
        }
        surrounding_mines
    }

    fn transform(abstraction: Vec<Vec<u8>>) -> Option<Vec<Vec<Cell>>> {
        println!("Transforming abstraction...");
        let mut table = Vec::new();
        for (y, row) in abstraction.iter().enumerate() {
            table.push(Vec::new());
            for (x, cell) in row.iter().enumerate() {
                match *cell {
                    0 => {
                        table[y].push(Cell::null(Self::calc_surrounding_mines(&abstraction, x, y)))
                    }
                    1 => table[y].push(Cell::mine()),
                    _ => unreachable!(),
                }
            }
        }
        Some(table)
    }

    fn new(num_of_mines: u16, x: u8, y: u8) -> Option<Table> {
        let random_points = Self::generate_random_points(num_of_mines, x, y)?;
        let abstraction = Self::generate_abstraction_by(random_points, x, y)?;
        let table = Self::transform(abstraction)?;
        Some(Table { table, x, y })
    }

    fn check_if_successful(&self) -> TableFeedback {
        for row in &self.table {
            for cell in row {
                if let Object::Null(_) = cell.object {
                    if let State::Covered(_) = &cell.state {
                        return TableFeedback::Continue;
                    }
                }
            }
        }
        TableFeedback::Success
    }

    fn click(&mut self, x: usize, y: usize) -> TableFeedback {
        if x >= self.x as usize || y >= self.y as usize {
            return TableFeedback::Ignored;
        }
        match self.table[y][x].click() {
            CellFeedback::Protection => TableFeedback::Protection,
            CellFeedback::Failure => TableFeedback::Failure,
            CellFeedback::ClickSurrounding => {
                self.click_surrounding(x, y);
                self.check_if_successful()
            }
            CellFeedback::Continue => self.check_if_successful(),
            CellFeedback::Useless => TableFeedback::Ignored,
        }
    }

    fn click_surrounding(&mut self, x: usize, y: usize) {
        for delta_y in -1..=1 {
            let processed_y = y as isize + delta_y;
            for delta_x in -1..=1 {
                let processed_x = x as isize + delta_x;
                self.click(processed_x as usize, processed_y as usize);
            }
        }
    }

    fn switch(&mut self, x: usize, y: usize) {
        if x >= self.x as usize || y >= self.y as usize {
            return;
        }
        self.table[y][x].switch();
    }

    fn uncover_all(&mut self) {
        for row in self.table.iter_mut() {
            for cell in row {
                cell.state = State::Discovered;
            }
        }
    }

    fn print(&self) {
        print!("{:<5}", "");
        for x in 0..self.x {
            print!("{} ", ('a' as u8 + x) as char);
        }
        println!();
        for (y, row) in self.table.iter().enumerate() {
            print!("{:<5}", y);
            for cell in row {
                print!("{} ", cell);
            }
            print!("{:>5}", y);
            println!();
        }
        print!("{:<5}", "");
        for x in 0..self.x {
            print!("{} ", ('a' as u8 + x) as char);
        }
        println!();
    }
}

enum TableFeedback {
    Ignored,
    Protection,
    Failure,
    Continue,
    Success,
}

pub struct Game {
    at_first: bool,
    table: Table,
}

impl Game {
    pub fn new(num_of_mines: u16, x: u8, y: u8) -> Option<Game> {
        if x > 26 || x == 0 || y == 0 {
            return None;
        }
        Some(Game {
            at_first: true,
            table: Table::new(num_of_mines, x, y)?,
        })
    }

    fn square(num_of_mines: u16, a: u8) -> Option<Game> {
        Self::new(num_of_mines, a, a)
    }

    pub fn default() -> Option<Game> {
        Self::square(10, 9)
    }

    fn help(&self) {
        println!("Usage:");
        println!("\t<command><column letter><row number>");
        println!("Commands:");
        println!("\tx: Open Cell");
        println!("\ts: Switch Tag");
        println!();
    }

    fn get_operation(&mut self) -> Operation {
        if self.at_first {
            self.help();
            self.at_first = false;
        }
        let op = input::str("Operation:");
        if op.len() < 3 {
            return Operation::Help;
        }
        let bs = op.as_bytes();
        if bs[1] >= 'a' as u8 && bs[1] <= 'z' as u8 {
            let x = bs[1] - 'a' as u8;
            let y = match op.as_str()[2..].parse::<u8>() {
                Ok(n) => n,
                Err(_) => return Operation::Help,
            };
            match bs[0] as char {
                'x' => Operation::Click(x, y),
                's' => Operation::Switch(x, y),
                _ => Operation::Help,
            }
        } else {
            Operation::Help
        }
    }

    pub fn play(&mut self) {
        loop {
            self.table.print();
            match self.get_operation() {
                Operation::Help => self.at_first = true,
                Operation::Click(x, y) => match self.table.click(x as usize, y as usize) {
                    TableFeedback::Ignored => (),
                    TableFeedback::Protection => {
                        println!("Protection: You've already planted a flag here.")
                    }
                    TableFeedback::Failure => {
                        self.table.uncover_all();
                        self.table.print();
                        println!("You Lose");
                        break;
                    }
                    TableFeedback::Continue => (),
                    TableFeedback::Success => {
                        self.table.uncover_all();
                        self.table.print();
                        println!("You Win");
                        break;
                    }
                },
                Operation::Switch(x, y) => self.table.switch(x as usize, y as usize),
            }
        }
    }
}

enum Operation {
    Help,
    Click(u8, u8),
    Switch(u8, u8),
}
