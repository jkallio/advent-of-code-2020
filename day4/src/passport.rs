pub struct Passport {
    pub byr: Option<i32>,        // (Birth Year)
    pub iyr: Option<i32>,        // (Issue Year)
    pub eyr: Option<i32>,        // (Expiration Year)
    pub hgt: Option<i32>,        // (Height) in cm
    pub hcl: Option<String>,     // (Hair Color) as "#rgb"
    pub ecl: Option<String>,     // (Eye Color) as "#rgb"
    pub pid: Option<i32>,        // (Passport ID)
    pub cid: Option<i32>         // (Country ID)
}

impl Passport {
    pub fn new() -> Self {
        return Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.byr != None
            && self.iyr != None
            && self.eyr != None
            && self.hgt != None
            && self.hcl != None
            && self.ecl != None
            && self.pid != None
            //&& self.cid != None // Ignore Country ID
    }

    pub fn merge(&mut self, pass:&Passport) {
        if self.byr == None { self.byr = pass.byr; }
        if self.iyr == None { self.iyr = pass.iyr; }
        if self.eyr == None { self.eyr = pass.eyr; }
        if self.hgt == None { self.hgt = pass.hgt; }
        if self.hcl == None { self.hcl = pass.hcl.clone(); }
        if self.ecl == None { self.ecl = pass.ecl.clone(); }
        if self.pid == None { self.pid = pass.pid; }
        if self.cid == None { self.cid = pass.cid; }
    }

    pub fn print_all(&self) {
        println!("\r\nPassport: {}", self.is_valid());
        match self.byr { Some(x) => println!("byr = {}", x), None => println!("byr = N/a") }
        match self.iyr { Some(x) => println!("iyr = {}", x), None => println!("iyr = N/a") }
        match self.eyr { Some(x) => println!("eyr = {}", x), None => println!("eyr = N/a") }
        match self.hgt { Some(x) => println!("hgt = {}", x), None => println!("hgt = N/a") }
        match &self.hcl { Some(x) => println!("hcl = {}", x), None => println!("hcl = N/a") }
        match &self.ecl { Some(x) => println!("ecl = {}", x), None => println!("ecl = N/a") }
        match self.pid { Some(x) => println!("pid = {}", x), None => println!("pid = N/a") }
        match self.cid { Some(x) => println!("cid = {}", x), None => println!("cid = N/a") }
    }
}
