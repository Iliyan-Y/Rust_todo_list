use std::io;

use ncurses::*;

fn init_ncurses() {
    initscr();
    noecho();
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

    display_todo_list(&mut todo_list);

    while !quit {
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            // ARROW KEYS
            'A' => quit = true, // up
            'B' => quit = true, // down
            'D' => quit = true, // left
            'C' => quit = true, // right
            // add new todo
            ' ' => quit = true,                      // SPACE
            '\n' => create_new_task(&mut todo_list), // ENTER
            _ => {
                let key_as_char = key as u8 as char;
                addstr(&key_as_char.to_string());
            }
        }
    }

    endwin();
}

fn display_todo_list(todo_list: &mut Vec<String>) {
    for (index, todo) in todo_list.iter().enumerate() {
        mv(index as i32, 1);
        addstr(todo);
    }

    mv(todo_list.len() as i32 + 1, 1);
    addstr("Press q to quit");
    refresh();
}

fn create_new_task(todo_list: &mut Vec<String>) {
    let mut new_task = String::new();

    endwin();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Enter task name: ");
    io::stdin().read_line(&mut new_task).expect("error ");

    todo_list.push(new_task);
    init_ncurses();
    display_todo_list(todo_list);
}
