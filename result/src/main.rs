#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Error".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn main() {
    println!("Hello, world!");

    let choice = get_choice("mainmenu");
    println!("choice = {:?}", choice);

    // Printing in the case when there might be an error
    // and the menu does not exist
    match choice {
        Ok(menu) => print_choice(&menu),
        Err(e) => println!("Error: {:?}", e),
    }
}
