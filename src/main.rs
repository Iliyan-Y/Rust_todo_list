use ncurses::*;

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut quit = false;

    addstr("Hello, world!");
    refresh();
    while !quit {
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }

    endwin();
}
