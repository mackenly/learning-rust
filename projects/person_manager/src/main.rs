extern crate rusqlite;

use rusqlite::{params, backup, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    role: String,
}

fn main() -> Result<()> {
    let mut conn: Connection = Connection::open_in_memory()?;
    if database_exists() {
        load_database(&mut conn)?;
    }
    create_tables(&conn)?;

    clear_screen();

    println!("Welcome to the person manager app!");

    // input loop
    loop {
        let input: String = prompt_for_input("\nEnter command (help for options):");
        match input.as_ref() {
            "add" | "create" | "new" | "a" | "n" => {
                let person: Person = prompt_for_person();
                add_person(&conn, &person.name, &person.role)?;
            }
            "list" | "ls" | "l" => {
                print_people(&conn)?;
            }
            "update" | "edit" | "patch" | "u" => {
                let id: i32 = prompt_for_id();
                let person: std::result::Result<Person, rusqlite::Error> = get_person(&conn, id);
                let updated_person: Person = prompt_for_person();
                match person {
                    Ok(ref person) => {
                        let mut updated_person: Person = updated_person;
                        updated_person.id = person.id;
                        if updated_person.name == "" {
                            updated_person.name = person.name.clone();
                        }
                        if updated_person.role == "" {
                            updated_person.role = person.role.clone();
                        }
                        update_person(&conn, &updated_person)?;
                    }
                    Err(ref _error) => {
                        println!("Person not found");
                    }
                }
            }
            "delete" | "remove" | "del" | "rm" | "d" => {
                let id: i32 = prompt_for_id();
                let person: std::result::Result<Person, rusqlite::Error> = get_person(&conn, id);
                match person {
                    Ok(ref person) => {
                        delete_person(&conn, &person)?;
                    }
                    Err(ref _error) => {
                        println!("Person not found");
                    }
                }
            }
            "reset" | "r" => {
                conn.execute("DROP TABLE person", params![])?;
                create_tables(&conn)?;
            }
            "quit" | "exit" | "q" => {
                break;
            }
            "clear" | "cls" | "c" => {
                clear_screen();
            }
            "help" | "h" => {
                println!("Commands:");
                println!("  add, create, new, a, n - Add a person");
                println!("  list, ls, l - List people");
                println!("  update, edit, patch, u - Update a person");
                println!("  delete, remove, del, rm, d - Delete a person");
                println!("  quit, exit, q - Quit");
                println!("  help, h - Show this help message");
            }
            _ => {
                println!("Invalid command");
            }
        }
    }

    clear_screen();
    
    backup_database(&conn)?;

    Ok(())
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            name  TEXT NOT NULL,
            role  TEXT
        );",
        params![],
    )?;
    Ok(())
}

fn database_exists() -> bool {
    std::path::Path::new("my_database.db").exists()
}

fn backup_database(conn: &Connection) -> Result<()> {
    let mut disk_conn: Connection = Connection::open("my_database.db")?;
    let backup: backup::Backup<'_, '_> = backup::Backup::new(&conn, &mut disk_conn)?;
    backup.run_to_completion(5, std::time::Duration::from_millis(250), None)?;
    Ok(())
}

fn load_database(conn: &mut Connection) -> Result<()> {
    let disk_conn: Connection = Connection::open("my_database.db")?;
    let backup: backup::Backup<'_, '_> = backup::Backup::new(&disk_conn, conn)?;
    backup.run_to_completion(5, std::time::Duration::from_millis(250), None)?;
    Ok(())
}

fn add_person(conn: &Connection, name: &str, role: &str) -> Result<()> {
    let me: Person = Person {
        id: 0,
        name: name.to_string(),
        role: role.to_string(),
    };
    conn.execute(
        "INSERT INTO person (name, role) VALUES (?1, ?2)",
        (&me.name, &me.role),
    )?;
    Ok(())
}

fn print_people(conn: &Connection) -> Result<()> {
    let mut stmt: rusqlite::Statement<'_> = conn.prepare("SELECT id, name, role FROM person")?;
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            role: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("{:?} {:?} is a {:?}", person.as_ref().unwrap().id, person.as_ref().unwrap().name, person.as_ref().unwrap().role);
    }
    Ok(())
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn prompt_for_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    clear_screen();
    input.trim().to_string()
}

fn prompt_for_person() -> Person {
    let name: String = prompt_for_input("Enter name:");
    let role: String = prompt_for_input("Enter role:");
    Person {
        id: 0,
        name: name,
        role: role,
    }
}

fn prompt_for_id() -> i32 {
    let id: i32 = prompt_for_input("Enter id:").parse::<i32>().unwrap();
    id
}

fn update_person(conn: &Connection, person: &Person) -> Result<()> {
    conn.execute(
        "UPDATE person SET name = ?1, role = ?2 WHERE id = ?3",
        (&person.name, &person.role, &person.id),
    )?;
    Ok(())
}

fn delete_person(conn: &Connection, person: &Person) -> Result<()> {
    conn.execute(
        "DELETE FROM person WHERE id = ?1",
        (&person.id,),
    )?;
    Ok(())
}

fn get_person(conn: &Connection, id: i32) -> Result<Person> {
    let mut stmt: rusqlite::Statement<'_> = conn.prepare("SELECT id, name, role FROM person WHERE id = ?1")?;
    let person_iter = stmt.query_map(params![id], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            role: row.get(2)?,
        })
    })?;

    for person in person_iter {
        return person;
    }
    panic!("Person not found");
}