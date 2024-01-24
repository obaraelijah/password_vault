mod pentry;


fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#" 
    ___________    ______ ________  _  _____________  __| _/ ___  _______   __ __|  |_/  |_  
    \____ \__  \  /  ___//  ___/\ \/ \/ /  _ \_  __ \/ __ |  \  \/ /\__  \ |  |  \  |\   __\ 
    |  |_> > __ \_\___ \ \___ \  \     (  <_> )  | \/ /_/ |   \   /  / __ \|  |  /  |_|  |   
    |   __(____  /____  >____  >  \/\_/ \____/|__|  \____ |    \_/  (____  /____/|____/__|   
    |__|       \/     \/     \/                          \/              \/                  
    
    "#;          
    println!("{ascii}");
    loop {
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search");
        println!("4. Quit");
    } 
}