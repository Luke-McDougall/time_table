// ***********************************************************************************
// Luke McDougall
//
// Personal practice/educational project. This is a simple command line program that 
// prints my time table information. 
// Currently it supports:
// 1. Hard coded time table info some of which is printed depending on what day it is.
// 2. A file based todo list system that prints items in green if they have been completed
//    and in red if they are uncompleted.
// 3. A command line switch '-t' which will make the program print the current day's time
//    table as well as the nex day's. Friday wraps around to monday.
// 4. The calender system has been implemented boys.
//
// ************************************************************************************

use std::{fs, io::Read, io, fs::File, env};
extern crate term;

struct Class {
    name: String,
    location: String,
    duration: String,
}

struct Day {
    classes: Vec<Class>,
}

fn main() {
    // Structs containing timetable info for a given weekday. Hardcoded for now
    let fri = Day {
        classes: vec![Class {
            name: String::from("Statistics Lecture"),
            location: String::from("405 201"),
            duration: String::from("8:00 am - 10:00 pm"),
        },
        Class {
            name: String::from("Science Comms Workshops"),
            location: String::from("400 222"),
            duration: String::from("11:00 am - 1:00 pm"),
        },
        Class {
            name: String::from("Database Systems Prac"),
            location: String::from("204 327"),
            duration: String::from("2:00 pm - 4:00 pm"),
        }],
    };

    let thu = Day {
        classes: vec![Class {
            name: String::from("Microcomputers Tutorial"),
            location: String::from("213 104"),
            duration: String::from("4:00 pm - 5:00 pm"),
        }],
    };

    let wed = Day {
        classes: vec![Class {
            name: String::from("Microcomputers Lab"),
            location: String::from("204 237"),
            duration: String::from("10:00 am - 12:00 pm"),
        },
        Class {
            name: String::from("Statistics Workshop"),
            location: String::from("314 217"),
            duration: String::from("2:00 pm - 3:00 pm"),
        }],
    };

    let tue = Day {
        classes: vec![Class {
            name: String::from("Database Systems Lecture"),
            location: String::from("405 201"),
            duration: String::from("10:00 am - 12:00 pm"),
        }],
    };

    let mon = Day {
        classes: vec![Class {
            name: String::from("Computer Systems Lab"),
            location: String::from("207 113/7"),
            duration: String::from("10:00 am - 1:00 pm"),
        },
        Class {
            name: String::from("Microcomputers Lecture"),
            location: String::from("210 102"),
            duration: String::from("3:00 pm - 5:00 pm"),
       }],
    };

    // Array of structs containing timetable info for each weekday
    let week = [mon, tue, wed, thu, fri];

    let buffer = io::stdin();
    // The date value will be piped to this program by the date command
    let mut date: String = String::new();

    buffer.read_line(&mut date).unwrap();
    let mut iter = date.split_whitespace();

    // Format of the date string: weekday month day time timezone year
    // Eg. Fri Aug 16 19:06:15 AWST 2019
    let weekday = iter.next().unwrap();
    let month = iter.next().unwrap();
    let day = iter.next().unwrap();
    let time = iter.next().unwrap();
    iter.next();    // Don't care about timezone
    let year = iter.next().unwrap();

    // Hour value determines the type of greeting. Good morning/afternoon/evening
    let hour: u32 = time[0..2].parse().unwrap();
    if hour < 12 {
        println!("Good morning!");
    } else if hour < 18 {
        println!("Good afternoon!");
    } else {
        println!("Good evening!");
    }

    // Create a string with the date formatted in a way that looks nicer to me.
    let fdate = format!("{} {} {} {}", weekday, month, day, year);

    let mut terminal = term::stdout().unwrap();
    terminal.fg(term::color::YELLOW).unwrap();
    print!("{}", fdate);

    terminal.fg(term::color::WHITE).unwrap();
    print!(" ~ "); 

    terminal.fg(term::color::YELLOW).unwrap();

    println!("{}", time);

    let today_num = time_table::date_number(year, month, day);
    let mut buffer: Vec<u8> = Vec::new();
    let mut file = File::open("/home/luke/.config/timetable/calender_bin.t").unwrap();
    file.read_to_end(&mut buffer).unwrap();

    let mut idx = 0;
    let mut a;
    let mut b;
    let mut c;
    let mut s;

    loop {
        if idx >= buffer.len() {
            break;
        }
        a = buffer[idx + 1];
        b = buffer[idx + 2];
        c = (a as i16) << 8 | b as i16;

        if c - today_num < 0 {
            idx += buffer[idx] as usize;
            continue;
        } else if c - today_num > 7 {
            break;
        }

        unsafe {
            s = String::from_utf8_unchecked(buffer[idx + 3..idx + buffer[idx] as usize].to_vec());
        }
        idx += buffer[idx] as usize;
        if c - today_num < 3 {
            terminal.fg(term::color::RED).unwrap();
            println!("*{} in {} days!", s, c - today_num);
        } else if c - today_num < 5 {
            terminal.fg(term::color::YELLOW).unwrap();
            println!("*{} in {} days.", s, c - today_num);
        } else {
            terminal.fg(term::color::GREEN).unwrap();
            println!("*{} in {} days.", s, c - today_num);
        }
    }

    let todo_list = fs::read_to_string("/home/luke/.config/timetable/todo.t").unwrap_or_else(|err| {
        eprintln!("Error processing file {}", err);
        String::new()
    });
    

    if todo_list.len() > 0 {
        // Partition the list into completed and uncompleted
        let mut completed = Vec::new();
        let mut uncompleted = Vec::new();

        // Remove any trailing whitespace that could have been in the file
        let todo_list = todo_list.trim();
        for sen in todo_list.split('\n') {
            if &sen[0..1] == "u" {
                uncompleted.push(sen);
            } else {
                completed.push(sen);
            }
        }
        terminal.fg(term::color::WHITE).unwrap();
        println!("\nTODO");
        terminal.fg(term::color::GREEN).unwrap();

        for e in &completed {
            println!("-> {}...Completed", &e[1..]);  // Slice to avoid printing the c/u character
        }

        terminal.fg(term::color::WHITE).unwrap();
        println!("---"); 
        terminal.fg(term::color::RED).unwrap();

        for e in &uncompleted {
            println!("-> {}", &e[1..]); 
        }
    }


    terminal.fg(term::color::WHITE).unwrap();
    println!("\nToday's Schedule:");
    let today =  match weekday {
        "Mon" =>  0,
        "Tue" =>  1,
        "Wed" =>  2,
        "Thu" =>  3,
        "Fri" =>  4,
        _ => {
            println!("It's the weekend baby!");
            5
        },
    };

    if today < 5 {
        // -t command line switch causes tomorrows schedule to be printed as well
        let mut repeats = 1; 
        if let Some(arg) = env::args().skip(1).next() {
            if arg == "-t" {
                repeats += 1;
            } else {
                eprintln!("Unrecognized argument: {}", arg);
            }
        }
        for i in 0..repeats {
            if i == 1 {println!("\nTomorrow's Schedule:");}
            for class in week[(today + i) % 5].classes.iter() {
                print!("{}", class.name);
                print!(" -- ");
                terminal.fg(term::color::CYAN).unwrap();
                print!("{}", class.location);
                terminal.fg(term::color::WHITE).unwrap();
                print!(" -- ");
                terminal.fg(term::color::YELLOW).unwrap();
                println!("{}", class.duration);
                terminal.fg(term::color::WHITE).unwrap();
            }
        }
    }
}
