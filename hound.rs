// Last Writing 20/07/2023

use std::env; // To collect arguments.
use std::path::Path; // To have the path of a file as object.
use std::process::{Command, Stdio}; // To spawn commands and use pipes.

use std::alloc::System; // Remove jemalloc to minimise size.
#[global_allocator]
static A: System = System;

const H_COL: [&str;7] = [
"\x1b[0m",   // 0: Default
"\x1b[31m",  // 1: Red
"\x1b[32m",  // 2: Green (file )
"\x1b[36m",  // 3: Cyan  ( folder )
"\x1b[33m",  // 4: Highlight
"\x1b[4m",   // 5: Underline
"\x1b[1m",   // 6: Bold
];
const H_ICO: &str = 
"
       ~                                     
   ~.  G7                                    
   :5~.?P!.                                  
    !YJJ5YJ!!.                    .^!!~^^.   
   ~JPYYYYY?!!~.               .~J5555555Y7. 
   7JYY??5Y!7?!               :Y55Y555YJ?JY! 
 .^JY55JYY77JY!              :Y5Y55Y?^.    . 
  ~J7:?Y?JY55Y7              ?555?^.         
     ~PYJYYYJ?7^...  ..... .75Y?~.           
     JB5?JJYJYY5YYYYY55555YJJ?^              
    .GG5J55Y5YY55YYY55YYY55YY?^              
     !5YYYYYY5YYY5YYYY5YJY5YY55J.            
      ~YJYYYYJ??JJYJ^.7Y5YYYYY5J.            
       :?J55???7!~:    JYYJY5YY!             
        :JY5?          !5J77Y5J^             
         !55^           !Y?^.?57             
         ^5J             :YJ. ^J7            
         !Y7              .YJ.  7J~          
         JJJ:              J~    J?          
        ^Y~J~             ?!     J!          
     ..^J!~?.          :^!~   .:~J:          
    .^^::...                  ...            
";

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let args_len: usize = raw_args.len();
    if args_len==1 { help(&raw_args[0]); return; }
    let mut baked_args: String = String::new();
    let mut ang_factor: usize = 1;


    // Arguments manager.
    for i in 1..=(args_len-1) {
        if raw_args[i].chars().nth(0).unwrap() != '-' {
            // implement the file verif and target identifier.
            target_identifier(&raw_args[i], &baked_args, ang_factor);
            continue;
        }

        match raw_args[i].as_str() {
            "--help"  => { help(&raw_args[i-1]); continue; },
            "--chomp" => { baked_args.push('c'); continue; },
			_ => (),
        }
		if raw_args[i].contains("--angry") {
			if angry(&raw_args[i], &mut ang_factor) == false {return;}
			continue;
		}
		
		if raw_args[i].chars().nth(1).unwrap() != '-' { // I need to fix this, the argument that contains only one dash ( - ) and doesn't exist aren't detected..
			if raw_args[i].contains('a') {
				if angry(&raw_args[i], &mut ang_factor) == false {return;}
			}
			if raw_args[i].contains('c') {
				baked_args.push('c');
			}
			continue;
		}
		err_arg(&raw_args[i]);
		return;
    }
}


// main-functions
fn target_destroyer(target: &str) -> bool { 
    if is_file(target) {
        let output = Command::new("shred")
            .arg(target)
            .output()
            .expect(format!("{}Hound have a problem with his jaw..{}", &H_COL[1], &H_COL[0]).as_str());
        if output.status.success() {
            return true;}
    }
    return false;
}

fn target_identifier(path: &String, args: &String, ang: usize) {
    let find = Command::new("find") // desired input 'find $path | sort --reverse'.
        .arg(path)
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("{}It seems that Hound cannot see the target!{}", &H_COL[1], &H_COL[0]).as_str());
    let sort = Command::new("sort")
        .arg("--reverse")
        .stdin(find.stdout.unwrap())
        .output()
        .expect(format!("{}It seems that Hound cannot see the target!{}", &H_COL[1], &H_COL[0]).as_str());

    let result = String::from_utf8_lossy(&sort.stdout);
    let mut targets: Vec<_> = result.split("\n").collect();
	let mut nb_tdone: usize = 0;
	let nb_tgoal: usize = (targets.len()-1)*ang +targets.len()-1;
    targets.pop();

    for target in targets {
        for iteration in 1..=ang {

            if is_file(target) {
                if target_destroyer(target) {
                    println!("\r\x1B[K- {}{}'{}'{}{} : Chomped! x{}{}", &H_COL[2], &H_COL[6], &target, &H_COL[0], &H_COL[1], &iteration, &H_COL[0]);}
                else {
                    println!("\r\x1B[K- {}{}'{}'{}{} : It seems that Hound struggle with the file{}", &H_COL[2], &H_COL[6], &target, &H_COL[0], &H_COL[1], &H_COL[0]);}
            }
			nb_tdone+=1;
			loading_bar(&nb_tdone, &nb_tgoal);
        }

        let target_was_file: bool = is_file(target);
        if !args.contains('c') {
            let rm = Command::new("rm")
                .arg("-r")
                .arg(target)
                .output()
                .expect(format!("{}Hound have a problem with his jaw..{}", &H_COL[1], &H_COL[0]).as_str());
            if rm.status.success() {
                if target_was_file {
                    println!("\r\x1B[K- {}{}'{}'{} : {}Devoured!{}", &H_COL[2], &H_COL[6], &target, &H_COL[1], &H_COL[6], &H_COL[0]);    
                    println!("");
				}
                else {
                    println!("\r\x1B[K- {}{}'{}'{} : {}Forgot!{}", &H_COL[3], &H_COL[6], &target, &H_COL[1], &H_COL[6], &H_COL[0]);
                    println!("");
				}
            }
            else {
                if target_was_file {
                    println!("\r\x1B[K- {}{}'{}'{} : It seems that Hound struggle with this file..{}", &H_COL[2], &H_COL[6], &target, &H_COL[1], &H_COL[0]);}
                else {
                    println!("\r\x1B[K- {}{}'{}'{} : It seems that Hound struggle with this folder..{}", &H_COL[3], &H_COL[6], &target, &H_COL[1], &H_COL[0]);}
            }
        }

        if does_exists(target) && !args.contains('c') {
            println!("");
            println!("{}Hound couldn't devour the file {}'{}'{}", &H_COL[1], &H_COL[2], &target, &H_COL[0]);
            println!("{}  - is a software protecting the file?{}", &H_COL[1], &H_COL[0]);
            println!("{}  - do you have the right to destroy this file?{}", &H_COL[1], &H_COL[0]);
            println!("");
        }
		nb_tdone+=1;
		loading_bar(&nb_tdone, &nb_tgoal);

    }
}

fn angry(part: &String, ang: &mut usize) -> bool {
	let nb_char: usize = part.chars().count();
	let index_eq: usize;
	let mut full: bool = true;

	if let Some(index) = part.chars().position(|c| c == 'a') {

		if index+1 <= nb_char-1 && part.chars().nth(index+1).unwrap() == '=' {
			index_eq = index +2;
			full = false;
		}
		else if index+5 <= nb_char-1 && part.chars().nth(index+5).unwrap() == '=' {
			index_eq = index +6;}
	
		else{
			println!("{}There's a syntax error when trying to define the {}'angry'{} argument{}", &H_COL[1], &H_COL[4], &H_COL[1], &H_COL[0]);
			println!("  {}usage : --angry={}[x]{} | -a={}[x]{}", &H_COL[1], &H_COL[4], &H_COL[1], &H_COL[4], &H_COL[0]);
			return false;
		}
	}

	else{
		println!("{}Please provide an argument for {}'angry'{}", &H_COL[1], &H_COL[4], &H_COL[0]); 
		println!("  {}usage : --angry={}[x]{} | -a={}[x]{}", &H_COL[1], &H_COL[4], &H_COL[1], &H_COL[4], &H_COL[0]);
		return false;
		}
	if index_eq == nb_char {
		println!("{}Please provide an argument for {}'angry'{}", &H_COL[1], &H_COL[4], &H_COL[0]); 
		println!("  {}usage : --angry={}[x]{} | -a={}[x]{}", &H_COL[1], &H_COL[4], &H_COL[1], &H_COL[4], &H_COL[0]);
		return false;
	}

	else {
		let mut nb_vchar: usize = 0;
		if full == false {
			for i in index_eq..nb_char {
				if let Some(_) = part.chars().nth(i).unwrap().to_digit(10) { nb_vchar+=1; }
			}
		}
		else { nb_vchar = nb_char-index_eq }

		match part[index_eq..index_eq+nb_vchar].parse::<usize>() {
			Ok(f_val) => {
				*ang = f_val;
				return true;
			}
			Err(_) => {
				println!("{}The value {:?} cannot be assigned to {}'angry'{}", &H_COL[1], &part[index_eq..nb_char], &H_COL[4], &H_COL[0]);
				return false;
			}
		}
	}
}

fn help(args: &String) {
    match args.as_str() {
        _ => main(),}

    fn main() {
        logo_display();
        println!("{}Here's everything you need to know about Hound! (v2.2.0){}", &H_COL[5], &H_COL[0]);
        println!("  Hound is an independant software created by {}Thomas Wongsokarto{}", &H_COL[3], &H_COL[0]);
        println!("  Designed for destroying data, it should be used knowing the consequences!");
        println!("  Usage: hound <switches..> [folder/file]");
        println!("");
        println!("  {}<Switches>{}", H_COL[6], H_COL[0]);
        println!("  -a (--angry) : the file will get {}massacred{} [x] more time.", &H_COL[1], &H_COL[0]);
        println!("  -c (--chomp) : doesn't {}devour{} the file at the end.", &H_COL[1], &H_COL[0]);
        println!("  {}Example{} : 'hound -a=6c file' will chomp the file 6 times and not {}devour{} it.", &H_COL[6], &H_COL[0], &H_COL[1], &H_COL[0]);
        println!("");
        println!("  {}! DISCLAMER !{}", &H_COL[6], &H_COL[0]);
        println!("  {}ONLY{} use Hound, on your {}OWN{} datas!", &H_COL[4], &H_COL[0], &H_COL[4], &H_COL[0]);
        println!("  Being {}BAD{} is like {}{}REALLY BAD!{}", &H_COL[1], &H_COL[0], &H_COL[6], &H_COL[1], &H_COL[0]);
        println!("");

    }
}



// sub-functions
fn loading_bar(curr: &usize, goal: &usize) {
	let lb_len: usize = 50;
	let d: usize = curr*lb_len/goal;
	println!("\r\x1B[K{}[{}{}]{} {}% [{}/{}]", &H_COL[1], "*".repeat(d), " ".repeat(lb_len-d), &H_COL[0], d*100/lb_len, &curr, &goal);
	println!("\x1B[2A");
	if curr==goal {
		println!("\r\n");}
}

fn does_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    return path.exists();
}

fn is_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    return path.is_file();
}

fn err_arg(arg: &String) {
    println!("{}! Argument {}'{}'{} doesn't exist !{}", &H_COL[1], &H_COL[4], arg, &H_COL[1], &H_COL[0]);
    println!("{}You can consult the available argument by taping {}'--help'{}", &H_COL[1], &H_COL[4], &H_COL[0]);
}

fn logo_display() {
	println!("{}{}{}", &H_COL[1], &H_ICO, &H_COL[0]);}
