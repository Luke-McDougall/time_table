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
//    table as well as the next day's. Friday wraps around to monday.
//
// I plan to test some different implementation techniques for the features I already have
// and to add a more long term calender like system that prints information about important
// upcoming events within a certain time period before they happen. Probably one week.
// ************************************************************************************

use std::{io::Read, io, fs::File, env};
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
    let mut terminal = term::stdout().unwrap();
    let buffer = io::stdin();
    // The date value will be piped to this program by the date command
    let mut date: String = String::new();

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
    buffer.read_line(&mut date).unwrap();
    let date_vec: Vec<&str> = date.split_whitespace().collect();
    let hour: u32 = date_vec[3][0..2].parse().unwrap();

    // Hour value determines the type of greeting. Good morning/afternoon/evening
    if hour < 12 {
        println!("Good morning!");
    } else if hour < 18 {
        println!("Good afternoon!");
    } else {
        println!("Good evening!");
    }

    let fdate = format!("{} {} {} {}", date_vec[0], date_vec[1], date_vec[2], date_vec[5]);
    terminal.fg(term::color::YELLOW).unwrap();
    print!("{}", fdate);

    terminal.fg(term::color::WHITE).unwrap();
    print!(" ~ "); 

    terminal.fg(term::color::YELLOW).unwrap();

    println!("{}", date_vec[3]);

    let today_num = date_number(&date_vec);
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
        c = (a as u16) << 8 | b as u16;

        if (c as i16 - today_num as i16) < 0 {
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

    let file_result = File::open("/home/luke/.config/timetable/todo.t");
    let mut file;
    let mut todo_list: String = String::new();
    match file_result {
        Ok(f) => {
            file = f;
            file.read_to_string(&mut todo_list);
        },
        Err(e) => todo_list = format!(" Problem opening todo file {:?}", e),
    }


    // Partition the list into completed and uncompleted
    // NOTE: Think about using filter for this. It might be better, it might not
    //       only one way to find out!
    let mut completed = Vec::new();
    let mut uncompleted = Vec::new();

    for sen in todo_list.split('\n') {
        // I think I need this because of trailing whitespace in the file.
        // Try using trim on the string before splitting it.
        if sen.len() < 1 {continue;}
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
        println!("-> {}...Completed", &e[1..]);
    }

    terminal.fg(term::color::WHITE).unwrap();
    println!("---"); 
    terminal.fg(term::color::RED).unwrap();

    for e in &uncompleted {
        println!("-> {}", &e[1..]);
    }

    terminal.fg(term::color::WHITE).unwrap();
    println!("\nToday's Schedule:");
    let today;
    match date_vec[0] {
        "Mon" => today = 0,
        "Tue" => today = 1,
        "Wed" => today = 2,
        "Thu" => today = 3,
        "Fri" => today = 4,
        _ => {
            today = 5;
            println!("It's the weekend baby!");
        },
    }

    if today < 5 {
        let mut repeats = 1; 
        let args: Vec<String> = env::args().collect();
        // TODO: Print an error message if an argument is passed that isn't supported.
        if args.len() == 2 && args[1] == "-t" {
            repeats += 1;
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
    
    // weekday, month, day, time, timzone, year.
}

// This function generates the total number of days into the current year todays date is.
// This is so that finding the difference in days between the current day and calender
// entries is easy.
fn date_number(date_vec: &Vec<&str>) -> u16 {
    let month: u16 = match date_vec[1] {
        "Jan" => 0,
        "Feb" => 31,
        "Mar" => 59,
        "Apr" => 90,
        "May" => 120,
        "Jun" => 151,
        "Jul" => 181,
        "Aug" => 212,
        "Sep" => 243,
        "Oct" => 273,
        "Nov" => 304,
        "Dec" => 334,
        _ => panic!("Something went wrong, no valid month found."),
    };

    let day: u16 = date_vec[2].parse().unwrap();
    let year: u16 = date_vec.last().unwrap().parse().unwrap();
    if year % 4 == 0 && month > 31 {
        month + day + 1
    } else {
        month + day
    }
}
