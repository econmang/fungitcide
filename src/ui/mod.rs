use tui::{
    backend::Backend,
    Frame,
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
};

pub fn draw_ui<B: Backend>(f: &mut Frame<B>) {
    let horiz_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(f.size());
   let vert_chunks_left = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(15),
            ].as_ref()
        )
        .split(horiz_chunks[0]);

    // Menu Bar
    let block = Block::default()
         .borders(Borders::NONE);
    f.render_widget(block, horiz_chunks[0]);

    // Editor Block
    let block = Block::default()
         .title("Editor")
         .borders(Borders::ALL);
    f.render_widget(block, horiz_chunks[1]);

    // Menu Items
    let block = Block::default()
         .title("Status")
         .borders(Borders::ALL);
    f.render_widget(block, vert_chunks_left[0]);
    let block = Block::default()
         .title("Files")
         .borders(Borders::ALL);
    f.render_widget(block, vert_chunks_left[1]);
    let block = Block::default()
         .title("Local Branches")
         .borders(Borders::ALL);
    f.render_widget(block, vert_chunks_left[2]);
    let block = Block::default()
         .title("Commits")
         .borders(Borders::ALL);
    f.render_widget(block, vert_chunks_left[3]);
    let block = Block::default()
         .title("Stash")
         .borders(Borders::ALL);
    f.render_widget(block, vert_chunks_left[4]);
}
