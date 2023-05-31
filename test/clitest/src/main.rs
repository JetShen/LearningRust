use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::io::{stdout, Write};


fn select(temp: &[&str]) -> usize{
    let index = &temp.iter().position(|&elem| elem.contains("|----"));
    //convertir el index a u8
    let index = index.unwrap_or(0) + 1;
    let mut selected_index = index;

    selected_index
}


fn main() -> Result<()> {
    // Mensajes disponibles

    let mut temp = vec![];

    let messages = vec![
        "┌───────────────────────────┐",
        "│ Selector de interfaces    │",
        "|---------------------------|",
        "│ Interfaz 1                │",
        "│ Interfaz 2                │",
        "│ Interfaz 3                │",
        "│ Interfaz 4                │",
        "└───────────────────────────┘",
    ];
    let interfaz1 = vec![
        "┌───────────────────────────┐",
        "│ primera interfaz          │",
        "|---------------------------|",
        "| Opcion                    |",
        "└───────────────────────────┘",
    ];

    let interfaz2 = vec![
        "┌───────────────────────────┐",
        "│ segunda interfaz          │",
        "|---------------------------|",
        "| Opcion                    |",
        "└───────────────────────────┘",
    ];

    let interfaz3 = vec![
        "┌───────────────────────────┐",
        "│ tercera interfaz          │",
        "|---------------------------|",
        "| Opcion                    |",
        "└───────────────────────────┘",
    ];

    let interfaz4 = vec![
        "┌───────────────────────────┐",
        "│ cuarta interfaz           │",
        "|---------------------------|",
        "| Opcion                    |",
        "└───────────────────────────┘",
    ];

    
    'inicio:loop {
    
    
    static mut FIRST_TIME: bool = true;

    unsafe {
        if FIRST_TIME {
            temp = messages.clone();
            FIRST_TIME = false;
        }
    }
    
    // Inicializar la posición del mensaje en "focus"
    let mut selected_index = select(&temp);
    let mut index = selected_index;
    // Habilitar el modo raw de la terminal
    enable_raw_mode()?;

    // Mostrar los mensajes iniciales
    show_messages(&temp, selected_index)?;
    

    loop{
        // Leer eventos de teclado
        if let Event::Key(KeyEvent { code, modifiers }) = crossterm::event::read()? {
            // Comprobar la tecla presionada
            match code {
                KeyCode::Esc => break 'inicio, // Salir del programa al presionar 'Esc'
                KeyCode::Char('q') => {
                    temp = messages.clone();
                    continue 'inicio; // Volver al inicio del bucle al presionar 'q'
                }
                KeyCode::Up => {
                    // Mover la flecha hacia arriba
                    if selected_index > index {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    // Mover la flecha hacia abajo
                    if selected_index < temp.len() - 2 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    // Mostrar la interfaz seleccionada
                    match selected_index {
                        3 => {
                            temp = interfaz1.clone();
                            selected_index = select(&temp);
                            },
                        4 => {
                            temp = interfaz2.clone();
                            selected_index = select(&temp);
                            },
                        5 => {
                            temp = interfaz3.clone();
                            selected_index = select(&temp);
                            },
                        6 => {
                            temp = interfaz4.clone();
                            selected_index = select(&temp);
                            },
                        _ => {}
                    }
                }
                _ => {} // Ignorar otros eventos de teclado
            }

            // Mostrar los mensajes actualizados
            show_messages(&temp, selected_index)?;
        }
    }

    // Deshabilitar el modo raw de la terminal


} Ok(())

}

fn show_messages(temp: &[&str], selected_index: usize) -> Result<()> {
    // Limpiar la pantalla y mover el cursor al inicio
    execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
    execute!(stdout(), crossterm::cursor::MoveTo(0, 0))?;

    // Mostrar los mensajes
    for (index, message) in temp.iter().enumerate() {
        if index == selected_index {
            // Agregar una flecha al mensaje seleccionado
            println!("> {}", message);
        } else {
            println!("  {}", message);
        }
    }

    // Mostrar el cursor
    execute!(stdout(), Show)?;

    // Forzar la impresión inmediata en la pantalla
    stdout().flush()?;

    Ok(())
}
