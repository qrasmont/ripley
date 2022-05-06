use std::fs::File;
use std::io::{self, prelude::*, BufReader, stdout};
use std::{thread, time};
use crossterm::{ExecutableCommand, QueueableCommand, terminal, cursor};

fn main() -> io::Result<()> {
    let file = File::open("test.log")?;
    let reader = BufReader::new(file);
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for read in reader.lines() {
        if let Ok(line) = read {
            if line.contains("+ ") || line.contains("~ "){

                stdout.queue(cursor::SavePosition).unwrap();
                stdout.execute(terminal::Clear(terminal::ClearType::All))?;

                let split = line.split(" -> ");

                for (i, name) in split.enumerate() {

                    let tabs = "\t".repeat(i);
                    stdout.write(format!("{}{}\n", tabs, name).as_bytes()).unwrap();
                }

                stdout.queue(cursor::RestorePosition).unwrap();
                stdout.flush().unwrap();

                thread::sleep(time::Duration::from_millis(500));
            }
        }
    }

    Ok(())
}
