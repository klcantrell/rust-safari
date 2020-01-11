use gen_css_modules_type as gen_type;
use notify::event::ModifyKind;
use notify::{Event, EventKind, Result as NotifyResult, Watcher};

fn main() -> NotifyResult<()> {
    println!("Enter quit() to exit!");

    let handler = Box::new(|res| match res {
        Ok(Event {
            kind: EventKind::Modify(ModifyKind::Any),
            paths,
            attrs: _,
        }) => gen_type::handle_on_modify(paths),
        Err(e) => println!("watch error: {:?}", e),
        _ => (),
    });
    let mut watcher = gen_type::create_watcher(gen_type::PATH_TO_WATCH, handler)?;

    let mut line = String::new();
    gen_type::read_line(&mut line)?;
    while line.trim() != "quit()" {
        gen_type::read_line(&mut line)?;
    }

    watcher.unwatch(gen_type::PATH_TO_WATCH)?;
    Ok(())
}
