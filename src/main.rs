extern crate colored;
use std::io::Write;
use regex::Regex;
use walkdir::{WalkDir};
use colored::*;
use std::fs::File;
use std::io::{BufReader, BufRead};

// Struct to store all the vulns/gas optimization and where they occur in the target
#[derive(Clone, Debug)]
struct RulesDataBase{
    id: String,
    description: String,
    location: Vec<String>,
    rule: String,
}
// Initialize the struct
impl Default for RulesDataBase {
    fn default () -> RulesDataBase {
        RulesDataBase {id: String::new(), description: String::new(), location: Vec::new(), rule: String::new()}
    }
}

static BANNER: &'static str = "
███████████                       █████             ████ 
░░███░░░░░███                     ░░███             ░░███ 
 ░███    ░███  █████ ████  █████  ███████    ██████  ░███ 
 ░██████████  ░░███ ░███  ███░░  ░░░███░    ███░░███ ░███ 
 ░███░░░░░███  ░███ ░███ ░░█████   ░███    ░███ ░███ ░███ 
 ░███    ░███  ░███ ░███  ░░░░███  ░███ ███░███ ░███ ░███ 
 █████   █████ ░░████████ ██████   ░░█████ ░░██████  █████
░░░░░   ░░░░░   ░░░░░░░░ ░░░░░░     ░░░░░   ░░░░░░  ░░░░░  ";

fn main() {
    banner();
    let path_name: String<> = String::from("test_folders"); // The directory where the solidity files are located
    let rule_path: String<> = String::from("rules"); // The directory where the rules files are located
    creating_rules(); // Creating new rules, if the user wants it
    let rules = filling_in_struct(&rule_path); // filling in the struct with rules
    let mut results: Vec<RulesDataBase>;
    let files_names: Vec<String> = is_solidity_file(&path_name); // Vector with solidity files to analyze


    println!("\n{}","Solidity files found:".blue());
    for file in &files_names{
        println!("{}", file.on_white().black())
    }
    print!("\n{}","---------Analyzing---------\n\n".purple());

    let created_file = std::fs::File::create("Results.md").expect("(Writing_to_a_file error) -> It was not possible to create the output");
    for file in &files_names{
        results = analyzing(file.to_string(), &path_name.to_string(), &rules); // Analyzing every single solidity file
        if writing_to_a_file(&results, &created_file){ // writing the results into a file
            print!("{}", "Output file created Successfully".green())
        }
        print!("\n{}{}\n","---------Results: ".green(),file.green());

        // This loop will print all the vulns/gas found by the analyzing function
        for result in results{
            if result.location.len() > 0{
                print!("---->{}\n",result.description.bright_yellow().bold());
                for (index, bug )in result.location.into_iter().enumerate(){
                    print!("{}{}#:\n","Case ".blue(), index);
                    print!("{}\n\n", bug.bright_red().bold());
                }
            }
        }
    }

}

// analyze the code using rules
// returns a struct with the results
fn analyzing(file_name: String, path_name: &String, rules: &Vec<RulesDataBase>) -> Vec<RulesDataBase>{
    let file = File::open(format!("{}/{}", path_name, file_name)).expect("(analyzing) - Failed to open the file");
    let reader = BufReader::new(file);
    let mut analyzed_block: Vec<RulesDataBase> = rules.to_vec();

    for (index, line) in reader.lines().enumerate(){
        let string_to_analyze = line.expect("(analyze) - failed to read the lines of the file");
        for (indexy, rule )in rules.into_iter().enumerate(){
            let re = Regex::new(rule.rule.as_str()).unwrap();
            if re.is_match(string_to_analyze.as_str()){
                analyzed_block[indexy].location.push(format!("{}{}{} -> {}","(Line-",index,")", string_to_analyze.as_str() ));
            }

        }

        
    }
    analyzed_block
}

// find solidity files
// returns a vector with all the solidity files found with
fn is_solidity_file(path_name: &String) -> Vec<String>{
    let mut files_names: Vec<String> = vec![];
    for entry in WalkDir::new(&path_name).into_iter().filter_map(|e| e.ok()){
    if entry.file_name()
                    .to_str()
                    .map(|s| s.ends_with(".sol"))
                    .unwrap_or(false){
                        files_names.push(entry.file_name().to_string_lossy().to_string())
                    }
    
    }

    files_names
                
}

// fills in ´RulesDataBase´ with rules inside 'rules/ '
// returns ´RulesDataBase´ filled with all the rules found
fn filling_in_struct(file_path: &String) -> Vec<RulesDataBase>{
    print!("{}|","Rules: ".blue());
    let mut files_names: Vec<String> = vec![];
    for entry in WalkDir::new(&file_path).into_iter().filter_map(|e| e.ok()){
        if  entry.file_name()
                    .to_str()
                    .map(|s| s.starts_with("rule_"))
                    .unwrap_or(false){
            files_names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    let mut rules_book: Vec<RulesDataBase> = vec![];
    for name in files_names{
        let file = File::open(format!("{}/{}", file_path, name)).expect("(reading the rules) - Failed to read the rules");
        let reader = BufReader::new(file);
        let mut rule = RulesDataBase::default();

        for (index, line) in reader.lines().enumerate(){
            let string_to_analyze = line.expect("(analyze) - failed to read the lines of the file");
            match index{
                0 => rule.id = string_to_analyze,
                1 => rule.description = string_to_analyze,
                2 => rule.rule = string_to_analyze,
                _ => ()
            }

        }
        print!("{}|", rule.id.green());
        rules_book.push(rule);
    }
    print!("\n");
    rules_book
}

// writes the results into a file
// returns true if everything went well
fn writing_to_a_file(results: &Vec<RulesDataBase>,mut file: &File) -> bool{

    file.write_all(BANNER.as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
    file.write_all(format!("{}{}{}","\n                   Hello, Welcome to Rustol 😀","\n\n---> Made by Jansen Cersosimo Moreira","\nsite: https://jansencmoreira.tech\nemail: jansenc@jansencmoreira.tech\n\n").as_bytes()).expect("(Writing_to_a_file error) -> It was not possible to creat the output");

    let analyzed_block: Vec<RulesDataBase> = results.to_vec();
    for result in analyzed_block{
        if result.location.len() > 0{
            file.write_all(format!("---->{}\n",result.description).as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
            for (index, bug )in result.location.into_iter().enumerate(){
                file.write_all(format!("{}{}#:\n","Case ", index).as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
                file.write_all(format!("{}\n\n", bug).as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
            }
        }
    }

    true
}

// Create rules from user input
// no return
fn creating_rules(){
    println!("{}", "----Rule-Creation----\n");
    loop{
        let mut s_or_n = String::new();
        println!("Want to add a new rule?{}", "(s/N): ");
        std::io::stdin().read_line(&mut s_or_n).expect("failed to readline");
        match s_or_n.as_str().trim_end() {
            "s" => (),
            "S" => (),
            _ => break
        }
        let mut id = String::new();
        let mut description = String::new();
        let mut re = String::new();
        println!("id:");
        std::io::stdin().read_line(&mut id).expect("failed to readline");
        println!("Description:");
        std::io::stdin().read_line(&mut description).expect("failed to readline");
        println!("Regex:");
        std::io::stdin().read_line(&mut re).expect("failed to readline");
        receiving_rules_from_user(id, description, re);
    }

}

// Writes the rules into a txt file
// no return
fn receiving_rules_from_user(id: String, description: String, re: String){
    let mut my_file = std::fs::File::create(format!("rules/rule_{}.txt", &id.trim_end())).expect("creation failed");
    my_file.write_all(format!("{}",id).as_bytes()).expect("write failed");
    my_file.write_all(format!("{}",description).as_bytes()).expect("write failed");
    my_file.write_all(format!("{}",re).as_bytes()).expect("write failed");
    println!("{}","-----New-Rule-Created-----".on_bright_green().blink());
}

// prints the banner
// no return
fn banner(){
    print!("{}",BANNER.bright_red());
    println!("{}{}{}","\n                   Hello, Welcome to Rustol 😀".blue(),"\n\n---> Made by Jansen Cersosimo Moreira".green(),"\nsite: https://jansencmoreira.tech\nemail: jansenc@jansencmoreira.tech\n\n".yellow());
    print!("{}","---------Starting----------\n\n".purple()); 
}