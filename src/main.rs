use ncurses::*;

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut quit = false;
    let mut todo_list = Vec::from(["Read book", "Play with the kids", "Walk the wife"]);

    display_todo_list(&mut todo_list);

    while !quit {
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            // ARROW KEYS
            'A' => quit = true,                 // up
            'B' => quit = true,                 // down
            'D' => add_to_list(&mut todo_list), // left
            'C' => quit = true,                 // right
            ' ' => quit = true,                 // SPACE
            '\n' => quit = true,                // ENTER
            // escae '`' => quit = true,
            _ => {
                // TODO: use to add new todo ???
                let key_as_char = key as u8 as char;
                addstr(&key_as_char.to_string());
            }
        }
    }

    endwin();
}

fn display_todo_list(todo_list: &mut Vec<&str>) {
    for (index, todo) in todo_list.iter().enumerate() {
        mv(index as i32, 1);
        addstr(todo);
    }

    mv(todo_list.len() as i32 + 1, 1);
    addstr("Press q to quit");
    refresh();
}

fn add_to_list(todo_list: &mut Vec<&str>) {
    clear();
    todo_list.push("new value");
    display_todo_list(todo_list);
}
