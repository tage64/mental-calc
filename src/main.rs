use dialoguer::{Confirmation, Input};
use mental_calc::{Add, Mixed, Mul, Sub, TaskProducer};
use num_traits::identities::Zero;
use std::io::{stdin, stdout, Result as IoResult, Write};
use std::num::NonZeroUsize;
use std::time::Instant;

type Number = i64;

fn main() -> IoResult<()> {
    println!("Hello and Welcome!\n");
    main_menu()?;
    println!("Good buy!");
    Ok(())
}

fn main_menu() -> IoResult<()> {
    loop {
        match menu("Main menu", &["New task", "Quit"])?.into() {
            1 => {
                if let Some(mut x) = setup_task()? {
                    run_task(&mut x.0, x.1)?;
                }
            }

            2 => break Ok(()),
            _ => unreachable!(),
        }
    }
}

fn setup_task() -> IoResult<Option<(Mixed<Number>, u32)>> {
    let mut task_producers: Vec<Box<TaskProducer<Number = Number>>> = Vec::with_capacity(1);
    let task_producer = loop {
        let title = format!(
            "Setup tasks.\nYou have selected {} mathematical operations.",
            task_producers.len()
        );
        match menu(&title, &["Add task", "Done", "Back to main menu"])?.into() {
            1 => {
                if let Some(x) = add_task_producer()? {
                    task_producers.push(x)
                }
            }
            2 => {
                if task_producers.len() > 0 {
                    break Mixed::new(task_producers);
                } else {
                    println!("You need to select at least one type of mathematical operation.");
                }
            }
            3 => return Ok(None),
            _ => unreachable!(),
        }
    };
    let number_of_tasks = Input::new()
        .with_prompt("Number of tasks")
        .default(10)
        .interact()?;
    Ok(Some((task_producer, number_of_tasks)))
}

fn add_task_producer() -> IoResult<Option<Box<TaskProducer<Number = Number>>>> {
    let negative = Confirmation::new()
        .with_text("Do you want to include negative numbers?")
        .default(false)
        .show_default(true)
        .interact()?;
    match menu(
        "Select mathematical operation",
        &["Addition", "Subtraction", "Multiplication", "Cancel"],
    )?
    .into()
    {
        1 => {
            let result_max = loop {
                let x: Number = Input::new().with_prompt("Maximum result").interact()?;
                if x <= 0 {
                    println!("Error: Must be grater than zero.");
                } else {
                    break x;
                }
            };
            Ok(Some(Box::new(Add::new(
                if negative {
                    -result_max
                } else {
                    Number::zero()
                },
                result_max,
            ))))
        }
        2 => {
            let result_max = loop {
                let x: Number = Input::new().with_prompt("Maximum result").interact()?;
                if x <= 0 {
                    println!("Error: Must be grater than zero.");
                } else {
                    break x;
                }
            };
            Ok(Some(Box::new(Sub::new(
                if negative {
                    -result_max
                } else {
                    Number::zero()
                },
                result_max,
            ))))
        }
        3 => {
            let factor_max = loop {
                let x: Number = Input::new().with_prompt("Max value of factor").interact()?;
                if x <= 0 {
                    println!("Error: Must be grater than zero.");
                } else {
                    break x;
                }
            };
            Ok(Some(Box::new(Mul::new(
                if negative {
                    -factor_max
                } else {
                    Number::zero()
                },
                factor_max,
            ))))
        }
        4 => return Ok(None),
        _ => unreachable!(),
    }
}

fn run_task<T: TaskProducer<Number = Number>>(
    task_producer: &mut T,
    number_of_tasks: u32,
) -> IoResult<()> {
    loop {
        let mut right = 0u32;
        let start = Instant::now();
        for _ in 0..number_of_tasks {
            let task = task_producer.next();
            let result = task
                .give_answer
                .call(Input::new().with_prompt(&task.text).interact()?);
            println!("{}", if result { "Right!" } else { "Wrong" });
            if result {
                right += 1;
            }
        }
        let duration = start.elapsed();
        println!("You got {}/{}", right, number_of_tasks);
        println!("It took {}.", {
            let all_secs = duration.as_secs();
            let hours = all_secs / 3600;
            let minits = (all_secs - hours * 3600) / 60;
            let secs = all_secs - hours * 3600 - minits * 60;
            let centis = duration.subsec_millis() / 10;
            format!(
                "{}{}{}.{}s",
                if hours != 0 {
                    format!("{}h", hours)
                } else {
                    "".to_string()
                },
                if minits != 0 {
                    format!("{}m", minits)
                } else {
                    "".to_string()
                },
                secs,
                centis
            )
        });
        print!("(Press enter to continue) ");
        stdout().flush()?;
        stdin().read_line(&mut String::new())?;
        println!();
        match menu(
            "What do you want to do now",
            &["Run the task again", "Back to main menu"],
        )?
        .into()
        {
            1 => continue,
            2 => break,
            _ => unreachable!(),
        }
    }
    Ok(())
}

/// Display a menu.
///
/// A list of alternatives is displayed along with an index for each alternative. The user selects an alternative by entering a number corresponding to an alternative.
/// Returns the index of the selected item + 1. Or the non zero based index.
fn menu(title: &str, choices: &[&str]) -> IoResult<NonZeroUsize> {
    loop {
        println!("{}\n", title);
        // Print choices along with the corresponding number.
        for (i, choice) in choices.iter().enumerate() {
            println!("{}) {}", i + 1, choice);
        }
        println!();
        match Input::new().interact() {
            Ok(x) if x > 0 && x <= choices.len() => {
                break Ok(unsafe { NonZeroUsize::new_unchecked(x) })
            }
            Ok(_) => {
                println!("Number not in valid range. Please try again.\n");
                continue;
            }
            Err(e) => break Err(e),
        }
    }
}
