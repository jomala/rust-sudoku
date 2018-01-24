use rand;
use rand::Rng;
use std::iter::Iterator;
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Coords {
    pub x: u8,
    pub y: u8
}

impl Display for Coords {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "({}, {})", self.x, self.y)
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub digit: Option<u8>,
    pub fixed: bool
}

#[allow(dead_code)]
pub enum Neighbour {
	In,
	Out,
	OutOfBounds,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RegionId(u32);

impl RegionId {
	fn incr(self) -> RegionId {
		let RegionId(i) = self;
		RegionId(i + 1)
	}
}

impl Display for RegionId {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "Region({})", self.0)
	}
}

impl Debug for RegionId {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "Region({})", self.0)
	}
}

#[derive(Clone)]
pub struct Regions {
	pub sums: HashMap<RegionId, u32>,
	pub cells: [[RegionId; 9]; 9],
	pub next_id: RegionId,
}

impl Display for Regions {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "Sums:\n{:?}\n", self.sums)?;
		write!(f, "Regions:\n")?;
		for y in 0..9 {
			for x in 0..9 {
				let reg = self.cells[y as usize][x as usize].0;
				write!(f, "{:>3} ", reg)?;
			}
			write!(f, "\n")?;
		}
		write!(f, "\n")
	}
}

impl Regions {
	pub fn none() -> Regions {
		let sums = HashMap::new();
		let cells = [[RegionId(0); 9]; 9];
		let row_reg_id = RegionId(0);
		let regions = Regions { sums: sums, cells: cells, next_id: row_reg_id };
		regions
	}
	
	pub fn new() -> Regions {
		let mut sums = HashMap::new();
		let mut cells = [[RegionId(0); 9]; 9];
		let mut row_reg_id = RegionId(0);
		for y in 0..9 {
			sums.insert(row_reg_id, 45);
			for x in 0..9 {
				cells[y as usize][x as usize] = row_reg_id;
			}
			row_reg_id = row_reg_id.incr();
		}
		let regions = Regions { sums: sums, cells: cells, next_id: row_reg_id };
		regions
	}
	
	pub fn join(&mut self, reg_id_a: RegionId, reg_id_b: RegionId) -> RegionId {
		let reg_id_sum = self.next_id;
		self.next_id = self.next_id.incr();
		
		let reg_a_sum = self.sums.remove(&reg_id_a).expect("Id not found");
		let reg_b_sum = self.sums.remove(&reg_id_b).expect("Id not found");
		self.sums.insert(reg_id_sum, reg_a_sum + reg_b_sum);
		
		for y in 0..9 {
			for x in 0..9 {
				let id = self.cells[y as usize][x as usize];
				if id == reg_id_a || id == reg_id_b {
					self.cells[y as usize][x as usize] = reg_id_sum;
				}
			}
		}
		
		reg_id_sum
	}
	
	pub fn remove(&mut self, loc: Coords, val: u32) -> RegionId {
		let reg_id = self.id(&loc);
		let count  = {self.count_region(reg_id)};
		{
			assert_eq!(self.sums.contains_key(&reg_id), true);
			let sum = self.sums.get_mut(&reg_id).unwrap();
			assert!(*sum >= val);
			if *sum == val {
				assert!(count == 1);
				return reg_id;
			}
			assert!(count > 1);
			*sum -= val;
		}
		
		let new_reg_id = self.next_id;
		self.next_id = self.next_id.incr();
		
		self.sums.insert(new_reg_id.clone(), val);
		self.cells[loc.y as usize][loc.x as usize] = new_reg_id;
		
		new_reg_id
	}
	
	pub fn id(&self, loc: &Coords) -> RegionId {
		self.cells[loc.y as usize][loc.x as usize]
	}
	
	pub fn is_neighbour(&self, reg_id: RegionId, loc: &Coords) -> bool {
        reg_id != self.id(&loc) &&
		((loc.x > 0 && reg_id == self.id(&Coords { x: loc.x - 1, y: loc.y })) ||
		 (loc.y > 0 && reg_id == self.id(&Coords { x: loc.x, y: loc.y - 1 })) ||
		 (loc.x < 8 && reg_id == self.id(&Coords { x: loc.x + 1, y: loc.y })) ||
		 (loc.y < 8 && reg_id == self.id(&Coords { x: loc.x, y: loc.y + 1 })))
	}
	
	pub fn count_region(&self, reg_id: RegionId) -> u32 {
		let mut count = 0;
		for y in 0..9 {
			for x in 0..9 {
				if self.id(&Coords { x, y }) == reg_id {
					count += 1;
				}
			}
		}
		count	
	}

	pub fn neighbours(&self, reg_id: RegionId) -> Vec<Coords> {
		let mut vec = Vec::new();
		for y in 0..9 {
			for x in 0..9 {
				let coords = Coords { x, y };
				if self.is_neighbour(reg_id, &coords) {
					vec.push(coords);
				}
			}
		}
		vec		
	}

	pub fn random_neighbour(&self, reg_id: RegionId) -> Coords {
		let vec = self.neighbours(reg_id);
		let idx = rand::thread_rng().gen_range(0, vec.len());
		vec[idx]
	}
	
	fn can_regions_join(&self, reg_id_a: RegionId, reg_id_b: RegionId, solution: &[[Cell; 9]; 9]) -> bool {
		let mut check = [false; 9];
		
		for y in 0..9 {
			for x in 0..9 {
				let c = Coords{ x, y};
				if self.id(&c) == reg_id_a || self.id(&c) == reg_id_b {
					match solution[y as usize][x as usize].digit {
						Some(v) => {
							if check[v as usize - 1] {
								return false; 
							}
							check[v as usize - 1] = true;
						}
						None => {
							assert!(false);
						}
					}
				}
			}
		}
        true
	}
}

#[derive(Clone)]
pub struct Field {
    pub cells: [[Cell; 9]; 9],
	pub regions: Rc<Regions>,
}

impl Display for Field {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "Cells:\n")?;
		for y in 0..9 {
			for x in 0..9 {
				match self.cells[y as usize][x as usize].digit {
					Some(d) => write!(f, "{}", d)?,
					None => write!(f, " ")?,
				};
			}
			write!(f, "\n")?;
		}

		write!(f, "Regions:\n{}\n", *(self.regions))
	}
}

impl Field {
    pub fn new() -> Field {
        let mut field = Field {
            cells: [[Cell{ digit: None, fixed: false }; 9]; 9],
			regions: Rc::new(Regions::none()),
        };
        field.fill_random();
        field
    }

	#[allow(dead_code)]
    pub fn new_with_regions() -> Field {
        let mut field = Field {
            cells: [[Cell{ digit: None, fixed: false }; 9]; 9],
			regions: Rc::new(Regions::new()),
        };
        field.fill_random_regions();
        field
    }

	pub fn count_empty(&self) -> u32 {
		let mut count = 0;
		for y in 0..9 {
			for x in 0..9 {
				if self.cells[y as usize][x as usize].digit.is_none() {
					count += 1;
				}
			}
		}
		count
	}
	
	pub fn count_options(&self) -> u32 {
		let mut temp = self.clone();
		let mut count = 0;
		for y in 0..9 {
			for x in 0..9 {
				if temp.cells[y as usize][x as usize].digit.is_none() {
					for n in 0..9 {
						if temp.find_conflict(&Coords{ x, y }, n).is_none()  {
							count += 1;						
						}
					}
				}
			}
		}
		count
	}
	
    pub fn get_cell(&mut self, x: u8, y: u8) -> &mut Cell {
        &mut self.cells[y as usize][x as usize]
    }
	
    pub fn find_conflict(&mut self, coords: &Coords,
                          digit: u8) -> Option<Coords> {
		// print!("Check {} as {}\n", coords, digit);
        for x in 0..9 {
            if x != coords.x {
                if let Some(cell_digit) = self.get_cell(x, coords.y).digit {
                    if cell_digit == digit {
						// print!("In row, found match at col {}\n", x);
                        return Some(Coords{ x: x, y: coords.y});
                    }
                }
            }
        }

        for y in 0..9 {
            if y != coords.y {
                if let Some(cell_digit) = self.get_cell(coords.x, y).digit {
                    if cell_digit == digit {
						// print!("In col, found match at row {}\n", y);
                        return Some(Coords{ x: coords.x, y: y});
                    }
                }
            }
        }

        let section = Coords{ x: coords.x / 3, y: coords.y / 3};
        for x in section.x * 3 .. (section.x + 1) * 3 {
            for y in section.y * 3 .. (section.y + 1) * 3 {
                if x != coords.x || y != coords.y {
                    if let Some(cell_digit) = self.get_cell(x, y).digit {
                        if cell_digit == digit {
							// print!("In box, found match at ({}, {})\n", x, y);
                            return Some(Coords{ x: x, y: y});
                        }
                    }
                }
            }
        }
		
		let reg_id = self.regions.id(coords);
		match self.find_region_conflict(coords, digit, reg_id) {
			Some(clash) => { return Some(clash); }
			None => {}
		}
		
		None
    }

    fn find_region_conflict(&mut self, coords: &Coords,
                            digit: u8, reg_id: RegionId)
							-> Option<Coords> {
		if self.regions.sums.len() == 0 { return None }
		let reg_sum = self.regions.sums[&reg_id];
		let mut sum: u32 = 0;
		let mut blank = 0;
		let mut last = Some(*coords);
		for y in 0..9 {
			for x in 0..9 {
				let c = Coords{ x, y};
				if c == *coords {
					//print!("Found self\n");
					sum += digit as u32;
				} else if self.regions.id(&c) == reg_id {
					last = Some(c);
					match self.cells[y as usize][x as usize].digit {
						Some(v) => {
							if v == digit {
								return last; 
							}
							sum += v as u32;
						}
						None => blank += 1,
					}
				}
			}
		}
		// print!("No same digit fail\n");
		// if sum > reg_sum {return last}
		// if sum == reg_sum && blank > 0 {return last}
		// if sum < reg_sum && blank == 0 {return last}
		if sum + (blank * (blank + 1) / 2) > reg_sum {return last}
		if sum + (blank * (19 - blank) / 2) < reg_sum {return last}
		
		// print("No conflict\n");
        None
	}
	
    pub fn clear(&mut self) {
        for y in 0..9 {
            for x in 0..9 {
                self.cells[y][x] = Cell{ digit: None, fixed: false };
            }
        }
		self.regions = Rc::new(Regions::new());
    }
	
    pub fn restart(&mut self) {
        for y in 0..9 {
            for x in 0..9 {
				if !self.cells[y][x].fixed {
					self.cells[y][x] = Cell{ digit: None, fixed: false };
				}
            }
        }
    }
	
	pub fn random_cells(&self) -> [[Cell; 9]; 9] {
		// Create a blank board with a single guess
		let mut field = self.clone();
        field.clear();
        let x = rand::thread_rng().gen_range(0u8, 9u8);
        let y = rand::thread_rng().gen_range(0u8, 9u8);
        let digit = rand::thread_rng().gen_range(1u8, 10u8);
        field.get_cell(x, y).digit = Some(digit);
		
		// Find a solution from that
        field.find_solution().unwrap().cells
	}

    pub fn fill_random(&mut self) {
		{
			self.cells = self.random_cells();
		}
		let mut fails = 20;

        while fails > 0 {
			// Try blanking a cell currently with a digit
            let mut x;
            let mut y;
            let digit;

            loop {
                x = rand::thread_rng().gen_range(0u8, 9u8);
                y = rand::thread_rng().gen_range(0u8, 9u8);
                if self.get_cell(x, y).digit.is_none() {
                    continue;
                }
                digit = self.get_cell(x, y).digit.unwrap();
                self.get_cell(x, y).digit = None;
                break;
            }

			// If there's still only one solution, continue
            let solutions = self.find_solutions(2);
            if solutions.len() == 1 {
                continue;
            }
			
			// Otherwise revert the change
            self.get_cell(x, y).digit = Some(digit);
			
			// Continue looking though for a set number of unsuccessful attempts
			fails -= 1;
        }
    }

    pub fn fill_random_regions(&mut self) {
		let solution_cells = self.random_cells();
		
		{
			// Create regions around every cell
			let region_list = Rc::get_mut(&mut self.regions).expect("Region found");
			for y in 0..9 {
				for x in 0..9 {
					let loc = Coords { x, y };
					let val = solution_cells[y as usize][x as usize].digit.unwrap() as u32;
					region_list.remove(loc, val);
				}
			}
		}
		
		{
			print!("Start\n{}\n", self);
			let solutions = self.find_solutions(3);
			assert!(solutions.len() == 1);
		}
		
		let mut fails = 3;

        'outer2: while fails > 0 {
			let regions_copy = {(*self.regions).clone()};
			{
				// Get the regions
				let reg_mut = Rc::get_mut(&mut self.regions).unwrap();
				let mut n_idx;
				let mut r_idx;
				
				while {
					// Pick a region
					assert!(reg_mut.sums.len() > 1);
					r_idx = {
						let x = rand::thread_rng().gen_range(0u8, 9u8);
						let y = rand::thread_rng().gen_range(0u8, 9u8);
						let loc = Coords { x, y };
						reg_mut.id(&loc)
					};
					
					// Pick a random neighbouring region
					n_idx = reg_mut.id(&reg_mut.random_neighbour(r_idx));
					
					!reg_mut.can_regions_join(r_idx, n_idx, &solution_cells)
				}
				{
					fails -= 1;
					if fails <= 0 {
						break 'outer2;
					}
				}
				// Attempt to join the regions
				let new_idx = reg_mut.join(r_idx, n_idx);
				print!("Trying {} + {} => {}", r_idx, n_idx, new_idx);
			}
			
			// See if there's still only one solution
            let solutions = self.find_solutions(3);
			// Might be zero if regions are too big
			// assert!(solutions.len() > 0);
			
            if solutions.len() == 1 {
				print!("One solution\n");
                continue;
            }
			
			// If not, revert and try again
			print!("{} solutions\n", solutions.len());
			self.regions = Rc::new(regions_copy);
			fails -= 1;
        }
    }

    pub fn fill_solution(&mut self) {
        if let Some(s) = self.find_solution() {
            self.cells = s.cells;
        }
    }

    pub fn find_solution(&mut self) -> Option<Field> {
        let solutions = self.find_solutions(1);
        if solutions.len() > 0 {
            return Some(solutions[0].clone());
        }
        None
    }

    pub fn find_solutions(&mut self, stop_at: u32) -> Vec<Field> {
        let mut solutions = Vec::new();
        let mut field = self.clone();
        field.find_solutions_impl(&mut solutions, stop_at);
        solutions
    }

    fn find_solutions_impl(&mut self, solutions: &mut Vec<Field>,
                           stop_at: u32) -> bool {
        let mut empty_cell: Option<Coords> = None;
        'outer: for y in 0..9 {
            'inner: for x in 0..9 {
                if self.get_cell(x, y).digit.is_none() {
                    empty_cell = Some(Coords{ x: x, y: y });
                    break 'outer;
                }
            }
        }

        if empty_cell.is_none() {
			// print!("Potential solution:\n{}\n", self);
            solutions.push(self.clone());
            return solutions.len() >= (stop_at as usize);
        }
        let coords = empty_cell.unwrap();

        let mut digits: Vec<u8> = (1..10).collect();
        rand::thread_rng().shuffle(&mut digits);

        for &digit in digits.iter() {
            if self.find_conflict(&coords, digit).is_none() { 
                self.get_cell(coords.x, coords.y).digit = Some(digit);
                if self.find_solutions_impl(solutions, stop_at) {
                    return true;
                }
                self.get_cell(coords.x, coords.y).digit = None;
            }
        }

        false
    }
}

#[test]
pub fn test_without_regions() {
	let field = Field::new();
	print!("Random Field:\n{}", field);
}

#[test]
pub fn test_regions() {
	let field = Field::new_with_regions();
	print!("Random Field:\n{}", field);
}