use ncurses::*;
use std::io;

const REGULAR: i16 = 0;
const HIGHLIGHT: i16 = 1;

fn init_ncurses() {
    initscr();
    noecho();
    start_color();
    init_pair(REGULAR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT, COLOR_BLACK, COLOR_GREEN);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    clear();
}

fn main() {
    init_ncurses();
    let mut quit = false;
    let mut todo_list = Vec::from([
        "Read book".to_string(),
        "Play with the kids".to_string(),
        "Walk the wife".to_string(),
    ]);
    let mut todo_cur: usize = 0;
    while !quit {
        display_todo_list(&mut todo_list, todo_cur);
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            // ARROW KEYS
            'A' => {
                if todo_cur != 0 {
                    todo_cur -= 1;
                }
            } // up
            'B' => {
                if todo_cur != todo_list.len() - 1 {
                    todo_cur += 1
                }
            } // down
            'D' => quit = true,                      // left
            'C' => quit = true,                      // right
            ' ' => quit = true,                      // SPACE
            '\n' => create_new_task(&mut todo_list), // ENTER
            _ => {
                // let key_as_char = key as u8;
                // addstr(&key_as_char.to_string());
            }
        }
    }

    endwin();
}

fn display_todo_list(todo_list: &mut Vec<String>, todo_cur: usize) {
    for (index, todo) in todo_list.iter().enumerate() {
        let pair = {
            if todo_cur == index {
                HIGHLIGHT
            } else {
                REGULAR
            }
        };
        attron(COLOR_PAIR(pair));
        mv(index as i32, 1);
        addstr(todo);
        attroff(COLOR_PAIR(pair));
    }

    mv(todo_list.len() as i32 + 1, 1);
    addstr("-------------------MENU----------------------");
    addstr("\n Press ENTER to add new task\n Press q to quit");
    refresh();
}

fn create_new_task(todo_list: &mut Vec<String>) {
    endwin();

    let mut new_task = String::new();
    clean_up_terminal();
    println!("Enter task name: ");
    io::stdin().read_line(&mut new_task).expect("error ");

    if !(new_task.trim().is_empty()) {
        todo_list.push(new_task);
    }

    init_ncurses();
    //  display_todo_list(todo_list, 0);
}

fn clean_up_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
