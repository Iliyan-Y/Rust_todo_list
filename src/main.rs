use ncurses::*;

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut quit = false;
    display_todo_list();

    refresh();
    while !quit {
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            // ARROW KEYS
            'A' => quit = true, // up
            'B' => quit = true, // down
            'D' => quit = true, // left
            'C' => quit = true, // right
            _ => {}
        }
    }

    endwin();
}

fn display_todo_list() {
    let mut todo_list = Vec::from(["Read book", "Play with the kids", "Walk the wife"]);

    for (index, todo) in todo_list.iter().enumerate() {
        mv(index as i32, 1);
        addstr(todo);
    }

    mv(todo_list.len() as i32 + 1, 1);
    addstr("Press q to quit");
}
